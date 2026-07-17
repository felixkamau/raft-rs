#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
pub struct LogEntry {
    term: u8,
    command: String,
}

#[allow(unused)]
#[derive(Debug, Default)]
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

impl StateModule {
    pub fn new(peer_count: usize) -> Self {
        StateModule {
            next_index: vec![0; peer_count],
            match_index: vec![0; peer_count],
            ..Default::default()
        }
    }

    pub fn heart_beat() {}

    /// invoked by candidates
    /// handle_request_vote function
    ///
    /// ## Arguments
    /// `term` candiate's term
    /// `candidate_id` candidate requesting vote
    ///
    /// ## Returns
    /// `term` current_term, for candidate to update itself
    /// `vote_granted` true means candidate received vote
    pub fn handle_request_vote(&mut self, term: u8, candidate_id: u8) -> (u8, bool) {
        if term < self.current_term {
            return (self.current_term, false);
        }
        if term > self.current_term {
            self.current_term = term;
            self.voted_for = None;
        }

        let can_vote = self.voted_for.is_none() || self.voted_for == Some(candidate_id);
        if can_vote {
            self.voted_for = Some(candidate_id);
        }
        (self.current_term, can_vote)
    }

    pub fn handle_append_entries(&mut self) -> (u8, bool) {
        (self.current_term, false)
    }
}
