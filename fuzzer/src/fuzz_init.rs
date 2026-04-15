use std::{env, fs, io::Write, path::{self, PathBuf}, sync::{Arc, RwLock, atomic::{AtomicBool, Ordering}}};

use angora_common::{config::{MEM_LIMIT, TIME_LIMIT}, defs};
use chrono::Local;
use log::{error, info};
use runtime::track;

use crate::{branches::{self, GlobalBranches}, check_dep, command::CommandOpt, depot::{self, Depot, sync_depot}, executor::{self, Executor}, fuzz_loop::fuzz_loop, handle_closure_init, set_crash_handler, stats::{self, ChartStats}};

pub fn fuzz_init() {
    pretty_env_logger::init();
    let in_dir = "../pdf";
    let cwd = env::current_dir().unwrap();
    let track_path = cwd.join("../build_track/pdftotext.taint").to_str().unwrap().to_string();
    let (seeds_dir, angora_out_dir) = initialize_directories(
        in_dir, "angora_out", false
    );
    let fast_path = cwd.join("../build_fast/pdftotext.fast").to_str().unwrap().to_string();
    let command_option = CommandOpt::new(
        "llvm",
        &track_path,
        vec![fast_path, "@@".to_string()],  // binary + input placeholder
        &angora_out_dir,
        "gd",
        MEM_LIMIT,
        TIME_LIMIT,
        false,
        false,
    );
    unsafe {
        handle_closure_init();
    }
    // println!("{:?}", command_option);
    check_dep::check_dep(in_dir, &angora_out_dir.to_str().unwrap(), &command_option);
    let depot = Arc::new(depot::Depot::new(seeds_dir, &angora_out_dir));
    // println!("{:?}", depot.dirs);
    let stats = Arc::new(RwLock::new(stats::ChartStats::new()));
    let global_branches = Arc::new(branches::GlobalBranches::new());
    let fuzzer_stats = create_stats_file_and_write_pid(&angora_out_dir);
    let running = Arc::new(AtomicBool::new(true));
    set_sigint_handler(running.clone());
    let mut executor = executor::Executor::new(
        command_option.specify(0),
        global_branches.clone(),
        depot.clone(),
        stats.clone(),
    );
    depot::sync_depot(&mut executor, running.clone(), &depot.dirs.seeds_dir);
    if depot.empty() {
        error!("Failed to find any branches during dry run.");
        error!("Please ensure that the binary has been instrumented and/or input directory is populated.");
        error!(
            "Please ensure that seed directory - {:?} has any file.",
            depot.dirs.seeds_dir
        );
        panic!();
    }
    let r = running.clone();
    let cmd = command_option.specify(1);
    let d = depot.clone();
    let b = global_branches.clone();
    let s = stats.clone();
    fuzz_loop(r, cmd, d, b, s);
}

fn set_sigint_handler(r: Arc<AtomicBool>) {
    ctrlc::set_handler(move || {
        warn!("Ending Fuzzing.");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting SIGINT handler!");
}



fn create_stats_file_and_write_pid(angora_out_dir: &PathBuf) -> PathBuf {
    // To be compatible with AFL.
    let fuzzer_stats = angora_out_dir.join("fuzzer_stats");
    let pid = unsafe { libc::getpid() as usize };
    let mut buffer = match fs::File::create(&fuzzer_stats) {
        Ok(a) => a,
        Err(e) => {
            error!("Could not create stats file: {:?}", e);
            panic!();
        },
    };
    write!(buffer, "fuzzer_pid : {}", pid).expect("Could not write to stats file");
    fuzzer_stats
}

fn initialize_directories(in_dir: &str, out_dir: &str, sync_afl: bool) -> (PathBuf, PathBuf) {
    let angora_out_dir = PathBuf::from(out_dir);

    let restart = in_dir == "-";
    if !restart {
        fs::create_dir(&angora_out_dir).expect("Output directory has existed!");
    }

    let out_dir = &angora_out_dir;
    let seeds_dir = if restart {
        let orig_out_dir = out_dir.with_extension(Local::now().to_rfc3339());
        fs::rename(&out_dir, orig_out_dir.clone()).unwrap();
        fs::create_dir(&out_dir).unwrap();
        PathBuf::from(orig_out_dir).join(defs::INPUTS_DIR)
    } else {
        PathBuf::from(in_dir)
    };

    (seeds_dir, angora_out_dir)
}
