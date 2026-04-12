pub const FUZZER_FILES: [&'static str; 3] = ["command_fuzz", "coverage_fuzz", "mutate_fuzz"];

pub const FUZZER_TYPE: &str = "FUZZER_TYPE";

#[cfg(target_os = "macos")]
pub const CLOSURE_GLOBAL_SECTION: &str = "__DATA,__cls_glob";

#[cfg(target_os = "linux")]
pub const CLOSURE_GLOBAL_SECTION: &str = ".cls_glob";
