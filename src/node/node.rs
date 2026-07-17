use crate::{role::Role, state_module::StateModule};

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
}
