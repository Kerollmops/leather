use std::ffi::CString;
use std::ptr;

use cowsql_raft_sys::{
    raft_event, raft_event__bindgen_ty_1, raft_event__bindgen_ty_1__bindgen_ty_1, RAFT_START,
};
use leather::Raft;

// static const char *address = "127.0.0.1:8080";
// static const char *dir = "/tmp/raft-quick-start";
// static struct raft_configuration conf;

fn main() {
    let address = CString::new("127.0.0.1:8800").unwrap();
    let mut raft = Raft::new(12, &address).unwrap();

    let mut event = raft_event {
        time: 0,
        type_: RAFT_START,
        unused: 0,
        capacity: 10, // ???
        reserved: [0; 4],
        __bindgen_anon_1: raft_event__bindgen_ty_1 {
            start: raft_event__bindgen_ty_1__bindgen_ty_1 {
                term: 0,
                voted_for: 0,
                metadata: ptr::null_mut(),
                start_index: 0,
                entries: ptr::null_mut(),
                n_entries: 0,
            },
        },
    };

    let update = raft.step(&mut event);
    dbg!(update).unwrap();
}
