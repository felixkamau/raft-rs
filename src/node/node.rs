use std::time::{Duration, Instant};

use crate::{
    role::Role,
    state_module::{StateModule, state_module::LogEntry},
};

pub struct Node {
    id: u8,
    role: Role,
    state: StateModule,

    election_deadline: Instant,
    election_timeout: Duration,
    // heartbeat_interval: Duration,
    // heartbeat_deadline: Instant,
}
fn random_timeout() -> Duration {
    let millis = rand::random_range(150..300);
    let duration: Duration = Duration::from_millis(millis);
    duration
}

impl Node {
    pub fn new(id: u8, peer_count: usize) -> Self {
        let timeout = random_timeout();
        Node {
            id: id,
            role: Role::default(),
            state: StateModule::new(peer_count),
            election_deadline: Instant::now() + timeout,
            election_timeout: timeout,
        }
    }

    pub fn reset_election_timer(&mut self) {
        self.election_timeout = random_timeout();
        self.election_deadline = Instant::now() + self.election_timeout;
    }

    pub fn election_timed_out(&self) -> bool {
        let timedout = Instant::now() >= self.election_deadline;
        timedout
    }

    pub fn on_request_vote(&mut self, term: u8, candidate_id: u8) -> (u8, bool) {
        let (current_term, granted) = self.state.handle_request_vote(term, candidate_id);
        if granted {
            self.role = Role::FOLLOWER; // Fall back and reset time here
            self.reset_election_timer();
        }
        (current_term, granted)
    }

    pub fn on_append_entries(
        &mut self,
        term: u8,
        leader_id: u8,
        prev_log_index: Option<usize>,
        prev_log_term: u8,
        entries: Vec<LogEntry>,
        leader_commit: usize,
    ) -> (u8, bool) {
        let old_term = self.state.current_term;
        let (current_term, appended) = self.state.handle_append_entries(
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        );
        if current_term > old_term && appended {
            // we heard from a vaild leader
            self.role = Role::FOLLOWER;
            // reset election timee here
            self.reset_election_timer();
        }
        (current_term, appended)
    }
}
