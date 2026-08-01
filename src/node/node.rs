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
    heartbeat_interval: Duration,
    heartbeat_deadline: Instant,
}
fn random_timeout() -> Duration {
    Duration::from_millis(rand::random_range(150..=300))
}

impl Node {
    pub fn new(id: u8, peer_count: usize) -> Self {
        let timeout = random_timeout();
        let heartbeat_interval = Duration::from_millis(50);

        Node {
            id: id,
            role: Role::default(),
            state: StateModule::new(peer_count),
            election_deadline: Instant::now() + timeout,
            election_timeout: timeout,
            heartbeat_interval,
            heartbeat_deadline: Instant::now() + heartbeat_interval,
        }
    }

    pub fn reset_election_timer(&mut self) {
        self.election_timeout = random_timeout();
        self.election_deadline = Instant::now() + self.election_timeout;
    }

    pub fn reset_heartbeat_timer(&mut self) {
        self.heartbeat_deadline = Instant::now() + self.heartbeat_interval;
    }

    pub fn heartbeat_due(&self) -> bool {
        self.role == Role::LEADER && Instant::now() >= self.heartbeat_deadline
    }

    pub fn send_heartbeats(&mut self) {
        self.reset_heartbeat_timer();

        // will be sending empty AppendEntries here
    }

    pub fn tick(&mut self) {
        match self.role {
            Role::FOLLOWER | Role::CANDIDATE => {
                if self.election_timed_out() {
                    self.start_election();
                }
            }
            Role::LEADER => {
                if self.heartbeat_due() {
                    self.send_heartbeats();
                }
            }
        }

        // Future impl
        // - retry failed AppendEntries
        // - advance commit index
        // - send snapshots
    }

    pub fn election_timed_out(&self) -> bool {
        Instant::now() >= self.election_deadline
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
        let (_current_term, appended) = self.state.handle_append_entries(
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        );

        if appended {
            // we heard from a valid leader
            self.role = Role::FOLLOWER;

            // reset election timer here
            self.reset_election_timer();
        }

        (self.state.current_term, appended)
    }

    fn start_election(&mut self) {
        // impl to be done
        // - increment current term
        // - become candidate
        // - vote for self
        // - reset election timer
        // - send RequestVote RPCs
    }
}
