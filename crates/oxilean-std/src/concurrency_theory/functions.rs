//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    ActionDirection, AxiomaticMemoryModel, BaseType, CCSProcess, CCSTransition, CSPProcess,
    CSPTrace, ConcurrentHistory, ConcurrentTriple, DeadlockFreedom, EarlyBisimulation, Event,
    FailureSet, GCounter, HeapPredicate, HistoryEvent, IrisProtocol, LabeledTransitionSystem,
    LamportClock, Marking, NameSubstitution, PetriNet, PetriTransition, PiName, PiProcess, Place,
    PolyaVariadic, ReachabilityProblem, SessionType, TypedChannel, TypingJudgment, VectorClock,
    ViewShiftRule,
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
pub fn bool_ty() -> Expr {
    cst("Bool")
}
pub fn nat_ty() -> Expr {
    cst("Nat")
}
pub fn string_ty() -> Expr {
    cst("String")
}
pub fn list_ty(a: Expr) -> Expr {
    app(cst("List"), a)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// CCSProcess : Type — an element of Milner's CCS
pub fn ccs_process_ty() -> Expr {
    type0()
}
/// CCSLabel : Type — action labels (channels)
pub fn ccs_label_ty() -> Expr {
    type0()
}
/// CCSTransition : Type — a labeled step Source --a--> Target
pub fn ccs_transition_ty() -> Expr {
    type0()
}
/// LabeledTransitionSystem : Type
pub fn lts_ty() -> Expr {
    type0()
}
/// strong_bisim : CCSProcess → CCSProcess → Prop
pub fn strong_bisim_ty() -> Expr {
    arrow(cst("CCSProcess"), arrow(cst("CCSProcess"), prop()))
}
/// weak_bisim : CCSProcess → CCSProcess → Prop
pub fn weak_bisim_ty() -> Expr {
    arrow(cst("CCSProcess"), arrow(cst("CCSProcess"), prop()))
}
/// trace_equivalence : CCSProcess → CCSProcess → Prop
pub fn trace_equivalence_ty() -> Expr {
    arrow(cst("CCSProcess"), arrow(cst("CCSProcess"), prop()))
}
/// lts_is_deterministic : LabeledTransitionSystem → Bool
pub fn lts_is_deterministic_ty() -> Expr {
    arrow(cst("LabeledTransitionSystem"), bool_ty())
}
/// strong_bisim_is_equivalence : ∀ P Q, strong_bisim P Q ↔ weak_bisim P Q → Prop
pub fn strong_bisim_is_equivalence_ty() -> Expr {
    arrow(cst("CCSProcess"), arrow(cst("CCSProcess"), prop()))
}
/// CSPProcess : Type
pub fn csp_process_ty() -> Expr {
    type0()
}
/// Event : Type — an event alphabet element
pub fn event_ty() -> Expr {
    type0()
}
/// CSPTrace : Type — a finite sequence of events
pub fn csp_trace_ty() -> Expr {
    list_ty(cst("Event"))
}
/// FailureSet : Type — set of (trace, refusal) pairs
pub fn failure_set_ty() -> Expr {
    type0()
}
/// DeadlockFreedom : CSPProcess → Prop
pub fn deadlock_freedom_ty() -> Expr {
    arrow(cst("CSPProcess"), prop())
}
/// traces : CSPProcess → List CSPTrace
pub fn csp_traces_ty() -> Expr {
    arrow(cst("CSPProcess"), list_ty(cst("CSPTrace")))
}
/// failures : CSPProcess → FailureSet
pub fn csp_failures_ty() -> Expr {
    arrow(cst("CSPProcess"), cst("FailureSet"))
}
/// refusals : CSPProcess → List (List Event)
pub fn csp_refusals_ty() -> Expr {
    arrow(cst("CSPProcess"), list_ty(list_ty(cst("Event"))))
}
/// csp_divergences : CSPProcess → List CSPTrace
pub fn csp_divergences_ty() -> Expr {
    arrow(cst("CSPProcess"), list_ty(cst("CSPTrace")))
}
/// PiProcess : Type
pub fn pi_process_ty() -> Expr {
    type0()
}
/// Name : Type — channel / name
pub fn pi_name_ty() -> Expr {
    type0()
}
/// NameSubstitution : Type — {y/x} substitution
pub fn name_substitution_ty() -> Expr {
    type0()
}
/// EarlyBisimulation : PiProcess → PiProcess → Prop
pub fn early_bisimulation_ty() -> Expr {
    arrow(cst("PiProcess"), arrow(cst("PiProcess"), prop()))
}
/// apply_subst : NameSubstitution → PiProcess → PiProcess
pub fn apply_subst_ty() -> Expr {
    arrow(
        cst("NameSubstitution"),
        arrow(cst("PiProcess"), cst("PiProcess")),
    )
}
/// PolyaVariadic : Type — polyadic π-calculus extension
pub fn polya_variadic_ty() -> Expr {
    type0()
}
/// scope_extrusion : ∀ P Q a, ... → Prop (scope extrusion lemma type)
pub fn scope_extrusion_ty() -> Expr {
    arrow(
        cst("PiProcess"),
        arrow(cst("PiProcess"), arrow(cst("PiName"), prop())),
    )
}
/// Place : Type
pub fn place_ty() -> Expr {
    type0()
}
/// Transition : Type
pub fn petri_transition_ty() -> Expr {
    type0()
}
/// Marking : Type — token assignment to places
pub fn marking_ty() -> Expr {
    type0()
}
/// PetriNet : Type
pub fn petri_net_ty() -> Expr {
    type0()
}
/// ReachabilityProblem : Type — can we reach M from M₀?
pub fn reachability_problem_ty() -> Expr {
    type0()
}
/// is_safe : PetriNet → Bool
pub fn petri_is_safe_ty() -> Expr {
    arrow(cst("PetriNet"), bool_ty())
}
/// is_bounded : PetriNet → Nat → Bool
pub fn petri_is_bounded_ty() -> Expr {
    arrow(cst("PetriNet"), arrow(nat_ty(), bool_ty()))
}
/// is_live : PetriNet → Bool
pub fn petri_is_live_ty() -> Expr {
    arrow(cst("PetriNet"), bool_ty())
}
/// coverability_tree : PetriNet → List Marking
pub fn coverability_tree_ty() -> Expr {
    arrow(cst("PetriNet"), list_ty(cst("Marking")))
}
/// enabled : PetriNet → Marking → PetriTransition → Bool
pub fn petri_enabled_ty() -> Expr {
    arrow(
        cst("PetriNet"),
        arrow(cst("Marking"), arrow(cst("PetriTransition"), bool_ty())),
    )
}
/// fire : PetriNet → Marking → PetriTransition → Option Marking
pub fn petri_fire_ty() -> Expr {
    arrow(
        cst("PetriNet"),
        arrow(
            cst("Marking"),
            arrow(cst("PetriTransition"), option_ty(cst("Marking"))),
        ),
    )
}
/// SessionType : Type
pub fn session_type_ty() -> Expr {
    type0()
}
/// TypedChannel : Type — a channel carrying a session type
pub fn typed_channel_ty() -> Expr {
    type0()
}
/// dual : SessionType → SessionType  (Send↔Recv, Select↔Branch)
pub fn dual_type_ty() -> Expr {
    arrow(cst("SessionType"), cst("SessionType"))
}
/// TypingJudgment : Type  (Γ ⊢ P : Δ)
pub fn typing_judgment_ty() -> Expr {
    type0()
}
/// dual_involutive : ∀ S, dual (dual S) = S
pub fn dual_involutive_ty() -> Expr {
    arrow(cst("SessionType"), prop())
}
/// well_typed : TypingJudgment → Bool
pub fn well_typed_ty() -> Expr {
    arrow(cst("TypingJudgment"), bool_ty())
}
/// HeapPredicate : Type
pub fn heap_predicate_ty() -> Expr {
    type0()
}
/// ConcurrentTriple : Type  ({P} C {Q})
pub fn concurrent_triple_ty() -> Expr {
    type0()
}
/// ViewShiftRule : Type  (P ={E}=> Q)
pub fn view_shift_rule_ty() -> Expr {
    type0()
}
/// IrisProtocol : Type — ghost state + protocol
pub fn iris_protocol_ty() -> Expr {
    type0()
}
/// sep_star : HeapPredicate → HeapPredicate → HeapPredicate  (P * Q)
pub fn sep_star_ty() -> Expr {
    arrow(
        cst("HeapPredicate"),
        arrow(cst("HeapPredicate"), cst("HeapPredicate")),
    )
}
/// sep_wand : HeapPredicate → HeapPredicate → HeapPredicate  (P -* Q)
pub fn sep_wand_ty() -> Expr {
    arrow(
        cst("HeapPredicate"),
        arrow(cst("HeapPredicate"), cst("HeapPredicate")),
    )
}
/// triple_valid : ConcurrentTriple → Prop
pub fn triple_valid_ty() -> Expr {
    arrow(cst("ConcurrentTriple"), prop())
}
/// frame_rule : ∀ P Q R C, {P} C {Q} → {P * R} C {Q * R}
pub fn frame_rule_ty() -> Expr {
    arrow(
        cst("ConcurrentTriple"),
        arrow(cst("HeapPredicate"), cst("ConcurrentTriple")),
    )
}
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::InstImplicit, name, dom, body)
}
/// MemoryModel : Type
pub fn memory_model_ty() -> Expr {
    type0()
}
/// SCConsistency : MemoryModel → Prop
pub fn sc_consistency_ty() -> Expr {
    arrow(cst("MemoryModel"), prop())
}
/// TSOConsistency : MemoryModel → Prop
pub fn tso_consistency_ty() -> Expr {
    arrow(cst("MemoryModel"), prop())
}
/// PSOConsistency : MemoryModel → Prop
pub fn pso_consistency_ty() -> Expr {
    arrow(cst("MemoryModel"), prop())
}
/// RelaxedConsistency : MemoryModel → Prop
pub fn relaxed_consistency_ty() -> Expr {
    arrow(cst("MemoryModel"), prop())
}
/// AcquireRelease : MemoryModel → Prop
pub fn acquire_release_ty() -> Expr {
    arrow(cst("MemoryModel"), prop())
}
/// sc_implies_tso : ∀ m, SCConsistency m → TSOConsistency m
pub fn sc_implies_tso_ty() -> Expr {
    impl_pi("m", cst("MemoryModel"), arrow(prop(), prop()))
}
/// fence_restores_sc : MemoryModel → Prop
pub fn fence_restores_sc_ty() -> Expr {
    arrow(cst("MemoryModel"), prop())
}
/// Linearizability : Type
pub fn linearizability_ty() -> Expr {
    type0()
}
/// consensus_number : Type → Nat
pub fn consensus_number_ty() -> Expr {
    arrow(type0(), nat_ty())
}
/// WaitFree : Type → Prop
pub fn wait_free_ty() -> Expr {
    arrow(type0(), prop())
}
/// LockFree : Type → Prop
pub fn lock_free_ty() -> Expr {
    arrow(type0(), prop())
}
/// ObstructionFree : Type → Prop
pub fn obstruction_free_ty() -> Expr {
    arrow(type0(), prop())
}
/// is_linearizable : ConcurrentTriple → Prop
pub fn is_linearizable_ty() -> Expr {
    arrow(cst("ConcurrentTriple"), prop())
}
/// herlihy_hierarchy : Nat → Type
pub fn herlihy_hierarchy_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// CASHasConsensusTwo : Prop
pub fn cas_consensus_two_ty() -> Expr {
    prop()
}
/// CAPProtocol : Type
pub fn cap_protocol_ty() -> Expr {
    type0()
}
/// TaDAAction : Type
pub fn tada_action_ty() -> Expr {
    type0()
}
/// RustBeltType : Type
pub fn rustbelt_type_ty() -> Expr {
    type0()
}
/// rely_guarantee : HeapPredicate → HeapPredicate → ConcurrentTriple → Prop
pub fn rely_guarantee_ty() -> Expr {
    arrow(
        cst("HeapPredicate"),
        arrow(cst("HeapPredicate"), arrow(cst("ConcurrentTriple"), prop())),
    )
}
/// atomic_triple : HeapPredicate → HeapPredicate → HeapPredicate → HeapPredicate → Prop
pub fn atomic_triple_ty() -> Expr {
    arrow(
        cst("HeapPredicate"),
        arrow(
            cst("HeapPredicate"),
            arrow(cst("HeapPredicate"), arrow(cst("HeapPredicate"), prop())),
        ),
    )
}
/// lifetime_token : Type → Prop
pub fn lifetime_token_ty() -> Expr {
    arrow(type0(), prop())
}
/// Transaction : Type
pub fn transaction_ty() -> Expr {
    type0()
}
/// Opacity : Transaction → Prop
pub fn opacity_ty() -> Expr {
    arrow(cst("Transaction"), prop())
}
/// ConflictSerializability : Transaction → Prop
pub fn conflict_serializability_ty() -> Expr {
    arrow(cst("Transaction"), prop())
}
/// STMLog : Type
pub fn stm_log_ty() -> Expr {
    type0()
}
/// tm_commit : Transaction → Option Marking
pub fn tm_commit_ty() -> Expr {
    arrow(cst("Transaction"), option_ty(cst("Marking")))
}
/// tm_abort : Transaction → Prop
pub fn tm_abort_ty() -> Expr {
    arrow(cst("Transaction"), prop())
}
/// opacity_implies_serializability : ∀ t, Opacity t → ConflictSerializability t
pub fn opacity_implies_ser_ty() -> Expr {
    impl_pi("t", cst("Transaction"), arrow(prop(), prop()))
}
/// EventStructure : Type
pub fn event_structure_ty() -> Expr {
    type0()
}
/// CausalOrder : EventStructure → Prop
pub fn causal_order_ty() -> Expr {
    arrow(cst("EventStructure"), prop())
}
/// ConflictRelation : EventStructure → Prop
pub fn conflict_relation_ty() -> Expr {
    arrow(cst("EventStructure"), prop())
}
/// StableConfig : EventStructure → Type
pub fn stable_config_ty() -> Expr {
    arrow(cst("EventStructure"), type0())
}
/// Pomset : Type
pub fn pomset_ty() -> Expr {
    type0()
}
/// pomset_refinement : Pomset → Pomset → Prop
pub fn pomset_refinement_ty() -> Expr {
    arrow(cst("Pomset"), arrow(cst("Pomset"), prop()))
}
/// ConfigurationStructure : Type
pub fn configuration_structure_ty() -> Expr {
    type0()
}
/// unfolding : PetriNet → EventStructure
pub fn unfolding_ty() -> Expr {
    arrow(cst("PetriNet"), cst("EventStructure"))
}
/// SpiProcess : Type
pub fn spi_process_ty() -> Expr {
    type0()
}
/// ValuePassingCCS : Type
pub fn value_passing_ccs_ty() -> Expr {
    type0()
}
/// EncryptedMessage : Type
pub fn encrypted_message_ty() -> Expr {
    type0()
}
/// spi_decrypt : EncryptedMessage → PiName → Option PiName
pub fn spi_decrypt_ty() -> Expr {
    arrow(
        cst("EncryptedMessage"),
        arrow(cst("PiName"), option_ty(cst("PiName"))),
    )
}
/// spi_bisimulation : SpiProcess → SpiProcess → Prop
pub fn spi_bisimulation_ty() -> Expr {
    arrow(cst("SpiProcess"), arrow(cst("SpiProcess"), prop()))
}
/// MobileAmbient : Type
pub fn mobile_ambient_ty() -> Expr {
    type0()
}
/// Actor : Type
pub fn actor_ty() -> Expr {
    type0()
}
/// ActorAddress : Type
pub fn actor_address_ty() -> Expr {
    type0()
}
/// Message : Type
pub fn actor_message_ty() -> Expr {
    type0()
}
/// ActorBehavior : Type
pub fn actor_behavior_ty() -> Expr {
    type0()
}
/// send_message : ActorAddress → Message → Prop
pub fn send_message_ty() -> Expr {
    arrow(cst("ActorAddress"), arrow(cst("Message"), prop()))
}
/// actor_fairness : Actor → Prop
pub fn actor_fairness_ty() -> Expr {
    arrow(cst("Actor"), prop())
}
/// ActorConfiguration : Type
pub fn actor_configuration_ty() -> Expr {
    type0()
}
/// LamportClock : Type
pub fn lamport_clock_ty() -> Expr {
    type0()
}
/// VectorClock : Type
pub fn vector_clock_ty() -> Expr {
    type0()
}
/// causally_before : LamportClock → LamportClock → Prop
pub fn causally_before_ty() -> Expr {
    arrow(cst("LamportClock"), arrow(cst("LamportClock"), prop()))
}
/// FLPImpossibility : Prop
pub fn flp_impossibility_ty() -> Expr {
    prop()
}
/// ConsensusProtocol : Type
pub fn consensus_protocol_ty() -> Expr {
    type0()
}
/// paxos_safe : ConsensusProtocol → Prop
pub fn paxos_safe_ty() -> Expr {
    arrow(cst("ConsensusProtocol"), prop())
}
/// raft_log_matching : ConsensusProtocol → Prop
pub fn raft_log_matching_ty() -> Expr {
    arrow(cst("ConsensusProtocol"), prop())
}
/// byzantine_fault_tolerance : ConsensusProtocol → Nat → Prop
pub fn byzantine_ft_ty() -> Expr {
    arrow(cst("ConsensusProtocol"), arrow(nat_ty(), prop()))
}
/// CRDTType : Type
pub fn crdt_ty() -> Expr {
    type0()
}
/// crdt_convergence : CRDTType → Prop
pub fn crdt_convergence_ty() -> Expr {
    arrow(cst("CRDTType"), prop())
}
/// IOAutomaton : Type
pub fn io_automaton_ty() -> Expr {
    type0()
}
/// io_trace : Type (List Event)
pub fn io_trace_ty() -> Expr {
    list_ty(cst("Event"))
}
/// external_behavior : IOAutomaton → Type
pub fn external_behavior_ty() -> Expr {
    arrow(cst("IOAutomaton"), type0())
}
/// io_simulation : IOAutomaton → IOAutomaton → Prop
pub fn io_simulation_ty() -> Expr {
    arrow(cst("IOAutomaton"), arrow(cst("IOAutomaton"), prop()))
}
/// ReactiveSystem : Type
pub fn reactive_system_ty() -> Expr {
    type0()
}
/// LTLFormula : Type
pub fn ltl_formula_ty() -> Expr {
    type0()
}
/// CTLFormula : Type
pub fn ctl_formula_ty() -> Expr {
    type0()
}
/// reactive_satisfies_ltl : ReactiveSystem → LTLFormula → Prop
pub fn reactive_satisfies_ltl_ty() -> Expr {
    arrow(cst("ReactiveSystem"), arrow(cst("LTLFormula"), prop()))
}
/// mu_calculus_fixed_point : (HeapPredicate → HeapPredicate) → HeapPredicate
pub fn mu_calculus_fp_ty() -> Expr {
    arrow(
        arrow(cst("HeapPredicate"), cst("HeapPredicate")),
        cst("HeapPredicate"),
    )
}
/// karp_miller_tree : PetriNet → List Marking
pub fn karp_miller_tree_ty() -> Expr {
    arrow(cst("PetriNet"), list_ty(cst("Marking")))
}
/// petri_liveness_condition : PetriNet → Marking → Prop
pub fn petri_liveness_condition_ty() -> Expr {
    arrow(cst("PetriNet"), arrow(cst("Marking"), prop()))
}
/// Register all concurrency theory axioms into the kernel environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("CCSProcess", ccs_process_ty()),
        ("CCSLabel", ccs_label_ty()),
        ("CCSTransition", ccs_transition_ty()),
        ("LabeledTransitionSystem", lts_ty()),
        ("strong_bisim", strong_bisim_ty()),
        ("weak_bisim", weak_bisim_ty()),
        ("trace_equivalence", trace_equivalence_ty()),
        ("lts_is_deterministic", lts_is_deterministic_ty()),
        (
            "strong_bisim_is_equivalence",
            strong_bisim_is_equivalence_ty(),
        ),
        ("CSPProcess", csp_process_ty()),
        ("Event", event_ty()),
        ("CSPTrace", csp_trace_ty()),
        ("FailureSet", failure_set_ty()),
        ("deadlock_freedom", deadlock_freedom_ty()),
        ("csp_traces", csp_traces_ty()),
        ("csp_failures", csp_failures_ty()),
        ("csp_refusals", csp_refusals_ty()),
        ("csp_divergences", csp_divergences_ty()),
        ("PiProcess", pi_process_ty()),
        ("PiName", pi_name_ty()),
        ("NameSubstitution", name_substitution_ty()),
        ("early_bisimulation", early_bisimulation_ty()),
        ("apply_subst", apply_subst_ty()),
        ("PolyaVariadic", polya_variadic_ty()),
        ("scope_extrusion", scope_extrusion_ty()),
        ("Place", place_ty()),
        ("PetriTransition", petri_transition_ty()),
        ("Marking", marking_ty()),
        ("PetriNet", petri_net_ty()),
        ("ReachabilityProblem", reachability_problem_ty()),
        ("petri_is_safe", petri_is_safe_ty()),
        ("petri_is_bounded", petri_is_bounded_ty()),
        ("petri_is_live", petri_is_live_ty()),
        ("coverability_tree", coverability_tree_ty()),
        ("petri_enabled", petri_enabled_ty()),
        ("petri_fire", petri_fire_ty()),
        ("SessionType", session_type_ty()),
        ("TypedChannel", typed_channel_ty()),
        ("dual", dual_type_ty()),
        ("TypingJudgment", typing_judgment_ty()),
        ("dual_involutive", dual_involutive_ty()),
        ("well_typed", well_typed_ty()),
        ("HeapPredicate", heap_predicate_ty()),
        ("ConcurrentTriple", concurrent_triple_ty()),
        ("ViewShiftRule", view_shift_rule_ty()),
        ("IrisProtocol", iris_protocol_ty()),
        ("sep_star", sep_star_ty()),
        ("sep_wand", sep_wand_ty()),
        ("triple_valid", triple_valid_ty()),
        ("frame_rule", frame_rule_ty()),
        ("MemoryModel", memory_model_ty()),
        ("SCConsistency", sc_consistency_ty()),
        ("TSOConsistency", tso_consistency_ty()),
        ("PSOConsistency", pso_consistency_ty()),
        ("RelaxedConsistency", relaxed_consistency_ty()),
        ("AcquireRelease", acquire_release_ty()),
        ("sc_implies_tso", sc_implies_tso_ty()),
        ("fence_restores_sc", fence_restores_sc_ty()),
        ("Linearizability", linearizability_ty()),
        ("consensus_number", consensus_number_ty()),
        ("WaitFree", wait_free_ty()),
        ("LockFree", lock_free_ty()),
        ("ObstructionFree", obstruction_free_ty()),
        ("is_linearizable", is_linearizable_ty()),
        ("herlihy_hierarchy", herlihy_hierarchy_ty()),
        ("CASHasConsensusTwo", cas_consensus_two_ty()),
        ("CAPProtocol", cap_protocol_ty()),
        ("TaDAAction", tada_action_ty()),
        ("RustBeltType", rustbelt_type_ty()),
        ("rely_guarantee", rely_guarantee_ty()),
        ("atomic_triple", atomic_triple_ty()),
        ("lifetime_token", lifetime_token_ty()),
        ("Transaction", transaction_ty()),
        ("Opacity", opacity_ty()),
        ("ConflictSerializability", conflict_serializability_ty()),
        ("STMLog", stm_log_ty()),
        ("tm_commit", tm_commit_ty()),
        ("tm_abort", tm_abort_ty()),
        ("opacity_implies_serializability", opacity_implies_ser_ty()),
        ("EventStructure", event_structure_ty()),
        ("CausalOrder", causal_order_ty()),
        ("ConflictRelation", conflict_relation_ty()),
        ("StableConfig", stable_config_ty()),
        ("Pomset", pomset_ty()),
        ("pomset_refinement", pomset_refinement_ty()),
        ("ConfigurationStructure", configuration_structure_ty()),
        ("unfolding", unfolding_ty()),
        ("SpiProcess", spi_process_ty()),
        ("ValuePassingCCS", value_passing_ccs_ty()),
        ("EncryptedMessage", encrypted_message_ty()),
        ("spi_decrypt", spi_decrypt_ty()),
        ("spi_bisimulation", spi_bisimulation_ty()),
        ("MobileAmbient", mobile_ambient_ty()),
        ("Actor", actor_ty()),
        ("ActorAddress", actor_address_ty()),
        ("Message", actor_message_ty()),
        ("ActorBehavior", actor_behavior_ty()),
        ("send_message", send_message_ty()),
        ("actor_fairness", actor_fairness_ty()),
        ("ActorConfiguration", actor_configuration_ty()),
        ("LamportClock", lamport_clock_ty()),
        ("VectorClock", vector_clock_ty()),
        ("causally_before", causally_before_ty()),
        ("FLPImpossibility", flp_impossibility_ty()),
        ("ConsensusProtocol", consensus_protocol_ty()),
        ("paxos_safe", paxos_safe_ty()),
        ("raft_log_matching", raft_log_matching_ty()),
        ("byzantine_fault_tolerance", byzantine_ft_ty()),
        ("CRDTType", crdt_ty()),
        ("crdt_convergence", crdt_convergence_ty()),
        ("IOAutomaton", io_automaton_ty()),
        ("io_trace", io_trace_ty()),
        ("external_behavior", external_behavior_ty()),
        ("io_simulation", io_simulation_ty()),
        ("ReactiveSystem", reactive_system_ty()),
        ("LTLFormula", ltl_formula_ty()),
        ("CTLFormula", ctl_formula_ty()),
        ("reactive_satisfies_ltl", reactive_satisfies_ltl_ty()),
        ("mu_calculus_fixed_point", mu_calculus_fp_ty()),
        ("karp_miller_tree", karp_miller_tree_ty()),
        ("petri_liveness_condition", petri_liveness_condition_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lts_deterministic() {
        let mut lts = LabeledTransitionSystem::new(3, 0);
        lts.add_transition(CCSTransition::new(0, "a", ActionDirection::Input, 1));
        lts.add_transition(CCSTransition::new(1, "b", ActionDirection::Output, 2));
        assert!(lts.is_deterministic());
        lts.add_transition(CCSTransition::new(0, "a", ActionDirection::Input, 2));
        assert!(!lts.is_deterministic());
    }
    #[test]
    fn test_strong_bisim_reflexive() {
        let mut lts = LabeledTransitionSystem::new(2, 0);
        lts.add_transition(CCSTransition::new(0, "a", ActionDirection::Input, 1));
        assert!(lts.strong_bisim(0, 0));
    }
    #[test]
    fn test_petri_fire() {
        let mut net = PetriNet::new();
        net.add_place(Place::new("p1"));
        net.add_place(Place::new("p2"));
        let mut pre = HashMap::new();
        pre.insert("p1".to_string(), 1);
        let mut post = HashMap::new();
        post.insert("p2".to_string(), 1);
        net.add_transition(PetriTransition::new("t1", pre, post));
        let mut m = Marking::new();
        m.set("p1", 1);
        net.set_initial(m.clone());
        let fired = net
            .fire(&net.transitions[0].clone(), &m)
            .expect("set_initial should succeed");
        assert_eq!(fired.get("p2"), 1);
        assert_eq!(fired.get("p1"), 0);
    }
    #[test]
    fn test_session_type_dual() {
        let st = SessionType::Send(Box::new(BaseType::Nat), Box::new(SessionType::End));
        let d = st.dual();
        assert!(matches!(d, SessionType::Recv(_, _)));
    }
    #[test]
    fn test_heap_predicate_frame() {
        let triple = ConcurrentTriple::new(
            HeapPredicate::Emp,
            "alloc",
            HeapPredicate::PointsTo("x".to_string(), "5".to_string()),
        );
        let framed = triple.frame(HeapPredicate::PointsTo("y".to_string(), "3".to_string()));
        assert!(matches!(framed.pre, HeapPredicate::Star(_, _)));
    }
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("CCSProcess")).is_some());
        assert!(env.get(&Name::str("PetriNet")).is_some());
        assert!(env.get(&Name::str("SessionType")).is_some());
        assert!(env.get(&Name::str("HeapPredicate")).is_some());
        assert!(env.get(&Name::str("MemoryModel")).is_some());
        assert!(env.get(&Name::str("LamportClock")).is_some());
        assert!(env.get(&Name::str("FLPImpossibility")).is_some());
        assert!(env.get(&Name::str("EventStructure")).is_some());
        assert!(env.get(&Name::str("CRDTType")).is_some());
        assert!(env.get(&Name::str("IOAutomaton")).is_some());
        assert!(env.get(&Name::str("Actor")).is_some());
        assert!(env.get(&Name::str("Linearizability")).is_some());
        assert!(env.get(&Name::str("Transaction")).is_some());
        assert!(env.get(&Name::str("SpiProcess")).is_some());
    }
    #[test]
    fn test_axiomatic_memory_model_sc() {
        let sc = AxiomaticMemoryModel::sequential_consistency();
        assert!(sc.is_sc());
        let tso = AxiomaticMemoryModel::total_store_order();
        assert!(!tso.is_sc());
        assert!(sc.is_stronger_than(&tso));
    }
    #[test]
    fn test_memory_model_fence() {
        let mut tso = AxiomaticMemoryModel::total_store_order();
        assert!(tso.store_load_reorder);
        tso.apply_fence();
        assert!(!tso.store_load_reorder);
        assert!(tso.is_sc());
    }
    #[test]
    fn test_lamport_clock_ordering() {
        let mut p0 = LamportClock::new(0);
        let t1 = p0.send_event();
        let mut p1 = LamportClock::new(1);
        let t2 = p1.receive_event(t1);
        assert!(LamportClock::causally_before(t1, t2));
    }
    #[test]
    fn test_vector_clock_causality() {
        let mut vc0 = VectorClock::new(2, 0);
        vc0.tick();
        let mut vc1 = VectorClock::new(2, 1);
        vc1.receive(&vc0);
        assert!(vc0.causally_before(&vc1));
        assert!(!vc1.causally_before(&vc0));
    }
    #[test]
    fn test_vector_clock_concurrent() {
        let mut vc0 = VectorClock::new(2, 0);
        vc0.tick();
        let mut vc1 = VectorClock::new(2, 1);
        vc1.tick();
        assert!(vc0.concurrent_with(&vc1));
    }
    #[test]
    fn test_concurrent_history_complete() {
        let mut h = ConcurrentHistory::new();
        h.push(HistoryEvent::Call {
            process: 0,
            op: "write".to_string(),
            arg: 42,
        });
        h.push(HistoryEvent::Return {
            process: 0,
            op: "write".to_string(),
            ret: 0,
        });
        assert!(h.is_complete());
        assert_eq!(h.processes().len(), 1);
        assert_eq!(h.calls().len(), 1);
    }
    #[test]
    fn test_gcounter_convergence() {
        let mut a = GCounter::new(3, 0);
        let mut b = GCounter::new(3, 1);
        a.increment();
        a.increment();
        b.increment();
        assert!(a.is_convergent_with(&b));
        a.merge(&b);
        assert_eq!(a.value(), 3);
    }
    #[test]
    fn test_gcounter_merge_idempotent() {
        let mut a = GCounter::new(2, 0);
        a.increment();
        let b = a.clone();
        a.merge(&b);
        assert_eq!(a.value(), 1);
    }
}
