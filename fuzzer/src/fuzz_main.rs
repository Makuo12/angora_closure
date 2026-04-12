use crate::fuzz_init::fuzz_init;




#[unsafe(no_mangle)]
pub extern "C" fn rust_fuzz_init() {
    // your actual fuzzing logic
    fuzz_init();
}
