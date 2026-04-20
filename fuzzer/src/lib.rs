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
mod fuzz_init;
mod fuzz_loop;
mod fuzz_main;
mod fuzz_type;
mod mut_input;
mod search;
mod stats;
pub mod track;

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
    unsafe fn set_angora_area_ptr(ptr: *const u8);
    unsafe fn set_angora_cmpid(ptr: u32);
    unsafe fn __angora_reset_context();
    unsafe fn close_open_file_handles();
    unsafe fn free_ptrs();
}
