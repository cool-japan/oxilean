//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AlphaEquivalenceChecker, BetaReducer, BinarySession, Context, LinearTypeChecker,
    SessionTypeCompatibility, SimpleType, Strategy, Term, TypeInferenceSystem,
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
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
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
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn option_ty(elem: Expr) -> Expr {
    app(cst("Option"), elem)
}
/// UntypedTerm: the type of untyped lambda terms (de Bruijn representation)
pub fn untyped_term_ty() -> Expr {
    type0()
}
/// Variable: de Bruijn index n refers to the n-th enclosing binder
pub fn variable_ty() -> Expr {
    arrow(nat_ty(), untyped_term_ty())
}
/// Abstraction: λ.t (binder in de Bruijn — no variable name needed)
pub fn abstraction_ty() -> Expr {
    arrow(untyped_term_ty(), untyped_term_ty())
}
/// Application: t s
pub fn application_ty() -> Expr {
    arrow(
        untyped_term_ty(),
        arrow(untyped_term_ty(), untyped_term_ty()),
    )
}
/// Substitution: t[s/n] — substitute s for de Bruijn index n in t
pub fn substitution_ty() -> Expr {
    arrow(
        untyped_term_ty(),
        arrow(untyped_term_ty(), arrow(nat_ty(), untyped_term_ty())),
    )
}
/// BetaRedex: t is a beta redex iff t = (λ.s) u
pub fn beta_redex_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// EtaRedex: t is an eta redex iff t = λ.(s (BVar 0)) with BVar 0 not free in s
pub fn eta_redex_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// BetaStep: one-step β-reduction t →β s
pub fn beta_step_ty() -> Expr {
    arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop()))
}
/// EtaStep: one-step η-reduction t →η s
pub fn eta_step_ty() -> Expr {
    arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop()))
}
/// BetaReduction: reflexive transitive closure of →β
pub fn beta_reduction_ty() -> Expr {
    arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop()))
}
/// BetaEquiv: β-equivalence (=β)
pub fn beta_equiv_ty() -> Expr {
    arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop()))
}
/// NormalForm: t has no β-redex
pub fn normal_form_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// WeakNormalForm: t can be reduced to a normal form
pub fn weak_normal_form_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// HeadNormalForm: the head of t is not a redex
pub fn head_normal_form_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// ChurchNumeral: the Church encoding of n as λf.λx.f^n x
pub fn church_numeral_ty() -> Expr {
    arrow(nat_ty(), untyped_term_ty())
}
/// ChurchSucc: the successor combinator S = λn.λf.λx.f (n f x)
pub fn church_succ_ty() -> Expr {
    untyped_term_ty()
}
/// ChurchPlus: addition combinator M = λm.λn.λf.λx.m f (n f x)
pub fn church_plus_ty() -> Expr {
    untyped_term_ty()
}
/// ChurchMul: multiplication combinator M = λm.λn.λf.m (n f)
pub fn church_mul_ty() -> Expr {
    untyped_term_ty()
}
/// ChurchExp: exponentiation combinator E = λm.λn.n m
pub fn church_exp_ty() -> Expr {
    untyped_term_ty()
}
/// ChurchPred: predecessor combinator (Kleene encoding)
pub fn church_pred_ty() -> Expr {
    untyped_term_ty()
}
/// ChurchIsZero: test whether a Church numeral is zero
pub fn church_is_zero_ty() -> Expr {
    arrow(untyped_term_ty(), bool_ty())
}
/// ChurchNumeralCorrect: church_numeral n reduces to the standard encoding
pub fn church_numeral_correct_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// ChurchArithCorrect: Church arithmetic is correct w.r.t. Nat arithmetic
pub fn church_arith_correct_ty() -> Expr {
    prop()
}
/// YCombinator: Curry's Y = λf.(λx.f(x x))(λx.f(x x))
pub fn y_combinator_ty() -> Expr {
    untyped_term_ty()
}
/// TuringCombinator: Turing's Θ fixed point combinator
pub fn turing_combinator_ty() -> Expr {
    untyped_term_ty()
}
/// RecursionTheorem: every function has a fixed point
pub fn recursion_theorem_ty() -> Expr {
    arrow(arrow(untyped_term_ty(), untyped_term_ty()), prop())
}
/// YFixedPoint: Y f =β f (Y f)
pub fn y_fixed_point_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// OmegaCombinator: Ω = (λx.x x)(λx.x x) — diverging term
pub fn omega_combinator_ty() -> Expr {
    untyped_term_ty()
}
/// OmegaDiverges: Ω has no normal form
pub fn omega_diverges_ty() -> Expr {
    prop()
}
/// NormalOrderRedex: leftmost-outermost redex
pub fn normal_order_redex_ty() -> Expr {
    arrow(untyped_term_ty(), option_ty(untyped_term_ty()))
}
/// ApplicativeOrderRedex: leftmost-innermost redex (call by value)
pub fn applicative_order_redex_ty() -> Expr {
    arrow(untyped_term_ty(), option_ty(untyped_term_ty()))
}
/// HeadReduction: reduce the head redex (lazy evaluation)
pub fn head_reduction_ty() -> Expr {
    arrow(untyped_term_ty(), option_ty(untyped_term_ty()))
}
/// NormalOrderStrategy: normal order finds normal form if one exists
pub fn normal_order_strategy_ty() -> Expr {
    prop()
}
/// StandardizationTheorem: if t has a normal form, normal order reaches it
pub fn standardization_theorem_ty() -> Expr {
    prop()
}
/// CallByValueReduction: strict/CBV evaluation
pub fn cbv_reduction_ty() -> Expr {
    arrow(untyped_term_ty(), option_ty(untyped_term_ty()))
}
/// CallByNeedReduction: lazy/sharing evaluation
pub fn cbn_reduction_ty() -> Expr {
    arrow(untyped_term_ty(), option_ty(untyped_term_ty()))
}
/// DiamondProperty: if t →₁ u and t →₁ v then ∃ w, u →₁ w and v →₁ w
pub fn diamond_property_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "t",
        untyped_term_ty(),
        pi(
            BinderInfo::Default,
            "u",
            untyped_term_ty(),
            pi(BinderInfo::Default, "v", untyped_term_ty(), prop()),
        ),
    )
}
/// ChurchRosserTheorem: β-reduction is confluent
pub fn church_rosser_theorem_ty() -> Expr {
    prop()
}
/// Confluence: the reflexive transitive closure of →β has the diamond property
pub fn confluence_ty() -> Expr {
    prop()
}
/// ParallelReduction: parallel β-reduction (β∥) — key for CR proof
pub fn parallel_reduction_ty() -> Expr {
    arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop()))
}
/// ParallelReductionComplete: t →β s implies t →β∥ s
pub fn parallel_reduction_complete_ty() -> Expr {
    prop()
}
/// ParallelMaxReduct: complete parallel development
pub fn parallel_max_reduct_ty() -> Expr {
    arrow(untyped_term_ty(), untyped_term_ty())
}
/// ParallelMaxReductProperty: diamond property for complete development
pub fn parallel_max_reduct_property_ty() -> Expr {
    prop()
}
/// BohmTree: the Böhm tree of a lambda term (possibly infinite)
pub fn bohm_tree_ty() -> Expr {
    type0()
}
/// BohmTreeOf: compute the Böhm tree of a term
pub fn bohm_tree_of_ty() -> Expr {
    arrow(untyped_term_ty(), bohm_tree_ty())
}
/// UnsolvableTerm: a term with no head normal form
pub fn unsolvable_term_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// SolvableTerm: a term with a head normal form
pub fn solvable_term_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// BohmEquiv: two terms are Böhm-equal iff their Böhm trees are equal
pub fn bohm_equiv_ty() -> Expr {
    arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop()))
}
/// ObservationalEquiv: contextual equivalence in the untyped λ-calculus
pub fn observational_equiv_ty() -> Expr {
    arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop()))
}
/// BohmTheorem: two distinct β-normal forms are separable by a context
pub fn bohm_theorem_ty() -> Expr {
    prop()
}
/// SimpleType: the type of simple types over a base type set
pub fn simple_type_ty() -> Expr {
    type0()
}
/// BaseType: a ground/atomic type
pub fn base_type_ty() -> Expr {
    arrow(type0(), simple_type_ty())
}
/// ArrowType: function type A → B in simple type theory
pub fn arrow_type_ty() -> Expr {
    arrow(simple_type_ty(), arrow(simple_type_ty(), simple_type_ty()))
}
/// TypingContext: a finite map from variables to simple types
pub fn typing_context_ty() -> Expr {
    type0()
}
/// STLCTyping: Γ ⊢ t : τ (simple type assignment)
pub fn stlc_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(simple_type_ty(), prop())),
    )
}
/// STLCSubjectReduction: if Γ ⊢ t : τ and t →β s then Γ ⊢ s : τ
pub fn stlc_subject_reduction_ty() -> Expr {
    prop()
}
/// STLCStrongNormalization: all well-typed STLC terms are strongly normalizing
pub fn stlc_strong_normalization_ty() -> Expr {
    prop()
}
/// STLCChurchRosser: STLC β-reduction is confluent
pub fn stlc_church_rosser_ty() -> Expr {
    prop()
}
/// STLCDecidability: type-checking STLC is decidable
pub fn stlc_decidability_ty() -> Expr {
    prop()
}
/// SystemFType: types of System F (including type variables and ∀)
pub fn system_f_type_ty() -> Expr {
    type0()
}
/// SystemFTerm: terms of System F (including type abstraction and application)
pub fn system_f_term_ty() -> Expr {
    type0()
}
/// SystemFTyping: typing judgment for System F
pub fn system_f_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(system_f_term_ty(), arrow(system_f_type_ty(), prop())),
    )
}
/// SystemFStrongNormalization: System F is strongly normalizing
pub fn system_f_sn_ty() -> Expr {
    prop()
}
/// SystemFConfluence: System F β-reduction is confluent
pub fn system_f_confluence_ty() -> Expr {
    prop()
}
/// SystemFParametricity: Reynolds's abstraction theorem
pub fn system_f_parametricity_ty() -> Expr {
    prop()
}
/// SystemFUndecidableTyping: type inference for System F is undecidable (Wells)
pub fn system_f_undecidable_ty() -> Expr {
    prop()
}
/// ChurchEncoding_NatF: System F encoding of natural numbers
/// ∀ α. (α → α) → α → α
pub fn church_nat_f_ty() -> Expr {
    system_f_type_ty()
}
/// ChurchEncoding_BoolF: System F encoding of booleans
/// ∀ α. α → α → α
pub fn church_bool_f_ty() -> Expr {
    system_f_type_ty()
}
/// ChurchEncoding_ListF: System F encoding of lists
/// ∀ α β. β → (α → β → β) → β
pub fn church_list_f_ty() -> Expr {
    system_f_type_ty()
}
/// Kind: the kind language of System Fω (* and →)
pub fn kind_ty() -> Expr {
    type0()
}
/// StarKind: the base kind *
pub fn star_kind_ty() -> Expr {
    kind_ty()
}
/// KindArrow: kind constructor A ⇒ B
pub fn kind_arrow_ty() -> Expr {
    arrow(kind_ty(), arrow(kind_ty(), kind_ty()))
}
/// TypeConstructor: a type-level function with a kind
pub fn type_constructor_ty() -> Expr {
    type0()
}
/// SystemFOmegaType: types of System Fω
pub fn system_fomega_type_ty() -> Expr {
    type0()
}
/// SystemFOmegaTerm: terms of System Fω
pub fn system_fomega_term_ty() -> Expr {
    type0()
}
/// SystemFOmegaTyping: typing judgment for Fω
pub fn system_fomega_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(
            system_fomega_term_ty(),
            arrow(system_fomega_type_ty(), prop()),
        ),
    )
}
/// SystemFOmegaSN: Fω is strongly normalizing
pub fn system_fomega_sn_ty() -> Expr {
    prop()
}
/// SystemFOmegaKindSoundness: well-kinded types have sound interpretations
pub fn system_fomega_kind_soundness_ty() -> Expr {
    prop()
}
/// PTS_Axiom: the set of axioms (s : s') for a pure type system
pub fn pts_axiom_ty() -> Expr {
    type0()
}
/// PTS_Rule: the set of rules (s₁, s₂, s₃) for Π-type formation
pub fn pts_rule_ty() -> Expr {
    type0()
}
/// PureTypeSystem: a PTS defined by sorts S, axioms A, rules R
pub fn pure_type_system_ty() -> Expr {
    type0()
}
/// PTSTyping: Γ ⊢ t : T in a PTS
pub fn pts_typing_ty() -> Expr {
    arrow(
        pure_type_system_ty(),
        arrow(
            typing_context_ty(),
            arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop())),
        ),
    )
}
/// CoC: the Calculus of Constructions (top of the λ-cube)
pub fn coc_ty() -> Expr {
    pure_type_system_ty()
}
/// LambdaArrow: simply-typed lambda calculus corner of cube
pub fn lambda_arrow_ty() -> Expr {
    pure_type_system_ty()
}
/// Lambda2: System F corner of cube (type polymorphism)
pub fn lambda2_ty() -> Expr {
    pure_type_system_ty()
}
/// LambdaOmega: System Fω corner (type constructors)
pub fn lambda_omega_ty() -> Expr {
    pure_type_system_ty()
}
/// LambdaP: λP (dependent types, Edinburgh LF)
pub fn lambda_p_ty() -> Expr {
    pure_type_system_ty()
}
/// PTSSubjectReduction: subject reduction holds for all functional PTS
pub fn pts_subject_reduction_ty() -> Expr {
    arrow(pure_type_system_ty(), prop())
}
/// PTSConfluence: confluence holds for all PTS (β-reduction)
pub fn pts_confluence_ty() -> Expr {
    arrow(pure_type_system_ty(), prop())
}
/// CoCStrongNormalization: the Calculus of Constructions is SN
pub fn coc_sn_ty() -> Expr {
    prop()
}
/// PCFType: types of PCF (Plotkin's Programming Computable Functions)
/// PCF = STLC + fixpoint + nat + bool
pub fn pcf_type_ty() -> Expr {
    type0()
}
/// PCFTerm: terms of PCF (including `fix`, `pred`, `succ`, `if-then-else`)
pub fn pcf_term_ty() -> Expr {
    type0()
}
/// PCFFixpoint: the fixed-point operator `fix : (τ → τ) → τ` in PCF
pub fn pcf_fixpoint_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "tau",
        simple_type_ty(),
        arrow(arrow(bvar(0), bvar(1)), bvar(0)),
    )
}
/// PCFTyping: typing judgment for PCF
pub fn pcf_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(pcf_term_ty(), arrow(pcf_type_ty(), prop())),
    )
}
/// PCFDenotationalSemantics: Scott domain semantics for PCF
pub fn pcf_denotational_semantics_ty() -> Expr {
    prop()
}
/// PCFAdequacy: operational and denotational semantics agree for PCF
pub fn pcf_adequacy_ty() -> Expr {
    prop()
}
/// PCFFullAbstraction: Milner's full abstraction problem for PCF
pub fn pcf_full_abstraction_ty() -> Expr {
    prop()
}
/// SubtypeRelation: A <: B (A is a subtype of B)
pub fn subtype_relation_ty() -> Expr {
    arrow(simple_type_ty(), arrow(simple_type_ty(), prop()))
}
/// SubtypingReflexivity: A <: A
pub fn subtyping_reflexivity_ty() -> Expr {
    arrow(simple_type_ty(), prop())
}
/// SubtypingTransitivity: A <: B → B <: C → A <: C
pub fn subtyping_transitivity_ty() -> Expr {
    arrow(
        simple_type_ty(),
        arrow(simple_type_ty(), arrow(simple_type_ty(), prop())),
    )
}
/// BoundedQuantification: ∀ X <: T. U (F-sub style bounded ∀)
pub fn bounded_quantification_ty() -> Expr {
    arrow(
        simple_type_ty(),
        arrow(arrow(simple_type_ty(), simple_type_ty()), simple_type_ty()),
    )
}
/// FBoundedPolymorphism: F-bounded quantification ∀ X <: F\[X\]. U
pub fn f_bounded_polymorphism_ty() -> Expr {
    arrow(
        arrow(simple_type_ty(), simple_type_ty()),
        arrow(arrow(simple_type_ty(), simple_type_ty()), simple_type_ty()),
    )
}
/// CoercionFunction: a coercion c : A → B witnessing A <: B
pub fn coercion_function_ty() -> Expr {
    arrow(simple_type_ty(), arrow(simple_type_ty(), prop()))
}
/// SubtypingSubjectReduction: subtying is preserved under reduction
pub fn subtyping_subject_reduction_ty() -> Expr {
    prop()
}
/// EffectLabel: a label for a computational effect
pub fn effect_label_ty() -> Expr {
    type0()
}
/// EffectSet: a set of effect labels
pub fn effect_set_ty() -> Expr {
    type0()
}
/// EffectType: a type annotated with effects τ !ε
pub fn effect_type_ty() -> Expr {
    arrow(simple_type_ty(), arrow(effect_set_ty(), simple_type_ty()))
}
/// EffectTyping: Γ ⊢ t : τ ! ε
pub fn effect_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(
            untyped_term_ty(),
            arrow(simple_type_ty(), arrow(effect_set_ty(), prop())),
        ),
    )
}
/// RegionType: a type annotated with memory region ρ
pub fn region_type_ty() -> Expr {
    type0()
}
/// RegionInference: algorithm to infer region annotations
pub fn region_inference_ty() -> Expr {
    arrow(untyped_term_ty(), region_type_ty())
}
/// GradedType: a type T graded by a semiring element r (coeffect)
pub fn graded_type_ty() -> Expr {
    type0()
}
/// CoeffectSystem: a type system tracking resource usage via graded types
pub fn coeffect_system_ty() -> Expr {
    type0()
}
/// CoeffectTyping: Γ ⊢ t : T graded by r
pub fn coeffect_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(graded_type_ty(), prop())),
    )
}
/// LinearType: a type that must be used exactly once
pub fn linear_type_ty() -> Expr {
    type0()
}
/// LinearContext: a linear typing context (each variable used exactly once)
pub fn linear_context_ty() -> Expr {
    type0()
}
/// LinearTyping: linear typing judgment Γ ⊢_L t : A
pub fn linear_typing_ty() -> Expr {
    arrow(
        linear_context_ty(),
        arrow(untyped_term_ty(), arrow(linear_type_ty(), prop())),
    )
}
/// LinearArrow: the linear function type A ⊸ B
pub fn linear_arrow_ty() -> Expr {
    arrow(linear_type_ty(), arrow(linear_type_ty(), linear_type_ty()))
}
/// LinearExchangeability: linear contexts can be reordered
pub fn linear_exchangeability_ty() -> Expr {
    prop()
}
/// AffineType: a type that may be used at most once (weakening allowed)
pub fn affine_type_ty() -> Expr {
    type0()
}
/// AffineTyping: affine typing judgment (weakening OK, no contraction)
pub fn affine_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(affine_type_ty(), prop())),
    )
}
/// RelevantType: a type that must be used at least once (contraction OK, no weakening)
pub fn relevant_type_ty() -> Expr {
    type0()
}
/// RelevantTyping: relevant typing judgment
pub fn relevant_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(relevant_type_ty(), prop())),
    )
}
/// BangModality: the `!A` modality — unrestricted use of A
pub fn bang_modality_ty() -> Expr {
    arrow(linear_type_ty(), linear_type_ty())
}
/// LFSignature: a signature Σ in Edinburgh LF
pub fn lf_signature_ty() -> Expr {
    type0()
}
/// LFContext: a context Γ in Edinburgh LF
pub fn lf_context_ty() -> Expr {
    type0()
}
/// LFTyping: Σ; Γ ⊢ t : T in Edinburgh LF
pub fn lf_typing_ty() -> Expr {
    arrow(
        lf_signature_ty(),
        arrow(
            lf_context_ty(),
            arrow(untyped_term_ty(), arrow(untyped_term_ty(), prop())),
        ),
    )
}
/// LFKindValidity: Σ; Γ ⊢ K kind
pub fn lf_kind_validity_ty() -> Expr {
    arrow(
        lf_signature_ty(),
        arrow(lf_context_ty(), arrow(kind_ty(), prop())),
    )
}
/// UTTUniverse: a universe in Luo's Unified Theory of Types
pub fn utt_universe_ty() -> Expr {
    type0()
}
/// UTTELType: a type in UTT (el-form, deriving a type from a set)
pub fn utt_el_type_ty() -> Expr {
    arrow(utt_universe_ty(), type0())
}
/// CICInductiveType: an inductive type in the Calculus of Inductive Constructions
pub fn cic_inductive_type_ty() -> Expr {
    type0()
}
/// CICElimination: the elimination/recursor for an inductive type
pub fn cic_elimination_ty() -> Expr {
    arrow(cic_inductive_type_ty(), untyped_term_ty())
}
/// CICPositivityCondition: strict positivity for inductive types
pub fn cic_positivity_condition_ty() -> Expr {
    arrow(cic_inductive_type_ty(), prop())
}
/// IntersectionType: A ∩ B — a term has both types A and B
pub fn intersection_type_ty() -> Expr {
    arrow(simple_type_ty(), arrow(simple_type_ty(), simple_type_ty()))
}
/// IntersectionTyping: Γ ⊢ t : A ∩ B
pub fn intersection_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(simple_type_ty(), prop())),
    )
}
/// FilterModel: a filter model of the untyped λ-calculus via intersection types
pub fn filter_model_ty() -> Expr {
    type0()
}
/// IntersectionTypeCompleteness: every normalizable term is typeable
pub fn intersection_type_completeness_ty() -> Expr {
    prop()
}
/// PrincipalTyping: every typeable term has a principal type scheme
pub fn principal_typing_ty() -> Expr {
    arrow(untyped_term_ty(), prop())
}
/// UnionType: A ∪ B — occurrence typing context
pub fn union_type_ty() -> Expr {
    arrow(simple_type_ty(), arrow(simple_type_ty(), simple_type_ty()))
}
/// OccurrenceTyping: type assignment based on control flow occurrences
pub fn occurrence_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(simple_type_ty(), prop())),
    )
}
/// FlowAnalysis: 0-CFA / control-flow analysis as a type system
pub fn flow_analysis_ty() -> Expr {
    arrow(untyped_term_ty(), type0())
}
/// GradualType: a type that may be partially unknown (includes `?`)
pub fn gradual_type_ty() -> Expr {
    type0()
}
/// DynType: the dynamic type `?` (unknown type)
pub fn dyn_type_ty() -> Expr {
    gradual_type_ty()
}
/// TypeConsistency: A ~ B (types are consistent, i.e., compatible under `?`)
pub fn type_consistency_ty() -> Expr {
    arrow(gradual_type_ty(), arrow(gradual_type_ty(), prop()))
}
/// GradualTyping: Γ ⊢ t : A in the gradually-typed λ-calculus
pub fn gradual_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(gradual_type_ty(), prop())),
    )
}
/// BlameLabel: a label tracking the source of a runtime type failure
pub fn blame_label_ty() -> Expr {
    type0()
}
/// BlameCalculusTerm: a term in the blame calculus (with casts and labels)
pub fn blame_calculus_term_ty() -> Expr {
    type0()
}
/// BlameTheorem: well-typed programs can only be blamed at boundaries
pub fn blame_theorem_ty() -> Expr {
    prop()
}
/// AbstractingGradualTyping: AGT framework lifting predicates to gradual types
pub fn abstracting_gradual_typing_ty() -> Expr {
    prop()
}
/// SessionType: the type of a communication channel endpoint
pub fn session_type_ty() -> Expr {
    type0()
}
/// SendType: !T.S — send a value of type T then continue as S
pub fn send_type_ty() -> Expr {
    arrow(
        simple_type_ty(),
        arrow(session_type_ty(), session_type_ty()),
    )
}
/// RecvType: ?T.S — receive a value of type T then continue as S
pub fn recv_type_ty() -> Expr {
    arrow(
        simple_type_ty(),
        arrow(session_type_ty(), session_type_ty()),
    )
}
/// EndType: the completed session
pub fn end_type_ty() -> Expr {
    session_type_ty()
}
/// DualSession: the dual of a session type (swap sends and receives)
pub fn dual_session_ty() -> Expr {
    arrow(session_type_ty(), session_type_ty())
}
/// BinarySessionTyping: Γ ⊢ P : S (binary session typing)
pub fn binary_session_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(session_type_ty(), prop())),
    )
}
/// MultipartySessionType: a global type for multiparty session protocols
pub fn multiparty_session_type_ty() -> Expr {
    type0()
}
/// GlobalToLocal: project a global type to a local session type for a role
pub fn global_to_local_ty() -> Expr {
    arrow(
        multiparty_session_type_ty(),
        arrow(nat_ty(), session_type_ty()),
    )
}
/// DeadlockFreedom: well-typed session processes do not deadlock
pub fn deadlock_freedom_ty() -> Expr {
    prop()
}
/// SessionTypeCompleteness: every interaction satisfying the global type is typeable
pub fn session_type_completeness_ty() -> Expr {
    prop()
}
/// RecursiveType: a type μX.F\[X\] — least fixed point of type functor F
pub fn recursive_type_ty() -> Expr {
    arrow(arrow(simple_type_ty(), simple_type_ty()), simple_type_ty())
}
/// IsoRecursiveFold: fold : F\[μX.F\] → μX.F
pub fn iso_recursive_fold_ty() -> Expr {
    arrow(
        arrow(simple_type_ty(), simple_type_ty()),
        arrow(simple_type_ty(), simple_type_ty()),
    )
}
/// IsoRecursiveUnfold: unfold : μX.F → F\[μX.F\]
pub fn iso_recursive_unfold_ty() -> Expr {
    arrow(
        arrow(simple_type_ty(), simple_type_ty()),
        arrow(simple_type_ty(), simple_type_ty()),
    )
}
/// EquiRecursiveType: equi-recursive interpretation (fold/unfold invisible)
pub fn equi_recursive_type_ty() -> Expr {
    arrow(arrow(simple_type_ty(), simple_type_ty()), simple_type_ty())
}
/// TypeUnrolling: μX.F ≡ F\[μX.F\] (equi-recursive unrolling)
pub fn type_unrolling_ty() -> Expr {
    arrow(arrow(simple_type_ty(), simple_type_ty()), prop())
}
/// RecursiveTypeContraction: well-founded recursive types have unique fixed points
pub fn recursive_type_contraction_ty() -> Expr {
    prop()
}
/// KindPolymorphism: ∀κ. F\[κ\] — quantification over kinds
pub fn kind_polymorphism_ty() -> Expr {
    arrow(arrow(kind_ty(), kind_ty()), kind_ty())
}
/// HigherKindedType: a type constructor of kind κ ⇒ κ'
pub fn higher_kinded_type_ty() -> Expr {
    arrow(kind_ty(), arrow(kind_ty(), type0()))
}
/// KindInference: algorithm to infer kinds for type expressions
pub fn kind_inference_ty() -> Expr {
    arrow(system_fomega_type_ty(), option_ty(kind_ty()))
}
/// KindSoundness: well-kinded type applications have well-kinded results
pub fn kind_soundness_ty() -> Expr {
    prop()
}
/// KindCompleteness: kind inference is complete for Fω types
pub fn kind_completeness_ty() -> Expr {
    prop()
}
/// RowType: a row of labeled types (for records or variants)
pub fn row_type_ty() -> Expr {
    type0()
}
/// ExtendRow: add a field l : T to a row R
pub fn extend_row_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(simple_type_ty(), arrow(row_type_ty(), row_type_ty())),
    )
}
/// EmptyRow: the empty row
pub fn empty_row_ty() -> Expr {
    row_type_ty()
}
/// RecordType: a closed record type from a row
pub fn record_type_ty() -> Expr {
    arrow(row_type_ty(), simple_type_ty())
}
/// VariantType: a closed variant type from a row
pub fn variant_type_ty() -> Expr {
    arrow(row_type_ty(), simple_type_ty())
}
/// RowPolymorphicTyping: Γ ⊢ t : {R} with row variable R
pub fn row_polymorphic_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(untyped_term_ty(), arrow(row_type_ty(), prop())),
    )
}
/// StructuralSubtyping: subtyping determined by structure, not names
pub fn structural_subtyping_ty() -> Expr {
    arrow(row_type_ty(), arrow(row_type_ty(), prop()))
}
/// SetoidCarrier: the carrier set of a setoid
pub fn setoid_carrier_ty() -> Expr {
    type0()
}
/// SetoidRelation: the equivalence relation of a setoid
pub fn setoid_relation_ty() -> Expr {
    arrow(setoid_carrier_ty(), arrow(setoid_carrier_ty(), prop()))
}
/// Setoid: a pair of a carrier and an equivalence relation
pub fn setoid_ty() -> Expr {
    type0()
}
/// QuotientType: the quotient A / ~ of a setoid
pub fn quotient_type_ty() -> Expr {
    arrow(setoid_ty(), type0())
}
/// QuotientIntro: \[a\] : A / ~ (introduction rule)
pub fn quotient_intro_ty() -> Expr {
    arrow(setoid_ty(), arrow(setoid_carrier_ty(), quotient_type_ty()))
}
/// QuotientElim: elimination from A / ~ to a P that respects ~
pub fn quotient_elim_ty() -> Expr {
    arrow(
        setoid_ty(),
        arrow(
            arrow(setoid_carrier_ty(), type0()),
            arrow(quotient_type_ty(), type0()),
        ),
    )
}
/// QuotientComputation: \[a\] elim f = f a
pub fn quotient_computation_ty() -> Expr {
    arrow(setoid_ty(), prop())
}
/// ProofIrrelevance: any two proofs of the same proposition are equal
pub fn proof_irrelevance_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        prop(),
        pi(
            BinderInfo::Default,
            "p",
            bvar(0),
            pi(BinderInfo::Default, "q", bvar(1), prop()),
        ),
    )
}
/// IrrelevantArgument: a term whose type-theoretic argument is proof-irrelevant
pub fn irrelevant_argument_ty() -> Expr {
    arrow(prop(), prop())
}
/// SquashType: propositional truncation ||A|| (collapse all proofs)
pub fn squash_type_ty() -> Expr {
    arrow(type0(), prop())
}
/// SquashIntro: a : A → ||A||
pub fn squash_intro_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// SquashElim: ||A|| → (A → P) → P for P : Prop
pub fn squash_elim_ty() -> Expr {
    arrow(prop(), arrow(arrow(type0(), prop()), prop()))
}
/// UniverseLevel: the type of universe levels (ω-many levels)
pub fn universe_level_ty() -> Expr {
    nat_ty()
}
/// RussellUniverse: Uᵢ : Uᵢ₊₁ in the Russell style (types live in universes)
pub fn russell_universe_ty() -> Expr {
    arrow(universe_level_ty(), type1())
}
/// TarskiUniverse: a Tarski-style universe Uᵢ with a decoding map elᵢ
pub fn tarski_universe_ty() -> Expr {
    type0()
}
/// TarskiDecode: el : Uᵢ → Type (decode a code to a type)
pub fn tarski_decode_ty() -> Expr {
    arrow(tarski_universe_ty(), type0())
}
/// CumulativeHierarchy: Uᵢ ⊆ Uᵢ₊₁ (lifting from one universe to the next)
pub fn cumulative_hierarchy_ty() -> Expr {
    arrow(
        universe_level_ty(),
        arrow(tarski_universe_ty(), tarski_universe_ty()),
    )
}
/// UniversePolymorphism: quantification over universe levels
pub fn universe_polymorphism_ty() -> Expr {
    arrow(arrow(universe_level_ty(), type1()), type1())
}
/// ResizingAxiom: propositional resizing (all propositions are small)
pub fn resizing_axiom_ty() -> Expr {
    prop()
}
/// DelimitedContinuation: type of delimited continuation operators (shift/reset)
pub fn delimited_continuation_ty() -> Expr {
    arrow(simple_type_ty(), arrow(simple_type_ty(), simple_type_ty()))
}
/// MonadicType: a monadic type T M (T wrapped in monad M)
pub fn monadic_type_ty() -> Expr {
    arrow(
        arrow(simple_type_ty(), simple_type_ty()),
        arrow(simple_type_ty(), simple_type_ty()),
    )
}
/// MonadicTyping: Γ ⊢ t : M\[A\] for monad M
pub fn monadic_typing_ty() -> Expr {
    arrow(
        typing_context_ty(),
        arrow(
            untyped_term_ty(),
            arrow(
                arrow(simple_type_ty(), simple_type_ty()),
                arrow(simple_type_ty(), prop()),
            ),
        ),
    )
}
/// AlgebraicEffectType: a type for algebraic effects with operations Op
pub fn algebraic_effect_type_ty() -> Expr {
    type0()
}
/// HandlerType: a handler mapping effects to values of type A
pub fn handler_type_ty() -> Expr {
    arrow(
        algebraic_effect_type_ty(),
        arrow(simple_type_ty(), simple_type_ty()),
    )
}
/// DependentSessionType: a session type indexed by values (dependent sessions)
pub fn dependent_session_type_ty() -> Expr {
    type0()
}
/// RefinementType: {x : T | P x} — a type refined by predicate P
pub fn refinement_type_ty() -> Expr {
    arrow(
        simple_type_ty(),
        arrow(arrow(simple_type_ty(), prop()), simple_type_ty()),
    )
}
/// LiquidType: refinement type with decidable predicates (Liquid Haskell style)
pub fn liquid_type_ty() -> Expr {
    arrow(
        simple_type_ty(),
        arrow(arrow(simple_type_ty(), bool_ty()), simple_type_ty()),
    )
}
/// NominalType: a type with freshness/name-binding (nominal logic)
pub fn nominal_type_ty() -> Expr {
    type0()
}
/// FreshnessPredicate: `a # t` — name a is fresh in term t
pub fn freshness_predicate_ty() -> Expr {
    arrow(nat_ty(), arrow(untyped_term_ty(), prop()))
}
/// AbstractionNominal: `⟨a⟩t` — nominal abstraction over fresh name a
pub fn abstraction_nominal_ty() -> Expr {
    arrow(nat_ty(), arrow(untyped_term_ty(), nominal_type_ty()))
}
/// Populate an `Environment` with lambda-calculus axioms and theorem stubs.
pub fn build_lambda_calculus_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("UntypedTerm", untyped_term_ty()),
        ("LcVariable", variable_ty()),
        ("LcAbstraction", abstraction_ty()),
        ("LcApplication", application_ty()),
        ("LcSubstitution", substitution_ty()),
        ("BetaRedex", beta_redex_ty()),
        ("EtaRedex", eta_redex_ty()),
        ("BetaStep", beta_step_ty()),
        ("EtaStep", eta_step_ty()),
        ("BetaReduction", beta_reduction_ty()),
        ("BetaEquiv", beta_equiv_ty()),
        ("NormalForm", normal_form_ty()),
        ("WeakNormalForm", weak_normal_form_ty()),
        ("HeadNormalForm", head_normal_form_ty()),
        ("ChurchNumeral", church_numeral_ty()),
        ("ChurchSucc", church_succ_ty()),
        ("ChurchPlus", church_plus_ty()),
        ("ChurchMul", church_mul_ty()),
        ("ChurchExp", church_exp_ty()),
        ("ChurchPred", church_pred_ty()),
        ("ChurchIsZero", church_is_zero_ty()),
        ("ChurchNumeralCorrect", church_numeral_correct_ty()),
        ("ChurchArithCorrect", church_arith_correct_ty()),
        ("YCombinator", y_combinator_ty()),
        ("TuringCombinator", turing_combinator_ty()),
        ("RecursionTheorem", recursion_theorem_ty()),
        ("YFixedPoint", y_fixed_point_ty()),
        ("OmegaCombinator", omega_combinator_ty()),
        ("OmegaDiverges", omega_diverges_ty()),
        ("NormalOrderRedex", normal_order_redex_ty()),
        ("ApplicativeOrderRedex", applicative_order_redex_ty()),
        ("HeadReduction", head_reduction_ty()),
        ("NormalOrderStrategy", normal_order_strategy_ty()),
        ("StandardizationTheorem", standardization_theorem_ty()),
        ("CallByValueReduction", cbv_reduction_ty()),
        ("CallByNeedReduction", cbn_reduction_ty()),
        ("DiamondProperty", diamond_property_ty()),
        ("ChurchRosserTheorem", church_rosser_theorem_ty()),
        ("Confluence", confluence_ty()),
        ("ParallelReduction", parallel_reduction_ty()),
        (
            "ParallelReductionComplete",
            parallel_reduction_complete_ty(),
        ),
        ("ParallelMaxReduct", parallel_max_reduct_ty()),
        (
            "ParallelMaxReductProperty",
            parallel_max_reduct_property_ty(),
        ),
        ("BohmTree", bohm_tree_ty()),
        ("BohmTreeOf", bohm_tree_of_ty()),
        ("UnsolvableTerm", unsolvable_term_ty()),
        ("SolvableTerm", solvable_term_ty()),
        ("BohmEquiv", bohm_equiv_ty()),
        ("ObservationalEquiv", observational_equiv_ty()),
        ("BohmTheorem", bohm_theorem_ty()),
        ("SimpleType", simple_type_ty()),
        ("BaseType", base_type_ty()),
        ("ArrowType", arrow_type_ty()),
        ("TypingContext", typing_context_ty()),
        ("STLCTyping", stlc_typing_ty()),
        ("STLCSubjectReduction", stlc_subject_reduction_ty()),
        ("STLCStrongNormalization", stlc_strong_normalization_ty()),
        ("STLCChurchRosser", stlc_church_rosser_ty()),
        ("STLCDecidability", stlc_decidability_ty()),
        ("SystemFType", system_f_type_ty()),
        ("SystemFTerm", system_f_term_ty()),
        ("SystemFTyping", system_f_typing_ty()),
        ("SystemFStrongNormalization", system_f_sn_ty()),
        ("SystemFConfluence", system_f_confluence_ty()),
        ("SystemFParametricity", system_f_parametricity_ty()),
        ("SystemFUndecidableTyping", system_f_undecidable_ty()),
        ("ChurchNatF", church_nat_f_ty()),
        ("ChurchBoolF", church_bool_f_ty()),
        ("ChurchListF", church_list_f_ty()),
        ("Kind", kind_ty()),
        ("StarKind", star_kind_ty()),
        ("KindArrow", kind_arrow_ty()),
        ("TypeConstructor", type_constructor_ty()),
        ("SystemFOmegaType", system_fomega_type_ty()),
        ("SystemFOmegaTerm", system_fomega_term_ty()),
        ("SystemFOmegaTyping", system_fomega_typing_ty()),
        ("SystemFOmegaSN", system_fomega_sn_ty()),
        (
            "SystemFOmegaKindSoundness",
            system_fomega_kind_soundness_ty(),
        ),
        ("PTSAxiom", pts_axiom_ty()),
        ("PTSRule", pts_rule_ty()),
        ("PureTypeSystem", pure_type_system_ty()),
        ("PTSTyping", pts_typing_ty()),
        ("CoC", coc_ty()),
        ("LambdaArrow", lambda_arrow_ty()),
        ("Lambda2", lambda2_ty()),
        ("LambdaOmega", lambda_omega_ty()),
        ("LambdaP", lambda_p_ty()),
        ("PTSSubjectReduction", pts_subject_reduction_ty()),
        ("PTSConfluence", pts_confluence_ty()),
        ("CoCStrongNormalization", coc_sn_ty()),
        ("PCFType", pcf_type_ty()),
        ("PCFTerm", pcf_term_ty()),
        ("PCFFixpoint", pcf_fixpoint_ty()),
        ("PCFTyping", pcf_typing_ty()),
        ("PCFDenotationalSemantics", pcf_denotational_semantics_ty()),
        ("PCFAdequacy", pcf_adequacy_ty()),
        ("PCFFullAbstraction", pcf_full_abstraction_ty()),
        ("SubtypeRelation", subtype_relation_ty()),
        ("SubtypingReflexivity", subtyping_reflexivity_ty()),
        ("SubtypingTransitivity", subtyping_transitivity_ty()),
        ("BoundedQuantification", bounded_quantification_ty()),
        ("FBoundedPolymorphism", f_bounded_polymorphism_ty()),
        ("CoercionFunction", coercion_function_ty()),
        (
            "SubtypingSubjectReduction",
            subtyping_subject_reduction_ty(),
        ),
        ("EffectLabel", effect_label_ty()),
        ("EffectSet", effect_set_ty()),
        ("EffectType", effect_type_ty()),
        ("EffectTyping", effect_typing_ty()),
        ("RegionType", region_type_ty()),
        ("RegionInference", region_inference_ty()),
        ("GradedType", graded_type_ty()),
        ("CoeffectSystem", coeffect_system_ty()),
        ("CoeffectTyping", coeffect_typing_ty()),
        ("LinearType", linear_type_ty()),
        ("LinearContext", linear_context_ty()),
        ("LinearTyping", linear_typing_ty()),
        ("LinearArrow", linear_arrow_ty()),
        ("LinearExchangeability", linear_exchangeability_ty()),
        ("AffineType", affine_type_ty()),
        ("AffineTyping", affine_typing_ty()),
        ("RelevantType", relevant_type_ty()),
        ("RelevantTyping", relevant_typing_ty()),
        ("BangModality", bang_modality_ty()),
        ("LFSignature", lf_signature_ty()),
        ("LFContext", lf_context_ty()),
        ("LFTyping", lf_typing_ty()),
        ("LFKindValidity", lf_kind_validity_ty()),
        ("UTTUniverse", utt_universe_ty()),
        ("UTTELType", utt_el_type_ty()),
        ("CICInductiveType", cic_inductive_type_ty()),
        ("CICElimination", cic_elimination_ty()),
        ("CICPositivityCondition", cic_positivity_condition_ty()),
        ("IntersectionType", intersection_type_ty()),
        ("IntersectionTyping", intersection_typing_ty()),
        ("FilterModel", filter_model_ty()),
        (
            "IntersectionTypeCompleteness",
            intersection_type_completeness_ty(),
        ),
        ("PrincipalTyping", principal_typing_ty()),
        ("UnionType", union_type_ty()),
        ("OccurrenceTyping", occurrence_typing_ty()),
        ("FlowAnalysis", flow_analysis_ty()),
        ("GradualType", gradual_type_ty()),
        ("DynType", dyn_type_ty()),
        ("TypeConsistency", type_consistency_ty()),
        ("GradualTyping", gradual_typing_ty()),
        ("BlameLabel", blame_label_ty()),
        ("BlameCalculusTerm", blame_calculus_term_ty()),
        ("BlameTheorem", blame_theorem_ty()),
        ("AbstractingGradualTyping", abstracting_gradual_typing_ty()),
        ("SessionType", session_type_ty()),
        ("SendType", send_type_ty()),
        ("RecvType", recv_type_ty()),
        ("EndType", end_type_ty()),
        ("DualSession", dual_session_ty()),
        ("BinarySessionTyping", binary_session_typing_ty()),
        ("MultipartySessionType", multiparty_session_type_ty()),
        ("GlobalToLocal", global_to_local_ty()),
        ("DeadlockFreedom", deadlock_freedom_ty()),
        ("SessionTypeCompleteness", session_type_completeness_ty()),
        ("RecursiveType", recursive_type_ty()),
        ("IsoRecursiveFold", iso_recursive_fold_ty()),
        ("IsoRecursiveUnfold", iso_recursive_unfold_ty()),
        ("EquiRecursiveType", equi_recursive_type_ty()),
        ("TypeUnrolling", type_unrolling_ty()),
        ("RecursiveTypeContraction", recursive_type_contraction_ty()),
        ("KindPolymorphism", kind_polymorphism_ty()),
        ("HigherKindedType", higher_kinded_type_ty()),
        ("KindInference", kind_inference_ty()),
        ("KindSoundness", kind_soundness_ty()),
        ("KindCompleteness", kind_completeness_ty()),
        ("RowType", row_type_ty()),
        ("ExtendRow", extend_row_ty()),
        ("EmptyRow", empty_row_ty()),
        ("RecordType", record_type_ty()),
        ("VariantType", variant_type_ty()),
        ("RowPolymorphicTyping", row_polymorphic_typing_ty()),
        ("StructuralSubtyping", structural_subtyping_ty()),
        ("SetoidCarrier", setoid_carrier_ty()),
        ("SetoidRelation", setoid_relation_ty()),
        ("Setoid", setoid_ty()),
        ("QuotientType", quotient_type_ty()),
        ("QuotientIntro", quotient_intro_ty()),
        ("QuotientElim", quotient_elim_ty()),
        ("QuotientComputation", quotient_computation_ty()),
        ("ProofIrrelevance", proof_irrelevance_ty()),
        ("IrrelevantArgument", irrelevant_argument_ty()),
        ("SquashType", squash_type_ty()),
        ("SquashIntro", squash_intro_ty()),
        ("SquashElim", squash_elim_ty()),
        ("UniverseLevel", universe_level_ty()),
        ("RussellUniverse", russell_universe_ty()),
        ("TarskiUniverse", tarski_universe_ty()),
        ("TarskiDecode", tarski_decode_ty()),
        ("CumulativeHierarchy", cumulative_hierarchy_ty()),
        ("UniversePolymorphism", universe_polymorphism_ty()),
        ("ResizingAxiom", resizing_axiom_ty()),
        ("DelimitedContinuation", delimited_continuation_ty()),
        ("MonadicType", monadic_type_ty()),
        ("MonadicTyping", monadic_typing_ty()),
        ("AlgebraicEffectType", algebraic_effect_type_ty()),
        ("HandlerType", handler_type_ty()),
        ("DependentSessionType", dependent_session_type_ty()),
        ("RefinementType", refinement_type_ty()),
        ("LiquidType", liquid_type_ty()),
        ("NominalType", nominal_type_ty()),
        ("FreshnessPredicate", freshness_predicate_ty()),
        ("AbstractionNominal", abstraction_nominal_ty()),
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
/// Build the Church numeral for `n`.
/// c_n = λf. λx. f^n x
pub fn church(n: usize) -> Term {
    let x = Term::Var(0);
    let f = Term::Var(1);
    let mut body = x;
    for _ in 0..n {
        body = Term::App(Box::new(f.clone()), Box::new(body));
    }
    Term::Lam(Box::new(Term::Lam(Box::new(body))))
}
/// Church successor: S = λn. λf. λx. f (n f x)
pub fn church_succ() -> Term {
    let n = Term::Var(2);
    let f = Term::Var(1);
    let x = Term::Var(0);
    let nfx = Term::App(
        Box::new(Term::App(Box::new(n), Box::new(f.clone()))),
        Box::new(x),
    );
    let body = Term::App(Box::new(f), Box::new(nfx));
    Term::Lam(Box::new(Term::Lam(Box::new(Term::Lam(Box::new(body))))))
}
/// Church addition: PLUS = λm. λn. λf. λx. m f (n f x)
pub fn church_plus() -> Term {
    let m = Term::Var(3);
    let n = Term::Var(2);
    let f = Term::Var(1);
    let x = Term::Var(0);
    let nfx = Term::App(
        Box::new(Term::App(Box::new(n), Box::new(f.clone()))),
        Box::new(x),
    );
    let body = Term::App(Box::new(Term::App(Box::new(m), Box::new(f))), Box::new(nfx));
    Term::Lam(Box::new(Term::Lam(Box::new(Term::Lam(Box::new(
        Term::Lam(Box::new(body)),
    ))))))
}
/// Church multiplication: MUL = λm. λn. λf. m (n f)
pub fn church_mul() -> Term {
    let m = Term::Var(2);
    let n = Term::Var(1);
    let f = Term::Var(0);
    let nf = Term::App(Box::new(n), Box::new(f));
    let body = Term::App(Box::new(m), Box::new(nf));
    Term::Lam(Box::new(Term::Lam(Box::new(Term::Lam(Box::new(body))))))
}
/// Apply PLUS to two Church numerals and reduce to normal form.
pub fn church_add(m: usize, n: usize) -> Term {
    let plus = church_plus();
    let cm = church(m);
    let cn = church(n);
    let applied = Term::App(
        Box::new(Term::App(Box::new(plus), Box::new(cm))),
        Box::new(cn),
    );
    let (result, _) = applied.normalize(10000);
    result
}
/// Type inference for the simply-typed lambda calculus (Curry style).
/// Returns `Some(ty)` if the term is typeable, `None` otherwise.
pub fn infer_type(ctx: &Context, term: &Term) -> Option<SimpleType> {
    match term {
        Term::Var(k) => ctx.get(*k).cloned(),
        Term::Lam(body) => {
            let _ = body;
            None
        }
        Term::App(f, a) => match infer_type(ctx, f)? {
            SimpleType::Arrow(dom, cod) => {
                if check_type(ctx, a, &dom) {
                    Some(*cod)
                } else {
                    None
                }
            }
            _ => None,
        },
    }
}
/// Type checking for the simply-typed lambda calculus.
/// Returns `true` iff `ctx ⊢ term : ty`.
pub fn check_type(ctx: &Context, term: &Term, ty: &SimpleType) -> bool {
    match (term, ty) {
        (Term::Lam(body), SimpleType::Arrow(dom, cod)) => {
            let ctx2 = ctx.extend(*dom.clone());
            check_type(&ctx2, body, cod)
        }
        (Term::App(f, a), _) => {
            if let Some(ft) = infer_type(ctx, f) {
                match ft {
                    SimpleType::Arrow(dom, cod) => *cod == *ty && check_type(ctx, a, &dom),
                    _ => false,
                }
            } else {
                false
            }
        }
        (Term::Var(k), _) => ctx.get(*k).map(|t| t == ty).unwrap_or(false),
        _ => false,
    }
}
/// Perform one-step β-reduction under the given strategy.
pub fn beta_step(term: &Term, strategy: Strategy) -> Option<Term> {
    match strategy {
        Strategy::NormalOrder => term.beta_step_normal(),
        Strategy::ApplicativeOrder => beta_step_applicative(term),
        Strategy::HeadReduction => beta_step_head(term),
    }
}
pub fn beta_step_applicative(term: &Term) -> Option<Term> {
    match term {
        Term::App(f, a) => {
            if let Some(a2) = beta_step_applicative(a) {
                return Some(Term::App(f.clone(), Box::new(a2)));
            }
            if let Some(f2) = beta_step_applicative(f) {
                return Some(Term::App(Box::new(f2), a.clone()));
            }
            if let Term::Lam(body) = f.as_ref() {
                return Some(body.subst(0, a));
            }
            None
        }
        Term::Lam(body) => beta_step_applicative(body).map(|b2| Term::Lam(Box::new(b2))),
        Term::Var(_) => None,
    }
}
pub fn beta_step_head(term: &Term) -> Option<Term> {
    match term {
        Term::App(f, a) => {
            if let Term::Lam(body) = f.as_ref() {
                return Some(body.subst(0, a));
            }
            beta_step_head(f).map(|f2| Term::App(Box::new(f2), a.clone()))
        }
        Term::Lam(body) => beta_step_head(body).map(|b2| Term::Lam(Box::new(b2))),
        Term::Var(_) => None,
    }
}
/// Result of linearity checking: a map from de Bruijn level → usage count.
#[allow(dead_code)]
pub type UsageMap = Vec<usize>;
#[cfg(test)]
mod tests {
    use super::*;
    /// Verify that the environment builds and contains key axioms.
    #[test]
    fn test_build_lambda_calculus_env() {
        let env = build_lambda_calculus_env();
        assert!(env.get(&Name::str("UntypedTerm")).is_some());
        assert!(env.get(&Name::str("YCombinator")).is_some());
        assert!(env.get(&Name::str("ChurchRosserTheorem")).is_some());
        assert!(env.get(&Name::str("SystemFStrongNormalization")).is_some());
        assert!(env.get(&Name::str("CoC")).is_some());
        assert!(env.get(&Name::str("BohmTheorem")).is_some());
    }
    /// Test Church numeral construction and normalization.
    #[test]
    fn test_church_numerals() {
        let c0 = church(0);
        assert_eq!(c0, Term::Lam(Box::new(Term::Lam(Box::new(Term::Var(0))))));
        let c1 = church(1);
        assert_eq!(
            c1,
            Term::Lam(Box::new(Term::Lam(Box::new(Term::App(
                Box::new(Term::Var(1)),
                Box::new(Term::Var(0))
            )))))
        );
        let c2 = church(2);
        assert!(c2.is_normal());
    }
    /// Test Church addition: 2 + 3 = 5.
    #[test]
    fn test_church_addition() {
        let result = church_add(2, 3);
        let expected = church(5);
        let (rn, _) = result.normalize(10000);
        let (en, _) = expected.normalize(10000);
        assert_eq!(rn, en);
    }
    /// Test beta-reduction: (λx.x) t →β t.
    #[test]
    fn test_identity_reduction() {
        let id = Term::Lam(Box::new(Term::Var(0)));
        let arg = Term::Var(0);
        let redex = Term::App(Box::new(id), Box::new(arg.clone()));
        let (result, steps) = redex.normalize(100);
        assert!(steps > 0);
        assert!(result.is_normal());
    }
    /// Test is_normal: variable and abstraction of variable are normal.
    #[test]
    fn test_is_normal() {
        assert!(Term::Var(0).is_normal());
        assert!(Term::Lam(Box::new(Term::Var(0))).is_normal());
        let redex = Term::App(
            Box::new(Term::Lam(Box::new(Term::Var(0)))),
            Box::new(Term::Var(1)),
        );
        assert!(!redex.is_normal());
    }
    /// Test type checking: identity function I : (α → α).
    #[test]
    fn test_stlc_identity() {
        let alpha = SimpleType::Base("α".into());
        let id = Term::Lam(Box::new(Term::Var(0)));
        let arrow_ty = SimpleType::arr(alpha.clone(), alpha.clone());
        let ctx = Context::empty();
        assert!(check_type(&ctx, &id, &arrow_ty));
    }
    /// Test type checking: K combinator K : α → β → α.
    #[test]
    fn test_stlc_k_combinator() {
        let alpha = SimpleType::Base("α".into());
        let beta = SimpleType::Base("β".into());
        let k = Term::Lam(Box::new(Term::Lam(Box::new(Term::Var(1)))));
        let ty = SimpleType::arr(alpha.clone(), SimpleType::arr(beta.clone(), alpha.clone()));
        let ctx = Context::empty();
        assert!(check_type(&ctx, &k, &ty));
    }
    /// Test reduction strategies: normal order vs. applicative order.
    #[test]
    fn test_reduction_strategies() {
        let id = || Term::Lam(Box::new(Term::Var(0)));
        let t = Term::App(
            Box::new(id()),
            Box::new(Term::App(Box::new(id()), Box::new(Term::Var(1)))),
        );
        assert!(!t.is_normal());
        let step_no = beta_step(&t, Strategy::NormalOrder);
        assert!(step_no.is_some());
        let step_ao = beta_step(&t, Strategy::ApplicativeOrder);
        assert!(step_ao.is_some());
    }
    /// Test that church(n) has correct size (2n+3 nodes).
    #[test]
    fn test_church_size() {
        assert_eq!(church(0).size(), 3);
        assert_eq!(church(1).size(), 5);
        let c3 = church(3);
        assert_eq!(c3.size(), 2 * 3 + 3);
    }
    /// Verify new axioms are registered in the environment.
    #[test]
    fn test_new_axioms_registered() {
        let env = build_lambda_calculus_env();
        assert!(env.get(&Name::str("PCFFixpoint")).is_some());
        assert!(env.get(&Name::str("PCFAdequacy")).is_some());
        assert!(env.get(&Name::str("BoundedQuantification")).is_some());
        assert!(env.get(&Name::str("FBoundedPolymorphism")).is_some());
        assert!(env.get(&Name::str("CoeffectSystem")).is_some());
        assert!(env.get(&Name::str("RegionInference")).is_some());
        assert!(env.get(&Name::str("LinearTyping")).is_some());
        assert!(env.get(&Name::str("BangModality")).is_some());
        assert!(env.get(&Name::str("AffineTyping")).is_some());
        assert!(env.get(&Name::str("LFTyping")).is_some());
        assert!(env.get(&Name::str("CICInductiveType")).is_some());
        assert!(env.get(&Name::str("FilterModel")).is_some());
        assert!(env.get(&Name::str("PrincipalTyping")).is_some());
        assert!(env.get(&Name::str("FlowAnalysis")).is_some());
        assert!(env.get(&Name::str("TypeConsistency")).is_some());
        assert!(env.get(&Name::str("BlameTheorem")).is_some());
        assert!(env.get(&Name::str("DualSession")).is_some());
        assert!(env.get(&Name::str("DeadlockFreedom")).is_some());
        assert!(env.get(&Name::str("MultipartySessionType")).is_some());
        assert!(env.get(&Name::str("IsoRecursiveFold")).is_some());
        assert!(env.get(&Name::str("TypeUnrolling")).is_some());
        assert!(env.get(&Name::str("KindPolymorphism")).is_some());
        assert!(env.get(&Name::str("HigherKindedType")).is_some());
        assert!(env.get(&Name::str("RecordType")).is_some());
        assert!(env.get(&Name::str("StructuralSubtyping")).is_some());
        assert!(env.get(&Name::str("QuotientType")).is_some());
        assert!(env.get(&Name::str("QuotientElim")).is_some());
        assert!(env.get(&Name::str("ProofIrrelevance")).is_some());
        assert!(env.get(&Name::str("SquashType")).is_some());
        assert!(env.get(&Name::str("RussellUniverse")).is_some());
        assert!(env.get(&Name::str("CumulativeHierarchy")).is_some());
        assert!(env.get(&Name::str("RefinementType")).is_some());
        assert!(env.get(&Name::str("NominalType")).is_some());
        assert!(env.get(&Name::str("AlgebraicEffectType")).is_some());
    }
    #[test]
    fn test_beta_reducer_converges() {
        let reducer = BetaReducer::new(Strategy::NormalOrder, 1000);
        let id = Term::Lam(Box::new(Term::Var(0)));
        let t = Term::App(Box::new(id), Box::new(Term::Var(0)));
        let (result, steps, converged) = reducer.reduce(&t);
        assert!(converged);
        assert!(steps >= 1);
        assert!(reducer.is_normal_form(&result));
    }
    #[test]
    fn test_beta_reducer_step_count() {
        let reducer = BetaReducer::new(Strategy::NormalOrder, 10000);
        let one_plus_one = church_add(1, 1);
        let steps_opt = reducer.count_steps(&one_plus_one);
        assert!(steps_opt.is_some());
    }
    #[test]
    fn test_alpha_equiv_identical() {
        let checker = AlphaEquivalenceChecker::new();
        let t = Term::Lam(Box::new(Term::Var(0)));
        assert!(checker.alpha_equiv(&t, &t));
    }
    #[test]
    fn test_alpha_equiv_different() {
        let checker = AlphaEquivalenceChecker::new();
        let t1 = Term::Lam(Box::new(Term::Var(0)));
        let t2 = Term::Lam(Box::new(Term::Var(1)));
        assert!(!checker.alpha_equiv(&t1, &t2));
    }
    #[test]
    fn test_alpha_equiv_normalized() {
        let checker = AlphaEquivalenceChecker::new();
        let id = Term::Lam(Box::new(Term::Var(0)));
        let t = Term::App(Box::new(id), Box::new(Term::Var(1)));
        let expected = Term::Var(1);
        assert!(checker.alpha_equiv_normalized(&t, &expected, 100));
    }
    #[test]
    fn test_type_inference_var() {
        let system = TypeInferenceSystem::new();
        let alpha = SimpleType::Base("α".into());
        let ctx = Context(vec![alpha.clone()]);
        assert_eq!(system.synthesize(&ctx, &Term::Var(0)), Some(alpha));
    }
    #[test]
    fn test_type_inference_app() {
        let system = TypeInferenceSystem::new();
        let alpha = SimpleType::Base("α".into());
        let beta = SimpleType::Base("β".into());
        let ctx = Context(vec![
            alpha.clone(),
            SimpleType::arr(alpha.clone(), beta.clone()),
        ]);
        let term = Term::App(Box::new(Term::Var(1)), Box::new(Term::Var(0)));
        assert_eq!(system.synthesize(&ctx, &term), Some(beta));
    }
    #[test]
    fn test_type_check_identity() {
        let system = TypeInferenceSystem::new();
        let alpha = SimpleType::Base("α".into());
        let id = Term::Lam(Box::new(Term::Var(0)));
        let ctx = Context::empty();
        assert!(system.check(&ctx, &id, &SimpleType::arr(alpha.clone(), alpha)));
    }
    #[test]
    fn test_type_inference_with_hint() {
        let system = TypeInferenceSystem::new();
        let alpha = SimpleType::Base("α".into());
        let id = Term::Lam(Box::new(Term::Var(0)));
        let ty = SimpleType::arr(alpha.clone(), alpha);
        let ctx = Context::empty();
        let result = system.infer_with_annotation(&ctx, &id, Some(&ty));
        assert!(result.is_some());
    }
    #[test]
    fn test_linear_identity_is_linear() {
        let checker = LinearTypeChecker::new();
        let id = Term::Lam(Box::new(Term::Var(0)));
        let uses = checker.count_uses(&id, 0);
        assert!(uses.is_empty());
        let uses_body = checker.count_uses(&Term::Var(0), 1);
        assert_eq!(uses_body, vec![1]);
    }
    #[test]
    fn test_linear_k_combinator_is_not_linear() {
        let checker = LinearTypeChecker::new();
        let body = Term::Var(1);
        let uses = checker.count_uses(&body, 2);
        assert_eq!(uses, vec![0, 1]);
        assert!(!checker.is_linear(&body, 2));
        assert!(checker.is_affine(&body, 2));
    }
    #[test]
    fn test_affine_vs_relevant() {
        let checker = LinearTypeChecker::new();
        let body = Term::App(
            Box::new(Term::App(Box::new(Term::Var(1)), Box::new(Term::Var(0)))),
            Box::new(Term::Var(0)),
        );
        let uses = checker.count_uses(&body, 2);
        assert_eq!(uses, vec![2, 1]);
        assert!(!checker.is_linear(&body, 2));
        assert!(!checker.is_affine(&body, 2));
        assert!(checker.is_relevant(&body, 2));
    }
    #[test]
    fn test_session_dual_send_recv() {
        let compat = SessionTypeCompatibility::new();
        let s = BinarySession::Send("Int".into(), Box::new(BinarySession::End));
        let dual = compat.dual(&s);
        assert_eq!(
            dual,
            BinarySession::Recv("Int".into(), Box::new(BinarySession::End))
        );
    }
    #[test]
    fn test_session_dual_involutive() {
        let compat = SessionTypeCompatibility::new();
        let s = BinarySession::Send(
            "Bool".into(),
            Box::new(BinarySession::Recv(
                "Int".into(),
                Box::new(BinarySession::End),
            )),
        );
        let d = compat.dual(&compat.dual(&s));
        assert_eq!(d, s);
    }
    #[test]
    fn test_session_compatible_send_recv() {
        let compat = SessionTypeCompatibility::new();
        let s1 = BinarySession::Send("Int".into(), Box::new(BinarySession::End));
        let s2 = BinarySession::Recv("Int".into(), Box::new(BinarySession::End));
        assert!(compat.compatible(&s1, &s2));
        assert!(compat.compatible(&s2, &s1));
    }
    #[test]
    fn test_session_incompatible_type_mismatch() {
        let compat = SessionTypeCompatibility::new();
        let s1 = BinarySession::Send("Int".into(), Box::new(BinarySession::End));
        let s2 = BinarySession::Recv("Bool".into(), Box::new(BinarySession::End));
        assert!(!compat.compatible(&s1, &s2));
    }
    #[test]
    fn test_session_compatible_end_end() {
        let compat = SessionTypeCompatibility::new();
        assert!(compat.compatible(&BinarySession::End, &BinarySession::End));
    }
    #[test]
    fn test_session_are_dual() {
        let compat = SessionTypeCompatibility::new();
        let s1 = BinarySession::Send("Nat".into(), Box::new(BinarySession::End));
        let s2 = BinarySession::Recv("Nat".into(), Box::new(BinarySession::End));
        assert!(compat.are_dual(&s1, &s2));
        assert!(compat.are_dual(&s2, &s1));
        assert!(!compat.are_dual(&s1, &s1));
    }
    #[test]
    fn test_session_select_offer_compatible() {
        let compat = SessionTypeCompatibility::new();
        let s1 = BinarySession::Select(vec![
            ("ok".into(), BinarySession::End),
            ("err".into(), BinarySession::End),
        ]);
        let s2 = BinarySession::Offer(vec![
            ("ok".into(), BinarySession::End),
            ("err".into(), BinarySession::End),
        ]);
        assert!(compat.compatible(&s1, &s2));
    }
}
