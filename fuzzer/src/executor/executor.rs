use crate::{
    branches, command::{self, CommandOpt}, cond_stmt::{self, NextState, ShmConds}, depot::{self, Depot}, executor::{StatusType, limit::SetLimit, pipe_fd::PipeFd}, handle_closure_reset, handle_fuzz, stats::{self, ChartStats, LocalStats, TimeIns}, track::{self, load_track_data}
};
use angora_common::{config, defs};

use std::{
    collections::HashMap, ffi::{CString, c_char, c_int}, path::Path, process::{Command, Stdio}, sync::{
        Arc, RwLock, atomic::{Ordering, compiler_fence}
    }, time
};
use wait_timeout::ChildExt;

pub struct Executor {
    pub cmd: CommandOpt,
    pub has_new_path: bool,
    pub t_conds: ShmConds,
    pub branches: branches::Branches,
    pub local_stats: LocalStats,
    fd: PipeFd,
    tmout_cnt: usize,
    envs: HashMap<String, String>,
    pub invariable_cnt: usize,
    pub last_f: u64,
    depot: Arc<Depot>,
    pub global_stats: Arc<RwLock<ChartStats>>,
}

impl Executor {
    pub fn new(
        cmd: CommandOpt,
        global_branches: Arc<branches::GlobalBranches>,
        depot: Arc<Depot>,
        global_stats: Arc<RwLock<ChartStats>>,
    ) -> Self {
        let t_conds = ShmConds::new();
        let branches = branches::Branches::new(global_branches);
        let mut envs = HashMap::new();
        envs.insert(
            defs::ASAN_OPTIONS_VAR.to_string(),
            defs::ASAN_OPTIONS_CONTENT.to_string(),
        );
        envs.insert(
            defs::MSAN_OPTIONS_VAR.to_string(),
            defs::MSAN_OPTIONS_CONTENT.to_string(),
        );
        envs.insert(
            defs::BRANCHES_SHM_ENV_VAR.to_string(),
            branches.get_id().to_string(),
        );
        envs.insert(
            defs::COND_STMT_ENV_VAR.to_string(),
            t_conds.get_id().to_string(),
        );
        envs.insert(
            defs::LD_LIBRARY_PATH_VAR.to_string(),
            cmd.ld_library.clone(),
        );
        let fd = PipeFd::new(&cmd.out_file);
        Self {
            has_new_path: false,
            branches,
            depot,
            envs,
            t_conds,
            tmout_cnt: 0,
            global_stats,
            local_stats: LocalStats::default(),
            invariable_cnt: 0,
            last_f: defs::UNREACHABLE,
            cmd,
            fd,
        }
    }
    fn run_init(&mut self) {
        self.has_new_path = false;
    }
    pub fn run_with_cond(
        &mut self,
        buf: &Vec<u8>,
        cond: &mut cond_stmt::CondStmt,
    ) -> (StatusType, u64) {
        self.run_init();
        self.t_conds.set(cond);
        let mut status = self.run_inner(buf);
        let output = self.t_conds.get_cond_output();
        let mut explored = false;
        let mut skip = false;
        skip |= self.check_explored(cond, status, output, &mut explored);
        skip |= self.check_invariable(output, cond);
        self.check_consistent(output, cond);
        self.do_if_has_new(buf, status, explored, cond.base.cmpid);
        status = self.check_timeout(status, cond);

        if skip {
            status = StatusType::Skip;
        }

        (status, output)
    }
    pub fn run_sync(&mut self, buf: &Vec<u8>) {
        self.run_init();
        let status = self.run_inner(buf);
        self.do_if_has_new(buf, status, false, 0);
    }
    fn check_timeout(&mut self, status: StatusType, cond: &mut cond_stmt::CondStmt) -> StatusType {
        // let mut ret_status = status;
        // if ret_status == StatusType::Error {
        //     self.rebind_forksrv();
        //     ret_status = StatusType::Timeout;
        // }

        // if ret_status == StatusType::Timeout {
        //     self.tmout_cnt = self.tmout_cnt + 1;
        //     if self.tmout_cnt >= config::TMOUT_SKIP {
        //         cond.to_timeout();
        //         ret_status = StatusType::Skip;
        //         self.tmout_cnt = 0;
        //     }
        // } else {
        //     self.tmout_cnt = 0;
        // };

        // ret_status
        StatusType::Crash
    }
    fn run_inner(&mut self, buf: &Vec<u8>) -> StatusType {
        self.write_test(buf);
        self.branches.clear_trace();
        compiler_fence(Ordering::SeqCst);
        // Path to fast fuzzer without taint tracking.
        let ret_status = self.call_fuzzer();
        compiler_fence(Ordering::SeqCst);
        ret_status
    }
    fn call_fuzzer(&mut self) -> StatusType {
        let ret_status: i32;
        let main_cstr = CString::new(self.cmd.main.0.as_str()).unwrap();
        let out_cstr = CString::new(self.cmd.out_file.as_str()).unwrap();
        let mut args: Vec<*mut c_char> = vec![
            main_cstr.as_ptr() as *mut c_char,
            out_cstr.as_ptr() as *mut c_char,
            std::ptr::null_mut(),
        ];
        let argv: *mut *mut c_char = args.as_mut_ptr();
        let argc: c_int = (args.len() - 1) as c_int;
        unsafe {
            let result = handle_fuzz(argc, argv);
            ret_status = result
        }
        unsafe { handle_closure_reset() };
        return StatusType::from_handle_fuzz(ret_status);
    }
    fn do_if_has_new(&mut self, buf: &Vec<u8>, status: StatusType, _explored: bool, cmpid: u32) {
        // new edge: one byte in bitmap
        let (has_new_path, has_new_edge, edge_num) = self.branches.has_new(status);
        if has_new_path {
            self.has_new_path = true;
            self.local_stats.find_new(&status);
            let id = self.depot.save(status, &buf, cmpid);
            if status == StatusType::Normal {
                self.local_stats.avg_edge_num.update(edge_num as f32);
                let speed = self.count_time();
                let speed_ratio = self.local_stats.avg_exec_time.get_ratio(speed as f32);
                self.local_stats.avg_exec_time.update(speed as f32);

                // Avoid track slow ones
                if (!has_new_edge && speed_ratio > 10 && id > 10) || (speed_ratio > 25 && id > 10) {
                    println!(
                        "Skip tracking id {}, speed: {}, speed_ratio: {}, has_new_edge: {}",
                        id, speed, speed_ratio, has_new_edge
                    );
                    return;
                }
                let crash_or_tmout = self.try_unlimited_memory(buf, cmpid);
                if !crash_or_tmout {
                    let cond_stmts = self.track(id, buf, speed);
                    if cond_stmts.len() > 0 {
                        self.depot.add_entries(cond_stmts);
                        // if self.cmd.enable_afl {
                        //     self.depot
                        //         .add_entries(vec![cond_stmt::CondStmt::get_afl_cond(
                        //             id, speed, edge_num,
                        //         )]);
                        // }
                    }
                }
            }
        }
    }
    fn count_time(&mut self) -> u32 {
        let t_start = time::Instant::now();
        for _ in 0..3 {
            // if self.cmd.is_stdin {
            //     self.fd.rewind();
            // }
            // self.run_target(&main_path, self.cmd.mem_limit, self.cmd.time_limit);
            let _ = self.call_fuzzer();
        }
        let used_t = t_start.elapsed();
        let used_us = (used_t.as_secs() as u32 * 1000_000) + used_t.subsec_nanos() / 1_000;
        used_us / 3
    }
    fn try_unlimited_memory(&mut self, buf: &Vec<u8>, cmpid: u32) -> bool {
        let mut skip = false;
        self.branches.clear_trace();
        // Not need we are not reading from stdin.
        // if self.cmd.is_stdin {
        //     self.fd.rewind();
        // }
        compiler_fence(Ordering::SeqCst);
        // Call main
        // let unmem_status = self.run_target(&main_path, config::MEM_LIMIT_TRACK, self.cmd.time_limit);
        let unmem_status = self.call_fuzzer();

        compiler_fence(Ordering::SeqCst);
        // find difference
        if unmem_status != StatusType::Normal {
            skip = true;
            warn!(
                "Behavior changes if we unlimit memory!! status={:?}",
                unmem_status
            );
            // crash or hang
            if self.branches.has_new(unmem_status).0 {
                self.depot.save(unmem_status, &buf, cmpid);
            }
        }
        skip
    }
    pub fn run(&mut self, buf: &Vec<u8>, cond: &mut cond_stmt::CondStmt) -> StatusType {
        self.run_init();
        let status = self.run_inner(buf);
        self.do_if_has_new(buf, status, false, 0);
        self.check_timeout(status, cond)
    }
    fn check_consistent(&self, output: u64, cond: &mut cond_stmt::CondStmt) {
        // This function checks if the branch is "fuzzable" under current conditions.
        // The Logic: If the very first time (num_exec == 1) we run the program, the distance is UNREACHABLE,
        // something is wrong.
        if output == defs::UNREACHABLE
            && cond.is_first_time()
            && self.local_stats.num_exec == 1.into()
            && cond.state.is_initial()
        {
            cond.is_consistent = false;
            warn!("inconsistent : {:?}", cond);
        }
    }
    fn check_invariable(&mut self, output: u64, cond: &mut cond_stmt::CondStmt) -> bool {
        let mut skip = false;
        if output == self.last_f {
            self.invariable_cnt += 1;
            if self.invariable_cnt >= config::MAX_INVARIABLE_NUM {
                debug!("output is invariable! f: {}", output);
                if cond.is_desirable {
                    cond.is_desirable = false;
                }
                // deterministic will not skip
                if !cond.state.is_det() && !cond.state.is_one_byte() {
                    skip = true;
                }
            }
        } else {
            self.invariable_cnt = 0;
        }
        self.last_f = output;
        skip
    }
    fn check_explored(
        &self,
        cond: &mut cond_stmt::CondStmt,
        _status: StatusType,
        output: u64,
        explored: &mut bool,
    ) -> bool {
        let mut skip = false;
        // If crash or timeout, constraints after the point won't be tracked.
        if output == 0 && !cond.is_done()
        //&& status == StatusType::Normal
        {
            debug!("Explored this condition!");
            skip = true;
            *explored = true;
            cond.mark_as_done();
        }
        skip
    }
    fn track(&mut self, id: usize, buf: &Vec<u8>, speed: u32) -> Vec<cond_stmt::CondStmt> {
        self.envs.insert(
            defs::TRACK_OUTPUT_VAR.to_string(),
            self.cmd.track_path.clone(),
        );

        let t_now: TimeIns = Default::default();

        self.write_test(buf);
        compiler_fence(Ordering::SeqCst);
        let ret_status = self.run_target_cmd(
            &self.cmd.track,
            config::MEM_LIMIT_TRACK,
            //self.cmd.time_limit *
            config::TIME_LIMIT_TRACK,
        );
        compiler_fence(Ordering::SeqCst);

        if ret_status != StatusType::Normal {
            error!(
                "Crash or hang while tracking! -- {:?},  id: {}",
                ret_status, id
            );
            return vec![];
        }

        let cond_list = load_track_data(
            Path::new(&self.cmd.track_path),
            id as u32,
            speed,
            self.cmd.mode.is_pin_mode(),
            false,
        );

        self.local_stats.track_time += t_now.into();
        cond_list
    }
    fn write_test(&mut self, buf: &Vec<u8>) {
        self.fd.write_buf(buf);
        // if self.cmd.is_stdin {
        //     self.fd.rewind();
        // }
    }
    pub fn update_log(&mut self) {
        self.global_stats
            .write()
            .unwrap()
            .sync_from_local(&mut self.local_stats);

        self.t_conds.clear();
        self.tmout_cnt = 0;
        self.invariable_cnt = 0;
        self.last_f = defs::UNREACHABLE;
    }
    fn run_target_cmd(
        &self,
        target: &(String, Vec<String>),
        mem_limit: u64,
        time_limit: u64,
    ) -> StatusType {
        // call how own program directly from here.

        let mut cmd = Command::new(&target.0);
        let mut child = cmd
            .args(&target.1)
            .stdin(Stdio::null())
            .env_clear()
            .envs(&self.envs)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .mem_limit(mem_limit.clone())
            .setsid()
            .pipe_stdin(self.fd.as_raw_fd(), self.cmd.is_stdin)
            .spawn()
            .expect("Could not run target");

        let timeout = time::Duration::from_secs(time_limit);
        let ret = match child.wait_timeout(timeout).unwrap() {
            Some(status) => {
                if let Some(status_code) = status.code() {
                    if (self.cmd.uses_asan && status_code == defs::MSAN_ERROR_CODE)
                        || (self.cmd.mode.is_pin_mode() && status_code > 128)
                    {
                        StatusType::Crash
                    } else {
                        StatusType::Normal
                    }
                } else {
                    StatusType::Crash
                }
            }
            None => {
                // Timeout
                // child hasn't exited yet
                child.kill().expect("Could not send kill signal to child.");
                child.wait().expect("Error during waiting for child.");
                StatusType::Timeout
            }
        };
        ret
    }
}
