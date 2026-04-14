use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};

use rand::thread_rng;

use crate::{
    branches::GlobalBranches, command::CommandOpt, cond_stmt::NextState, depot::Depot, executor::Executor, fuzz_type::FuzzType, handle_closure_init, search::{AFLFuzz, CbhSearch, DetFuzz, ExploitFuzz, FnFuzz, GdSearch, LenFuzz, MbSearch, OneByteFuzz, RandomSearch, SearchHandler, SearchMethod}, set_crash_handler, stats::ChartStats
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
    
    println!("[fuzz_loop] starting loop");
    let mut iteration = 0usize;

    while running.load(Ordering::Relaxed) {
        iteration += 1;
        println!("[fuzz_loop] iteration={}", iteration);

        let entry = match depot.get_entry() {
            Some(e) => e,
            None => {
                println!("[fuzz_loop] depot.get_entry() returned None, breaking");
                break;
            }
        };

        let mut cond = entry.0;
        let priority = entry.1;
        println!("[fuzz_loop] got entry: cmpid={}, belong={}, state={:?}, priority={:?}",
            cond.base.cmpid, cond.base.belong, cond.state, priority);

        if priority.is_done() {
            println!("[fuzz_loop] priority is done, breaking");
            break;
        }

        if cond.is_done() {
            println!("[fuzz_loop] cond is done, skipping: cmpid={}", cond.base.cmpid);
            depot.update_entry(cond);
            continue;
        }

        let belong_input = cond.base.belong as usize;
        let buf = depot.get_input_buf(belong_input);
        println!("[fuzz_loop] belong_input={}, buf_len={}", belong_input, buf.len());

        {
            let fuzz_type = cond.get_fuzz_type();
            println!("[fuzz_loop] fuzz_type={:?}", fuzz_type);

            let handler = SearchHandler::new(running.clone(), &mut executor, &mut cond, buf);
            println!("[fuzz_loop] handler created, offsets={:?}", handler.cond.offsets);

            match fuzz_type {
                FuzzType::ExploreFuzz => {
                    println!("[fuzz_loop] ExploreFuzz: time_expired={}, state={:?}",
                        handler.cond.is_time_expired(), handler.cond.state);

                    if handler.cond.is_time_expired() {
                        println!("[fuzz_loop] time expired, advancing state");
                        handler.cond.next_state();
                    }

                    if handler.cond.state.is_one_byte() {
                        println!("[fuzz_loop] running OneByteFuzz");
                        OneByteFuzz::new(handler).run();
                    } else if handler.cond.state.is_det() {
                        println!("[fuzz_loop] running DetFuzz");
                        DetFuzz::new(handler).run();
                    } else {
                        println!("[fuzz_loop] running search method={:?}", search_method);
                        match search_method {
                            SearchMethod::Gd => {
                                GdSearch::new(handler).run(&mut thread_rng());
                            },
                            SearchMethod::Random => {
                                RandomSearch::new(handler).run();
                            },
                            SearchMethod::Cbh => {
                                CbhSearch::new(handler).run();
                            },
                            SearchMethod::Mb => {
                                MbSearch::new(handler).run();
                            },
                        }
                    }
                },
                FuzzType::ExploitFuzz => {
                    println!("[fuzz_loop] ExploitFuzz: state={:?}", handler.cond.state);
                    if handler.cond.state.is_one_byte() {
                        println!("[fuzz_loop] ExploitFuzz running OneByteFuzz");
                        let mut fz = OneByteFuzz::new(handler);
                        fz.run();
                        fz.handler.cond.to_unsolvable();
                    } else {
                        println!("[fuzz_loop] ExploitFuzz running ExploitFuzz");
                        ExploitFuzz::new(handler).run();
                    }
                },
                FuzzType::AFLFuzz => {
                    println!("[fuzz_loop] running AFLFuzz");
                    AFLFuzz::new(handler).run();
                },
                FuzzType::LenFuzz => {
                    println!("[fuzz_loop] running LenFuzz");
                    LenFuzz::new(handler).run();
                },
                FuzzType::CmpFnFuzz => {
                    println!("[fuzz_loop] running FnFuzz");
                    FnFuzz::new(handler).run();
                },
                FuzzType::OtherFuzz => {
                    println!("[fuzz_loop] OtherFuzz — unknown fuzz type, skipping");
                    warn!("Unknown fuzz type!!");
                },
            }
        }

        println!("[fuzz_loop] updating entry: cmpid={}, state={:?}", cond.base.cmpid, cond.state);
        depot.update_entry(cond);
    }

    println!("[fuzz_loop] loop exited after {} iterations", iteration);
}