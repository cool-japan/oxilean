//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{BTreeMap, HashMap, HashSet};

use super::types::{
    ApproximateVerification, Assertion, Command, ConcurrentSeparationLogicExt, EffectSystem,
    FractionalPerm, GhostHeap, Heap, HeapPred, HoareTriple, Namespace, NumericalDomain,
    ProbabilisticHoareLogic, RelyCondition, RelyGuaranteeLogic, Transition, TypeAndEffect,
    VerificationCondition, LTS,
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
pub fn string_ty() -> Expr {
    cst("String")
}
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// Assertion: a predicate over program states.
/// Type: State → Prop  (State is a type parameter)
pub fn assertion_ty() -> Expr {
    arrow(type0(), prop())
}
/// HoareTriple: {P} C {Q} — partial Hoare triple.
/// Type: Prop → Program → Prop → Prop
pub fn hoare_triple_ty() -> Expr {
    arrow(prop(), arrow(type0(), arrow(prop(), prop())))
}
/// TotalHoareTriple: [P] C [Q] — total Hoare triple (guarantees termination).
/// Type: Prop → Program → Prop → Prop
pub fn total_hoare_triple_ty() -> Expr {
    arrow(prop(), arrow(type0(), arrow(prop(), prop())))
}
/// SkipRule: {P} skip {P}.
/// Type: {P : Prop} → HoareTriple P skip P
pub fn skip_rule_ty() -> Expr {
    pi(BinderInfo::Default, "P", prop(), prop())
}
/// AssignRule: {P[e/x]} x := e {P}.
/// Type: Prop
pub fn assign_rule_ty() -> Expr {
    prop()
}
/// SeqRule: {P} C1 {R}, {R} C2 {Q} ⊢ {P} C1;C2 {Q}.
/// Type: Prop → Prop → Prop → Prop
pub fn seq_rule_ty() -> Expr {
    arrow(prop(), arrow(prop(), arrow(prop(), prop())))
}
/// WhileRule: {I ∧ b} C {I} ⊢ {I} while b do C {I ∧ ¬b}.
/// Type: Prop → Prop → Prop
pub fn while_rule_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// ConsequenceRule: P ⊢ P', {P'} C {Q'}, Q' ⊢ Q ⊢ {P} C {Q}.
/// Type: Prop
pub fn consequence_rule_ty() -> Expr {
    prop()
}
/// VerificationCondition: a formula that must be checked to validate a Hoare proof.
/// Type: Prop
pub fn verification_condition_ty() -> Expr {
    prop()
}
/// WP: weakest precondition transformer — wp(C, Q) is the weakest P such that {P} C {Q}.
/// Type: Program → Prop → Prop
pub fn wp_ty() -> Expr {
    arrow(type0(), arrow(prop(), prop()))
}
/// WLP: weakest liberal precondition (partial correctness, may diverge).
/// Type: Program → Prop → Prop
pub fn wlp_ty() -> Expr {
    arrow(type0(), arrow(prop(), prop()))
}
/// SP: strongest postcondition — sp(C, P) is the strongest Q such that {P} C {Q}.
/// Type: Prop → Program → Prop
pub fn sp_ty() -> Expr {
    arrow(prop(), arrow(type0(), prop()))
}
/// WPSoundness: {wp(C,Q)} C {Q}.
/// Type: {C : Program} → {Q : Prop} → HoareTriple (wp C Q) C Q
pub fn wp_soundness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "C",
        type0(),
        pi(BinderInfo::Default, "Q", prop(), prop()),
    )
}
/// WPCompleteness: if {P} C {Q} then P ⊢ wp(C, Q).
/// Type: {P Q : Prop} → {C : Program} → HoareTriple P C Q → Prop
pub fn wp_completeness_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        prop(),
        pi(
            BinderInfo::Default,
            "Q",
            prop(),
            pi(BinderInfo::Default, "C", type0(), arrow(prop(), prop())),
        ),
    )
}
/// PredicateTransformer: a monotone endofunction on predicates.
/// Type: (Prop → Prop)
pub fn predicate_transformer_ty() -> Expr {
    arrow(prop(), prop())
}
/// AngelicPT: angelic predicate transformer (may-semantics, liberal).
/// Type: Program → Prop → Prop
pub fn angelic_pt_ty() -> Expr {
    arrow(type0(), arrow(prop(), prop()))
}
/// DemonicPT: demonic predicate transformer (must-semantics, conservative).
/// Type: Program → Prop → Prop
pub fn demonic_pt_ty() -> Expr {
    arrow(type0(), arrow(prop(), prop()))
}
/// HeapPredicate: a predicate over heaps (including ∗ and -∗ connectives).
/// Type: Heap → Prop
pub fn heap_predicate_ty() -> Expr {
    arrow(type0(), prop())
}
/// SepStar: separating conjunction P ∗ Q.
/// Type: HeapPred → HeapPred → HeapPred
pub fn sep_star_ty() -> Expr {
    arrow(
        arrow(type0(), prop()),
        arrow(arrow(type0(), prop()), arrow(type0(), prop())),
    )
}
/// SepWand: separating implication (magic wand) P -∗ Q.
/// Type: HeapPred → HeapPred → HeapPred
pub fn sep_wand_ty() -> Expr {
    arrow(
        arrow(type0(), prop()),
        arrow(arrow(type0(), prop()), arrow(type0(), prop())),
    )
}
/// PointsTo: l ↦ v — the heap contains exactly location l with value v.
/// Type: Nat → Nat → HeapPred
pub fn points_to_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(type0(), prop())))
}
/// EmpPredicate: the empty heap predicate emp.
/// Type: HeapPred
pub fn emp_predicate_ty() -> Expr {
    arrow(type0(), prop())
}
/// FrameRule: {P} C {Q} ⊢ {P ∗ R} C {Q ∗ R} when mod(C) ∩ fv(R) = ∅.
/// Type: Prop → Prop → Prop → Prop
pub fn frame_rule_ty() -> Expr {
    arrow(prop(), arrow(prop(), arrow(prop(), prop())))
}
/// SepLogicTriple: a Hoare triple in separation logic.
/// Type: HeapPred → Program → HeapPred → Prop
pub fn sep_logic_triple_ty() -> Expr {
    arrow(
        arrow(type0(), prop()),
        arrow(type0(), arrow(arrow(type0(), prop()), prop())),
    )
}
/// BiAbduction: given P and Q, find A and B such that P ∗ A ⊢ Q ∗ B.
/// Type: HeapPred → HeapPred → Option (HeapPred × HeapPred)
pub fn bi_abduction_ty() -> Expr {
    arrow(
        arrow(type0(), prop()),
        arrow(arrow(type0(), prop()), option_ty(type0())),
    )
}
/// SpatiallyDisjoint: two heap predicates have disjoint footprints.
/// Type: HeapPred → HeapPred → Prop
pub fn spatially_disjoint_ty() -> Expr {
    arrow(
        arrow(type0(), prop()),
        arrow(arrow(type0(), prop()), prop()),
    )
}
/// ConcurrentTriple: {P} C {Q} for a concurrent program C.
/// Type: HeapPred → ConcProgram → HeapPred → Prop
pub fn concurrent_triple_ty() -> Expr {
    arrow(
        arrow(type0(), prop()),
        arrow(type0(), arrow(arrow(type0(), prop()), prop())),
    )
}
/// ParallelCompositionRule: {P1} C1 {Q1}, {P2} C2 {Q2} ⊢ {P1∗P2} C1||C2 {Q1∗Q2}.
/// Type: Prop
pub fn parallel_composition_rule_ty() -> Expr {
    prop()
}
/// CriticalSectionRule: resource invariant + lock/unlock rule.
/// Type: Prop → Prop → Prop
pub fn critical_section_rule_ty() -> Expr {
    arrow(prop(), arrow(prop(), prop()))
}
/// ThreadLocal: a predicate about the local state of a single thread.
/// Type: (State → Prop)
pub fn thread_local_ty() -> Expr {
    arrow(type0(), prop())
}
/// SharedInvariant: a predicate protected by a lock or other synchronization mechanism.
/// Type: (State → Prop) → Prop
pub fn shared_invariant_ty() -> Expr {
    arrow(arrow(type0(), prop()), prop())
}
/// OwnershipTransfer: transferring ownership of a resource between threads.
/// Type: HeapPred → HeapPred → Prop
pub fn ownership_transfer_ty() -> Expr {
    arrow(
        arrow(type0(), prop()),
        arrow(arrow(type0(), prop()), prop()),
    )
}
/// AtomicTriple: a logically atomic triple ⟨P⟩ C ⟨Q⟩.
/// Type: Prop → Program → Prop → Prop
pub fn atomic_triple_ty() -> Expr {
    arrow(prop(), arrow(type0(), arrow(prop(), prop())))
}
/// RelyCondition: an action the environment may perform.
/// Type: State → State → Prop
pub fn rely_condition_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// GuaranteeCondition: an action this thread promises not to violate.
/// Type: State → State → Prop
pub fn guarantee_condition_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// RelyGuaranteeTriple: (R, G) ⊢ {P} C {Q}.
/// Type: (State → State → Prop) → (State → State → Prop) → Prop → Program → Prop → Prop
pub fn rely_guarantee_triple_ty() -> Expr {
    arrow(
        arrow(type0(), arrow(type0(), prop())),
        arrow(
            arrow(type0(), arrow(type0(), prop())),
            arrow(prop(), arrow(type0(), arrow(prop(), prop()))),
        ),
    )
}
/// RelyGuaranteeParallel: composition rule for rely-guarantee.
/// Type: Prop
pub fn rely_guarantee_parallel_ty() -> Expr {
    prop()
}
/// StabilityCondition: P is stable under R if R-steps preserve P.
/// Type: (State → Prop) → (State → State → Prop) → Prop
pub fn stability_condition_ty() -> Expr {
    arrow(
        arrow(type0(), prop()),
        arrow(arrow(type0(), arrow(type0(), prop())), prop()),
    )
}
/// RelyGuaranteeConsequence: consequence rule for R-G.
/// Type: Prop
pub fn rely_guarantee_consequence_ty() -> Expr {
    prop()
}
/// IrisProp: a proposition in the Iris base logic (step-indexed).
/// Type: Type
pub fn iris_prop_ty() -> Expr {
    type0()
}
/// IrisEntails: Iris entailment P ⊢ Q.
/// Type: IrisProp → IrisProp → Prop
pub fn iris_entails_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// IrisSepStar: separating conjunction in Iris.
/// Type: IrisProp → IrisProp → IrisProp
pub fn iris_sep_star_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// IrisWand: magic wand in Iris.
/// Type: IrisProp → IrisProp → IrisProp
pub fn iris_wand_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// IrisLater: the later modality ▷P.
/// Type: IrisProp → IrisProp
pub fn iris_later_ty() -> Expr {
    arrow(type0(), type0())
}
/// IrisAlways: the always modality □P (persistent propositions).
/// Type: IrisProp → IrisProp
pub fn iris_always_ty() -> Expr {
    arrow(type0(), type0())
}
/// IrisExcl: exclusive ownership token Excl(v).
/// Type: {V : Type} → V → IrisProp
pub fn iris_excl_ty() -> Expr {
    pi(BinderInfo::Default, "V", type0(), arrow(bvar(0), type0()))
}
/// IrisAgree: agreement resource: both owners agree on a value.
/// Type: {V : Type} → V → IrisProp
pub fn iris_agree_ty() -> Expr {
    pi(BinderInfo::Default, "V", type0(), arrow(bvar(0), type0()))
}
/// IrisAuth: authoritative element in the auth camera.
/// Type: {A : Type} → A → IrisProp
pub fn iris_auth_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(bvar(0), type0()))
}
/// IrisFragment: fragment element in the auth camera.
/// Type: {A : Type} → A → IrisProp
pub fn iris_fragment_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(bvar(0), type0()))
}
/// ResourceAlgebra: a CMRA (Canonical Metric Resource Algebra) — unital, partial monoid
/// with a validity predicate and core map.
/// Type: Type → Prop
pub fn resource_algebra_ty() -> Expr {
    arrow(type0(), prop())
}
/// GhostState: logical (ghost) state tracked in the Iris ghost heap.
/// Type: {A : Type} → ResourceAlgebra A → IrisProp
pub fn ghost_state_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(prop(), type0()))
}
/// GhostUpdate: a ghost update modality |==> P (frame-preserving update).
/// Type: IrisProp → IrisProp
pub fn ghost_update_ty() -> Expr {
    arrow(type0(), type0())
}
/// FramePreservingUpdate: a → b is frame-preserving if for all frames f valid (a⊗f)
/// there exists b' with b ⊗ f' valid and b ≼ b'.
/// Type: {A : Type} → A → A → Prop
pub fn frame_preserving_update_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// CMRA: a Camera (generalized RA used in Iris).
/// Type: Type → Prop
pub fn cmra_ty() -> Expr {
    arrow(type0(), prop())
}
/// CMRAOp: the partial composition operation of a CMRA.
/// Type: {A : Type} → A → A → Option A
pub fn cmra_op_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(bvar(0), arrow(bvar(1), option_ty(bvar(2)))),
    )
}
/// CMRAValid: validity predicate of a CMRA.
/// Type: {A : Type} → A → Prop
pub fn cmra_valid_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(bvar(0), prop()))
}
/// CMRACore: the core map γ : A → Option A (idempotent part).
/// Type: {A : Type} → A → Option A
pub fn cmra_core_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(bvar(0), option_ty(bvar(1))),
    )
}
/// Invariant: a persistent heap predicate protected by an Iris invariant.
/// Type: Namespace → IrisProp → IrisProp
pub fn invariant_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), type0()))
}
/// InvariantAlloc: allocate a new invariant.
/// Type: {P : IrisProp} → P ⊢ |==> ∃ N, inv(N, P)
pub fn invariant_alloc_ty() -> Expr {
    pi(BinderInfo::Default, "P", type0(), prop())
}
/// InvariantOpen: open an invariant for one step.
/// Type: {N : Namespace} → {P Q E : IrisProp} → inv(N,P) ∗ (P -∗ ▷P ∗ Q) ⊢ Q
pub fn invariant_open_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "N",
        nat_ty(),
        pi(
            BinderInfo::Default,
            "P",
            type0(),
            pi(BinderInfo::Default, "Q", type0(), prop()),
        ),
    )
}
/// NamespaceMask: the set of open invariants (mask E ⊆ Namespace).
/// Type: Type
pub fn namespace_mask_ty() -> Expr {
    type0()
}
/// MaskSubset: mask inclusion E1 ⊆ E2.
/// Type: NamespaceMask → NamespaceMask → Prop
pub fn mask_subset_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// FancyUpdate: fancy update modality |={E1,E2}=> P.
/// Type: NamespaceMask → NamespaceMask → IrisProp → IrisProp
pub fn fancy_update_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), type0())))
}
/// FractionalPermission: a fractional permission q ∈ (0, 1].
/// Type: Type
pub fn fractional_permission_ty() -> Expr {
    type0()
}
/// FractionalPointsTo: l ↦{q} v — fractional ownership of location l.
/// Type: Nat → Nat → FractionalPermission → HeapPred
pub fn fractional_points_to_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(type0(), arrow(type0(), prop()))),
    )
}
/// PermissionSplit: p = p1 + p2 justifies l↦{p}v ⊢ l↦{p1}v ∗ l↦{p2}v.
/// Type: Prop
pub fn permission_split_ty() -> Expr {
    prop()
}
/// PermissionCombine: l↦{p1}v ∗ l↦{p2}v ⊢ l↦{p1+p2}v.
/// Type: Prop
pub fn permission_combine_ty() -> Expr {
    prop()
}
/// CountingPermission: a counting-based permission (multiset of capabilities).
/// Type: Type
pub fn counting_permission_ty() -> Expr {
    type0()
}
/// WritePermission: full (1) fractional permission, allows mutation.
/// Type: FractionalPermission
pub fn write_permission_ty() -> Expr {
    type0()
}
/// ReadPermission: a fraction q < 1, read-only.
/// Type: FractionalPermission
pub fn read_permission_ty() -> Expr {
    type0()
}
/// RankingFunction: a function from states to a well-founded set,
/// decreasing on each loop iteration.
/// Type: {S : Type} → (S → Nat) → Program → Prop
pub fn ranking_function_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "S",
        type0(),
        arrow(arrow(bvar(0), nat_ty()), arrow(type0(), prop())),
    )
}
/// TerminationProof: a proof that program C terminates from every state satisfying P.
/// Type: Prop → Program → Prop
pub fn termination_proof_ty() -> Expr {
    arrow(prop(), arrow(type0(), prop()))
}
/// TotalCorrectnessRule: total correctness proof rule using a ranking function.
/// Type: Prop
pub fn total_correctness_rule_ty() -> Expr {
    prop()
}
/// LoopVariant: the variant expression for a while loop (must decrease).
/// Type: Type
pub fn loop_variant_ty() -> Expr {
    type0()
}
/// LoopInvariant: the invariant for a while loop.
/// Type: (State → Prop)
pub fn loop_invariant_ty() -> Expr {
    arrow(type0(), prop())
}
/// WellFoundedOrder: a well-founded order — every non-empty set has a minimal element.
/// Type: {A : Type} → (A → A → Prop) → Prop
pub fn well_founded_order_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(arrow(bvar(0), arrow(bvar(1), prop())), prop()),
    )
}
/// AbstractSpec: an abstract specification (set of allowed behaviors).
/// Type: Type
pub fn abstract_spec_ty() -> Expr {
    type0()
}
/// ConcreteImpl: a concrete implementation.
/// Type: Type
pub fn concrete_impl_ty() -> Expr {
    type0()
}
/// DataRefinement: a concrete implementation refines an abstract spec.
/// Type: AbstractSpec → ConcreteImpl → Prop
pub fn data_refinement_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// SimulationRelation: a relation witnessing a simulation (forward simulation).
/// Type: {A C : Type} → (A → C → Prop) → Prop
pub fn simulation_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "C",
            type0(),
            arrow(arrow(bvar(1), arrow(bvar(1), prop())), prop()),
        ),
    )
}
/// BackwardSimulation: a relation witnessing backward simulation.
/// Type: {A C : Type} → (C → A → Prop) → Prop
pub fn backward_simulation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "C",
            type0(),
            arrow(arrow(bvar(1), arrow(bvar(2), prop())), prop()),
        ),
    )
}
/// RefinementMapping: a coupling function between abstract and concrete states.
/// Type: {A C : Type} → (C → A) → Prop
pub fn refinement_mapping_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "C",
            type0(),
            arrow(arrow(bvar(1), bvar(2)), prop()),
        ),
    )
}
/// AbadiLamport: Abadi-Lamport theorem: forward simulation ∧ backward simulation ⟹ refinement.
/// Type: Prop
pub fn abadi_lamport_ty() -> Expr {
    prop()
}
/// Register all program logics axioms in the kernel environment.
pub fn build_program_logics_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("Assertion", assertion_ty()),
        ("HoareTriple", hoare_triple_ty()),
        ("TotalHoareTriple", total_hoare_triple_ty()),
        ("SkipRule", skip_rule_ty()),
        ("AssignRule", assign_rule_ty()),
        ("SeqRule", seq_rule_ty()),
        ("WhileRule", while_rule_ty()),
        ("ConsequenceRule", consequence_rule_ty()),
        ("VerificationCondition", verification_condition_ty()),
        ("WP", wp_ty()),
        ("WLP", wlp_ty()),
        ("SP", sp_ty()),
        ("WPSoundness", wp_soundness_ty()),
        ("WPCompleteness", wp_completeness_ty()),
        ("PredicateTransformer", predicate_transformer_ty()),
        ("AngelicPT", angelic_pt_ty()),
        ("DemonicPT", demonic_pt_ty()),
        ("HeapPredicate", heap_predicate_ty()),
        ("SepStar", sep_star_ty()),
        ("SepWand", sep_wand_ty()),
        ("PointsTo", points_to_ty()),
        ("EmpPredicate", emp_predicate_ty()),
        ("FrameRule", frame_rule_ty()),
        ("SepLogicTriple", sep_logic_triple_ty()),
        ("BiAbduction", bi_abduction_ty()),
        ("SpatiallyDisjoint", spatially_disjoint_ty()),
        ("ConcurrentTriple", concurrent_triple_ty()),
        ("ParallelCompositionRule", parallel_composition_rule_ty()),
        ("CriticalSectionRule", critical_section_rule_ty()),
        ("ThreadLocal", thread_local_ty()),
        ("SharedInvariant", shared_invariant_ty()),
        ("OwnershipTransfer", ownership_transfer_ty()),
        ("AtomicTriple", atomic_triple_ty()),
        ("RelyCondition", rely_condition_ty()),
        ("GuaranteeCondition", guarantee_condition_ty()),
        ("RelyGuaranteeTriple", rely_guarantee_triple_ty()),
        ("RelyGuaranteeParallel", rely_guarantee_parallel_ty()),
        ("StabilityCondition", stability_condition_ty()),
        ("RelyGuaranteeConsequence", rely_guarantee_consequence_ty()),
        ("IrisProp", iris_prop_ty()),
        ("IrisEntails", iris_entails_ty()),
        ("IrisSepStar", iris_sep_star_ty()),
        ("IrisWand", iris_wand_ty()),
        ("IrisLater", iris_later_ty()),
        ("IrisAlways", iris_always_ty()),
        ("IrisExcl", iris_excl_ty()),
        ("IrisAgree", iris_agree_ty()),
        ("IrisAuth", iris_auth_ty()),
        ("IrisFragment", iris_fragment_ty()),
        ("ResourceAlgebra", resource_algebra_ty()),
        ("GhostState", ghost_state_ty()),
        ("GhostUpdate", ghost_update_ty()),
        ("FramePreservingUpdate", frame_preserving_update_ty()),
        ("CMRA", cmra_ty()),
        ("CMRAOp", cmra_op_ty()),
        ("CMRAValid", cmra_valid_ty()),
        ("CMRACore", cmra_core_ty()),
        ("Invariant", invariant_ty()),
        ("InvariantAlloc", invariant_alloc_ty()),
        ("InvariantOpen", invariant_open_ty()),
        ("NamespaceMask", namespace_mask_ty()),
        ("MaskSubset", mask_subset_ty()),
        ("FancyUpdate", fancy_update_ty()),
        ("FractionalPermission", fractional_permission_ty()),
        ("FractionalPointsTo", fractional_points_to_ty()),
        ("PermissionSplit", permission_split_ty()),
        ("PermissionCombine", permission_combine_ty()),
        ("CountingPermission", counting_permission_ty()),
        ("WritePermission", write_permission_ty()),
        ("ReadPermission", read_permission_ty()),
        ("RankingFunction", ranking_function_ty()),
        ("TerminationProof", termination_proof_ty()),
        ("TotalCorrectnessRule", total_correctness_rule_ty()),
        ("LoopVariant", loop_variant_ty()),
        ("LoopInvariant", loop_invariant_ty()),
        ("WellFoundedOrder", well_founded_order_ty()),
        ("AbstractSpec", abstract_spec_ty()),
        ("ConcreteImpl", concrete_impl_ty()),
        ("DataRefinement", data_refinement_ty()),
        ("SimulationRelation", simulation_relation_ty()),
        ("BackwardSimulation", backward_simulation_ty()),
        ("RefinementMapping", refinement_mapping_ty()),
        ("AbadiLamport", abadi_lamport_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    env
}
/// A guarantee condition: transitions this thread promises to only perform.
pub type GuaranteeCondition<S> = RelyCondition<S>;
/// Check stability: a predicate `P` (given as a set of states) is stable under rely `R`
/// if for every transition (s → s') in R, s ∈ P → s' ∈ P.
pub fn is_stable<S: Clone + Eq + std::hash::Hash>(
    predicate: &HashSet<S>,
    rely: &RelyCondition<S>,
) -> bool {
    rely.transitions.iter().all(|t| {
        if predicate.contains(&t.before) {
            predicate.contains(&t.after)
        } else {
            true
        }
    })
}
/// Check (simple) trace inclusion: every trace of `concrete` is a trace of `abstract_lts`.
/// We check by seeing if the concrete LTS's reachable state-transitions are covered.
pub fn trace_inclusion_holds<S>(concrete: &LTS<S>, abstract_lts: &LTS<S>) -> bool
where
    S: Clone + Eq + std::hash::Hash + std::fmt::Debug,
{
    let abstract_labels: HashSet<String> = abstract_lts
        .transitions
        .values()
        .flat_map(|v| v.iter().map(|(l, _)| l.clone()))
        .collect();
    concrete.labels_subset_of(&abstract_labels)
}
/// Return a list of named Hoare logic proof rules.
///
/// ```
/// use oxilean_std::program_logics::hoare_logic_rules;
/// let rules = hoare_logic_rules();
/// assert!(!rules.is_empty());
/// assert!(rules.iter().any(|(name, _)| *name == "Skip"));
/// ```
pub fn hoare_logic_rules() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Skip", "{P} skip {P}"), ("Assign", "{P[e/x]} x := e {P}"), ("Seq",
        "{P} C1 {R}   {R} C2 {Q}\n──────────────────────\n{P} C1; C2 {Q}"),
        ("If",
        "{P ∧ b} C1 {Q}   {P ∧ ¬b} C2 {Q}\n──────────────────────────────────\n{P} if b then C1 else C2 {Q}"),
        ("While",
        "{I ∧ b} C {I}\n────────────────────────\n{I} while b do C {I ∧ ¬b}"),
        ("Consequence",
        "P ⊢ P'   {P'} C {Q'}   Q' ⊢ Q\n──────────────────────────\n{P} C {Q}"),
        ("Frame",
        "{P} C {Q}   mod(C) # fv(R) = ∅\n──────────────────────────\n{P ∗ R} C {Q ∗ R}"),
        ("Parallel",
        "{P1} C1 {Q1}   {P2} C2 {Q2}\n──────────────────────────────\n{P1 ∗ P2} C1 ‖ C2 {Q1 ∗ Q2}"),
    ]
}
/// Return the weakest precondition calculus rules as strings.
///
/// ```
/// use oxilean_std::program_logics::wp_rules;
/// let rules = wp_rules();
/// assert!(rules.iter().any(|(name, _)| *name == "WP-Skip"));
/// ```
pub fn wp_rules() -> Vec<(&'static str, &'static str)> {
    vec![
        ("WP-Skip", "wp(skip, Q) = Q"),
        ("WP-Assign", "wp(x := e, Q) = Q[e/x]"),
        ("WP-Seq", "wp(C1; C2, Q) = wp(C1, wp(C2, Q))"),
        (
            "WP-If",
            "wp(if b then C1 else C2, Q) = (b → wp(C1,Q)) ∧ (¬b → wp(C2,Q))",
        ),
        (
            "WP-While",
            "wp(while b do C, Q) = lfp(λX. (¬b → Q) ∧ (b → wp(C, X)))",
        ),
        ("WP-Sound", "{wp(C,Q)} C {Q}"),
        ("WP-Complete", "{P} C {Q} ⊢ P → wp(C,Q)"),
    ]
}
/// Return the Iris proof rules for invariant access.
///
/// ```
/// use oxilean_std::program_logics::iris_invariant_rules;
/// let rules = iris_invariant_rules();
/// assert!(!rules.is_empty());
/// ```
pub fn iris_invariant_rules() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Inv-Alloc", "▷P ⊢ |={E}=> ∃ N ∉ E, inv(N, P)"),
        (
            "Inv-Open",
            "inv(N, P) ∗ (▷P -∗ |={E}=> ▷P ∗ Q) ⊢ |={E∪{N},E}=> Q",
        ),
        ("Inv-Pers", "□ inv(N, P)"),
        ("GhostAlloc", "⊢ |==> ∃ γ, own(γ, a)"),
        ("GhostUpdate", "own(γ, a) ⊢ |==> own(γ, b)   (when a ~~> b)"),
        ("Frame-Iris", "E ⊆ E' → |={E}=> P ⊢ |={E'}=> P"),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_program_logics_env() {
        let env = build_program_logics_env();
        assert!(env.get(&Name::str("HoareTriple")).is_some());
        assert!(env.get(&Name::str("WP")).is_some());
        assert!(env.get(&Name::str("FrameRule")).is_some());
        assert!(env.get(&Name::str("IrisProp")).is_some());
        assert!(env.get(&Name::str("CMRA")).is_some());
        assert!(env.get(&Name::str("Invariant")).is_some());
        assert!(env.get(&Name::str("FractionalPermission")).is_some());
        assert!(env.get(&Name::str("DataRefinement")).is_some());
    }
    #[test]
    fn test_assertion_operations() {
        let p = Assertion::new("x > 0");
        let q = Assertion::new("y < 10");
        let pq = p.and(&q);
        assert!(pq.formula.contains("x > 0"));
        assert!(pq.formula.contains("y < 10"));
        let neg = p.negate();
        assert!(neg.formula.contains("¬"));
        let s = p.subst("x", "x + 1");
        assert_eq!(s.formula, "x + 1 > 0");
    }
    #[test]
    fn test_wp_skip() {
        let c = Command::Skip;
        let q = Assertion::new("x >= 0");
        let wp = c.wp(&q);
        assert_eq!(wp.formula, "x >= 0");
    }
    #[test]
    fn test_wp_assign() {
        let c = Command::Assign("x".to_string(), "x + 1".to_string());
        let q = Assertion::new("x > 5");
        let wp = c.wp(&q);
        assert_eq!(wp.formula, "x + 1 > 5");
    }
    #[test]
    fn test_wp_seq() {
        let c = Command::Seq(
            Box::new(Command::Assign("x".to_string(), "x + 1".to_string())),
            Box::new(Command::Skip),
        );
        let q = Assertion::new("x > 5");
        let wp = c.wp(&q);
        assert_eq!(wp.formula, "x + 1 > 5");
    }
    #[test]
    fn test_heap_disjoint_union() {
        let mut h1 = Heap::empty();
        h1.write(0, 42);
        let mut h2 = Heap::empty();
        h2.write(1, 99);
        assert!(h1.is_disjoint(&h2));
        let h3 = h1.disjoint_union(&h2);
        assert_eq!(h3.size(), 2);
        assert_eq!(h3.read(0), Some(42));
        assert_eq!(h3.read(1), Some(99));
    }
    #[test]
    fn test_fractional_permissions() {
        let wp = FractionalPerm::write();
        assert!(wp.is_write());
        let (h1, h2) = wp.split_half();
        assert!(!h1.is_write());
        assert!(!h2.is_write());
        let combined = h1.combine(&h2).expect("combine should succeed");
        assert!(combined.is_write());
        assert!(combined.combine(&h1).is_none());
    }
    #[test]
    fn test_ghost_heap() {
        let mut gh: GhostHeap<u64> = GhostHeap::empty();
        gh.alloc("counter", 0u64);
        assert_eq!(gh.read("counter"), Some(&0u64));
        let ok = gh.update("counter", 42u64);
        assert!(ok);
        assert_eq!(gh.read("counter"), Some(&42u64));
    }
    #[test]
    fn test_stability() {
        use std::collections::HashSet;
        let mut pred: HashSet<u32> = HashSet::new();
        pred.insert(1);
        pred.insert(2);
        let mut rely: RelyCondition<u32> = RelyCondition::empty();
        rely.add(Transition::new(1u32, 2u32));
        assert!(is_stable(&pred, &rely));
        rely.add(Transition::new(2u32, 3u32));
        assert!(!is_stable(&pred, &rely));
    }
    #[test]
    fn test_hoare_logic_rules() {
        let rules = hoare_logic_rules();
        assert!(!rules.is_empty());
        assert!(rules.iter().any(|(n, _)| *n == "Frame"));
        assert!(rules.iter().any(|(n, _)| *n == "While"));
    }
}
/// Build a kernel environment with all program-logics axioms.
/// (Alias for `build_program_logics_env` returning a `Result`.)
pub fn build_env() -> Result<Environment, String> {
    Ok(build_program_logics_env())
}
#[cfg(test)]
mod tests_proglogics_ext {
    use super::*;
    use std::fmt;
    #[test]
    fn test_concurrent_separation_logic() {
        let iris = ConcurrentSeparationLogicExt::iris();
        assert!(iris.fractional_permissions);
        assert!(iris.supports_rely_guarantee);
        let triple = iris.concurrent_triple("emp", "x := 0", "x = 0");
        assert!(triple.contains("emp"));
        assert!(iris.race_condition_freedom());
    }
    #[test]
    fn test_rely_guarantee() {
        let rg = RelyGuaranteeLogic::new("y=0", "x≥0", "x=0", "x≥0");
        let triple = rg.rg_triple("x := x + 1");
        assert!(triple.contains("R:"));
        let stab = rg.stability_check();
        assert!(stab.contains("stable"));
    }
    #[test]
    fn test_effect_system() {
        let ae = EffectSystem::algebraic_effects();
        assert!(ae.is_algebraic);
        assert!(!ae.monad_based);
        let desc = ae.effect_handling_description();
        assert!(desc.contains("algebraically"));
        let free = ae.free_monad_presentation();
        assert!(free.contains("IO"));
    }
    #[test]
    fn test_type_and_effect() {
        let pure_te = TypeAndEffect::pure_type("Int");
        assert!(pure_te.is_pure());
        let judge = pure_te.type_and_effect_judgment();
        assert!(judge.contains("∅"));
        let eff = TypeAndEffect::effectful("Unit", vec!["IO".to_string()]);
        assert!(!eff.is_pure());
    }
    #[test]
    fn test_probabilistic_hoare_logic() {
        let phl = ProbabilisticHoareLogic::phl("μ_init", "flip", "p(heads)=0.5");
        let wpt = phl.expectation_transformer();
        assert!(wpt.contains("wp(flip"));
        let mm = phl.mciver_morgan_rule();
        assert!(mm.contains("McIver-Morgan"));
    }
    #[test]
    fn test_approximate_verification() {
        let av = ApproximateVerification::pac_verification(0.01, 0.05, "safety");
        assert!((av.confidence - 0.95).abs() < 1e-10);
        let samples = av.sample_complexity();
        assert!(samples > 0);
        let stmt = av.soundness_statement();
        assert!(stmt.contains("safety"));
    }
    #[test]
    fn test_numerical_domain() {
        let intervals = NumericalDomain::intervals();
        assert!(!intervals.is_relational);
        let cost = intervals.precision_cost_tradeoff();
        assert!(cost.contains("O(n)"));
        let oct = NumericalDomain::octagons();
        assert!(oct.is_relational);
        assert!(oct.is_more_precise_than_intervals());
        let poly = NumericalDomain::polyhedra();
        assert!(poly.is_relational);
    }
}
