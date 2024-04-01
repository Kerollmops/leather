use std::ffi::CStr;
use std::{mem, ptr};

use cowsql_raft_sys::{raft, raft_close, raft_event, raft_id, raft_init, raft_step, raft_update};

pub struct Raft(raft);

impl Raft {
    pub fn new<A: AsRef<CStr>>(id: raft_id, address: A) -> Result<Raft, ()> {
        let mut r = mem::MaybeUninit::uninit();
        match unsafe {
            raft_init(
                r.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                id,
                address.as_ref().as_ptr(),
            )
        } {
            0 => Ok(Raft(unsafe { r.assume_init() })),
            _ => Err(()),
        }
    }

    pub fn step(&mut self, event: &mut raft_event) -> Result<raft_update, ()> {
        let mut update = mem::MaybeUninit::uninit();
        match unsafe { raft_step(&mut self.0, event, update.as_mut_ptr()) } {
            0 => Ok(unsafe { update.assume_init() }),
            _ => Err(()),
        }
    }
}

impl Drop for Raft {
    fn drop(&mut self) {
        unsafe { raft_close(&mut self.0, None) }
    }
}
