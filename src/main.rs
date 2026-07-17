mod node;
mod state_module;
mod role;

use crate::node::Node;
fn main() {
    let mut node = Node::new(1, 2);
    // node.on_request_vote();
}
