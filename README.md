# raft-rs

A Rust implementation of the **Raft Consensus Algorithm**, following the paper **In Search of an Understandable Consensus Algorithm (Extended Version)** by Diego Ongaro and John Ousterhout.

The goal of this project is to build a production-quality Raft implementation in idiomatic Rust while documenting the implementation process. Once the consensus layer is complete, it will power a fault-tolerant, replicated **distributed key-value store**.

> **Status:** 🚧 Work in Progress

---

## Goals

- [x] Implement the Raft consensus algorithm from scratch
- [x] Follow the Raft paper as closely as possible
- [x] Keep the implementation modular and easy to understand
- [ ] Build a distributed key-value store on top of Raft
- [x] Document the implementation journey

---

## Project Structure

```text
src/
├── node/
│   └── node.rs          # Top-level Raft node
├── role/
│   └── role.rs          # Follower, Candidate, Leader
├── state_module/
│   └── state_module.rs  # Core Raft state machine
```

### Node

Coordinates the different parts of a Raft server.

Responsibilities:

- Maintaining node identity
- Managing the current role
- Receiving RPCs
- Delegating protocol logic to the state module

### State Module

Implements the core Raft protocol.

**Persistent State**

- `current_term`
- `voted_for`
- `log`

**Volatile State**

- `commit_index`
- `last_applied`

**Leader State**

- `next_index`
- `match_index`

Implemented RPC handlers:

- `handle_request_vote`
- `handle_append_entries`

### Roles

- Follower
- Candidate
- Leader

---

# Implementation Checklist

## Section 5.1 — Raft Basics

- [x] Node roles
- [x] Current term
- [x] Persistent log
- [x] Commit index
- [x] Last applied

## Section 5.2 — Leader Election

- [x] RequestVote RPC
- [x] Vote granting
- [x] Term updates
- [x] Election timer
- [x] Randomized election timeout
- [ ] Candidate election
- [ ] Majority vote counting
- [ ] Leader transition

## Section 5.3 — Log Replication

- [x] AppendEntries RPC
- [x] Previous log validation
- [x] Log consistency checks
- [x] Conflict detection
- [x] Log truncation
- [x] Append new entries
- [x] Commit index updates
- [ ] Leader heartbeats
- [ ] Leader replication loop
- [ ] nextIndex tracking
- [ ] matchIndex tracking

## Section 5.4 — Safety

- [ ] Election restriction
- [ ] Commit entries from current term only
- [ ] State machine safety

## Section 7 — Log Compaction

- [ ] InstallSnapshot RPC
- [ ] Snapshot creation
- [ ] Log compaction

## Persistence

- [ ] Write-Ahead Log (WAL)
- [ ] Persist current term
- [ ] Persist voted_for
- [ ] Persist log entries
- [ ] Crash recovery

## Networking

- [ ] Tokio runtime
- [ ] RPC transport
- [ ] Cluster communication
- [ ] Heartbeat scheduler

## Key-Value Store

- [ ] Apply committed entries
- [ ] State machine
- [ ] Client API
- [ ] Put
- [ ] Get
- [ ] Delete
- [ ] Append
- [ ] Linearizable reads

---

# Building

## Prerequisites

- Rust (Edition 2024)
- Cargo

## Clone

```bash
git clone https://github.com/<your-username>/raft-rs.git
cd raft-rs
```

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

---

# Example

```rust
use raft_rs::node::Node;

fn main() {
    let node = Node::new(1, 3);

    println!("Node initialized!");
}
```

---

# Learning Objectives

This project explores:

- Distributed systems
- Consensus algorithms
- State machine replication
- Leader election
- Fault tolerance
- Network programming
- Rust ownership
- Async programming with Tokio
- Building distributed databases

---

# References

- Diego Ongaro & John Ousterhout, *In Search of an Understandable Consensus Algorithm (Extended Version)*
- https://raft.github.io/
- https://raft.github.io/raft.pdf

---

## License

MIT
