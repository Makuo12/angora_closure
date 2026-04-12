use std::{env, fs, io::Write, path::{self, PathBuf}, sync::{Arc, RwLock, atomic::AtomicBool}};

use angora_common::config::MEM_LIMIT;
use log::{error, info};
use runtime::track;

use crate::{branches::GlobalBranches, check_dep, command::CommandOpt, depot::{self, Depot, sync_depot}, executor::Executor, fuzz_loop::fuzz_loop, stats::ChartStats};

pub fn fuzz_init() {
    println!("Init fuzzer");
    let cwd = env::current_dir().unwrap();
    let in_dir = cwd.join("../pdf");
    let angora_out_dir = std::env::current_dir().unwrap().join("angora_out");
    if !angora_out_dir.exists() {
        fs::create_dir_all(&angora_out_dir).unwrap();
    }
    let fast_path = cwd.join("../build_fast/pdftotext").to_str().unwrap().to_string();
    let track_path = cwd.join("../build_track/pdftotext").to_str().unwrap().to_string();
    println!("fast_path: {}", fast_path);
    println!("track_path: {}", track_path);
    let cmd_opt = CommandOpt::new(
        "llvm",
        "-",
        vec![],
        &angora_out_dir,
        (fast_path, vec![]),
        (track_path, vec![]),
        "gd",
        MEM_LIMIT,
        MEM_LIMIT,
        false,
        false
    );
    check_dep::check_dep(in_dir.to_str().unwrap(), angora_out_dir.to_str().unwrap(), &cmd_opt);
    let depot = Arc::new(Depot::new(in_dir, &angora_out_dir));
    println!("{:?}", depot.dirs);
    let stats = Arc::new(RwLock::new(ChartStats::new()));
    let global_branches = Arc::new(GlobalBranches::new());
    let _ = create_stats_file_and_write_pid(&angora_out_dir);
    let running = Arc::new(AtomicBool::new(true));
    let mut executor = Executor::new(
        cmd_opt.specify(0),
        global_branches.clone(),
        depot.clone(),
        stats.clone(),
    );

    sync_depot(&mut executor, running.clone(), &depot.dirs.seeds_dir);

    if depot.empty() {
        error!("Failed to find any branches during dry run.");
        error!("Please ensure that the binary has been instrumented and/or input directory is populated.");
        error!(
            "Please ensure that seed directory - {:?} has any file.",
            depot.dirs.seeds_dir
        );
        panic!();
    }
    fuzz_loop(running, cmd_opt, depot, global_branches, stats);
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