//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

use super::functions::*;

/// CRDT type selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CRDTType {
    /// Grow-only counter.
    GCounter,
    /// Positive-Negative counter.
    PNCounter,
    /// Grow-only set.
    GSet,
    /// Two-phase (add/remove once) set.
    TwoPhaseSet,
    /// Last-Write-Wins register.
    LWWRegister,
}
impl CRDTType {
    /// Describe the merge semantics.
    pub fn merge(&self) -> &'static str {
        match self {
            CRDTType::GCounter => "component-wise max",
            CRDTType::PNCounter => "component-wise max on both P and N vectors",
            CRDTType::GSet => "set union",
            CRDTType::TwoPhaseSet => "union of add-sets and remove-sets (remove wins)",
            CRDTType::LWWRegister => "keep entry with largest timestamp",
        }
    }
    /// Describe the query semantics.
    pub fn query(&self) -> &'static str {
        match self {
            CRDTType::GCounter => "sum of counter vector",
            CRDTType::PNCounter => "sum(P) - sum(N)",
            CRDTType::GSet => "the set of all added elements",
            CRDTType::TwoPhaseSet => "elements in add-set but not in remove-set",
            CRDTType::LWWRegister => "value with the latest timestamp",
        }
    }
}
/// Raft node roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RaftRole {
    Follower,
    Candidate,
    Leader,
}
/// A simple FIFO message queue for simulation.
pub struct MessageQueue {
    pub queue: VecDeque<Message>,
}
impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            queue: VecDeque::new(),
        }
    }
    pub fn send(&mut self, msg: Message) {
        self.queue.push_back(msg);
    }
    pub fn receive(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
/// Participant state in 2PC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TwoPCState {
    Init,
    Prepared,
    Committed,
    Aborted,
}
/// A Lamport logical clock for a single process.
#[derive(Clone, Debug)]
pub struct LamportClock {
    /// The current logical time.
    pub time: u64,
}
impl LamportClock {
    /// Create a new Lamport clock at time 0.
    pub fn new() -> Self {
        LamportClock { time: 0 }
    }
    /// Increment the clock for a local event. Returns the new timestamp.
    pub fn tick(&mut self) -> u64 {
        self.time += 1;
        self.time
    }
    /// Update the clock upon receiving a message with timestamp `recv_time`.
    /// Sets time = max(time, recv_time) + 1.
    pub fn receive(&mut self, recv_time: u64) -> u64 {
        self.time = self.time.max(recv_time) + 1;
        self.time
    }
}
/// A vector clock for `n` processes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorClock {
    /// The clock vector. `clocks[i]` = logical time at process i.
    pub clocks: Vec<u64>,
    /// The id of this process.
    pub process_id: usize,
}
impl VectorClock {
    /// Create a new vector clock for process `pid` in a system of `n` processes.
    pub fn new(pid: usize, n: usize) -> Self {
        VectorClock {
            clocks: vec![0; n],
            process_id: pid,
        }
    }
    /// Increment this process's component for a local event.
    pub fn tick(&mut self) {
        self.clocks[self.process_id] += 1;
    }
    /// Update the clock upon receiving a message with vector clock `other`.
    /// Sets each component to the max, then increments this process's component.
    pub fn receive(&mut self, other: &VectorClock) {
        for i in 0..self.clocks.len() {
            self.clocks[i] = self.clocks[i].max(other.clocks[i]);
        }
        self.clocks[self.process_id] += 1;
    }
    /// Return true if `self` causally precedes `other` (self < other in vector clock order).
    ///
    /// VC₁ < VC₂ iff VC₁ ≤ VC₂ componentwise and VC₁ ≠ VC₂.
    pub fn causally_precedes(&self, other: &VectorClock) -> bool {
        let leq = self.clocks.iter().zip(&other.clocks).all(|(a, b)| a <= b);
        let neq = self.clocks != other.clocks;
        leq && neq
    }
    /// Return true if `self` and `other` are concurrent (incomparable).
    pub fn concurrent(&self, other: &VectorClock) -> bool {
        !self.causally_precedes(other) && !other.causally_precedes(self) && self != other
    }
}
/// The three CAP properties a distributed system can have.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CAPProperty {
    Consistency,
    Availability,
    PartitionTolerance,
}
/// Ring-based leader election (Chang-Roberts algorithm).
///
/// Each node has a unique id; the node with the maximum id is elected leader.
/// Messages travel clockwise around the ring.
#[allow(dead_code)]
pub struct LeaderElectionRing {
    /// Sorted node ids on the ring (clockwise order).
    pub nodes: Vec<u64>,
    /// Index of the elected leader in `nodes`, if election is complete.
    pub leader: Option<usize>,
}
impl LeaderElectionRing {
    /// Create a ring from a list of node ids.
    pub fn new(ids: Vec<u64>) -> Self {
        LeaderElectionRing {
            nodes: ids,
            leader: None,
        }
    }
    /// Run Chang-Roberts algorithm. Returns the id of the elected leader.
    ///
    /// Each node initially sends its own id. When a node receives an id:
    ///   - if received > own id: forward it
    ///   - if received == own id: we are the leader
    ///   - if received < own id: discard
    pub fn elect_leader(&mut self) -> u64 {
        if self.nodes.is_empty() {
            return 0;
        }
        let n = self.nodes.len();
        let (leader_idx, &leader_id) = self
            .nodes
            .iter()
            .enumerate()
            .max_by_key(|(_, &id)| id)
            .expect("nodes is non-empty: checked by n == 0 guard");
        self.leader = Some(leader_idx);
        let _messages = n;
        leader_id
    }
    /// Return the number of ring nodes.
    pub fn ring_size(&self) -> usize {
        self.nodes.len()
    }
    /// Check if a leader has been elected.
    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }
    /// Return the leader's node id, if elected.
    pub fn leader_id(&self) -> Option<u64> {
        self.leader.map(|i| self.nodes[i])
    }
}
/// Consistency levels in distributed systems, ordered strongest to weakest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConsistencyLevel {
    /// Linearizability (atomic consistency): real-time ordering of all operations.
    Linearizable = 7,
    /// Sequential consistency: all operations appear in some sequential order.
    Sequential = 6,
    /// Strict serializability: serializability + real-time order.
    StrictSerializable = 5,
    /// Serializability: transactions appear to execute serially.
    Serializable = 4,
    /// Causal consistency: causally related operations are seen in order.
    Causal = 3,
    /// Read-your-writes: a process sees its own writes.
    ReadYourWrites = 2,
    /// Eventual consistency: replicas eventually converge.
    Eventual = 1,
}
impl ConsistencyLevel {
    /// Return true if this consistency level implies `other`.
    pub fn implies(&self, other: ConsistencyLevel) -> bool {
        (*self as u8) >= (other as u8)
    }
    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            ConsistencyLevel::Linearizable => "Linearizable",
            ConsistencyLevel::Sequential => "Sequential",
            ConsistencyLevel::StrictSerializable => "StrictSerializable",
            ConsistencyLevel::Serializable => "Serializable",
            ConsistencyLevel::Causal => "Causal",
            ConsistencyLevel::ReadYourWrites => "ReadYourWrites",
            ConsistencyLevel::Eventual => "Eventual",
        }
    }
}
/// Total-order broadcast (atomic broadcast) emulator.
///
/// Assigns sequence numbers to messages so all recipients deliver them in
/// the same order, satisfying the total order and agreement properties.
#[allow(dead_code)]
pub struct TotalOrderBroadcast {
    /// Global sequence counter.
    pub sequence: u64,
    /// Delivered message log in total order.
    pub log: Vec<(u64, Vec<u8>)>,
}
impl TotalOrderBroadcast {
    /// Create a new total-order broadcast log.
    pub fn new() -> Self {
        TotalOrderBroadcast {
            sequence: 0,
            log: Vec::new(),
        }
    }
    /// Broadcast a message; assigns it the next sequence number.
    /// Returns the assigned sequence number.
    pub fn broadcast(&mut self, msg: Vec<u8>) -> u64 {
        self.sequence += 1;
        self.log.push((self.sequence, msg));
        self.sequence
    }
    /// Deliver all messages in total order (by sequence number).
    pub fn deliver_all(&self) -> Vec<(u64, &Vec<u8>)> {
        let mut ordered: Vec<(u64, &Vec<u8>)> = self.log.iter().map(|(s, m)| (*s, m)).collect();
        ordered.sort_by_key(|(s, _)| *s);
        ordered
    }
    /// Check if the log satisfies total order: sequence numbers are strictly increasing.
    pub fn is_totally_ordered(&self) -> bool {
        self.log.windows(2).all(|w| w[0].0 < w[1].0)
    }
    /// Number of messages broadcast so far.
    pub fn message_count(&self) -> usize {
        self.log.len()
    }
}
/// Snapshot isolation concurrency control validator.
///
/// Tracks concurrent transactions and detects write-write conflicts
/// (write skew) that violate snapshot isolation guarantees.
#[allow(dead_code)]
pub struct SnapshotIsolationValidator {
    /// Map from transaction_id to (start_ts, commit_ts, write_set).
    pub transactions: Vec<(u64, u64, u64, Vec<u64>)>,
    /// Global timestamp counter.
    pub clock: u64,
}
impl SnapshotIsolationValidator {
    /// Create a new validator.
    pub fn new() -> Self {
        SnapshotIsolationValidator {
            transactions: Vec::new(),
            clock: 0,
        }
    }
    /// Begin a transaction. Returns (transaction_id, start_timestamp).
    pub fn begin_transaction(&mut self) -> (usize, u64) {
        self.clock += 1;
        let tid = self.transactions.len();
        self.transactions
            .push((tid as u64, self.clock, 0, Vec::new()));
        (tid, self.clock)
    }
    /// Record a write to key `key` by transaction `tid`.
    pub fn write(&mut self, tid: usize, key: u64) {
        if let Some(tx) = self.transactions.get_mut(tid) {
            tx.3.push(key);
        }
    }
    /// Attempt to commit transaction `tid`. Returns true if no write-write conflict.
    pub fn commit(&mut self, tid: usize) -> bool {
        self.clock += 1;
        let commit_ts = self.clock;
        if let Some(tx) = self.transactions.get(tid) {
            let start_ts = tx.1;
            let write_set: Vec<u64> = tx.3.clone();
            let conflict = self.transactions.iter().any(|other| {
                other.0 != tid as u64
                    && other.2 > start_ts
                    && other.2 < commit_ts
                    && other.3.iter().any(|k| write_set.contains(k))
            });
            if !conflict {
                if let Some(tx_mut) = self.transactions.get_mut(tid) {
                    tx_mut.2 = commit_ts;
                }
                return true;
            }
        }
        false
    }
    /// Return the number of successfully committed transactions.
    pub fn committed_count(&self) -> usize {
        self.transactions.iter().filter(|tx| tx.2 > 0).count()
    }
}
/// Byzantine fault tolerance configuration.
#[derive(Debug, Clone)]
pub struct ByzantineFaultTolerance {
    /// Maximum number of Byzantine (adversarial) faults to tolerate.
    pub f: usize,
    /// Total number of replicas.
    pub n: usize,
}
impl ByzantineFaultTolerance {
    /// Create a new BFT configuration.
    pub fn new(f: usize, n: usize) -> Self {
        ByzantineFaultTolerance { f, n }
    }
    /// Returns true iff n ≥ 3f + 1 (necessary and sufficient for BFT consensus).
    pub fn is_tolerant(&self) -> bool {
        self.n >= 3 * self.f + 1
    }
    /// Minimum number of replicas needed to tolerate `f` Byzantine faults.
    pub fn min_replicas_needed(&self) -> usize {
        3 * self.f + 1
    }
}
/// Gossip (epidemic) dissemination protocol.
#[derive(Debug, Clone)]
pub struct GossipProtocol {
    /// Number of peers contacted per round (fan-out).
    pub fan_out: usize,
    /// Number of gossip rounds.
    pub rounds: usize,
}
impl GossipProtocol {
    /// Create a new gossip protocol configuration.
    pub fn new(fan_out: usize, rounds: usize) -> Self {
        GossipProtocol { fan_out, rounds }
    }
    /// Expected dissemination time (rounds) to reach all n nodes.
    /// Approximate: O(log n / log fan_out).
    pub fn dissemination_time(&self, n: usize) -> f64 {
        if self.fan_out <= 1 || n <= 1 {
            return n as f64;
        }
        (n as f64).log2() / (self.fan_out as f64).log2()
    }
    /// Fraction of nodes informed after `rounds` rounds (simplified model).
    /// Uses: informed ≈ n * (1 - (1 - 1/n)^(fan_out * rounds)).
    pub fn coverage(&self, n: usize) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let expected =
            n as f64 * (1.0 - (1.0 - 1.0 / n as f64).powi((self.fan_out * self.rounds) as i32));
        (expected / n as f64).min(1.0)
    }
}
/// G-Counter CRDT: grow-only counter supporting increment and merge.
///
/// Each replica maintains its own counter; the value is the sum of all counters.
#[derive(Clone, Debug)]
pub struct GCounter {
    /// Per-replica increment counts.
    pub counts: Vec<u64>,
    /// This replica's id.
    pub replica_id: usize,
}
impl GCounter {
    /// Create a new G-Counter for `n_replicas` replicas, owned by `replica_id`.
    pub fn new(replica_id: usize, n_replicas: usize) -> Self {
        GCounter {
            counts: vec![0; n_replicas],
            replica_id,
        }
    }
    /// Increment this replica's counter.
    pub fn increment(&mut self) {
        self.counts[self.replica_id] += 1;
    }
    /// Increment by `delta`.
    pub fn increment_by(&mut self, delta: u64) {
        self.counts[self.replica_id] += delta;
    }
    /// Return the current value (sum of all replica counters).
    pub fn value(&self) -> u64 {
        self.counts.iter().sum()
    }
    /// Merge with another G-Counter state. State-based CRDT merge = pointwise max.
    pub fn merge(&mut self, other: &GCounter) {
        for i in 0..self.counts.len() {
            self.counts[i] = self.counts[i].max(other.counts[i]);
        }
    }
}
/// A generic state in a TLA+ specification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TLAState {
    /// Named variables and their integer values.
    pub vars: HashMap<String, i64>,
}
impl TLAState {
    pub fn new() -> Self {
        TLAState {
            vars: HashMap::new(),
        }
    }
    pub fn set(&mut self, var: &str, val: i64) {
        self.vars.insert(var.to_string(), val);
    }
    pub fn get(&self, var: &str) -> i64 {
        *self.vars.get(var).unwrap_or(&0)
    }
}
/// Paxos protocol configuration.
#[derive(Debug, Clone)]
pub struct PaxosProtocol {
    /// Number of acceptors.
    pub num_acceptors: usize,
    /// Number of proposers.
    pub num_proposers: usize,
}
impl PaxosProtocol {
    /// Create a new Paxos protocol configuration.
    pub fn new(num_acceptors: usize, num_proposers: usize) -> Self {
        PaxosProtocol {
            num_acceptors,
            num_proposers,
        }
    }
    /// Minimum quorum size: majority of acceptors.
    pub fn quorum_size(&self) -> usize {
        self.num_acceptors / 2 + 1
    }
    /// Returns the two phases of Paxos.
    pub fn phases(&self) -> Vec<&'static str> {
        vec!["Phase 1: Prepare/Promise", "Phase 2: Accept/Accepted"]
    }
    /// Returns true iff the protocol is safe (quorums intersect).
    pub fn is_safe(&self) -> bool {
        self.quorum_size() * 2 > self.num_acceptors
    }
}
/// A Paxos proposal.
#[derive(Clone, Debug)]
pub struct PaxosProposal {
    pub proposal_number: u64,
    pub value: Option<u64>,
}
/// Causal broadcast protocol using vector clocks for ordering.
///
/// Implements the vector-clock–based causal delivery algorithm:
/// a message is only delivered when all causally prior messages have been delivered.
#[allow(dead_code)]
pub struct CausalBroadcast {
    /// Number of processes in the system.
    pub num_processes: usize,
    /// This process's id.
    pub process_id: usize,
    /// Current vector clock.
    pub vc: Vec<u64>,
    /// Queue of messages pending delivery: (sender_id, sender_vc, payload).
    pub pending: VecDeque<(usize, Vec<u64>, Vec<u8>)>,
    /// Delivered messages in causal order.
    pub delivered: Vec<(usize, Vec<u8>)>,
}
impl CausalBroadcast {
    /// Create a new causal broadcast for process `pid` in a `n`-process system.
    pub fn new(pid: usize, n: usize) -> Self {
        CausalBroadcast {
            num_processes: n,
            process_id: pid,
            vc: vec![0; n],
            pending: VecDeque::new(),
            delivered: Vec::new(),
        }
    }
    /// Broadcast a message. Increments this process's VC component.
    /// Returns the VC attached to the message.
    pub fn broadcast(&mut self, payload: Vec<u8>) -> Vec<u64> {
        self.vc[self.process_id] += 1;
        drop(payload);
        self.vc.clone()
    }
    /// Receive a message from process `sender` with vector clock `msg_vc`.
    /// Enqueues for delivery if not immediately deliverable.
    pub fn receive(&mut self, sender: usize, msg_vc: Vec<u64>, payload: Vec<u8>) {
        self.pending.push_back((sender, msg_vc, payload));
        self.try_deliver();
    }
    /// Attempt to deliver pending messages in causal order.
    fn try_deliver(&mut self) {
        let mut delivered_any = true;
        while delivered_any {
            delivered_any = false;
            let mut i = 0;
            while i < self.pending.len() {
                let deliverable = {
                    let (sender, msg_vc, _) = &self.pending[i];
                    let sender_ok = msg_vc[*sender] == self.vc[*sender] + 1;
                    let others_ok = (0..self.num_processes)
                        .filter(|&j| j != *sender)
                        .all(|j| msg_vc[j] <= self.vc[j]);
                    sender_ok && others_ok
                };
                if deliverable {
                    let (sender, msg_vc, payload) = self
                        .pending
                        .remove(i)
                        .expect("index i is valid: checked by loop bounds");
                    self.vc[sender] += 1;
                    let _ = msg_vc;
                    self.delivered.push((sender, payload));
                    delivered_any = true;
                } else {
                    i += 1;
                }
            }
        }
    }
    /// Number of delivered messages so far.
    pub fn num_delivered(&self) -> usize {
        self.delivered.len()
    }
}
/// A 2PC participant.
#[derive(Clone, Debug)]
pub struct TwoPCParticipant {
    pub id: usize,
    pub state: TwoPCState,
    /// Whether this participant is willing to commit (vote).
    pub vote: bool,
}
impl TwoPCParticipant {
    pub fn new(id: usize, vote: bool) -> Self {
        TwoPCParticipant {
            id,
            state: TwoPCState::Init,
            vote,
        }
    }
    /// Phase 1: coordinator sends Prepare. Returns the vote.
    pub fn prepare(&mut self) -> bool {
        self.state = if self.vote {
            TwoPCState::Prepared
        } else {
            TwoPCState::Aborted
        };
        self.vote
    }
    /// Phase 2: coordinator sends Commit or Abort.
    pub fn decide(&mut self, commit: bool) {
        self.state = if commit {
            TwoPCState::Committed
        } else {
            TwoPCState::Aborted
        };
    }
}
/// A 3PC participant.
#[derive(Clone, Debug)]
pub struct ThreePCParticipant {
    pub id: usize,
    pub state: ThreePCState,
    pub vote: bool,
}
impl ThreePCParticipant {
    pub fn new(id: usize, vote: bool) -> Self {
        ThreePCParticipant {
            id,
            state: ThreePCState::Init,
            vote,
        }
    }
    /// Phase 1: CanCommit?
    pub fn can_commit(&mut self) -> bool {
        self.vote
    }
    /// Phase 2: PreCommit (coordinator got all-yes in Phase 1).
    pub fn pre_commit(&mut self) {
        if self.vote {
            self.state = ThreePCState::PreCommitted;
        }
    }
    /// Phase 3: DoCommit.
    pub fn do_commit(&mut self) {
        self.state = ThreePCState::Committed;
    }
    /// Abort.
    pub fn abort(&mut self) {
        self.state = ThreePCState::Aborted;
    }
}
/// Paxos acceptor state.
#[derive(Clone, Debug)]
pub struct PaxosAcceptor {
    /// Highest proposal number seen in Phase 1.
    pub promised: u64,
    /// Last accepted proposal number.
    pub accepted_number: u64,
    /// Last accepted value.
    pub accepted_value: Option<u64>,
}
impl PaxosAcceptor {
    pub fn new() -> Self {
        PaxosAcceptor {
            promised: 0,
            accepted_number: 0,
            accepted_value: None,
        }
    }
    /// Phase 1 (Prepare): accept if proposal_number > promised.
    /// Returns `Some((accepted_number, accepted_value))` on promise, `None` on reject.
    pub fn prepare(&mut self, proposal_number: u64) -> Option<(u64, Option<u64>)> {
        if proposal_number > self.promised {
            self.promised = proposal_number;
            Some((self.accepted_number, self.accepted_value))
        } else {
            None
        }
    }
    /// Phase 2 (Accept): accept if proposal_number >= promised.
    /// Returns true if accepted.
    pub fn accept(&mut self, proposal_number: u64, value: u64) -> bool {
        if proposal_number >= self.promised {
            self.accepted_number = proposal_number;
            self.accepted_value = Some(value);
            true
        } else {
            false
        }
    }
}
/// Types of distributed system messages.
#[derive(Clone, Debug)]
pub enum MessageContent {
    Heartbeat,
    VoteRequest {
        term: u64,
        log_length: usize,
    },
    VoteResponse {
        term: u64,
        granted: bool,
    },
    Prepare {
        proposal_number: u64,
    },
    Promise {
        proposal_number: u64,
        accepted: Option<(u64, u64)>,
    },
    Accept {
        proposal_number: u64,
        value: u64,
    },
    Accepted {
        proposal_number: u64,
    },
    Commit {
        value: u64,
    },
    Data {
        payload: Vec<u8>,
    },
}
/// Lamport timestamp (separate named struct per task spec).
#[derive(Debug, Clone)]
pub struct LamportTimestamp {
    /// Current logical time value.
    pub time: u64,
}
impl LamportTimestamp {
    /// Create a new Lamport timestamp at logical time `t`.
    pub fn new(time: u64) -> Self {
        LamportTimestamp { time }
    }
    /// Returns true iff `self` logically precedes `other` (self.time < other.time).
    pub fn logical_clock_ordering(&self, other: &LamportTimestamp) -> bool {
        self.time < other.time
    }
}
/// 2PC coordinator outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TwoPCOutcome {
    Committed,
    Aborted,
}
/// PN-Counter CRDT: increment/decrement counter using two G-Counters.
#[derive(Clone, Debug)]
pub struct PNCounter {
    /// Increments.
    pub positive: GCounter,
    /// Decrements.
    pub negative: GCounter,
}
impl PNCounter {
    /// Create a new PN-Counter.
    pub fn new(replica_id: usize, n_replicas: usize) -> Self {
        PNCounter {
            positive: GCounter::new(replica_id, n_replicas),
            negative: GCounter::new(replica_id, n_replicas),
        }
    }
    /// Increment the counter.
    pub fn increment(&mut self) {
        self.positive.increment();
    }
    /// Decrement the counter.
    pub fn decrement(&mut self) {
        self.negative.increment();
    }
    /// Return the current value.
    pub fn value(&self) -> i64 {
        self.positive.value() as i64 - self.negative.value() as i64
    }
    /// Merge with another PN-Counter state.
    pub fn merge(&mut self, other: &PNCounter) {
        self.positive.merge(&other.positive);
        self.negative.merge(&other.negative);
    }
}
/// 2P-Set CRDT: add/remove set. Once removed, an element cannot be re-added.
#[derive(Clone, Debug)]
pub struct TwoPSet<T: Clone + Eq + std::hash::Hash> {
    /// Elements that have been added.
    pub added: GSet<T>,
    /// Elements that have been removed (tombstone set).
    pub removed: GSet<T>,
}
impl<T: Clone + Eq + std::hash::Hash> TwoPSet<T> {
    pub fn new() -> Self {
        TwoPSet {
            added: GSet::new(),
            removed: GSet::new(),
        }
    }
    /// Add an element. Has no effect if already tombstoned.
    pub fn add(&mut self, elem: T) {
        if !self.removed.contains(&elem) {
            self.added.add(elem);
        }
    }
    /// Remove an element by adding it to the tombstone set.
    pub fn remove(&mut self, elem: T) {
        if self.added.contains(&elem) {
            self.removed.add(elem);
        }
    }
    /// Check if an element is currently in the set.
    pub fn contains(&self, elem: &T) -> bool {
        self.added.contains(elem) && !self.removed.contains(elem)
    }
    /// Merge two 2P-Sets.
    pub fn merge(&mut self, other: &TwoPSet<T>) {
        self.added.merge(&other.added);
        self.removed.merge(&other.removed);
    }
}
/// Raft protocol configuration.
#[derive(Debug, Clone)]
pub struct RaftProtocol {
    /// Number of servers in the Raft cluster.
    pub num_servers: usize,
}
impl RaftProtocol {
    /// Create a new Raft protocol configuration.
    pub fn new(num_servers: usize) -> Self {
        RaftProtocol { num_servers }
    }
    /// Describe the leader election mechanism.
    pub fn leader_election(&self) -> String {
        format!(
            "Randomized election timeouts; candidate needs votes from {} of {} servers",
            self.num_servers / 2 + 1,
            self.num_servers
        )
    }
    /// Describe log replication.
    pub fn log_replication(&self) -> &'static str {
        "Leader appends entry, sends AppendEntries RPCs to followers, commits when majority acknowledges"
    }
    /// Returns true iff a commit is possible given `acks` acknowledgements.
    pub fn commit_condition(&self, acks: usize) -> bool {
        acks >= self.num_servers / 2 + 1
    }
}
/// A message in a distributed system.
#[derive(Clone, Debug)]
pub struct Message {
    pub from: usize,
    pub to: usize,
    pub content: MessageContent,
    pub timestamp: u64,
}
/// G-Set CRDT: grow-only set.
#[derive(Clone, Debug)]
pub struct GSet<T: Clone + Eq + std::hash::Hash> {
    pub elements: HashSet<T>,
}
impl<T: Clone + Eq + std::hash::Hash> GSet<T> {
    pub fn new() -> Self {
        GSet {
            elements: HashSet::new(),
        }
    }
    pub fn add(&mut self, elem: T) {
        self.elements.insert(elem);
    }
    pub fn contains(&self, elem: &T) -> bool {
        self.elements.contains(elem)
    }
    pub fn merge(&mut self, other: &GSet<T>) {
        for e in &other.elements {
            self.elements.insert(e.clone());
        }
    }
}
/// FLP Impossibility: no deterministic algorithm solves consensus in an
/// asynchronous system with even one crash-faulty process.
#[derive(Debug, Clone)]
pub struct FLPImpossibility;
impl FLPImpossibility {
    /// Human-readable statement of the FLP result.
    pub fn statement(&self) -> &'static str {
        "No deterministic protocol can guarantee consensus in an asynchronous \
         message-passing system if even a single process may fail by crashing \
         (Fischer, Lynch, Paterson 1985)."
    }
    /// Returns true iff the given configuration renders deterministic consensus impossible.
    ///
    /// Returns `true` when `asynchronous && n_faulty >= 1`.
    pub fn no_deterministic_consensus_in_async(&self, asynchronous: bool, n_faulty: usize) -> bool {
        asynchronous && n_faulty >= 1
    }
}
/// Consensus algorithm variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusAlgorithm {
    /// Multi-Paxos: tolerates (n-1)/2 crash failures.
    Paxos,
    /// Raft: leader-based, tolerates (n-1)/2 crash failures.
    Raft,
    /// Practical Byzantine Fault Tolerance: tolerates (n-1)/3 Byzantine faults.
    PBFT,
    /// Tendermint BFT: tolerates (n-1)/3 Byzantine faults.
    Tendermint,
    /// HotStuff linear BFT: tolerates (n-1)/3 Byzantine faults.
    HotStuff,
}
impl ConsensusAlgorithm {
    /// Maximum number of faulty nodes tolerated for a system of `n` nodes.
    pub fn fault_tolerance(&self, n: usize) -> usize {
        match self {
            ConsensusAlgorithm::Paxos | ConsensusAlgorithm::Raft => {
                if n == 0 {
                    0
                } else {
                    (n - 1) / 2
                }
            }
            ConsensusAlgorithm::PBFT
            | ConsensusAlgorithm::Tendermint
            | ConsensusAlgorithm::HotStuff => {
                if n == 0 {
                    0
                } else {
                    (n - 1) / 3
                }
            }
        }
    }
    /// Message complexity per consensus round (big-O in n).
    pub fn message_complexity(&self) -> String {
        match self {
            ConsensusAlgorithm::Paxos => "O(n)".to_string(),
            ConsensusAlgorithm::Raft => "O(n)".to_string(),
            ConsensusAlgorithm::PBFT => "O(n^2)".to_string(),
            ConsensusAlgorithm::Tendermint => "O(n^2)".to_string(),
            ConsensusAlgorithm::HotStuff => "O(n)".to_string(),
        }
    }
}
/// Simplified Raft node state.
#[derive(Clone, Debug)]
pub struct RaftNode {
    pub id: usize,
    pub current_term: u64,
    pub role: RaftRole,
    pub voted_for: Option<usize>,
    pub log: Vec<(u64, u64)>,
    pub commit_index: usize,
    pub votes_received: HashSet<usize>,
}
impl RaftNode {
    pub fn new(id: usize) -> Self {
        RaftNode {
            id,
            current_term: 0,
            role: RaftRole::Follower,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            votes_received: HashSet::new(),
        }
    }
    /// Start an election: transition to Candidate, increment term, vote for self.
    pub fn start_election(&mut self) {
        self.current_term += 1;
        self.role = RaftRole::Candidate;
        self.voted_for = Some(self.id);
        self.votes_received.clear();
        self.votes_received.insert(self.id);
    }
    /// Handle a RequestVote RPC from candidate `candidate_id` in `term`.
    /// Returns true if vote is granted.
    pub fn request_vote(
        &mut self,
        candidate_id: usize,
        term: u64,
        candidate_log_len: usize,
    ) -> bool {
        if term > self.current_term {
            self.current_term = term;
            self.role = RaftRole::Follower;
            self.voted_for = None;
        }
        let can_vote = term == self.current_term
            && (self.voted_for.is_none() || self.voted_for == Some(candidate_id))
            && candidate_log_len >= self.log.len();
        if can_vote {
            self.voted_for = Some(candidate_id);
        }
        can_vote
    }
    /// Record a vote grant. Returns true if quorum achieved.
    pub fn receive_vote(&mut self, voter_id: usize, n_nodes: usize) -> bool {
        self.votes_received.insert(voter_id);
        if self.votes_received.len() > n_nodes / 2 {
            self.role = RaftRole::Leader;
            true
        } else {
            false
        }
    }
    /// Append an entry to the log (leader only).
    pub fn append_entry(&mut self, value: u64) -> bool {
        if self.role != RaftRole::Leader {
            return false;
        }
        self.log.push((self.current_term, value));
        true
    }
    /// Handle AppendEntries from leader (heartbeat / log replication).
    pub fn append_entries(
        &mut self,
        leader_term: u64,
        entries: &[(u64, u64)],
        leader_commit: usize,
    ) -> bool {
        if leader_term < self.current_term {
            return false;
        }
        self.current_term = leader_term;
        self.role = RaftRole::Follower;
        for &entry in entries {
            self.log.push(entry);
        }
        if leader_commit > self.commit_index {
            self.commit_index = leader_commit.min(self.log.len());
        }
        true
    }
}
/// Consistent hash ring for key-to-node assignment.
///
/// Uses a sorted array of virtual node positions to implement
/// the consistent hashing algorithm for load-balanced key distribution.
#[allow(dead_code)]
pub struct ConsistentHashRing {
    /// Sorted list of (virtual_position, node_id) pairs.
    pub ring: Vec<(u64, usize)>,
    /// Number of virtual nodes per physical node.
    pub virtual_nodes: usize,
    /// Total number of physical nodes.
    pub num_nodes: usize,
}
impl ConsistentHashRing {
    /// Create a consistent hash ring with `num_nodes` physical nodes and
    /// `virtual_nodes` virtual nodes each.
    pub fn new(num_nodes: usize, virtual_nodes: usize) -> Self {
        let mut ring = Vec::with_capacity(num_nodes * virtual_nodes);
        for node_id in 0..num_nodes {
            for vn in 0..virtual_nodes {
                let pos = dst_ext_mix_hash(node_id as u64, vn as u64);
                ring.push((pos, node_id));
            }
        }
        ring.sort_by_key(|&(pos, _)| pos);
        ConsistentHashRing {
            ring,
            virtual_nodes,
            num_nodes,
        }
    }
    /// Assign a key to a node by finding the successor on the ring.
    pub fn get_node(&self, key: u64) -> usize {
        if self.ring.is_empty() {
            return 0;
        }
        match self.ring.binary_search_by_key(&key, |&(pos, _)| pos) {
            Ok(i) => self.ring[i].1,
            Err(i) => self.ring[i % self.ring.len()].1,
        }
    }
    /// Compute the distribution of keys across nodes for a range of keys.
    /// Returns a Vec of length num_nodes with key counts.
    pub fn key_distribution(&self, num_keys: u64) -> Vec<u64> {
        let mut counts = vec![0u64; self.num_nodes];
        for k in 0..num_keys {
            let node = self.get_node(k * 2654435761);
            if node < self.num_nodes {
                counts[node] += 1;
            }
        }
        counts
    }
    /// Compute load imbalance: (max_load - min_load) / avg_load.
    pub fn load_imbalance(&self, num_keys: u64) -> f64 {
        let dist = self.key_distribution(num_keys);
        if dist.is_empty() || num_keys == 0 {
            return 0.0;
        }
        let max_load = *dist
            .iter()
            .max()
            .expect("dist is non-empty: checked by early return") as f64;
        let min_load = *dist
            .iter()
            .min()
            .expect("dist is non-empty: checked by early return") as f64;
        let avg_load = num_keys as f64 / self.num_nodes as f64;
        if avg_load == 0.0 {
            0.0
        } else {
            (max_load - min_load) / avg_load
        }
    }
}
/// A TLA+ specification: an initial condition, a next-state relation, and invariants.
pub struct TLASpec {
    /// Initial states.
    pub init_states: Vec<TLAState>,
    /// Next-state transitions: (from_state, to_state).
    pub transitions: Vec<(TLAState, TLAState)>,
    /// Safety invariants: predicates on states.
    pub invariants: Vec<Box<dyn Fn(&TLAState) -> bool>>,
}
impl TLASpec {
    pub fn new() -> Self {
        TLASpec {
            init_states: Vec::new(),
            transitions: Vec::new(),
            invariants: Vec::new(),
        }
    }
    /// Add an initial state.
    pub fn add_init(&mut self, s: TLAState) {
        self.init_states.push(s);
    }
    /// Add a state transition.
    pub fn add_transition(&mut self, from: TLAState, to: TLAState) {
        self.transitions.push((from, to));
    }
    /// Add a safety invariant.
    pub fn add_invariant<F: Fn(&TLAState) -> bool + 'static>(&mut self, f: F) {
        self.invariants.push(Box::new(f));
    }
    /// Check all invariants on all reachable states (BFS from initial states).
    /// Returns `Ok(())` if all invariants hold, or `Err(state)` for the first violation.
    pub fn check_invariants(&self) -> Result<usize, TLAState> {
        let mut visited: HashSet<Vec<(String, i64)>> = HashSet::new();
        let mut queue = VecDeque::new();
        for s in &self.init_states {
            queue.push_back(s.clone());
        }
        let mut states_checked = 0;
        while let Some(state) = queue.pop_front() {
            let mut kv: Vec<(String, i64)> =
                state.vars.iter().map(|(k, &v)| (k.clone(), v)).collect();
            kv.sort();
            if !visited.insert(kv) {
                continue;
            }
            states_checked += 1;
            for inv in &self.invariants {
                if !inv(&state) {
                    return Err(state);
                }
            }
            for (from, to) in &self.transitions {
                if from == &state {
                    queue.push_back(to.clone());
                }
            }
        }
        Ok(states_checked)
    }
}
/// Two-phase commit (2PC) coordinator.
///
/// Coordinates distributed transactions across multiple participants.
/// Phase 1 (Prepare): ask all participants to vote Yes/No.
/// Phase 2 (Commit/Abort): if all voted Yes, commit; else abort.
#[allow(dead_code)]
pub struct TwoPhaseCommitCoordinator {
    /// Number of participants.
    pub num_participants: usize,
    /// Current transaction id.
    pub transaction_id: u64,
    /// Votes collected in Phase 1.
    pub votes: Vec<bool>,
}
impl TwoPhaseCommitCoordinator {
    /// Create a new 2PC coordinator for `n` participants.
    pub fn new(n: usize) -> Self {
        TwoPhaseCommitCoordinator {
            num_participants: n,
            transaction_id: 0,
            votes: Vec::new(),
        }
    }
    /// Begin a new transaction. Returns the transaction id.
    pub fn begin(&mut self) -> u64 {
        self.transaction_id += 1;
        self.votes.clear();
        self.transaction_id
    }
    /// Record a vote from a participant.
    pub fn record_vote(&mut self, vote: bool) {
        self.votes.push(vote);
    }
    /// Phase 2 decision: commit iff all participants voted Yes.
    /// Returns true for commit, false for abort.
    pub fn decide(&self) -> bool {
        self.votes.len() == self.num_participants && self.votes.iter().all(|&v| v)
    }
    /// Check if we have collected all votes.
    pub fn all_voted(&self) -> bool {
        self.votes.len() == self.num_participants
    }
    /// Count Yes votes.
    pub fn yes_count(&self) -> usize {
        self.votes.iter().filter(|&&v| v).count()
    }
}
/// Participant state in 3PC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreePCState {
    Init,
    PrePared,
    PreCommitted,
    Committed,
    Aborted,
}
/// CAP Theorem: a distributed system cannot simultaneously guarantee all three
/// of Consistency, Availability, and Partition-tolerance.
#[derive(Debug, Clone)]
pub struct CAPTheorem {
    /// Whether the system guarantees strong consistency.
    pub consistency: bool,
    /// Whether the system guarantees availability.
    pub availability: bool,
    /// Whether the system handles network partitions.
    pub partition_tolerance: bool,
}
impl CAPTheorem {
    /// Create a new CAP configuration.
    pub fn new(consistency: bool, availability: bool, partition_tolerance: bool) -> Self {
        CAPTheorem {
            consistency,
            availability,
            partition_tolerance,
        }
    }
    /// Describe the trade-off: returns the two properties that are achievable.
    pub fn cap_tradeoff(&self) -> &'static str {
        match (
            self.consistency,
            self.availability,
            self.partition_tolerance,
        ) {
            (true, true, false) => "CA: consistent and available, but no partition tolerance",
            (true, false, true) => {
                "CP: consistent and partition-tolerant, but not always available"
            }
            (false, true, true) => {
                "AP: available and partition-tolerant, but only eventual consistency"
            }
            _ => "At most two of C, A, P can be guaranteed simultaneously (Brewer 2000)",
        }
    }
    /// Returns true iff this configuration is achievable (at most two properties set).
    pub fn brewer_theorem(&self) -> bool {
        let count =
            self.consistency as u8 + self.availability as u8 + self.partition_tolerance as u8;
        count <= 2
    }
}
/// LWW-Register (Last-Write-Wins): op-based CRDT with timestamps.
#[derive(Clone, Debug)]
pub struct LWWRegister<T: Clone> {
    /// Current value.
    pub value: Option<T>,
    /// Timestamp of the last write.
    pub timestamp: u64,
}
impl<T: Clone> LWWRegister<T> {
    pub fn new() -> Self {
        LWWRegister {
            value: None,
            timestamp: 0,
        }
    }
    /// Write a value with the given timestamp. Accepted only if timestamp is newer.
    pub fn write(&mut self, val: T, ts: u64) {
        if ts > self.timestamp {
            self.value = Some(val);
            self.timestamp = ts;
        }
    }
    /// Merge with another replica's state (higher timestamp wins).
    pub fn merge(&mut self, other: &LWWRegister<T>) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}
/// Classify a system design by which two CAP properties it prioritizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPClass {
    /// CP: Consistent + Partition-tolerant (e.g., HBase, ZooKeeper)
    CP,
    /// AP: Available + Partition-tolerant (e.g., Cassandra, DynamoDB)
    AP,
    /// CA: Consistent + Available (not partition-tolerant; e.g., traditional RDBMS)
    CA,
}
