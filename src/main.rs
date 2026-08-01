mod node;
mod role;
mod state_module;

use crate::node::Node;
fn main() {
    // let mut node = Node::new(1, 2);
    // node.on_request_vote();
    let mut nodes: Vec<Node> = (1..=3).map(|id| Node::new(id, 2)).collect();
    // Node 1 requests a vote from Node 2
    let (term, granted) = nodes[1].on_request_vote(1, 1);

    println!("Term: {}, Vote granted: {}", term, granted);
}
