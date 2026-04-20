use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
};

use rand::thread_rng;
use runtime_fast::shm_conds::SHM_CONDS;

use crate::{
    branches::GlobalBranches,
    command::CommandOpt,
    cond_stmt::NextState,
    depot::Depot,
    executor::Executor,
    fuzz_type::FuzzType,
    search::{
        AFLFuzz, CbhSearch, DetFuzz, ExploitFuzz, FnFuzz, GdSearch, LenFuzz, MbSearch, OneByteFuzz,
        RandomSearch, SearchHandler, SearchMethod,
    },
    set_crash_handler,
    stats::ChartStats,
};

pub fn fuzz_loop(
    running: Arc<AtomicBool>,
    cmd_opt: CommandOpt,
    depot: Arc<Depot>,
    global_branches: Arc<GlobalBranches>,
    global_stats: Arc<RwLock<ChartStats>>,
) {
    let search_method = cmd_opt.search_method;
    let mut executor = Executor::new(
        cmd_opt,
        global_branches,
        depot.clone(),
        global_stats.clone(),
    );
    executor.map_branch_counting_shm();
    executor.cond_shm();
    executor.reset_shm_conds();
    while running.load(Ordering::Relaxed) {
        let entry = match depot.get_entry() {
            Some(e) => e,
            None => {
                break;
            }
        };

        let mut cond = entry.0;
        let priority = entry.1;
        if priority.is_done() {
            break;
        }

        if cond.is_done() {
            depot.update_entry(cond);
            continue;
        }
        trace!("{:?}", cond);
        let belong_input = cond.base.belong as usize;
        let buf = depot.get_input_buf(belong_input);
        {
            let fuzz_type = cond.get_fuzz_type();
            let handler = SearchHandler::new(running.clone(), &mut executor, &mut cond, buf);
            match fuzz_type {
                FuzzType::ExploreFuzz => {
                    if handler.cond.is_time_expired() {
                        handler.cond.next_state();
                    }
                    if handler.cond.state.is_one_byte() {
                        OneByteFuzz::new(handler).run();
                    } else if handler.cond.state.is_det() {
                        DetFuzz::new(handler).run();
                    } else {
                        match search_method {
                            SearchMethod::Gd => {
                                GdSearch::new(handler).run(&mut thread_rng());
                            }
                            SearchMethod::Random => {
                                RandomSearch::new(handler).run();
                            }
                            SearchMethod::Cbh => {
                                CbhSearch::new(handler).run();
                            }
                            SearchMethod::Mb => {
                                MbSearch::new(handler).run();
                            }
                        }
                    }
                }
                FuzzType::ExploitFuzz => {
                    if handler.cond.state.is_one_byte() {
                        let mut fz = OneByteFuzz::new(handler);
                        fz.run();
                        fz.handler.cond.to_unsolvable();
                    } else {
                        ExploitFuzz::new(handler).run();
                    }
                }
                FuzzType::AFLFuzz => {
                    AFLFuzz::new(handler).run();
                }
                FuzzType::LenFuzz => {
                    LenFuzz::new(handler).run();
                }
                FuzzType::CmpFnFuzz => {
                    FnFuzz::new(handler).run();
                }
                FuzzType::OtherFuzz => {
                    warn!("Unknown fuzz type!!");
                }
            }
        }

        depot.update_entry(cond);
    }
}
