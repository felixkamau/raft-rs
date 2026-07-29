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

    /// innvoked by leader to replicate log entries also used
    /// as heart beat.
    ///
    /// handle_append_entries
    ///
    /// ## Arguments
    /// `term` leader's term.
    /// `leader_id` so followers can redirect clients.
    /// `prev_log_index` index of log entry preceeding new ones.
    /// `prev_log_term` term of the `prev_log_index` entry
    /// `entries[]` log entries to store (empty for heart beat; may send more
    ///  more than one for effieciney)
    /// `leader_commit` leader commit index
    pub fn handle_append_entries(
        &mut self,
        term: u8,
        _leader_id: u8,
        prev_log_index: Option<usize>,
        prev_log_term: u8,
        entries: Vec<LogEntry>,
        leader_commit: usize,
    ) -> (u8, bool) {
        // Reject stale terms.
        if term < self.current_term {
            return (self.current_term, false);
        }

        // Update to newer term.
        if term > self.current_term {
            self.current_term = term;
            self.voted_for = None;
        }

        // Validate previous log entry (if one exists).
        if let Some(prev_idx) = prev_log_index {
            if prev_idx >= self.log.len() {
                return (self.current_term, false);
            }

            if self.log[prev_idx].term != prev_log_term {
                return (self.current_term, false);
            }
        }

        let insert_idx = match prev_log_index {
            Some(idx) => idx + 1,
            None => 0,
        };

        // Delete conflicting entries.
        for (offset, entry) in entries.iter().enumerate() {
            let log_idx = insert_idx + offset;

            if log_idx < self.log.len() && self.log[log_idx].term != entry.term {
                self.log.truncate(log_idx);
                break;
            }
        }

        // Append new entries.
        for (offset, entry) in entries.into_iter().enumerate() {
            let log_idx = insert_idx + offset;

            if log_idx >= self.log.len() {
                self.log.push(entry);
            }
        }

        // Update commit index.
        if leader_commit > self.commit_index as usize && !self.log.is_empty() {
            self.commit_index = std::cmp::min(leader_commit, self.log.len() - 1) as u8;
        }

        (self.current_term, true)
    }
}
