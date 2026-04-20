mod my_constants;

use std::cell::RefCell;
use std::collections::HashMap;

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

thread_local! {
    static PTR: RefCell<HashMap<Key, bool>> = RefCell::new(HashMap::new());
    static PTR_FILE: RefCell<HashMap<KeyFile, bool>> = RefCell::new(HashMap::new());
}

fn add_ptr(ptr: *mut c_void) {
    PTR.with(|map| {
        map.borrow_mut().insert(Key(ptr), true);
    });
}

fn remove_ptr(ptr: *mut c_void) -> u8 {
    let mut value = 0;
    PTR.with(|map| {
        if map.borrow_mut().remove(&Key(ptr)).is_some() {
            value = 1;
        }
    });
    value
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn myMalloc(size: usize) -> *mut c_void {
    let ptr = unsafe { libc::malloc(size) };
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    add_ptr(ptr);
    // println!("myMalloc: {:p}", ptr);
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn myCalloc(nobj: usize, size: usize) -> *mut c_void {
    let ptr = unsafe { libc::calloc(nobj, size) };
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    add_ptr(ptr);
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn myRealloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    let new_ptr = unsafe { libc::realloc(ptr, size) };
    if new_ptr.is_null() {
        return std::ptr::null_mut();
    }
    remove_ptr(ptr);
    add_ptr(new_ptr);
    new_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn myFree(ptr: *mut c_void) {
    if remove_ptr(ptr) == 1 {
        // println!("myFree: {:p}", ptr);
        unsafe { libc::free(ptr) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_ptrs() {
    PTR.with(|map| {
        let count = map.borrow().len();
        // println!("free_ptrs: cleaning up {} pointers", count);
        for (key, _) in map.borrow_mut().drain() {
            unsafe { libc::free(key.0) };
        }
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fopen_hook(pathname: *const c_char, mode: *const c_char) -> *mut FILE {
    let fp = unsafe { libc::fopen(pathname, mode) };
    if fp.is_null() {
        return std::ptr::null_mut();
    }
    PTR_FILE.with(|map| {
        map.borrow_mut().insert(KeyFile(fp), true);
    });
    fp
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fclose_hook(fp: *mut FILE) -> c_int {
    let result = unsafe { libc::fclose(fp) };
    if result == 0 {
        PTR_FILE.with(|map| {
            map.borrow_mut().remove(&KeyFile(fp));
        });
    }
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_open_file_handles() {
    PTR_FILE.with(|map| {
        let count = map.borrow().len();
        // println!("close_open_file_handles: closing {} file handles", count);
        for (key, _) in map.borrow_mut().drain() {
            unsafe { libc::fclose(key.0) };
        }
    });
}
