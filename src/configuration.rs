use std::ffi::CStr;
use std::mem;

use cowsql_raft_sys::{
    raft_configuration, raft_configuration_add, raft_configuration_close, raft_configuration_init,
    raft_id, RAFT_SPARE, RAFT_STANDBY, RAFT_VOTER,
};

pub struct Configuration(raft_configuration);

impl Configuration {
    pub fn new() -> Configuration {
        let mut conf = mem::MaybeUninit::uninit();
        unsafe {
            raft_configuration_init(conf.as_mut_ptr());
            Configuration(conf.assume_init())
        }
    }

    pub fn add<A: AsRef<CStr>>(&mut self, id: raft_id, address: A, role: Role) {
        unsafe { raft_configuration_add(&mut self.0, id, address.as_ref().as_ptr(), role as i32) };
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Configuration {
    fn drop(&mut self) {
        unsafe { raft_configuration_close(&mut self.0) };
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Role {
    /// Replicate log, does not participate in quorum.
    Standby = RAFT_STANDBY,
    /// Replicate log, does participate in quorum.
    Voter = RAFT_VOTER,
    /// Does not replicate log, or participate in quorum.
    Spare = RAFT_SPARE,
}
