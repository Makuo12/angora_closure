use std::ffi::c_int;

use libc::{SIGABRT, SIGBUS, SIGFPE, SIGILL, SIGSEGV, SIGTRAP};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusType {
    Normal,
    Timeout,
    Crash,
    Skip,
    Error,
}


impl StatusType {
    pub fn from_handle_fuzz(result: c_int) -> Self {
        match result {
            0 => StatusType::Normal,
            // crash signals
            n if n == SIGSEGV => StatusType::Crash,
            n if n == SIGBUS => StatusType::Crash,
            n if n == SIGFPE => StatusType::Crash,
            n if n == SIGILL => StatusType::Crash,
            n if n == SIGABRT => StatusType::Crash,
            n if n == SIGTRAP => StatusType::Crash,
            // any other non-zero result is an error
            _ => StatusType::Error,
        }
    }
}
