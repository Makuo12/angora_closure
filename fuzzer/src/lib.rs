#![cfg_attr(feature = "unstable", feature(core_intrinsics))]

use std::ffi::{c_char, c_int};

use runtime::DfsanLabel;

#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;

mod branches;
pub mod cond_stmt;
mod depot;
pub mod executor;
mod mut_input;
mod search;
mod stats;
pub mod track;

mod fuzz_loop;
mod fuzz_main;
mod fuzz_init;
mod fuzz_type;

mod bind_cpu;
mod check_dep;
mod command;
mod tmpfs;

unsafe extern "C" {
    unsafe fn dfsan_read_label(addr: *const i8, size: usize) -> DfsanLabel;
    unsafe fn handle_closure_init();
    unsafe fn handle_closure_reset();
    unsafe fn set_crash_handler();
    unsafe fn handle_fuzz(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

