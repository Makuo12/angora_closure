use std::ffi::{CStr, CString, c_int};

use crate::fuzz_init::fuzz_init;

#[derive(Debug)]
pub struct Args {
    pub mode: Option<String>,
    pub input_dir: Option<String>,
    pub output_dir: Option<String>,
    pub track_target: Option<String>,
    pub fast_target: Option<String>,
    pub normal_target: Option<String>,

}

impl Args {
    fn new() -> Self {
        return Self {
            mode: Option::None, 
            input_dir: Option::None, 
            output_dir: Option::None,
            track_target: Option::None,
            fast_target: Option::None,
            normal_target:Option::None
        }
    }
}

// ./switch2.fast -m llvm -i ../input -o ../output -t ./switch2.taint -f ./switch2.fast -n ./switch2_main.fast

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_fuzz_init(argc: c_int, argv: *const *const char) {
    // your actual fuzzing logic
    let mut args: Args = Args::new();
    let mut i = 0; 
    while i < argc as usize {
        let ptr = *argv.add(i) as *const i8;
        if ptr.is_null() {
            break;
        }
        let value = CStr::from_ptr(ptr).to_string_lossy();
        let next_arg = | index: usize | -> Option<String> {
            if (index + 1) >= argc as usize {
                return Option::None
            } 
            let next_ptr = *argv.add(index+1) as *const i8;
            if next_ptr.is_null() {
                return Option::None
            }
            let arg = CStr::from_ptr(next_ptr).to_string_lossy().to_string();
            return Some(arg);
        };
        match value.as_ref() {
            "-m" => {
                args.mode = next_arg(i);
                i += 1;
            }
            "-i" => {
                args.input_dir = next_arg(i);
                i += 1;
            }
            "-o" => {
                args.output_dir = next_arg(i);
                i += 1;
            }
            "-t" => {
                args.track_target = next_arg(i);
                i += 1;
            }
            "-f" => {
                args.fast_target = next_arg(i);
                i += 1;
            }
            "-n" => {
                args.normal_target = next_arg(i);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    fuzz_init(argc as i32, args);
}
