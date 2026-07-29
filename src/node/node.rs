use crate::{
    role::Role,
    state_module::{StateModule, state_module::LogEntry},
};

pub struct Node {
    id: u8,
    role: Role,
    state: StateModule,
}

impl Node {
    pub fn new(id: u8, peer_count: usize) -> Self {
        Node {
            id: id,
            role: Role::default(),
            state: StateModule::new(peer_count),
        }
    }

    pub fn on_request_vote(&mut self, term: u8, candidate_id: u8) -> (u8, bool) {
        let (current_term, granted) = self.state.handle_request_vote(term, candidate_id);
        if granted {
            self.role = Role::FOLLOWER; // Fall back and reset time here
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
        if current_term > old_term {
            // we heard from a vaild leader
            self.role = Role::FOLLOWER;
            // reset election timee here
        }
        (current_term, appended)
    }
}
