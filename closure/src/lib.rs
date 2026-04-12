mod my_constants;

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use libc::{FILE, c_char, c_int};
use std::ffi::c_void;


#[derive(Eq, Hash, PartialEq)]
struct Key(*mut c_void);
unsafe impl Send for Key {}
unsafe impl Sync for Key {}

#[derive(Eq, Hash, PartialEq)]
struct KeyFile(*mut FILE);
unsafe impl Send for KeyFile {}
unsafe impl Sync for KeyFile {}

static PTR: OnceLock<Mutex<HashMap<Key, bool>>> = OnceLock::new();
static PTR_FILE: OnceLock<Mutex<HashMap<KeyFile, bool>>> = OnceLock::new();


fn add_ptr(ptr: *mut c_void) {
    let key = Key(ptr);
    let map = PTR.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map_lock = map.lock().unwrap();
    map_lock.insert(key, true);
}

fn remove_ptr(ptr: *mut c_void) -> u8 {
    let key = Key(ptr);
    if let Some(map) = PTR.get() {
        let mut map_lock = match map.lock() {
            Ok(lock) => lock,
            Err(_) => return 0,
        };
        // remove() returns Some(value) if key existed, None if not
        return match map_lock.remove(&key) {
            Some(_) => 1, // key found and removed
            None => 0,    // key didn't exist
        };
    }
    0 // map not initialized
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn myMalloc(size: usize) -> *mut c_void {
    let ptr: *mut c_void = unsafe { libc::malloc(size) };
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    add_ptr(ptr);
    return ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn myCalloc(nobj: usize, size: usize) -> *mut c_void {
    let ptr: *mut c_void = unsafe { libc::calloc(nobj, size) };
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    add_ptr(ptr);
    return ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn myRealloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    let new_ptr: *mut c_void = unsafe { libc::realloc(ptr, size) };
    if new_ptr.is_null() {
        return std::ptr::null_mut();
    }
    add_ptr(new_ptr);
    return new_ptr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn myFree(ptr: *mut c_void) {
    unsafe { libc::free(ptr) };
    remove_ptr(ptr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_ptrs() {
    if let Some(map) = PTR.get() {
        if let Ok(ptrs) = map.lock() {
            for (key, _) in ptrs.iter() {
                unsafe { libc::free(key.0) };
            }
        }
    }
}

// Closing Open File Handles
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fopen_hook(pathname: *const c_char, mode: *const c_char) -> *mut FILE {
    let fp = unsafe { libc::fopen(pathname, mode) };
    if fp.is_null() {
        return std::ptr::null_mut();
    }
    let map = PTR_FILE.get_or_init(|| Mutex::new(HashMap::new())).lock();
    if let Ok(mut map_lock) = map {
        map_lock.insert(KeyFile(fp), true);
    }
    return fp;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fclose_hook(fp: *mut FILE) -> c_int {
    let result: c_int = unsafe { libc::fclose(fp) };
    if result == 0 {
        let key = KeyFile(fp);
        if let Some(map) = PTR_FILE.get() {
            if let Ok(mut map_lock) = map.lock() {
                map_lock.remove(&key);
            }
        }
    }
    return result;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_open_file_handles() {
    if let Some(map) = PTR_FILE.get() {
        if let Ok(files) = map.lock() {
            for (key, _) in files.iter() {
                unsafe { libc::fclose(key.0) };
            }
        }
    }
}
