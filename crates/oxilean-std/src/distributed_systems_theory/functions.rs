//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    ByzantineFaultTolerance, CAPClass, CAPProperty, CAPTheorem, ConsistencyLevel, FLPImpossibility,
    GCounter, LamportClock, LamportTimestamp, PNCounter, PaxosAcceptor, PaxosProtocol, RaftNode,
    RaftProtocol, RaftRole, ThreePCParticipant, TwoPCOutcome, TwoPCParticipant, TwoPCState,
    TwoPSet, VectorClock,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn real_ty() -> Expr {
    cst("Real")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
/// `DistributedSystem : Nat → Prop` — a system with n nodes
pub fn distributed_system_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `CAPConsistency : Prop → Prop` — system satisfies strong consistency
pub fn cap_consistency_ty() -> Expr {
    arrow(prop(), prop())
}
/// `CAPAvailability : Prop → Prop` — system satisfies availability
pub fn cap_availability_ty() -> Expr {
    arrow(prop(), prop())
}
/// `CAPPartitionTolerance : Prop → Prop` — system tolerates network partitions
pub fn cap_partition_tolerance_ty() -> Expr {
    arrow(prop(), prop())
}
/// `CAPTheorem : Prop → Prop`
/// No distributed system can simultaneously guarantee all three CAP properties.
pub fn cap_theorem_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ConsensusProtocol : Nat → Prop` — consensus protocol for n nodes
pub fn consensus_protocol_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PaxosProtocol : Nat → Nat → Prop` — Paxos with n nodes and f fault tolerance
pub fn paxos_protocol_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `RaftProtocol : Nat → Nat → Prop` — Raft consensus protocol
pub fn raft_protocol_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PBFTProtocol : Nat → Nat → Prop` — Practical Byzantine Fault Tolerance
pub fn pbft_protocol_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ByzantineFaultTolerance : Prop → Nat → Prop`
/// Protocol P tolerates up to f Byzantine faults.
pub fn byzantine_fault_tolerance_ty() -> Expr {
    arrow(prop(), arrow(nat_ty(), prop()))
}
/// `AtomicBroadcast : Prop → Prop` — total-order multicast
pub fn atomic_broadcast_ty() -> Expr {
    arrow(prop(), prop())
}
/// `VectorClock : Nat → Prop` — vector clock for n processes
pub fn vector_clock_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `LamportTimestamp : Prop → Nat` — Lamport logical timestamp of event
pub fn lamport_timestamp_ty() -> Expr {
    arrow(prop(), nat_ty())
}
/// `CausallyPrecedes : Prop → Prop → Prop` — happens-before relation
pub fn causally_precedes_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// `CRDT : Prop → Prop` — a Conflict-free Replicated Data Type
pub fn crdt_ty() -> Expr {
    arrow(prop(), prop())
}
/// `StateBasedCRDT : Prop → Prop` — state-based (convergent) CRDT
pub fn state_based_crdt_ty() -> Expr {
    arrow(prop(), prop())
}
/// `OpBasedCRDT : Prop → Prop` — operation-based (commutative) CRDT
pub fn op_based_crdt_ty() -> Expr {
    arrow(prop(), prop())
}
/// `EventualConsistency : Prop → Prop`
pub fn eventual_consistency_ty() -> Expr {
    arrow(prop(), prop())
}
/// `StrongEventualConsistency : Prop → Prop`
pub fn strong_eventual_consistency_ty() -> Expr {
    arrow(prop(), prop())
}
/// `Linearizability : Prop → Prop` — correctness condition for concurrent objects
pub fn linearizability_ty() -> Expr {
    arrow(prop(), prop())
}
/// `Serializability : Prop → Prop` — correctness condition for transactions
pub fn serializability_ty() -> Expr {
    arrow(prop(), prop())
}
/// `StrictSerializability : Prop → Prop` — serializability + real-time order
pub fn strict_serializability_ty() -> Expr {
    arrow(prop(), prop())
}
/// `TwoPhaseCommit : Nat → Prop` — 2PC protocol with n participants
pub fn two_phase_commit_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ThreePhaseCommit : Nat → Prop` — 3PC protocol (non-blocking)
pub fn three_phase_commit_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `SafetyProperty : Prop → Prop` — TLA+ style safety predicate
pub fn safety_property_ty() -> Expr {
    arrow(prop(), prop())
}
/// `LivenessProperty : Prop → Prop` — TLA+ style liveness predicate
pub fn liveness_property_ty() -> Expr {
    arrow(prop(), prop())
}
/// `FLPImpossibility : Prop`
/// In an asynchronous system with even one crash failure, consensus is impossible.
pub fn flp_impossibility_ty() -> Expr {
    prop()
}
/// `FischerLynchPatersonThm : Prop → Prop`
/// The FLP impossibility result.
pub fn flp_theorem_ty() -> Expr {
    arrow(prop(), prop())
}
/// `ByzantineGeneralsProblem : Nat → Nat → Prop`
/// Byzantine Generals: n generals, f traitors; solvable iff n ≥ 3f+1.
pub fn byzantine_generals_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PaxosCorrectness : Prop → Prop` — Paxos safety + liveness under benign failures
pub fn paxos_correctness_ty() -> Expr {
    arrow(prop(), prop())
}
/// `RaftLeaderElection : Prop → Prop` — Raft elects at most one leader per term
pub fn raft_leader_election_ty() -> Expr {
    arrow(prop(), prop())
}
/// `VectorClockConsistency : Prop → Prop`
/// Vector clocks characterize causal ordering exactly.
pub fn vector_clock_consistency_ty() -> Expr {
    arrow(prop(), prop())
}
/// Build the distributed systems theory environment.
pub fn build_distributed_systems_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("DistributedSystem", distributed_system_ty()),
        ("CAPConsistency", cap_consistency_ty()),
        ("CAPAvailability", cap_availability_ty()),
        ("CAPPartitionTolerance", cap_partition_tolerance_ty()),
        ("CAPTheorem", cap_theorem_ty()),
        ("ConsensusProtocol", consensus_protocol_ty()),
        ("PaxosProtocol", paxos_protocol_ty()),
        ("RaftProtocol", raft_protocol_ty()),
        ("PBFTProtocol", pbft_protocol_ty()),
        ("ByzantineFaultTolerance", byzantine_fault_tolerance_ty()),
        ("AtomicBroadcast", atomic_broadcast_ty()),
        ("VectorClock", vector_clock_ty()),
        ("LamportTimestamp", lamport_timestamp_ty()),
        ("CausallyPrecedes", causally_precedes_ty()),
        ("CRDT", crdt_ty()),
        ("StateBasedCRDT", state_based_crdt_ty()),
        ("OpBasedCRDT", op_based_crdt_ty()),
        ("EventualConsistency", eventual_consistency_ty()),
        (
            "StrongEventualConsistency",
            strong_eventual_consistency_ty(),
        ),
        ("Linearizability", linearizability_ty()),
        ("Serializability", serializability_ty()),
        ("StrictSerializability", strict_serializability_ty()),
        ("TwoPhaseCommit", two_phase_commit_ty()),
        ("ThreePhaseCommit", three_phase_commit_ty()),
        ("SafetyProperty", safety_property_ty()),
        ("LivenessProperty", liveness_property_ty()),
        ("FLPImpossibility", flp_impossibility_ty()),
        ("FLPTheorem", flp_theorem_ty()),
        ("ByzantineGeneralsProblem", byzantine_generals_ty()),
        ("PaxosCorrectness", paxos_correctness_ty()),
        ("RaftLeaderElection", raft_leader_election_ty()),
        ("VectorClockConsistency", vector_clock_consistency_ty()),
        ("ProcessId", nat_ty()),
        ("LogicalTime", nat_ty()),
        ("Term", nat_ty()),
        ("LogIndex", nat_ty()),
        ("NodeState", arrow(nat_ty(), prop())),
        ("MessageHistory", list_ty(prop())),
        ("TransactionLog", list_ty(prop())),
        ("PartitionEvent", prop()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
/// Run a single round of Paxos with `acceptors`.
///
/// The proposer uses `proposal_number` and `proposed_value`.
/// Returns `Some(decided_value)` if a quorum is reached, `None` otherwise.
pub fn paxos_round(
    acceptors: &mut Vec<PaxosAcceptor>,
    proposal_number: u64,
    proposed_value: u64,
) -> Option<u64> {
    let n = acceptors.len();
    let quorum = n / 2 + 1;
    let mut promises = Vec::new();
    for acc in acceptors.iter_mut() {
        if let Some(p) = acc.prepare(proposal_number) {
            promises.push(p);
        }
    }
    if promises.len() < quorum {
        return None;
    }
    let value_to_propose = promises
        .iter()
        .filter(|(n, v)| *n > 0 && v.is_some())
        .max_by_key(|(n, _)| *n)
        .and_then(|(_, v)| *v)
        .unwrap_or(proposed_value);
    let mut accepts = 0;
    for acc in acceptors.iter_mut() {
        if acc.accept(proposal_number, value_to_propose) {
            accepts += 1;
        }
    }
    if accepts >= quorum {
        Some(value_to_propose)
    } else {
        None
    }
}
/// Run a full 2PC protocol.
///
/// Returns `Committed` if all participants vote yes, `Aborted` otherwise.
pub fn two_phase_commit(participants: &mut Vec<TwoPCParticipant>) -> TwoPCOutcome {
    let all_yes = participants.iter_mut().all(|p| p.prepare());
    let outcome = if all_yes {
        TwoPCOutcome::Committed
    } else {
        TwoPCOutcome::Aborted
    };
    for p in participants.iter_mut() {
        p.decide(outcome == TwoPCOutcome::Committed);
    }
    outcome
}
/// Run a full 3PC protocol. Returns `true` if committed.
pub fn three_phase_commit(participants: &mut Vec<ThreePCParticipant>) -> bool {
    let all_yes = participants.iter_mut().all(|p| p.can_commit());
    if !all_yes {
        for p in participants.iter_mut() {
            p.abort();
        }
        return false;
    }
    for p in participants.iter_mut() {
        p.pre_commit();
    }
    for p in participants.iter_mut() {
        p.do_commit();
    }
    true
}
/// Check if a system of `n` nodes can tolerate `f` Byzantine faults.
///
/// The classical result: need n ≥ 3f+1 for Byzantine agreement.
pub fn can_tolerate_byzantine(n: usize, f: usize) -> bool {
    n > 3 * f
}
/// Compute the maximum number of Byzantine faults tolerated by `n` nodes.
pub fn max_byzantine_faults(n: usize) -> usize {
    if n < 4 {
        0
    } else {
        (n - 1) / 3
    }
}
/// Check if FLP impossibility applies to the given system model.
///
/// FLP applies to asynchronous systems with at least 1 crash-faulty node.
/// Returns `true` if consensus is impossible (async + crash-faulty).
pub fn flp_impossibility(asynchronous: bool, n_faulty: usize) -> bool {
    asynchronous && n_faulty >= 1
}
/// Given the chosen CAP properties, determine if the combination is achievable.
///
/// According to the CAP theorem, a distributed system can satisfy at most 2 of 3.
/// Returns `true` if the combination is feasible.
pub fn cap_feasible(properties: &HashSet<CAPProperty>) -> bool {
    !(properties.contains(&CAPProperty::Consistency)
        && properties.contains(&CAPProperty::Availability)
        && properties.contains(&CAPProperty::PartitionTolerance))
}
/// Determine the CAP class from the set of supported properties.
pub fn cap_classify(properties: &HashSet<CAPProperty>) -> Option<CAPClass> {
    let c = properties.contains(&CAPProperty::Consistency);
    let a = properties.contains(&CAPProperty::Availability);
    let p = properties.contains(&CAPProperty::PartitionTolerance);
    match (c, a, p) {
        (true, false, true) => Some(CAPClass::CP),
        (false, true, true) => Some(CAPClass::AP),
        (true, true, false) => Some(CAPClass::CA),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lamport_clock() {
        let mut c1 = LamportClock::new();
        let mut c2 = LamportClock::new();
        let t1 = c1.tick();
        assert_eq!(t1, 1);
        let t2 = c1.tick();
        assert_eq!(t2, 2);
        let t3 = c2.receive(t2);
        assert_eq!(t3, 3, "After receiving t=2, c2 should be at max(0,2)+1 = 3");
        let t4 = c2.tick();
        assert_eq!(t4, 4);
    }
    #[test]
    fn test_vector_clock_causality() {
        let mut vc1 = VectorClock::new(0, 2);
        let mut vc2 = VectorClock::new(1, 2);
        vc1.tick();
        vc2.receive(&vc1);
        vc2.tick();
        assert!(
            vc1.causally_precedes(&vc2),
            "vc1 should causally precede vc2"
        );
        assert!(
            !vc2.causally_precedes(&vc1),
            "vc2 should NOT causally precede vc1"
        );
        let mut vc3 = VectorClock::new(1, 2);
        vc3.tick();
        let vc1_copy = VectorClock {
            clocks: vec![1, 0],
            process_id: 0,
        };
        assert!(
            vc1_copy.concurrent(&vc3),
            "vc1=[1,0] and vc3=[0,1] should be concurrent"
        );
    }
    #[test]
    fn test_g_counter_crdt() {
        let mut r0 = GCounter::new(0, 3);
        let mut r1 = GCounter::new(1, 3);
        let mut r2 = GCounter::new(2, 3);
        r0.increment_by(5);
        r1.increment_by(3);
        r2.increment_by(2);
        r0.merge(&r1);
        r0.merge(&r2);
        assert_eq!(r0.value(), 10, "G-Counter total should be 5+3+2=10");
        r0.merge(&r1);
        assert_eq!(r0.value(), 10, "G-Counter merge should be idempotent");
    }
    #[test]
    fn test_pn_counter_crdt() {
        let mut r0 = PNCounter::new(0, 2);
        let mut r1 = PNCounter::new(1, 2);
        r0.increment();
        r0.increment();
        r1.increment();
        r1.decrement();
        r0.merge(&r1);
        assert_eq!(r0.value(), 2, "PN-Counter: 2 + 0 = 2");
    }
    #[test]
    fn test_two_pset_crdt() {
        let mut s1: TwoPSet<u32> = TwoPSet::new();
        let mut s2: TwoPSet<u32> = TwoPSet::new();
        s1.add(1u32);
        s1.add(2u32);
        s2.add(2u32);
        s2.add(3u32);
        s1.remove(2u32);
        s1.merge(&s2);
        assert!(s1.contains(&1u32), "1 should be present");
        assert!(!s1.contains(&2u32), "2 was removed, should not be present");
        assert!(s1.contains(&3u32), "3 should be present from s2");
        s1.add(2u32);
        assert!(!s1.contains(&2u32), "Cannot re-add tombstoned element");
    }
    #[test]
    fn test_paxos_round() {
        let mut acceptors: Vec<PaxosAcceptor> = (0..5).map(|_| PaxosAcceptor::new()).collect();
        let result = paxos_round(&mut acceptors, 1, 42);
        assert_eq!(
            result,
            Some(42),
            "Paxos should decide value 42 with fresh acceptors"
        );
        let result2 = paxos_round(&mut acceptors, 2, 99);
        assert!(result2.is_some(), "Second Paxos round should also decide");
        assert_eq!(
            result2,
            Some(42),
            "Paxos should preserve previously accepted value 42"
        );
    }
    #[test]
    fn test_raft_election() {
        let n = 5;
        let mut nodes: Vec<RaftNode> = (0..n).map(RaftNode::new).collect();
        nodes[0].start_election();
        assert_eq!(nodes[0].role, RaftRole::Candidate);
        assert_eq!(nodes[0].current_term, 1);
        let term = nodes[0].current_term;
        let log_len = nodes[0].log.len();
        for voter_id in 1..n {
            let granted = nodes[voter_id].request_vote(0, term, log_len);
            if granted {
                let became_leader = nodes[0].receive_vote(voter_id, n);
                if became_leader {
                    break;
                }
            }
        }
        assert_eq!(
            nodes[0].role,
            RaftRole::Leader,
            "Node 0 should win election with majority"
        );
    }
    #[test]
    fn test_two_phase_commit() {
        let mut ps: Vec<TwoPCParticipant> =
            (0..4).map(|i| TwoPCParticipant::new(i, true)).collect();
        let outcome = two_phase_commit(&mut ps);
        assert_eq!(outcome, TwoPCOutcome::Committed, "All-yes should commit");
        assert!(ps.iter().all(|p| p.state == TwoPCState::Committed));
        let mut ps2: Vec<TwoPCParticipant> = vec![
            TwoPCParticipant::new(0, true),
            TwoPCParticipant::new(1, false),
            TwoPCParticipant::new(2, true),
        ];
        let outcome2 = two_phase_commit(&mut ps2);
        assert_eq!(outcome2, TwoPCOutcome::Aborted, "One-no should abort");
        assert!(ps2.iter().all(|p| p.state == TwoPCState::Aborted));
    }
    #[test]
    fn test_cap_theorem() {
        let cp: HashSet<CAPProperty> = [CAPProperty::Consistency, CAPProperty::PartitionTolerance]
            .iter()
            .cloned()
            .collect();
        assert!(cap_feasible(&cp), "CP should be feasible");
        assert_eq!(cap_classify(&cp), Some(CAPClass::CP));
        let all: HashSet<CAPProperty> = [
            CAPProperty::Consistency,
            CAPProperty::Availability,
            CAPProperty::PartitionTolerance,
        ]
        .iter()
        .cloned()
        .collect();
        assert!(!cap_feasible(&all), "All-three CAP should be infeasible");
        assert_eq!(cap_classify(&all), None);
    }
    #[test]
    fn test_byzantine_faults() {
        assert!(
            can_tolerate_byzantine(4, 1),
            "4 nodes can tolerate 1 Byzantine fault"
        );
        assert!(
            !can_tolerate_byzantine(3, 1),
            "3 nodes cannot tolerate 1 Byzantine fault"
        );
        assert_eq!(
            max_byzantine_faults(10),
            3,
            "10 nodes: max Byzantine faults = 3"
        );
        assert_eq!(
            max_byzantine_faults(3),
            0,
            "3 nodes: cannot tolerate any Byzantine fault"
        );
        assert!(flp_impossibility(true, 1), "Async + 1 faulty: FLP applies");
        assert!(
            !flp_impossibility(false, 1),
            "Sync system: FLP does not apply"
        );
    }
    #[test]
    fn test_consistency_levels() {
        assert!(ConsistencyLevel::Linearizable.implies(ConsistencyLevel::Eventual));
        assert!(ConsistencyLevel::Linearizable.implies(ConsistencyLevel::Causal));
        assert!(!ConsistencyLevel::Eventual.implies(ConsistencyLevel::Causal));
        assert_eq!(ConsistencyLevel::Linearizable.name(), "Linearizable");
        assert_eq!(ConsistencyLevel::Eventual.name(), "Eventual");
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_distributed_systems_env(&mut env);
        assert!(env.get(&Name::str("PaxosProtocol")).is_some());
        assert!(env.get(&Name::str("VectorClock")).is_some());
        assert!(env.get(&Name::str("CAPTheorem")).is_some());
        assert!(env.get(&Name::str("TwoPhaseCommit")).is_some());
        assert!(env.get(&Name::str("FLPImpossibility")).is_some());
    }
}
/// Build the distributed systems theory environment (alias for build_distributed_systems_env).
pub fn build_env(env: &mut Environment) {
    build_distributed_systems_env(env);
}
/// `FLPImpossibilityStrong : Nat → Prop`
/// In any asynchronous model with ≥1 crash failure, consensus on n nodes is impossible.
pub fn dst_ext_flp_impossibility_strong_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `AsyncConsensusLowerBound : Nat → Nat → Prop`
/// Any asynchronous consensus protocol must take Ω(f·log n) message rounds.
pub fn dst_ext_async_consensus_lower_bound_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `CrashFailureModel : Nat → Nat → Prop`
/// System with n processes where at most f can crash.
pub fn dst_ext_crash_failure_model_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `OmissionFailureModel : Nat → Nat → Prop`
/// System where processes may omit sends/receives.
pub fn dst_ext_omission_failure_model_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `TimedAsynchronousModel : Real → Real → Prop`
/// Partially synchronous model with bounds on message delay.
pub fn dst_ext_timed_async_model_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `PaxosSafety : Prop`
/// Paxos never decides two different values (agreement property).
pub fn dst_ext_paxos_safety_ty() -> Expr {
    prop()
}
/// `PaxosLiveness : Nat → Prop`
/// Paxos eventually decides a value when at most f < n/2 processes fail.
pub fn dst_ext_paxos_liveness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `MultiPaxos : Nat → Nat → Prop`
/// Multi-Paxos protocol for replicated state machine with n replicas and f faults.
pub fn dst_ext_multi_paxos_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PaxosPhaseOneMessage : Nat → Prop`
/// Paxos Phase 1 (Prepare/Promise) message type parametrized by ballot number.
pub fn dst_ext_paxos_phase1_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PaxosPhaseTwoMessage : Nat → Real → Prop`
/// Paxos Phase 2 (Accept/Accepted) message type.
pub fn dst_ext_paxos_phase2_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `RaftLeaderUniqueness : Prop`
/// In any given term, at most one leader is elected.
pub fn dst_ext_raft_leader_uniqueness_ty() -> Expr {
    prop()
}
/// `RaftLogMatching : Prop`
/// If two logs have an entry with the same index and term, they agree on all prior entries.
pub fn dst_ext_raft_log_matching_ty() -> Expr {
    prop()
}
/// `RaftLeaderCompleteness : Prop`
/// A committed log entry will always be present in all future leaders' logs.
pub fn dst_ext_raft_leader_completeness_ty() -> Expr {
    prop()
}
/// `RaftStateMachineSafety : Prop`
/// All state machines apply the same commands in the same order.
pub fn dst_ext_raft_state_machine_safety_ty() -> Expr {
    prop()
}
/// `RaftMembershipChange : Nat → Nat → Prop`
/// Joint consensus for cluster membership changes from n1 to n2 nodes.
pub fn dst_ext_raft_membership_change_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `PBFTSafety : Nat → Prop`
/// PBFT guarantees safety with n ≥ 3f+1 replicas and up to f Byzantine faults.
pub fn dst_ext_pbft_safety_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `PBFTLiveness : Nat → Prop`
/// PBFT guarantees liveness under weak synchrony assumptions.
pub fn dst_ext_pbft_liveness_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `ThresholdSignature : Nat → Nat → Prop`
/// (t, n)-threshold signature scheme: t-of-n parties must cooperate to sign.
pub fn dst_ext_threshold_signature_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ByzantineAgreement : Nat → Nat → Prop`
/// Byzantine agreement with n generals and f traitors is solvable iff n ≥ 3f+1.
pub fn dst_ext_byzantine_agreement_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `AuthenticatedByzantine : Nat → Nat → Prop`
/// Byzantine agreement with digital signatures requires only n ≥ 2f+1.
pub fn dst_ext_authenticated_byzantine_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ConsistentHashing : Nat → Nat → Prop`
/// Consistent hashing ring with n virtual nodes and k keys.
pub fn dst_ext_consistent_hashing_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), prop()))
}
/// `ConsistentHashingBalance : Nat → Real → Prop`
/// Expected load per node is (1/n) ± ε with high probability.
pub fn dst_ext_consistent_hashing_balance_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `DHTChord : Nat → Prop`
/// Chord DHT with identifier space 2^n, O(log n) lookup hops.
pub fn dst_ext_dht_chord_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DHTKademlia : Nat → Prop`
/// Kademlia DHT using XOR metric for routing table.
pub fn dst_ext_dht_kademlia_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `DHTLookupComplexity : Nat → Prop`
/// Any DHT lookup terminates in O(log n) steps for n nodes.
pub fn dst_ext_dht_lookup_complexity_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// `EventualConsistencyConvergence : Real → Prop`
/// After quiescing for time δ, all replicas return the same value.
pub fn dst_ext_eventual_consistency_convergence_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `MonotonicReadConsistency : Prop`
/// Once a client reads a value, subsequent reads never return older values.
pub fn dst_ext_monotonic_read_consistency_ty() -> Expr {
    prop()
}
/// `MonotonicWriteConsistency : Prop`
/// A process's writes are serialized in the order they were issued.
pub fn dst_ext_monotonic_write_consistency_ty() -> Expr {
    prop()
}
/// `ReadYourWritesConsistency : Prop`
/// A process always sees its own writes reflected in subsequent reads.
pub fn dst_ext_read_your_writes_ty() -> Expr {
    prop()
}
/// `WritesFollowReadsConsistency : Prop`
/// Writes following a read are sequenced after the read's value.
pub fn dst_ext_writes_follow_reads_ty() -> Expr {
    prop()
}
/// `CRDTStrongEventualConsistency : Prop`
/// CRDTs guarantee SEC: all replicas that receive the same updates converge.
pub fn dst_ext_crdt_sec_ty() -> Expr {
    prop()
}
/// `CRDTJoinSemilattice : Prop`
/// State-based CRDT state forms a join-semilattice with ⊔ as merge.
pub fn dst_ext_crdt_join_semilattice_ty() -> Expr {
    prop()
}
/// `CRDTInflation : Prop`
/// Every CRDTupdate is monotone: s ⊑ update(s).
pub fn dst_ext_crdt_inflation_ty() -> Expr {
    prop()
}
/// `OrSetCRDT : Prop`
/// Observed-Remove Set: add wins over concurrent remove.
pub fn dst_ext_or_set_crdt_ty() -> Expr {
    prop()
}
/// `MVRegisterCRDT : Prop`
/// Multi-Value Register: concurrent writes create a set of concurrent values.
pub fn dst_ext_mv_register_crdt_ty() -> Expr {
    prop()
}
/// `NTPSyncBound : Real → Real → Prop`
/// NTP achieves clock synchronization within δ ms of UTC given RTT ≤ Δ ms.
pub fn dst_ext_ntp_sync_bound_ty() -> Expr {
    arrow(real_ty(), arrow(real_ty(), prop()))
}
/// `ClockDriftRate : Real → Prop`
/// A physical clock drifts at most ρ from real time.
pub fn dst_ext_clock_drift_rate_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `ChristianAlgorithm : Real → Prop`
/// Christian's algorithm achieves synchronization within (Δ/2) ms.
pub fn dst_ext_christian_algorithm_ty() -> Expr {
    arrow(real_ty(), prop())
}
/// `BerkeleyAlgorithm : Nat → Real → Prop`
/// Berkeley average-based clock sync for n nodes with drift ≤ δ.
pub fn dst_ext_berkeley_algorithm_ty() -> Expr {
    arrow(nat_ty(), arrow(real_ty(), prop()))
}
/// `HappensBefore : Prop → Prop → Prop`
/// Lamport's happens-before relation: a → b if a caused b.
pub fn dst_ext_happens_before_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// `LamportClockConsistency : Prop`
/// If a → b then L(a) < L(b) (clock condition).
pub fn dst_ext_lamport_clock_consistency_ty() -> Expr {
    prop()
}
/// `VectorClockCharacterization : Prop`
/// VC(a) < VC(b) iff a → b (strong clock condition for vector clocks).
pub fn dst_ext_vector_clock_characterization_ty() -> Expr {
    prop()
}
/// `CausalDelivery : Prop`
/// Causal broadcast: messages are delivered in causal order.
pub fn dst_ext_causal_delivery_ty() -> Expr {
    prop()
}
/// `CausalConsistency : Prop`
/// All processes agree on the order of causally related operations.
pub fn dst_ext_causal_consistency_ty() -> Expr {
    prop()
}
/// `SnapshotIsolation : Prop`
/// Transactions read from a consistent snapshot; writes conflict only on concurrent updates.
pub fn dst_ext_snapshot_isolation_ty() -> Expr {
    prop()
}
/// `RepeatableRead : Prop`
/// Once a transaction reads a value, subsequent reads return the same value.
pub fn dst_ext_repeatable_read_ty() -> Expr {
    prop()
}
/// `PhantomRead : Prop`
/// Phantom read anomaly: a query returns different rows when re-executed in the same transaction.
pub fn dst_ext_phantom_read_ty() -> Expr {
    prop()
}
/// `WriteSkew : Prop`
/// Write skew anomaly: two concurrent transactions read overlapping data and write disjoint data.
pub fn dst_ext_write_skew_ty() -> Expr {
    prop()
}
/// Register all extended distributed systems theory axioms into the environment.
pub fn register_distributed_systems_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        (
            "FLPImpossibilityStrong",
            dst_ext_flp_impossibility_strong_ty(),
        ),
        (
            "AsyncConsensusLowerBound",
            dst_ext_async_consensus_lower_bound_ty(),
        ),
        ("CrashFailureModel", dst_ext_crash_failure_model_ty()),
        ("OmissionFailureModel", dst_ext_omission_failure_model_ty()),
        ("TimedAsynchronousModel", dst_ext_timed_async_model_ty()),
        ("PaxosSafety", dst_ext_paxos_safety_ty()),
        ("PaxosLiveness", dst_ext_paxos_liveness_ty()),
        ("MultiPaxos", dst_ext_multi_paxos_ty()),
        ("PaxosPhaseOneMessage", dst_ext_paxos_phase1_ty()),
        ("PaxosPhaseTwoMessage", dst_ext_paxos_phase2_ty()),
        ("RaftLeaderUniqueness", dst_ext_raft_leader_uniqueness_ty()),
        ("RaftLogMatching", dst_ext_raft_log_matching_ty()),
        (
            "RaftLeaderCompleteness",
            dst_ext_raft_leader_completeness_ty(),
        ),
        (
            "RaftStateMachineSafety",
            dst_ext_raft_state_machine_safety_ty(),
        ),
        ("RaftMembershipChange", dst_ext_raft_membership_change_ty()),
        ("PBFTSafety", dst_ext_pbft_safety_ty()),
        ("PBFTLiveness", dst_ext_pbft_liveness_ty()),
        ("ThresholdSignature", dst_ext_threshold_signature_ty()),
        ("ByzantineAgreement", dst_ext_byzantine_agreement_ty()),
        (
            "AuthenticatedByzantine",
            dst_ext_authenticated_byzantine_ty(),
        ),
        ("ConsistentHashing", dst_ext_consistent_hashing_ty()),
        (
            "ConsistentHashingBalance",
            dst_ext_consistent_hashing_balance_ty(),
        ),
        ("DHTChord", dst_ext_dht_chord_ty()),
        ("DHTKademlia", dst_ext_dht_kademlia_ty()),
        ("DHTLookupComplexity", dst_ext_dht_lookup_complexity_ty()),
        (
            "EventualConsistencyConvergence",
            dst_ext_eventual_consistency_convergence_ty(),
        ),
        (
            "MonotonicReadConsistency",
            dst_ext_monotonic_read_consistency_ty(),
        ),
        (
            "MonotonicWriteConsistency",
            dst_ext_monotonic_write_consistency_ty(),
        ),
        ("ReadYourWrites", dst_ext_read_your_writes_ty()),
        ("WritesFollowReads", dst_ext_writes_follow_reads_ty()),
        ("CRDTStrongEventualConsistency", dst_ext_crdt_sec_ty()),
        ("CRDTJoinSemilattice", dst_ext_crdt_join_semilattice_ty()),
        ("CRDTInflation", dst_ext_crdt_inflation_ty()),
        ("OrSetCRDT", dst_ext_or_set_crdt_ty()),
        ("MVRegisterCRDT", dst_ext_mv_register_crdt_ty()),
        ("NTPSyncBound", dst_ext_ntp_sync_bound_ty()),
        ("ClockDriftRate", dst_ext_clock_drift_rate_ty()),
        ("ChristianAlgorithm", dst_ext_christian_algorithm_ty()),
        ("BerkeleyAlgorithm", dst_ext_berkeley_algorithm_ty()),
        ("HappensBefore", dst_ext_happens_before_ty()),
        (
            "LamportClockConsistency",
            dst_ext_lamport_clock_consistency_ty(),
        ),
        (
            "VectorClockCharacterization",
            dst_ext_vector_clock_characterization_ty(),
        ),
        ("CausalDelivery", dst_ext_causal_delivery_ty()),
        ("CausalConsistency", dst_ext_causal_consistency_ty()),
        ("SnapshotIsolation", dst_ext_snapshot_isolation_ty()),
        ("RepeatableRead", dst_ext_repeatable_read_ty()),
        ("PhantomRead", dst_ext_phantom_read_ty()),
        ("WriteSkew", dst_ext_write_skew_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| format!("Failed to register {}: {:?}", name, e))?;
    }
    Ok(())
}
/// Simple multiplicative hash mixing two u64 values.
pub fn dst_ext_mix_hash(a: u64, b: u64) -> u64 {
    let mut h = a.wrapping_mul(6364136223846793005).wrapping_add(b);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    h
}
