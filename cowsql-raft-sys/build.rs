extern crate cc;

#[cfg(feature = "bindgen")]
extern crate bindgen;

#[cfg(feature = "bindgen")]
#[path = "bindgen.rs"]
mod generate;

use std::path::PathBuf;
use std::{env, fs};

// macro_rules! warn {
//     ($message:expr) => {
//         println!("cargo:warning={}", $message);
//     };
// }

fn main() {
    let raft = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("raft");
    let raft_src = raft.join("src");
    let raft_include = raft.join("include");

    // We don't want to use autoconf to simply change a single define in a header file
    let raft_header_in_content = fs::read_to_string(raft_include.join("raft.h.in")).unwrap();
    let raft_header_content = raft_header_in_content.replace("@enable_v0@", "no");
    fs::write(raft_include.join("raft.h"), raft_header_content).unwrap();

    #[cfg(feature = "bindgen")]
    generate::generate();

    let mut builder = cc::Build::new();

    builder
        .file(raft_src.join("byte.c"))
        .file(raft_src.join("client.c"))
        .file(raft_src.join("compress.c"))
        .file(raft_src.join("configuration.c"))
        .file(raft_src.join("convert.c"))
        .file(raft_src.join("election.c"))
        .file(raft_src.join("entry.c"))
        .file(raft_src.join("err.c"))
        .file(raft_src.join("heap.c"))
        .file(raft_src.join("membership.c"))
        .file(raft_src.join("message.c"))
        .file(raft_src.join("progress.c"))
        .file(raft_src.join("random.c"))
        .file(raft_src.join("raft.c"))
        .file(raft_src.join("recv.c"))
        .file(raft_src.join("recv_append_entries.c"))
        .file(raft_src.join("recv_append_entries_result.c"))
        .file(raft_src.join("recv_request_vote.c"))
        .file(raft_src.join("recv_request_vote_result.c"))
        .file(raft_src.join("recv_install_snapshot.c"))
        .file(raft_src.join("recv_timeout_now.c"))
        .file(raft_src.join("restore.c"))
        .file(raft_src.join("replication.c"))
        .file(raft_src.join("state.c"))
        .file(raft_src.join("syscall.c"))
        .file(raft_src.join("timeout.c"))
        .file(raft_src.join("tracing.c"))
        .file(raft_src.join("trail.c"))
        // Not clobbering the base pointer helps bpftrace construct backtraces
        .flag_if_supported("-fno-omit-frame-pointer");

    builder.compile("libraft.a")
}
