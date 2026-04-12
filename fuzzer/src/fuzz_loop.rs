use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};

use log::trace;
use rand::thread_rng;

use crate::{
    branches::GlobalBranches, command::CommandOpt, depot::Depot, executor::Executor, handle_closure_init, search::{GdSearch, SearchHandler}, set_crash_handler, stats::ChartStats
};



pub fn fuzz_loop(
    running: Arc<AtomicBool>,
    cmd_opt: CommandOpt,
    depot: Arc<Depot>,
    global_branches: Arc<GlobalBranches>,
    global_stats: Arc<RwLock<ChartStats>>,
) {
    // let search_method = cmd_opt.search_method;
    let mut executor = Executor::new(
        cmd_opt,
        global_branches,
        depot.clone(),
        global_stats.clone(),
    );
    unsafe {
        handle_closure_init();
        set_crash_handler();
    }
    println!("starting loop");
    while running.load(Ordering::Relaxed) {
        println!("in loop");
        let entry = match depot.get_entry() {
            Some(e) => e,
            None => break,
        };
        let mut cond = entry.0;
        let priority = entry.1;
        println!("1 in loop");
        if priority.is_done() {
            break;
        }

        if cond.is_done() {
            depot.update_entry(cond);
            continue;
        }
        println!("2 in loop");
        trace!("{:?}", cond);

        let belong_input = cond.base.belong as usize;
        let buf = depot.get_input_buf(belong_input);
        {
            // let fuzz_type = cond.get_fuzz_type();
            println!("4 in loop");
            let handler = SearchHandler::new(running.clone(), &mut executor, &mut cond, buf);
            GdSearch::new(handler).run(&mut thread_rng());
        }
    }
}
