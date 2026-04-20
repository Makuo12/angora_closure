use super::{forkcli, shm_branches, shm_conds};
use std::ops::DerefMut;

use std::sync::Once;

static START: Once = Once::new();

// #[ctor]
// fn fast_init() {
//     START.call_once(|| {
//         shm_branches::map_branch_counting_shm();
//         forkcli::start_forkcli();
//     });
// }

#[no_mangle]
pub extern "C" fn __angora_trace_cmp(
    condition: u32,
    cmpid: u32,
    context: u32,
    arg1: u64,
    arg2: u64,
) -> u32 {
    let conds = shm_conds::SHM_CONDS.lock().expect("SHM mutex poisoned.");
    let msg = match conds.as_ref() {
        Some(c) => format!(
            "TRACE_CMP: got cmpid={}, context={} | want cmpid={}, context={}, rt_order={}\n",
            cmpid, context, c.cond.cmpid, c.cond.context, c.rt_order
    ),
        None => format!("TRACE_CMP: got cmpid={}, context={} | SHM_CONDS=None\n", cmpid, context),
    };
    unsafe {
        libc::write(2, msg.as_ptr() as *const libc::c_void, msg.len());
    }
    drop(conds);
    let mut conds = shm_conds::SHM_CONDS.lock().expect("SHM mutex poisoned.");
    match conds.deref_mut() {
        &mut Some(ref mut c) => {
            if c.check_match(cmpid, context) {
                return c.update_cmp(condition, arg1, arg2);
            }
        },
        _ => {},
    }
    condition
}

#[no_mangle]
pub extern "C" fn __angora_trace_switch(cmpid: u32, context: u32, condition: u64) -> u64 {
    let mut conds = shm_conds::SHM_CONDS.lock().expect("SHM mutex poisoned.");
    match conds.deref_mut() {
        &mut Some(ref mut c) => {
            if c.check_match(cmpid, context) {
                return c.update_switch(condition);
            }
        },
        _ => {},
    }
    condition
}
