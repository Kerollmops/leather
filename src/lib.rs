mod configuration;
mod raft;

pub use configuration::{Configuration, Role};
use cowsql_raft_sys::{
    raft_event, raft_id, raft_index, raft_term, raft_time, RAFT_VERSION_MAJOR, RAFT_VERSION_MINOR,
    RAFT_VERSION_RELEASE,
};
pub use raft::Raft;

pub type RaftTime = raft_time;
pub type RaftTerm = raft_term;
pub type RaftId = raft_id;
pub type RaftIndex = raft_index;

pub fn version() -> (u32, u32, u32) {
    (RAFT_VERSION_MAJOR, RAFT_VERSION_MINOR, RAFT_VERSION_RELEASE)
}

pub struct Event {
    /// Event timestamp. Must always be filled with the current time.
    pub time: RaftTime,
    /// Disk capacity that has been reserved and is guaranteed to be available.
    pub capacity: u16, // Why is it an u16?
    pub type_: EventType,
}

pub enum EventType {
    Start {
        term: RaftTerm,
        voted_for: RaftId,
        metadata: SnapshotMetadata,
        start_index: RaftIndex,
        entries: Vec<RaftEntry>,
    },
}

impl From<Event> for raft_event {
    fn from(event: Event) -> Self {
        todo!()
    }
}

/// Hold metadata associated with a snapshot.
pub struct SnapshotMetadata {
    pub index: RaftIndex,
    pub term: RaftTerm,
    pub configuration: Configuration,
    pub configuration_index: RaftIndex,
}

pub struct RaftEntry {
    pub term: RaftTerm,
    pub type_: EntryType,
}

pub enum EntryType {
    Command(Vec<u8>),
    Barrier,
    Change(Vec<u8>),
}
