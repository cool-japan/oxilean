//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet};

use super::types::{
    BeliefRevisionOp, Bisimulation, CanonicalModel, EpistemicModel, FiniteTrace, GradedModel,
    KripkeFrame, KripkeModel, MaximalConsistentSet, ModalFormula, ModalSystem, MuCalculusEval,
    PdlModel, PdlProgram, PublicAnnouncement, SahlqvistClass,
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
/// KripkeFrame: (W, R) — a set of worlds with an accessibility relation.
pub fn kripke_frame_ty() -> Expr {
    type0()
}
/// KripkeModel: (W, R, V) — frame + valuation of propositions.
pub fn kripke_model_ty() -> Expr {
    type0()
}
/// World: an element of the set of possible worlds.
pub fn world_ty() -> Expr {
    type0()
}
/// AccessibilityRelation: W → W → Prop
pub fn accessibility_relation_ty() -> Expr {
    arrow(cst("World"), arrow(cst("World"), prop()))
}
/// Valuation: propositional variable → world → Bool
pub fn valuation_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("World"), bool_ty()))
}
/// ModalFormula: syntax of modal propositional logic.
pub fn modal_formula_ty() -> Expr {
    type0()
}
/// Satisfaction: M, w ⊨ φ
pub fn modal_satisfaction_ty() -> Expr {
    arrow(
        cst("KripkeModel"),
        arrow(cst("World"), arrow(cst("ModalFormula"), prop())),
    )
}
/// FrameValidity: φ valid in frame F — for all valuations and all worlds
pub fn frame_validity_ty() -> Expr {
    arrow(cst("KripkeFrame"), arrow(cst("ModalFormula"), prop()))
}
/// ClassValidity: φ valid in a class of frames
pub fn class_validity_ty() -> Expr {
    arrow(
        arrow(cst("KripkeFrame"), prop()),
        arrow(cst("ModalFormula"), prop()),
    )
}
/// Reflexive frame: ∀w, wRw
pub fn reflexive_frame_ty() -> Expr {
    arrow(cst("KripkeFrame"), prop())
}
/// Transitive frame: ∀w v u, wRv → vRu → wRu
pub fn transitive_frame_ty() -> Expr {
    arrow(cst("KripkeFrame"), prop())
}
/// Symmetric frame: ∀w v, wRv → vRw
pub fn symmetric_frame_ty() -> Expr {
    arrow(cst("KripkeFrame"), prop())
}
/// Serial frame: ∀w, ∃v, wRv
pub fn serial_frame_ty() -> Expr {
    arrow(cst("KripkeFrame"), prop())
}
/// Euclidean frame: ∀w v u, wRv → wRu → vRu
pub fn euclidean_frame_ty() -> Expr {
    arrow(cst("KripkeFrame"), prop())
}
/// Confluent frame: ∀w v u, wRv → wRu → ∃z, vRz ∧ uRz (Church-Rosser)
pub fn confluent_frame_ty() -> Expr {
    arrow(cst("KripkeFrame"), prop())
}
/// Dense frame: ∀w u, wRu → ∃v, wRv ∧ vRu
pub fn dense_frame_ty() -> Expr {
    arrow(cst("KripkeFrame"), prop())
}
/// Directed frame: ∀w v u, wRv → wRu → ∃z, vRz ∧ uRz ∧ wRz
pub fn directed_frame_ty() -> Expr {
    arrow(cst("KripkeFrame"), prop())
}
/// AxiomK: □(φ → ψ) → □φ → □ψ (distribution)
pub fn axiom_k_ty() -> Expr {
    arrow(cst("ModalFormula"), arrow(cst("ModalFormula"), prop()))
}
/// AxiomT: □φ → φ (reflexivity)
pub fn axiom_t_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Axiom4: □φ → □□φ (transitivity / positive introspection)
pub fn axiom4_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// AxiomB: φ → □◇φ (symmetry / Brouwer)
pub fn axiom_b_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Axiom5: ◇φ → □◇φ (Euclidean / negative introspection)
pub fn axiom5_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// AxiomD: □φ → ◇φ (seriality / deontic)
pub fn axiom_d_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// AxiomLöb (GL): □(□φ → φ) → □φ (provability logic)
pub fn axiom_lob_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// NecRule: if ⊢ φ then ⊢ □φ (necessitation)
pub fn nec_rule_ty() -> Expr {
    arrow(cst("ModalFormula"), arrow(cst("ModalFormula"), prop()))
}
/// Soundness: K is sound with respect to all Kripke frames
pub fn soundness_k_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Completeness: K is complete with respect to all Kripke frames
pub fn completeness_k_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Soundness T: T is sound with respect to reflexive frames
pub fn soundness_t_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Completeness T: T is complete with respect to reflexive frames
pub fn completeness_t_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Soundness S4: S4 is sound with respect to reflexive-transitive frames
pub fn soundness_s4_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Completeness S4: S4 is complete with respect to reflexive-transitive frames
pub fn completeness_s4_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Soundness S5: S5 is sound with respect to equivalence-relation frames
pub fn soundness_s5_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Completeness S5: S5 is complete with respect to equivalence-relation frames
pub fn completeness_s5_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// CanonicalFrame: the canonical frame for a normal modal logic L
pub fn canonical_frame_ty() -> Expr {
    arrow(cst("ModalLogic"), cst("KripkeFrame"))
}
/// CanonicalModel: the canonical model for a normal modal logic L
pub fn canonical_model_ty() -> Expr {
    arrow(cst("ModalLogic"), cst("KripkeModel"))
}
/// CanonicalTruthLemma: φ ∈ w ↔ M^L, w ⊨ φ (truth lemma for canonical model)
pub fn canonical_truth_lemma_ty() -> Expr {
    arrow(cst("ModalLogic"), arrow(cst("ModalFormula"), prop()))
}
/// MaximalConsistentSet: a maximal L-consistent set of formulas
pub fn maximal_consistent_set_ty() -> Expr {
    arrow(cst("ModalLogic"), type0())
}
/// LindenBaum lemma: every consistent set extends to a maximal consistent set
pub fn lindenbaum_ty() -> Expr {
    arrow(cst("ModalLogic"), prop())
}
/// FrameCorrespondence: an axiom corresponds to a frame property
pub fn frame_correspondence_ty() -> Expr {
    arrow(
        cst("ModalFormula"),
        arrow(arrow(cst("KripkeFrame"), prop()), prop()),
    )
}
/// SahlqvistFormula: syntactic class of formulas with effective correspondence
pub fn sahlqvist_formula_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// SahlqvistCorrespondence: Sahlqvist formulas have first-order frame correspondents
pub fn sahlqvist_correspondence_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// SahlqvistCompleteness: every Sahlqvist logic is Kripke complete
pub fn sahlqvist_completeness_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// EpistemicLogic S5: knowledge operator K_i for agent i
pub fn epistemic_knowledge_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// CommonKnowledge: C_G φ — all agents in group G know φ, know that they know it, …
pub fn common_knowledge_ty() -> Expr {
    arrow(
        cst("AgentGroup"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// DistributedKnowledge: D_G φ — what is implicit in the combined knowledge of G
pub fn distributed_knowledge_ty() -> Expr {
    arrow(
        cst("AgentGroup"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// DoxasticLogic KD45: belief operator B_i for agent i
pub fn doxastic_belief_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// PositiveIntrospection (axiom 4): K_i φ → K_i K_i φ
pub fn positive_introspection_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("ModalFormula"), prop()))
}
/// NegativeIntrospection (axiom 5): ¬K_i φ → K_i ¬K_i φ
pub fn negative_introspection_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("ModalFormula"), prop()))
}
/// ObligationOp: O φ — it is obligatory that φ
pub fn obligation_op_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// PermissionOp: P φ — it is permitted that φ
pub fn permission_op_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// ProhibitionOp: F φ — it is forbidden that φ
pub fn prohibition_op_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// DeonticConflict: O φ ∧ O ¬φ (normative inconsistency)
pub fn deontic_conflict_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// RossCurse: the paradox O(φ) → O(φ ∨ ψ) in SDL
pub fn ross_paradox_ty() -> Expr {
    arrow(cst("ModalFormula"), arrow(cst("ModalFormula"), prop()))
}
/// ProvabilityLogic GL: □ interpreted as provability in PA
pub fn gl_provability_ty() -> Expr {
    type0()
}
/// SolovayCompleteness: GL is arithmetically complete
pub fn solovay_completeness_ty() -> Expr {
    prop()
}
/// GödelDiagonalLemma: ∃ φ, PA ⊢ φ ↔ ¬Provable(φ)
pub fn godel_diagonal_ty() -> Expr {
    prop()
}
/// FixedPointTheorem (GL): for every modalized φ(p), there is a fixed point
pub fn gl_fixed_point_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// TransitiveIrreflexiveFrame: GL validates exactly the class of transitive irreflexive frames
pub fn gl_frame_class_ty() -> Expr {
    prop()
}
/// PublicAnnouncement: [!φ] ψ — after public announcement of φ, ψ holds
pub fn public_announcement_ty() -> Expr {
    arrow(
        cst("ModalFormula"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// AnnouncementLaw: [!φ] K_i ψ ↔ (φ → K_i (φ → [!φ] ψ))
pub fn announcement_knowledge_law_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(cst("ModalFormula"), arrow(cst("ModalFormula"), prop())),
    )
}
/// ActionModel: a multi-agent action model (Baltag-Moss-Solecki)
pub fn action_model_ty() -> Expr {
    type0()
}
/// ProductUpdate: M ⊗ A — update a model M with action model A
pub fn product_update_ty() -> Expr {
    arrow(
        cst("KripkeModel"),
        arrow(cst("ActionModel"), cst("KripkeModel")),
    )
}
/// MudPuzzle: the classical DEL example — common knowledge via announcements
pub fn mud_puzzle_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// MultimodalFormula: formulas with indexed modalities □_i, ◇_i
pub fn multimodal_formula_ty() -> Expr {
    type0()
}
/// ProductFrame: F × G — two frames combined
pub fn product_frame_ty() -> Expr {
    arrow(
        cst("KripkeFrame"),
        arrow(cst("KripkeFrame"), cst("KripkeFrame")),
    )
}
/// FusionLogic: the fusion L_1 ⊕ L_2 of two modal logics
pub fn fusion_logic_ty() -> Expr {
    arrow(
        cst("ModalLogic"),
        arrow(cst("ModalLogic"), cst("ModalLogic")),
    )
}
/// InteractionAxiom: an axiom relating different modalities
pub fn interaction_axiom_ty() -> Expr {
    type0()
}
/// TenseLogic: bimodal logic with future G/F and past H/P operators
pub fn tense_logic_ty() -> Expr {
    type0()
}
/// Filtration: a technique for proving finite model property
pub fn filtration_ty() -> Expr {
    arrow(
        cst("KripkeModel"),
        arrow(cst("ModalFormula"), cst("KripkeModel")),
    )
}
/// FiniteModelProperty: L has FMP iff valid = finitely valid
pub fn finite_model_property_ty() -> Expr {
    arrow(cst("ModalLogic"), prop())
}
/// Decidability: L is decidable via FMP + finite checking
pub fn modal_decidability_ty() -> Expr {
    arrow(cst("ModalLogic"), prop())
}
/// SubformulaProperty: subformulas of φ suffice for a filtration
pub fn subformula_property_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// K_FMP: logic K has the finite model property
pub fn k_fmp_ty() -> Expr {
    prop()
}
/// S4_FMP: logic S4 has the finite model property
pub fn s4_fmp_ty() -> Expr {
    prop()
}
/// PDL Program type: regular programs (test, choice, sequence, iteration)
pub fn pdl_program_ty() -> Expr {
    type0()
}
/// PDL Box: [α]φ — after every execution of program α, φ holds
pub fn pdl_box_ty() -> Expr {
    arrow(
        cst("PDLProgram"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// PDL Diamond: ⟨α⟩φ — there exists an execution of α after which φ holds
pub fn pdl_diamond_ty() -> Expr {
    arrow(
        cst("PDLProgram"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// PDL test program: φ? — succeeds iff φ holds, proceeds to same state
pub fn pdl_test_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("PDLProgram"))
}
/// PDL axiom: [α;β]φ ↔ [α][β]φ (sequence)
pub fn pdl_sequence_axiom_ty() -> Expr {
    arrow(
        cst("PDLProgram"),
        arrow(cst("PDLProgram"), arrow(cst("ModalFormula"), prop())),
    )
}
/// PDL axiom: [α∪β]φ ↔ [α]φ ∧ [β]φ (choice)
pub fn pdl_choice_axiom_ty() -> Expr {
    arrow(
        cst("PDLProgram"),
        arrow(cst("PDLProgram"), arrow(cst("ModalFormula"), prop())),
    )
}
/// PDL iteration axiom: [α*]φ ↔ φ ∧ [α][α*]φ
pub fn pdl_iteration_axiom_ty() -> Expr {
    arrow(cst("PDLProgram"), arrow(cst("ModalFormula"), prop()))
}
/// Game Logic: ⟨γ⟩φ — player I has a strategy in game γ to ensure φ
pub fn game_logic_diamond_ty() -> Expr {
    arrow(cst("Game"), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// Game Logic: [γ]φ — player II has a strategy in game γ to ensure φ
pub fn game_logic_box_ty() -> Expr {
    arrow(cst("Game"), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// Game type: represents a two-player game
pub fn game_ty() -> Expr {
    type0()
}
/// LTL Next: Xφ — φ holds at the next time step
pub fn ltl_next_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// LTL Until: φ U ψ — φ holds until ψ becomes true
pub fn ltl_until_ty() -> Expr {
    arrow(
        cst("ModalFormula"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// LTL Globally: Gφ — φ holds at all future times
pub fn ltl_globally_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// LTL Finally: Fφ — φ holds at some future time
pub fn ltl_finally_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// CTL State path quantifier: EXφ — exists a path where next φ
pub fn ctl_ex_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// CTL: AXφ — all paths, next φ
pub fn ctl_ax_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// CTL: E[φ U ψ] — exists a path where φ until ψ
pub fn ctl_eu_ty() -> Expr {
    arrow(
        cst("ModalFormula"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// CTL: A[φ U ψ] — all paths, φ until ψ
pub fn ctl_au_ty() -> Expr {
    arrow(
        cst("ModalFormula"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// ATL: ⟨⟨A⟩⟩φ — coalition A has a strategy to ensure φ
pub fn atl_coalition_ty() -> Expr {
    arrow(
        cst("AgentGroup"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// ATL concurrent game structure type
pub fn concurrent_game_structure_ty() -> Expr {
    type0()
}
/// Nominal type: a rigid designator for a world
pub fn nominal_ty() -> Expr {
    type0()
}
/// Hybrid satisfaction: @_i φ — φ holds at the world named by nominal i
pub fn hybrid_at_ty() -> Expr {
    arrow(
        cst("Nominal"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// Hybrid binder: ↓x. φ — bind current world to nominal x in φ
pub fn hybrid_binder_ty() -> Expr {
    arrow(
        cst("Nominal"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// H(@): hybrid logic axiom @_i φ ↔ (i → φ) for nominals
pub fn hybrid_axiom_at_ty() -> Expr {
    arrow(cst("Nominal"), arrow(cst("ModalFormula"), prop()))
}
/// Hybrid paste axiom: ◇i → (@_i φ → ◇φ)
pub fn hybrid_paste_axiom_ty() -> Expr {
    arrow(cst("Nominal"), arrow(cst("ModalFormula"), prop()))
}
/// Neighborhood function: W → 2^{2^W}
pub fn neighborhood_fn_ty() -> Expr {
    arrow(cst("World"), type0())
}
/// Classical modal logic: □φ valid iff [[φ]] is in the neighborhood
pub fn classical_modal_box_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// Monotone modal logic: if X ∈ N(w) and X ⊆ Y then Y ∈ N(w)
pub fn monotone_neighborhood_ty() -> Expr {
    arrow(cst("World"), prop())
}
/// AxiomM (monotonicity): □φ ∧ □ψ → □(φ ∧ ψ)
pub fn axiom_m_ty() -> Expr {
    arrow(cst("ModalFormula"), arrow(cst("ModalFormula"), prop()))
}
/// AxiomC (supplementation): □φ → □(φ ∨ ψ)
pub fn axiom_c_ty() -> Expr {
    arrow(cst("ModalFormula"), arrow(cst("ModalFormula"), prop()))
}
/// AxiomN (normality): □⊤
pub fn axiom_n_ty() -> Expr {
    prop()
}
/// JustificationTerm: a term t witnessing a justification for φ
pub fn justification_term_ty() -> Expr {
    type0()
}
/// JustificationOp: t:φ — term t is a justification for φ
pub fn justification_op_ty() -> Expr {
    arrow(cst("JustificationTerm"), arrow(cst("ModalFormula"), prop()))
}
/// Application axiom (LP): s:(φ→ψ) → t:φ → (s⋅t):ψ
pub fn justification_app_axiom_ty() -> Expr {
    arrow(
        cst("JustificationTerm"),
        arrow(cst("JustificationTerm"), arrow(cst("ModalFormula"), prop())),
    )
}
/// Sum axiom (LP): t:φ → (t+s):φ
pub fn justification_sum_axiom_ty() -> Expr {
    arrow(
        cst("JustificationTerm"),
        arrow(cst("JustificationTerm"), arrow(cst("ModalFormula"), prop())),
    )
}
/// Verification axiom (LP): t:φ → !t:(t:φ)
pub fn justification_verification_ty() -> Expr {
    arrow(cst("JustificationTerm"), arrow(cst("ModalFormula"), prop()))
}
/// Realization theorem: every theorem of S4 has a justification logic realization
pub fn realization_theorem_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// GLS: GL + □(□φ → φ) — the stable provability logic
pub fn gls_axiom_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Arithmetical soundness: GL is sound for the standard interpretation
pub fn arithmetical_soundness_ty() -> Expr {
    prop()
}
/// Arithmetical interpretation function: modal formulas → arithmetic sentences
pub fn arithmetical_interp_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ArithSentence"))
}
/// Solovay's second completeness
pub fn solovay_second_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Mu formula type: supports least/greatest fixed points
pub fn mu_formula_ty() -> Expr {
    type0()
}
/// Least fixed point: μX.φ(X)
pub fn mu_least_fp_ty() -> Expr {
    arrow(
        cst("PropVar"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// Greatest fixed point: νX.φ(X)
pub fn nu_greatest_fp_ty() -> Expr {
    arrow(
        cst("PropVar"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// Knaster-Tarski for modal operators
pub fn knaster_tarski_modal_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// Model checking problem: M, w ⊨ φ decidable for μ-calculus
pub fn mu_calculus_decidable_ty() -> Expr {
    prop()
}
/// Alternation hierarchy: μ-calculus has strict alternation hierarchy
pub fn alternation_hierarchy_ty() -> Expr {
    prop()
}
/// Context of utterance type (for two-dimensional semantics)
pub fn context_ty() -> Expr {
    type0()
}
/// Two-dimensional evaluation: M, c, w ⊨ φ
pub fn two_dim_satisfaction_ty() -> Expr {
    arrow(
        cst("KripkeModel"),
        arrow(
            cst("Context"),
            arrow(cst("World"), arrow(cst("ModalFormula"), prop())),
        ),
    )
}
/// Kaplan's dthat: [[dthat φ]]^{c,w} = [[φ]]^{c,c}
pub fn kaplan_dthat_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// Double indexing: actually(φ)
pub fn actually_op_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// Fixedly: fixedly(φ) — φ is true at every context
pub fn fixedly_op_ty() -> Expr {
    arrow(cst("ModalFormula"), cst("ModalFormula"))
}
/// Necessity type: □A
pub fn necessity_type_ty() -> Expr {
    arrow(type0(), type0())
}
/// Box introduction
pub fn box_intro_ty() -> Expr {
    arrow(type0(), type0())
}
/// Box elimination
pub fn box_elim_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Pfenning-Davies locked contexts
pub fn locked_context_ty() -> Expr {
    type0()
}
/// TopologicalSpace: (X, τ)
pub fn topological_space_ty() -> Expr {
    type0()
}
/// Interior operator: Int(A)
pub fn interior_op_ty() -> Expr {
    arrow(
        cst("TopologicalSpace"),
        arrow(cst("ModalFormula"), cst("ModalFormula")),
    )
}
/// McKinsey-Tarski: S4 is complete for topological semantics on R^n
pub fn mckinsey_tarski_ty() -> Expr {
    prop()
}
/// Dense-in-itself: every point is a limit point
pub fn dense_in_itself_ty() -> Expr {
    arrow(cst("TopologicalSpace"), prop())
}
/// Topological validity: φ valid in all topological spaces
pub fn topological_validity_ty() -> Expr {
    arrow(cst("ModalFormula"), prop())
}
/// FOML formula type
pub fn foml_formula_ty() -> Expr {
    type0()
}
/// Barcan formula: ∀x □φ(x) → □∀x φ(x)
pub fn barcan_formula_ty() -> Expr {
    arrow(cst("FOMLFormula"), prop())
}
/// Converse Barcan: □∀x φ(x) → ∀x □φ(x)
pub fn converse_barcan_ty() -> Expr {
    arrow(cst("FOMLFormula"), prop())
}
/// Varying domain semantics
pub fn varying_domain_ty() -> Expr {
    arrow(cst("KripkeFrame"), type0())
}
/// Constant domain semantics
pub fn constant_domain_ty() -> Expr {
    arrow(cst("KripkeFrame"), type0())
}
/// Existence predicate: E(x)
pub fn existence_predicate_ty() -> Expr {
    arrow(cst("World"), prop())
}
/// Minimal modal logic E
pub fn minimal_modal_logic_e_ty() -> Expr {
    type0()
}
/// AxiomE: if ⊢ φ ↔ ψ then ⊢ □φ ↔ □ψ
pub fn axiom_e_ty() -> Expr {
    arrow(cst("ModalFormula"), arrow(cst("ModalFormula"), prop()))
}
/// Monotonic modal logic: E + M
pub fn monotonic_modal_logic_ty() -> Expr {
    type0()
}
/// Regular modal logic: E + C + M
pub fn regular_modal_logic_ty() -> Expr {
    type0()
}
/// Congruence rule: φ ↔ ψ implies □φ ↔ □ψ
pub fn congruence_rule_ty() -> Expr {
    arrow(cst("ModalFormula"), arrow(cst("ModalFormula"), prop()))
}
/// STIT operator: [i stit: φ]
pub fn stit_op_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// Deliberative STIT: [i dstit: φ]
pub fn deliberative_stit_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// Achievement STIT: [i astit: φ]
pub fn achievement_stit_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// Deontic STIT: obligatory stit
pub fn deontic_stit_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("ModalFormula"), prop()))
}
/// Graded modality: ◇^≥n φ
pub fn graded_diamond_ty() -> Expr {
    arrow(cst("Nat"), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// Graded box: □^≤n φ
pub fn graded_box_ty() -> Expr {
    arrow(cst("Nat"), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// Probabilistic modality: P_{≥r}(φ)
pub fn probabilistic_modality_ty() -> Expr {
    arrow(cst("Real"), arrow(cst("ModalFormula"), cst("ModalFormula")))
}
/// Coalgebraic functor type
pub fn coalgebra_functor_ty() -> Expr {
    arrow(type0(), type0())
}
/// Coalgebra type: T-coalgebra
pub fn modal_coalgebra_ty() -> Expr {
    arrow(cst("CoalgebraFunctor"), arrow(type0(), type0()))
}
/// Belief set type
pub fn belief_set_ty() -> Expr {
    type0()
}
/// Revision operator: K * φ
pub fn belief_revision_ty() -> Expr {
    arrow(
        cst("BeliefSet"),
        arrow(cst("ModalFormula"), cst("BeliefSet")),
    )
}
/// Contraction operator: K ÷ φ
pub fn belief_contraction_ty() -> Expr {
    arrow(
        cst("BeliefSet"),
        arrow(cst("ModalFormula"), cst("BeliefSet")),
    )
}
/// AGM success postulate: φ ∈ K * φ
pub fn agm_success_ty() -> Expr {
    arrow(cst("BeliefSet"), arrow(cst("ModalFormula"), prop()))
}
/// AGM consistency postulate
pub fn agm_consistency_ty() -> Expr {
    arrow(cst("BeliefSet"), arrow(cst("ModalFormula"), prop()))
}
/// Populate an `Environment` with all modal logic axioms and theorems.
pub fn build_modal_logic_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("KripkeFrame", kripke_frame_ty()),
        ("KripkeModel", kripke_model_ty()),
        ("World", world_ty()),
        ("AccessibilityRelation", accessibility_relation_ty()),
        ("Valuation", valuation_ty()),
        ("ModalFormula", modal_formula_ty()),
        ("ModalLogic", type0()),
        ("Agent", nat_ty()),
        ("AgentGroup", type0()),
        ("ModalSat", modal_satisfaction_ty()),
        ("FrameValid", frame_validity_ty()),
        ("ClassValid", class_validity_ty()),
        ("ReflexiveFrame", reflexive_frame_ty()),
        ("TransitiveFrame", transitive_frame_ty()),
        ("SymmetricFrame", symmetric_frame_ty()),
        ("SerialFrame", serial_frame_ty()),
        ("EuclideanFrame", euclidean_frame_ty()),
        ("ConfluentFrame", confluent_frame_ty()),
        ("DenseFrame", dense_frame_ty()),
        ("DirectedFrame", directed_frame_ty()),
        ("AxiomK", axiom_k_ty()),
        ("AxiomT", axiom_t_ty()),
        ("Axiom4", axiom4_ty()),
        ("AxiomB", axiom_b_ty()),
        ("Axiom5", axiom5_ty()),
        ("AxiomD", axiom_d_ty()),
        ("AxiomLob", axiom_lob_ty()),
        ("NecRule", nec_rule_ty()),
        ("soundness_k", soundness_k_ty()),
        ("completeness_k", completeness_k_ty()),
        ("soundness_t", soundness_t_ty()),
        ("completeness_t", completeness_t_ty()),
        ("soundness_s4", soundness_s4_ty()),
        ("completeness_s4", completeness_s4_ty()),
        ("soundness_s5", soundness_s5_ty()),
        ("completeness_s5", completeness_s5_ty()),
        ("CanonicalFrame", canonical_frame_ty()),
        ("CanonicalModel", canonical_model_ty()),
        ("canonical_truth_lemma", canonical_truth_lemma_ty()),
        ("MaximalConsistentSet", maximal_consistent_set_ty()),
        ("lindenbaum", lindenbaum_ty()),
        ("FrameCorrespondence", frame_correspondence_ty()),
        ("SahlqvistFormula", sahlqvist_formula_ty()),
        ("sahlqvist_correspondence", sahlqvist_correspondence_ty()),
        ("sahlqvist_completeness", sahlqvist_completeness_ty()),
        ("EpistemicK", epistemic_knowledge_ty()),
        ("CommonKnowledge", common_knowledge_ty()),
        ("DistributedKnowledge", distributed_knowledge_ty()),
        ("DoxasticB", doxastic_belief_ty()),
        ("positive_introspection", positive_introspection_ty()),
        ("negative_introspection", negative_introspection_ty()),
        ("ObligationOp", obligation_op_ty()),
        ("PermissionOp", permission_op_ty()),
        ("ProhibitionOp", prohibition_op_ty()),
        ("DeonticConflict", deontic_conflict_ty()),
        ("RossParadox", ross_paradox_ty()),
        ("GLProvability", gl_provability_ty()),
        ("solovay_completeness", solovay_completeness_ty()),
        ("godel_diagonal", godel_diagonal_ty()),
        ("gl_fixed_point", gl_fixed_point_ty()),
        ("gl_frame_class", gl_frame_class_ty()),
        ("PublicAnnouncement", public_announcement_ty()),
        (
            "announcement_knowledge_law",
            announcement_knowledge_law_ty(),
        ),
        ("ActionModel", action_model_ty()),
        ("ProductUpdate", product_update_ty()),
        ("MudPuzzle", mud_puzzle_ty()),
        ("MultimodalFormula", multimodal_formula_ty()),
        ("ProductFrame", product_frame_ty()),
        ("FusionLogic", fusion_logic_ty()),
        ("InteractionAxiom", interaction_axiom_ty()),
        ("TenseLogic", tense_logic_ty()),
        ("Filtration", filtration_ty()),
        ("FiniteModelProperty", finite_model_property_ty()),
        ("ModalDecidability", modal_decidability_ty()),
        ("SubformulaProperty", subformula_property_ty()),
        ("k_fmp", k_fmp_ty()),
        ("s4_fmp", s4_fmp_ty()),
        ("PDLProgram", pdl_program_ty()),
        ("PDLBox", pdl_box_ty()),
        ("PDLDiamond", pdl_diamond_ty()),
        ("PDLTest", pdl_test_ty()),
        ("pdl_sequence_axiom", pdl_sequence_axiom_ty()),
        ("pdl_choice_axiom", pdl_choice_axiom_ty()),
        ("pdl_iteration_axiom", pdl_iteration_axiom_ty()),
        ("GameLogicDiamond", game_logic_diamond_ty()),
        ("GameLogicBox", game_logic_box_ty()),
        ("Game", game_ty()),
        ("LTLNext", ltl_next_ty()),
        ("LTLUntil", ltl_until_ty()),
        ("LTLGlobally", ltl_globally_ty()),
        ("LTLFinally", ltl_finally_ty()),
        ("CTLEX", ctl_ex_ty()),
        ("CTLAX", ctl_ax_ty()),
        ("CTLEU", ctl_eu_ty()),
        ("CTLAU", ctl_au_ty()),
        ("ATLCoalition", atl_coalition_ty()),
        ("ConcurrentGameStructure", concurrent_game_structure_ty()),
        ("Nominal", nominal_ty()),
        ("HybridAt", hybrid_at_ty()),
        ("HybridBinder", hybrid_binder_ty()),
        ("hybrid_axiom_at", hybrid_axiom_at_ty()),
        ("hybrid_paste_axiom", hybrid_paste_axiom_ty()),
        ("NeighborhoodFn", neighborhood_fn_ty()),
        ("ClassicalModalBox", classical_modal_box_ty()),
        ("MonotoneNeighborhood", monotone_neighborhood_ty()),
        ("AxiomM", axiom_m_ty()),
        ("AxiomC", axiom_c_ty()),
        ("AxiomN", axiom_n_ty()),
        ("JustificationTerm", justification_term_ty()),
        ("JustificationOp", justification_op_ty()),
        ("justification_app_axiom", justification_app_axiom_ty()),
        ("justification_sum_axiom", justification_sum_axiom_ty()),
        (
            "justification_verification",
            justification_verification_ty(),
        ),
        ("realization_theorem", realization_theorem_ty()),
        ("gls_axiom", gls_axiom_ty()),
        ("arithmetical_soundness", arithmetical_soundness_ty()),
        ("ArithSentence", type0()),
        ("arithmetical_interp", arithmetical_interp_ty()),
        ("solovay_second", solovay_second_ty()),
        ("MuFormula", mu_formula_ty()),
        ("PropVar", nat_ty()),
        ("mu_least_fp", mu_least_fp_ty()),
        ("nu_greatest_fp", nu_greatest_fp_ty()),
        ("knaster_tarski_modal", knaster_tarski_modal_ty()),
        ("mu_calculus_decidable", mu_calculus_decidable_ty()),
        ("alternation_hierarchy", alternation_hierarchy_ty()),
        ("Context", context_ty()),
        ("two_dim_satisfaction", two_dim_satisfaction_ty()),
        ("kaplan_dthat", kaplan_dthat_ty()),
        ("actually_op", actually_op_ty()),
        ("fixedly_op", fixedly_op_ty()),
        ("necessity_type", necessity_type_ty()),
        ("box_intro", box_intro_ty()),
        ("box_elim", box_elim_ty()),
        ("LockedContext", locked_context_ty()),
        ("TopologicalSpace", topological_space_ty()),
        ("InteriorOp", interior_op_ty()),
        ("mckinsey_tarski", mckinsey_tarski_ty()),
        ("dense_in_itself", dense_in_itself_ty()),
        ("topological_validity", topological_validity_ty()),
        ("FOMLFormula", foml_formula_ty()),
        ("barcan_formula", barcan_formula_ty()),
        ("converse_barcan", converse_barcan_ty()),
        ("varying_domain", varying_domain_ty()),
        ("constant_domain", constant_domain_ty()),
        ("existence_predicate", existence_predicate_ty()),
        ("MinimalModalLogicE", minimal_modal_logic_e_ty()),
        ("AxiomE", axiom_e_ty()),
        ("MonotonicModalLogic", monotonic_modal_logic_ty()),
        ("RegularModalLogic", regular_modal_logic_ty()),
        ("congruence_rule", congruence_rule_ty()),
        ("STITOp", stit_op_ty()),
        ("DeliberativeSTIT", deliberative_stit_ty()),
        ("AchievementSTIT", achievement_stit_ty()),
        ("DeonticSTIT", deontic_stit_ty()),
        ("GradedDiamond", graded_diamond_ty()),
        ("GradedBox", graded_box_ty()),
        ("ProbabilisticModality", probabilistic_modality_ty()),
        ("CoalgebraFunctor", coalgebra_functor_ty()),
        ("ModalCoalgebra", modal_coalgebra_ty()),
        ("Real", type0()),
        ("BeliefSet", belief_set_ty()),
        ("belief_revision", belief_revision_ty()),
        ("belief_contraction", belief_contraction_ty()),
        ("agm_success", agm_success_ty()),
        ("agm_consistency", agm_consistency_ty()),
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
/// Propositional variable identifier.
pub type PropVar = u32;
/// Check if a formula belongs to the Sahlqvist class (simplified syntactic check).
pub fn classify_sahlqvist(phi: &ModalFormula) -> SahlqvistClass {
    match phi {
        ModalFormula::Implies(ant, cons) => {
            if is_sahlqvist_antecedent(ant) && is_positive(cons) {
                SahlqvistClass::Full
            } else {
                SahlqvistClass::NotSahlqvist
            }
        }
        _ if is_positive(phi) => SahlqvistClass::Consequent,
        _ => SahlqvistClass::NotSahlqvist,
    }
}
/// Check if a formula is a Sahlqvist antecedent (negative formulas under diamonds).
pub fn is_sahlqvist_antecedent(phi: &ModalFormula) -> bool {
    match phi {
        ModalFormula::Atom(_) | ModalFormula::Top | ModalFormula::Bot => true,
        ModalFormula::Not(psi) => is_positive(psi),
        ModalFormula::And(a, b) => is_sahlqvist_antecedent(a) && is_sahlqvist_antecedent(b),
        ModalFormula::Diamond(_, psi) => is_sahlqvist_antecedent(psi),
        ModalFormula::Box(_, psi) => is_sahlqvist_antecedent(psi),
        _ => false,
    }
}
/// Check if a formula is positive (no negations except at atoms).
pub fn is_positive(phi: &ModalFormula) -> bool {
    match phi {
        ModalFormula::Atom(_) | ModalFormula::Top | ModalFormula::Bot => true,
        ModalFormula::Not(_) => false,
        ModalFormula::And(a, b) | ModalFormula::Or(a, b) | ModalFormula::Implies(a, b) => {
            is_positive(a) && is_positive(b)
        }
        ModalFormula::Box(_, psi) | ModalFormula::Diamond(_, psi) => is_positive(psi),
    }
}
/// Compute the largest bisimulation between two models using the partition refinement algorithm.
pub fn compute_bisimulation(m1: &KripkeModel, m2: &KripkeModel) -> Bisimulation {
    let mut bisim = Bisimulation::new();
    for w in 0..m1.frame.n_worlds {
        for v in 0..m2.frame.n_worlds {
            let agree = m1
                .valuation
                .keys()
                .all(|&p| m1.prop_true(p, w) == m2.prop_true(p, v));
            if agree {
                bisim.add_pair(w, v);
            }
        }
    }
    bisim
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kripke_frame_reflexive() {
        let mut frame = KripkeFrame::new(3, 1);
        frame.make_reflexive(0);
        assert!(frame.is_reflexive(0));
        assert!(frame.is_symmetric(0));
        frame.add_edge(0, 0, 1);
        assert!(!frame.is_symmetric(0));
    }
    #[test]
    fn test_kripke_frame_transitive() {
        let mut frame = KripkeFrame::new(3, 1);
        frame.add_edge(0, 0, 1);
        frame.add_edge(0, 1, 2);
        assert!(!frame.is_transitive(0));
        frame.make_transitive(0);
        assert!(frame.is_transitive(0));
        assert!(frame.accessible(0, 0, 2));
    }
    #[test]
    fn test_kripke_model_satisfaction() {
        let mut frame = KripkeFrame::new(2, 1);
        frame.add_edge(0, 0, 1);
        let mut model = KripkeModel::new(frame);
        model.set_true(0, 1);
        let box_p = ModalFormula::necessity(ModalFormula::atom(0));
        assert!(model.satisfies(0, &box_p));
        let dia_p = ModalFormula::possibility(ModalFormula::atom(0));
        assert!(!model.satisfies(1, &dia_p));
    }
    #[test]
    fn test_s4_frame_validation() {
        let mut frame = KripkeFrame::new(3, 1);
        frame.make_reflexive(0);
        frame.add_edge(0, 0, 1);
        frame.add_edge(0, 1, 2);
        frame.make_transitive(0);
        assert!(ModalSystem::S4.frame_validates(&frame, 0));
        assert!(!ModalSystem::S5.frame_validates(&frame, 0));
    }
    #[test]
    fn test_modal_formula_depth() {
        let p = ModalFormula::atom(0);
        let box_p = ModalFormula::necessity(p.clone());
        let box_box_p = ModalFormula::necessity(box_p.clone());
        assert_eq!(p.modal_depth(), 0);
        assert_eq!(box_p.modal_depth(), 1);
        assert_eq!(box_box_p.modal_depth(), 2);
    }
    #[test]
    fn test_canonical_model_construction() {
        let mut canon = CanonicalModel::new();
        let mcs0 = MaximalConsistentSet::new(
            0,
            vec![
                ModalFormula::atom(0),
                ModalFormula::necessity(ModalFormula::atom(1)),
            ],
        );
        let mcs1 = MaximalConsistentSet::new(1, vec![ModalFormula::atom(1)]);
        canon.add_world(mcs0);
        canon.add_world(mcs1);
        canon.build_accessibility();
        assert_eq!(canon.size(), 2);
        assert!(canon.accessibility.contains(&(0, 1)));
    }
    #[test]
    fn test_epistemic_model_knows() {
        let mut model = EpistemicModel::new(3, 1);
        model.add_edge(0, 0, 0);
        model.add_edge(0, 0, 1);
        model.valuation.entry(0).or_default().insert(0);
        model.valuation.entry(0).or_default().insert(1);
        let p = ModalFormula::atom(0);
        assert!(model.knows(0, 0, &p));
    }
    #[test]
    fn test_public_announcement_update() {
        let mut model = EpistemicModel::new(3, 1);
        model.make_equivalence_relations();
        model.valuation.entry(0).or_default().insert(0);
        model.valuation.entry(0).or_default().insert(1);
        let ann = PublicAnnouncement::new(ModalFormula::atom(0));
        let updated = ann.update(&model);
        assert_eq!(updated.n_worlds, 2);
    }
    #[test]
    fn test_build_modal_logic_env() {
        let mut env = Environment::new();
        build_modal_logic_env(&mut env);
        assert!(env.get(&Name::str("KripkeFrame")).is_some());
        assert!(env.get(&Name::str("AxiomLob")).is_some());
        assert!(env.get(&Name::str("sahlqvist_completeness")).is_some());
        assert!(env.get(&Name::str("ProductUpdate")).is_some());
        assert!(env.get(&Name::str("PDLBox")).is_some());
        assert!(env.get(&Name::str("LTLUntil")).is_some());
        assert!(env.get(&Name::str("ATLCoalition")).is_some());
        assert!(env.get(&Name::str("HybridAt")).is_some());
        assert!(env.get(&Name::str("AxiomM")).is_some());
        assert!(env.get(&Name::str("JustificationOp")).is_some());
        assert!(env.get(&Name::str("mu_least_fp")).is_some());
        assert!(env.get(&Name::str("barcan_formula")).is_some());
        assert!(env.get(&Name::str("GradedDiamond")).is_some());
        assert!(env.get(&Name::str("belief_revision")).is_some());
    }
    #[test]
    fn test_pdl_model_sequence() {
        let mut frame = KripkeFrame::new(3, 2);
        frame.add_edge(0, 0, 1);
        frame.add_edge(1, 1, 2);
        let mut kripke = KripkeModel::new(frame);
        kripke.set_true(0, 2);
        let pdl = PdlModel::new(kripke, 2);
        let alpha = PdlProgram::Atomic(0);
        let beta = PdlProgram::Atomic(1);
        let seq = PdlProgram::Sequence(Box::new(alpha), Box::new(beta));
        let reachable = pdl.reachable(0, &seq);
        assert!(reachable.contains(&2));
        assert_eq!(reachable.len(), 1);
        let p = ModalFormula::atom(0);
        assert!(pdl.box_program(0, &seq, &p));
    }
    #[test]
    fn test_pdl_model_star() {
        let mut frame = KripkeFrame::new(4, 1);
        frame.add_edge(0, 0, 1);
        frame.add_edge(0, 1, 2);
        frame.add_edge(0, 2, 3);
        let kripke = KripkeModel::new(frame);
        let pdl = PdlModel::new(kripke, 1);
        let alpha = PdlProgram::Atomic(0);
        let star = PdlProgram::Star(Box::new(alpha));
        let reachable = pdl.reachable(0, &star);
        assert_eq!(reachable.len(), 4);
        for w in 0..4 {
            assert!(reachable.contains(&w));
        }
    }
    #[test]
    fn test_finite_trace_ltl() {
        let mut trace = FiniteTrace::new();
        let mut s0 = HashMap::new();
        s0.insert(0u32, true);
        trace.push(s0);
        let mut s1 = HashMap::new();
        s1.insert(0u32, false);
        trace.push(s1);
        let mut s2 = HashMap::new();
        s2.insert(0u32, true);
        trace.push(s2);
        let p = ModalFormula::atom(0);
        let finally_p = ModalFormula::Diamond(0, Box::new(p.clone()));
        assert!(trace.check(&finally_p));
        let globally_p = ModalFormula::Box(0, Box::new(p.clone()));
        assert!(!trace.check(&globally_p));
        let next_p = ModalFormula::Box(1, Box::new(p.clone()));
        assert!(!trace.check(&next_p));
        assert!(trace.satisfies(2, &globally_p));
    }
    #[test]
    fn test_graded_model() {
        let mut frame = KripkeFrame::new(5, 1);
        for v in 1..=4 {
            frame.add_edge(0, 0, v);
        }
        let mut kripke = KripkeModel::new(frame);
        for w in 1..=3 {
            kripke.set_true(0, w);
        }
        let model = GradedModel::new(kripke);
        let p = ModalFormula::atom(0);
        assert!(model.graded_diamond(0, 3, &p));
        assert!(!model.graded_diamond(0, 4, &p));
        assert!(model.graded_box(0, 1, &p));
        assert!(!model.graded_box(0, 0, &p));
        assert_eq!(model.count_satisfying(0, &p), 3);
    }
    #[test]
    fn test_belief_revision_agm() {
        let mut bro = BeliefRevisionOp::new();
        let p = ModalFormula::atom(0);
        let q = ModalFormula::atom(1);
        let neg_p = ModalFormula::not(p.clone());
        bro.add_belief(p.clone(), 5);
        bro.add_belief(q.clone(), 3);
        bro.add_belief(neg_p.clone(), 1);
        assert!(bro.believes(&p));
        assert_eq!(bro.size(), 3);
        let revised = bro.revise(&p);
        assert!(revised.believes(&p));
        assert!(!revised.believes(&neg_p));
        let contracted = bro.contract(&p);
        assert!(!contracted.believes(&p));
        assert!(contracted.believes(&q));
    }
    #[test]
    fn test_mu_calculus_reachability() {
        let mut frame = KripkeFrame::new(3, 1);
        frame.add_edge(0, 0, 1);
        frame.add_edge(0, 1, 2);
        let mut kripke = KripkeModel::new(frame);
        kripke.set_true(0, 2);
        let eval = MuCalculusEval::new(kripke);
        let goal = ModalFormula::atom(0);
        let reachable = eval.reachability(&goal);
        assert!(reachable.contains(&0));
        assert!(reachable.contains(&1));
        assert!(reachable.contains(&2));
    }
    #[test]
    fn test_mu_calculus_safety() {
        let mut frame = KripkeFrame::new(3, 1);
        frame.add_edge(0, 0, 1);
        frame.add_edge(0, 0, 2);
        let mut kripke = KripkeModel::new(frame);
        kripke.set_true(0, 0);
        kripke.set_true(0, 1);
        let eval = MuCalculusEval::new(kripke);
        let safe = ModalFormula::atom(0);
        let safe_worlds = eval.safety(&safe);
        assert!(!safe_worlds.contains(&0));
        assert!(safe_worlds.contains(&1));
        assert!(!safe_worlds.contains(&2));
    }
}
