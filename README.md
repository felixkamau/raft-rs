# raft-rs

A Rust implementation of the **Raft Consensus Algorithm** based on the paper [*In Search of an Understandable Consensus Algorithm (Extended Version)*](https://raft.github.io/raft.pdf) by Diego Ongaro and John Ousterhout (Stanford University).

This repository is designed to build a robust Raft consensus engine, which will serve as the distributed consensus layer for a fault-tolerant, replicated **Key-Value (KV) Store**.

---

## 📌 Project Overview and Intent

The primary goal of `raft-rs` is to implement the Raft consensus protocol in idiomatic Rust, providing:
1. **Strong Consistency**: Linearizable state machine replication across a cluster of nodes.
2. **High Availability**: Dynamic leader election and automatic failover upon node or network failures.
3. **Key-Value State Machine**: An application layer built on top of Raft to process `Get`, `Put`, `Append`, and `Delete` operations.

---

## Current Architecture and Code Base

The repository is modularized into three core components:

* **[src/role/role.rs](file:///c:/Users/Administrator/Documents/devprojects/raft-rs/src/role/role.rs)**:
  * Defines the node role enum: `FOLLOWER`, `CANDIDATE`, `LEADER`.
  * Provides default state (`FOLLOWER`) and role conversion utilities (`TryFrom<&str>`).

* **[src/state_module/state_module.rs](file:///c:/Users/Administrator/Documents/devprojects/raft-rs/src/state_module/state_module.rs)**:
  * `LogEntry`: Encapsulates log index information, term numbers, and command payloads.
  * `StateModule`: Maintains Raft state:
    * **Persistent State**: `current_term`, `voted_for`, `log`.
    * **Volatile State (All Servers)**: `commit_index`, `last_applied`.
    * **Volatile State (Leaders)**: `next_index`, `match_index`.
  * `handle_request_vote`: Processes incoming `RequestVote` RPCs, handling term synchronization and vote granting logic.
  * `handle_append_entries`: Processes incoming `AppendEntries` RPCs, handling stale term rejection, log consistency checks (`prev_log_index` / `prev_log_term`), log truncation of conflicting entries, entry appending, and updating `commit_index`.

* **[src/node/node.rs](file:///c:/Users/Administrator/Documents/devprojects/raft-rs/src/node/node.rs)**:
  * `Node`: Serves as the top-level entity coordinating node identity (`id`), state machine module (`StateModule`), and active cluster role (`Role`).
  * Exposes handlers `on_request_vote` and `on_append_entries` to drive node state transitions.

---

## Raft Paper Implementation Status

Below is the implementation matrix comparing features outlined in the [Raft Paper](https://raft.github.io/raft.pdf) with what is currently built and planned:

| Raft Paper Section | Feature / Specification | Implementation Status | Code Location / Notes |
| :--- | :--- | :---: | :--- |
| **5.1 Raft Basics** | Node Roles (`Follower`, `Candidate`, `Leader`) & Terms | Complete | [src/role/role.rs](src/role/role.rs) |
| **5.2 Leader Election** | `RequestVote` RPC handling & term updating | Partial | [src/state_module/state_module.rs](raft-rs/src/state_module/state_module.rs) |
| **5.2 Leader Election** | Election Timers & Randomized Timeouts | Planned | Requires async timer tasks (`tokio`) |
| **5.2 Leader Election** | Requesting votes & transitioning Candidate ➔ Leader | Planned | Majority vote collection logic |
| **5.3 Log Replication** | `AppendEntries` RPC log matching & truncation | Complete | [src/state_module/state_module.rs](raft-rs/src/state_module/state_module.rs) |
| **5.3 Log Replication** | Heartbeat dispatch from Leader | Planned | Ticker loop in Leader state |
| **5.4 Safety** | Election Restriction (Checking Candidate's last log term/index) | Planned | Candidate log up-to-date validation in `handle_request_vote` |
| **5.4 Safety** | Committing entries from previous terms | Planned | Leader commit index calculation |
| **5.5/5.6 Fault Tolerance** | Persistent Storage to Disk (WAL) | Planned | Persisting `current_term`, `voted_for`, `log` |
| **Section 7** | Log Compaction & Snapshots (`InstallSnapshot` RPC) | Planned | Snapshotting state machine state |
| **Section 8** | Client Interaction & State Machine Application | Planned | `apply_ch` channel for committed entries |
| **Extension** | **Key-Value Storage Engine** (`Put`, `Get`, `Delete`) | Planned | In-memory / persistent KV engine on top of Raft |

---

## Roadmap to KV Store

To evolve `raft-rs` into a fully functional distributed Key-Value store, the development roadmap follows these steps:

1. **Async Runtime & Networking**:
   - Integrate `tokio` for async timers (heartbeats, election timeouts).
   - Implement gRPC (using `tonic`) or TCP transport for inter-node RPC communications.

2. **Raft Election & Consensus Completion**:
   - Implement randomized election timeout (150ms–300ms).
   - Add election restriction validation in `handle_request_vote` (checking candidate's last log index & term).
   - Implement leader heartbeat generation and vote collection.

3. **Persistence & Storage Layer**:
   - Save `current_term`, `voted_for`, and `log` to stable storage before responding to RPCs.

4. **State Machine Integration**:
   - Create an `ApplyMsg` / `apply_ch` channel that sends committed `LogEntry` items when `commit_index > last_applied`.

5. **Key-Value Store Layer**:
   - Build a `KvStore` module (e.g. wrapping a thread-safe `HashMap<String, String>`).
   - Parse KV commands (`Put`, `Get`, `Append`, `Delete`) stored in `LogEntry::command`.
   - Implement client API handles for routing requests to the leader and supporting linearizable reads.

---

## Getting Started

### Prerequisites
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (Edition 2024 compatible compiler)

### Building & Running Tests

Clone the repository and run cargo tests:
```bash
cargo build
cargo test
```

### Basic Code Example

```rust
use raft_rs::node::Node;

fn main() {
    // Instantiate node ID 1 in a 3-node cluster (peer_count = 2)
    let mut node = Node::new(1, 2);
    
    // Node starts in FOLLOWER role with clean state
    println!("Node initialized");
}
```

---

## References
- Diego Ongaro and John Ousterhout. [*In Search of an Understandable Consensus Algorithm (Extended Version)*](https://raft.github.io/raft.pdf).
- Raft Website & Resources: [https://raft.github.io/](https://raft.github.io/)
