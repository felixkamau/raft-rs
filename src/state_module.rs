#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
pub struct LogEntry {
    term: u32,
    command: String,
}

#[allow(unused)]
#[derive(Debug)]
pub struct StateModule {
    /// Persistant state should be stored
    /// in stable memory updated before responding
    /// to RPCs
    pub current_term: u8,
    pub voted_for: Option<u8>,
    pub log: Vec<LogEntry>,
    /// Volatile States
    pub commit_index: u8, // what majority have confirmed
    pub last_applied: u8,
    /// re-initialised after election
    pub next_index: Vec<u8>, // per client
    pub match_index: Vec<u8>, // per client
}
