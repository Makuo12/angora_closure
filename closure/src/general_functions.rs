use inkwell::module::Module;
use std::ffi::c_void;
use std::os::raw::c_int;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::my_constants::FUZZER_FILES;

pub fn get_or_insert_function<'ctx>(
    module: &Module<'ctx>,
    name: &str,
    original: inkwell::values::FunctionValue<'ctx>,
) -> inkwell::values::FunctionValue<'ctx> {
    module
        .get_function(name)
        .unwrap_or_else(|| module.add_function(name, original.get_type(), None))
}

pub fn is_fuzzer_file(file_name: &str) -> bool {
    FUZZER_FILES
        .iter()
        .any(|&fuzzer_file| file_name.contains(fuzzer_file))
}
