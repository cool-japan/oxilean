//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AbstractState, AbstractTransformer, AbstractionFunction, ArrayBoundsAnalysis,
    AssumeGuaranteeContract, Bound, ConcretizationFunction, DataflowAnalysis, DelayedWidening,
    FixpointComputation, GaloisConnection, IntervalDomain, IntervalWidening, LoopBound,
    NullPointerAnalysis, OctagonDomain, ParityDomain, PolyhedralDomain,
    ProbabilisticAbstractDomain, ReachabilityAnalysis, SignDomain, TaintAnalysis,
    TemplatePolyhedronDomain, ZonotopeDomain,
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
/// `AbstractDomain : Type → Type`
///
/// An abstract domain for type A: a lattice structure (⊥, ⊤, ⊑, ⊔, ⊓)
/// equipped with widening ▽ and narrowing △ operators.
pub fn abstract_domain_ty() -> Expr {
    impl_pi("A", type0(), type0())
}
/// `AbstractLattice : ∀ (A : Type), AbstractDomain A → Prop`
///
/// Proof that an abstract domain forms a complete lattice.
pub fn abstract_lattice_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("AbstractDomain"), bvar(0)), prop()),
    )
}
/// `IntervalDomain : Type`
///
/// The interval domain: elements are pairs \[l, u\] where l, u ∈ ℤ∪{-∞,+∞}.
/// Provides a sound over-approximation of sets of integers.
pub fn interval_domain_ty() -> Expr {
    type0()
}
/// `OctagonDomain : Nat → Type`
///
/// The octagon domain for n variables: constraints of the form ±xi ± xj ≤ c.
/// A weakly relational domain that is more precise than intervals.
pub fn octagon_domain_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `PolyhedralDomain : Nat → Type`
///
/// The polyhedral domain for n variables: linear constraints Ax ≤ b.
/// Exact but computationally expensive (exponential in general).
pub fn polyhedral_domain_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SignDomain : Type`
///
/// The sign domain: abstract values are Bottom, Neg, Zero, Pos, NonNeg, NonPos, Top.
pub fn sign_domain_ty() -> Expr {
    type0()
}
/// `ParityDomain : Type`
///
/// The parity domain: abstract values are Bottom, Even, Odd, Top.
pub fn parity_domain_ty() -> Expr {
    type0()
}
/// `AbstractJoin : ∀ (A : Type), A → A → A`
///
/// The join (least upper bound ⊔) of two abstract values.
pub fn abstract_join_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), bvar(2))))
}
/// `AbstractMeet : ∀ (A : Type), A → A → A`
///
/// The meet (greatest lower bound ⊓) of two abstract values.
pub fn abstract_meet_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), bvar(2))))
}
/// `AbstractWiden : ∀ (A : Type), A → A → A`
///
/// The widening operator ▽: a ▽ b extrapolates to ensure convergence.
/// Satisfies: a ⊑ a ▽ b and b ⊑ a ▽ b (over-approximation).
pub fn abstract_widen_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), bvar(2))))
}
/// `AbstractNarrow : ∀ (A : Type), A → A → A`
///
/// The narrowing operator △: a △ b refines a using constraints from b.
/// Satisfies: a △ b ⊑ a (monotone refinement).
pub fn abstract_narrow_ty() -> Expr {
    impl_pi("A", type0(), arrow(bvar(0), arrow(bvar(1), bvar(2))))
}
/// `AbstractState : Type`
///
/// An abstract state: a map from variable names to abstract values.
/// Represents a set of concrete program states.
pub fn abstract_state_ty() -> Expr {
    type0()
}
/// `AbstractTransformer : AbstractState → AbstractState`
///
/// The abstract transformer \[stmt\]^#: maps pre-states to post-states.
/// Must be a sound over-approximation of the concrete transformer.
pub fn abstract_transformer_ty() -> Expr {
    arrow(cst("AbstractState"), cst("AbstractState"))
}
/// `TransferFunction : Type`
///
/// The abstract semantics of a statement: assigns/conditions map to
/// abstract transformers. Composes to give semantics of programs.
pub fn transfer_function_ty() -> Expr {
    type0()
}
/// `JoinSemilattice : AbstractState → AbstractState → AbstractState`
///
/// Combines two abstract states from different control-flow paths via ⊔.
/// Used at join points in the CFG.
pub fn join_semilattice_ty() -> Expr {
    arrow(
        cst("AbstractState"),
        arrow(cst("AbstractState"), cst("AbstractState")),
    )
}
/// `FixpointComputation : TransferFunction → AbstractState → AbstractState`
///
/// Computes lfp([\[while P: C\]]^#) via Kleene iteration with widening.
/// Terminates because widening accelerates convergence.
pub fn fixpoint_computation_ty() -> Expr {
    arrow(
        cst("TransferFunction"),
        arrow(cst("AbstractState"), cst("AbstractState")),
    )
}
/// `SoundnessCondition : AbstractTransformer → Prop`
///
/// Soundness: γ(F^#(a)) ⊇ F(γ(a)) for all abstract values a.
pub fn soundness_condition_ty() -> Expr {
    arrow(cst("AbstractTransformer"), prop())
}
/// `GaloisConnection : ∀ (C A : Type), (C → A) → (A → C) → Prop`
///
/// A Galois connection (α, γ): ℘(C) ⇌ A with
/// α(S) ⊑ a ↔ S ⊆ γ(a) (adjunction condition).
pub fn galois_connection_ty() -> Expr {
    impl_pi(
        "C",
        type0(),
        impl_pi(
            "A",
            type0(),
            arrow(
                arrow(bvar(1), bvar(1)),
                arrow(arrow(bvar(2), bvar(2)), prop()),
            ),
        ),
    )
}
/// `AbstractionFunction : ∀ (C A : Type), List C → A`
///
/// α(S) = smallest abstract value containing all elements of S.
/// The abstraction function is the left adjoint.
pub fn abstraction_function_ty() -> Expr {
    impl_pi(
        "C",
        type0(),
        impl_pi("A", type0(), arrow(app(cst("List"), bvar(1)), bvar(1))),
    )
}
/// `ConcretizationFunction : ∀ (C A : Type), A → List C`
///
/// γ(a) = set of all concrete values a represents.
/// The concretization function is the right adjoint.
pub fn concretization_function_ty() -> Expr {
    impl_pi(
        "C",
        type0(),
        impl_pi("A", type0(), arrow(bvar(0), app(cst("List"), bvar(2)))),
    )
}
/// `OptimalAbstraction : ∀ (C A : Type), List C → A`
///
/// α(S) = ⊓{a : S ⊆ γ(a)} = the best (most precise) abstract value for S.
/// Exists when A has all meets and γ preserves them.
pub fn optimal_abstraction_ty() -> Expr {
    impl_pi(
        "C",
        type0(),
        impl_pi("A", type0(), arrow(app(cst("List"), bvar(1)), bvar(1))),
    )
}
/// `GaloisInsertion : ∀ (C A : Type), GaloisConnection C A → Prop`
///
/// A Galois insertion: additionally satisfies α(γ(a)) = a (surjectivity of α).
pub fn galois_insertion_ty() -> Expr {
    impl_pi(
        "C",
        type0(),
        impl_pi(
            "A",
            type0(),
            arrow(app2(cst("GaloisConnection"), bvar(1), bvar(0)), prop()),
        ),
    )
}
/// `DataflowAnalysis : Type`
///
/// Forward or backward dataflow analysis: associates abstract values to
/// program points satisfying the dataflow equations.
pub fn dataflow_analysis_ty() -> Expr {
    type0()
}
/// `ReachabilityAnalysis : Type → Prop`
///
/// Which states are reachable from the initial state?
/// Computes the set of reachable abstract states via forward analysis.
pub fn reachability_analysis_ty() -> Expr {
    arrow(type0(), prop())
}
/// `NullPointerAnalysis : Type`
///
/// May/must null pointer analysis: tracks whether pointers may be null,
/// must be null, or are definitely non-null at each program point.
pub fn null_pointer_analysis_ty() -> Expr {
    type0()
}
/// `ArrayBoundsAnalysis : Type`
///
/// Interval analysis for array indices: proves array accesses are within bounds.
/// Combines the interval domain with loop invariant generation.
pub fn array_bounds_analysis_ty() -> Expr {
    type0()
}
/// `TaintAnalysis : Type`
///
/// Track information flow from sources (user input) to sinks (security-critical ops).
/// A non-relational analysis using a two-element lattice {clean, tainted}.
pub fn taint_analysis_ty() -> Expr {
    type0()
}
/// `LiveVariableAnalysis : Type`
///
/// Backward analysis: a variable is live at a point if its value may be used later.
/// Used for dead code elimination and register allocation.
pub fn live_variable_analysis_ty() -> Expr {
    type0()
}
/// `AvailableExprAnalysis : Type`
///
/// Forward analysis: an expression is available if it has been computed and
/// its operands have not changed since. Used for common subexpression elimination.
pub fn available_expr_analysis_ty() -> Expr {
    type0()
}
/// `IntervalWidening : IntervalDomain → IntervalDomain → IntervalDomain`
///
/// Standard interval widening: if lower bound decreased, jump to -∞;
/// if upper bound increased, jump to +∞. Ensures termination.
pub fn interval_widening_ty() -> Expr {
    arrow(
        cst("IntervalDomain"),
        arrow(cst("IntervalDomain"), cst("IntervalDomain")),
    )
}
/// `ConvexHullWidening : PolyhedralDomain → PolyhedralDomain → PolyhedralDomain`
///
/// Polyhedral widening via convex hull followed by constraint removal.
/// Keeps only constraints that held in both input polyhedra.
pub fn convex_hull_widening_ty() -> Expr {
    arrow(
        app(cst("PolyhedralDomain"), nat_ty()),
        arrow(
            app(cst("PolyhedralDomain"), nat_ty()),
            app(cst("PolyhedralDomain"), nat_ty()),
        ),
    )
}
/// `DelayedWidening : Nat → AbstractState → AbstractState → AbstractState`
///
/// Apply widening only after k fixpoint iterations, then switch to widening.
/// Improves precision for loops with small iteration counts.
pub fn delayed_widening_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            cst("AbstractState"),
            arrow(cst("AbstractState"), cst("AbstractState")),
        ),
    )
}
/// `LoopBound : AbstractState → Nat`
///
/// Estimated upper bound on loop iteration count, derived from the abstract state.
/// Used to decide when to apply widening in DelayedWidening.
pub fn loop_bound_ty() -> Expr {
    arrow(cst("AbstractState"), nat_ty())
}
/// Register all abstract interpretation axioms into the given environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("AbstractDomain", abstract_domain_ty()),
        ("AbstractLattice", abstract_lattice_ty()),
        ("IntervalDomain", interval_domain_ty()),
        ("OctagonDomain", octagon_domain_ty()),
        ("PolyhedralDomain", polyhedral_domain_ty()),
        ("SignDomain", sign_domain_ty()),
        ("ParityDomain", parity_domain_ty()),
        ("AbstractJoin", abstract_join_ty()),
        ("AbstractMeet", abstract_meet_ty()),
        ("AbstractWiden", abstract_widen_ty()),
        ("AbstractNarrow", abstract_narrow_ty()),
        ("AbstractState", abstract_state_ty()),
        ("AbstractTransformer", abstract_transformer_ty()),
        ("TransferFunction", transfer_function_ty()),
        ("JoinSemilattice", join_semilattice_ty()),
        ("FixpointComputation", fixpoint_computation_ty()),
        ("SoundnessCondition", soundness_condition_ty()),
        ("GaloisConnection", galois_connection_ty()),
        ("AbstractionFunction", abstraction_function_ty()),
        ("ConcretizationFunction", concretization_function_ty()),
        ("OptimalAbstraction", optimal_abstraction_ty()),
        ("GaloisInsertion", galois_insertion_ty()),
        ("DataflowAnalysis", dataflow_analysis_ty()),
        ("ReachabilityAnalysis", reachability_analysis_ty()),
        ("NullPointerAnalysis", null_pointer_analysis_ty()),
        ("ArrayBoundsAnalysis", array_bounds_analysis_ty()),
        ("TaintAnalysis", taint_analysis_ty()),
        ("LiveVariableAnalysis", live_variable_analysis_ty()),
        ("AvailableExprAnalysis", available_expr_analysis_ty()),
        ("IntervalWidening", interval_widening_ty()),
        ("ConvexHullWidening", convex_hull_widening_ty()),
        ("DelayedWidening", delayed_widening_ty()),
        ("LoopBound", loop_bound_ty()),
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
/// `EllipsoidDomain : Nat → Type`
///
/// Ellipsoidal abstract domain for n variables: x^T P x ≤ 1 (P positive definite).
/// Used in control verification and Lyapunov-based analysis.
pub fn ellipsoid_domain_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `ZonotopeDomain : Nat → Nat → Type`
///
/// Zonotope domain with n dimensions and m generators: x = c + Σ ε_i g_i, |ε_i| ≤ 1.
/// Closed under linear maps, used in Taylor model arithmetic and reachability.
pub fn zonotope_domain_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `TemplatePolyhedronDomain : Nat → Nat → Type`
///
/// Template polyhedron domain: cx ≤ d for a fixed template matrix C.
/// Balances precision and efficiency by fixing constraint directions.
pub fn template_polyhedron_domain_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), type0()))
}
/// `ReducedProductDomain : ∀ (A B : Type), A → B → Type`
///
/// Reduced product of two abstract domains: pairs (a, b) with reduction
/// operator to improve precision via information exchange.
pub fn reduced_product_domain_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        impl_pi("B", type0(), arrow(bvar(1), arrow(bvar(1), type0()))),
    )
}
/// `SmashingDomain : ∀ (A : Type), AbstractDomain A → Type`
///
/// Smashing (direct) product: collapse all abstract values into a single
/// abstract element. Used when domain precision exceeds analysis needs.
pub fn smashing_domain_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("AbstractDomain"), bvar(0)), type0()),
    )
}
/// `PowersetLiftingDomain : ∀ (A : Type), AbstractDomain A → Type`
///
/// Powerset lifting: abstract values are finite sets of base-domain elements.
/// Exponential cost but can express disjunctive information.
pub fn powerset_lifting_domain_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("AbstractDomain"), bvar(0)), type0()),
    )
}
/// `TraceAbstraction : Type → Type`
///
/// Abstraction of execution traces: maps concrete trace sets to abstract traces.
/// Underpins hyperproperties and relational verification.
pub fn trace_abstraction_ty() -> Expr {
    impl_pi("A", type0(), type0())
}
/// `AbstractTrace : Type`
///
/// An abstract execution trace: sequence of abstract states over time.
/// Used in temporal abstract interpretation and model checking.
pub fn abstract_trace_ty() -> Expr {
    type0()
}
/// `TraceSemanticsMonotone : AbstractTrace → AbstractTrace → Prop`
///
/// Monotonicity of abstract trace transformers: longer traces refine shorter.
pub fn trace_semantics_monotone_ty() -> Expr {
    arrow(cst("AbstractTrace"), arrow(cst("AbstractTrace"), prop()))
}
/// `ThreadModularAnalysis : Nat → AbstractState → AbstractState`
///
/// Thread-modular abstract interpretation: analyze each thread independently
/// with an interference abstraction summarizing effects of other threads.
pub fn thread_modular_analysis_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("AbstractState"), cst("AbstractState")))
}
/// `InterferenceAbstraction : AbstractState → AbstractState → AbstractState`
///
/// Abstract interference from concurrent threads: over-approximate writes
/// by other threads to establish sound sequential reasoning per thread.
pub fn interference_abstraction_ty() -> Expr {
    arrow(
        cst("AbstractState"),
        arrow(cst("AbstractState"), cst("AbstractState")),
    )
}
/// `PartialOrderReductionAbs : AbstractState → AbstractState`
///
/// Partial-order reduction in abstract interpretation: prune redundant
/// interleavings while preserving abstract reachability.
pub fn partial_order_reduction_abs_ty() -> Expr {
    arrow(cst("AbstractState"), cst("AbstractState"))
}
/// `LocksetAnalysis : Type`
///
/// Lockset-based data race detection: tracks which locks protect each
/// memory location across concurrent threads.
pub fn lockset_analysis_ty() -> Expr {
    type0()
}
/// `WideningOperator : ∀ (A : Type), AbstractDomain A → A → A → A`
///
/// Generic widening operator: given a domain, produces the widened element.
/// Must satisfy: a ⊑ a ▽ b and b ⊑ a ▽ b.
pub fn widening_operator_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("AbstractDomain"), bvar(0)),
            arrow(bvar(1), arrow(bvar(2), bvar(3))),
        ),
    )
}
/// `NarrowingOperator : ∀ (A : Type), AbstractDomain A → A → A → A`
///
/// Generic narrowing operator: a △ b ⊑ a (refinement below a).
/// Used after fixpoint to improve precision without losing soundness.
pub fn narrowing_operator_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(
            app(cst("AbstractDomain"), bvar(0)),
            arrow(bvar(1), arrow(bvar(2), bvar(3))),
        ),
    )
}
/// `ExtrapolationLemma : ∀ (A : Type), WideningOperator A → Prop`
///
/// Termination guarantee for Kleene iteration with widening:
/// every ascending chain a₀ ⊑ a₁ ⊑ ... stabilizes under ▽.
pub fn extrapolation_lemma_ty() -> Expr {
    impl_pi(
        "A",
        type0(),
        arrow(app(cst("WideningOperator"), bvar(0)), prop()),
    )
}
/// `AcceleratedFixpoint : AbstractState → AbstractState`
///
/// Accelerated fixpoint: detect linear recurrences in the Kleene sequence
/// and jump to the closed-form limit (acceleration by closed forms).
pub fn accelerated_fixpoint_ty() -> Expr {
    arrow(cst("AbstractState"), cst("AbstractState"))
}
/// `ProbabilisticAbstractDomain : Type`
///
/// Abstract domain for probabilistic programs: over-approximates distributions
/// over concrete states, e.g., interval-valued probability masses.
pub fn probabilistic_abstract_domain_ty() -> Expr {
    type0()
}
/// `AbstractDistribution : Type → Type`
///
/// Abstract probability distribution over A: a sound over-approximation of
/// sets of probability distributions (e.g., by bounding probabilities).
pub fn abstract_distribution_ty() -> Expr {
    impl_pi("A", type0(), type0())
}
/// `AbstractExpectation : AbstractDistribution → (Type → Real) → Real`
///
/// Abstract expected value: an over-approximation of E_μ\[f\] for abstract μ.
/// Used in abstract interpretation of probabilistic loops.
pub fn abstract_expectation_ty() -> Expr {
    arrow(
        app(cst("AbstractDistribution"), type0()),
        arrow(arrow(type0(), cst("Real")), cst("Real")),
    )
}
/// `ProbabilisticSoundness : AbstractDistribution → Prop`
///
/// Soundness of probabilistic abstraction: γ(a_μ) ⊇ μ_concrete for all
/// concrete distributions consistent with abstract state.
pub fn probabilistic_soundness_ty() -> Expr {
    arrow(app(cst("AbstractDistribution"), type0()), prop())
}
/// `BddAbstractDomain : Nat → Type`
///
/// BDD-based abstract domain for Boolean programs: abstract values are
/// Boolean formulae represented as reduced ordered BDDs.
pub fn bdd_abstract_domain_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `SatAbstractDomain : Type`
///
/// SAT-based abstract domain: abstract values are propositional formulae;
/// emptiness checking reduces to SAT solving.
pub fn sat_abstract_domain_ty() -> Expr {
    type0()
}
/// `SymbolicTransformer : SatAbstractDomain → SatAbstractDomain`
///
/// Symbolic abstract transformer: abstract assignment x := e maps a
/// formula φ to ∃x. φ ∧ x' = e (image computation via SAT/BDD).
pub fn symbolic_transformer_ty() -> Expr {
    arrow(cst("SatAbstractDomain"), cst("SatAbstractDomain"))
}
/// `AbstractModelChecking : AbstractState → Prop → Bool`
///
/// Abstract model checking: verify that all states in the abstract
/// invariant satisfy property P (may return false positives).
pub fn abstract_model_checking_ty() -> Expr {
    arrow(cst("AbstractState"), arrow(prop(), cst("Bool")))
}
/// `SeparationLogicAbsDomain : Type`
///
/// Abstract domain based on separation logic: abstract heap predicates
/// (e.g., list segments, tree shapes) for shape analysis.
pub fn separation_logic_abs_domain_ty() -> Expr {
    type0()
}
/// `AbstractHeapPredicate : Type → Prop`
///
/// An abstract heap predicate: describes a set of memory configurations.
/// Composable via separating conjunction ∗.
pub fn abstract_heap_predicate_ty() -> Expr {
    arrow(type0(), prop())
}
/// `SeparatingConjunction : AbstractHeapPredicate → AbstractHeapPredicate → AbstractHeapPredicate`
///
/// The separating conjunction P ∗ Q: P and Q hold on disjoint heap parts.
pub fn separating_conjunction_ty() -> Expr {
    arrow(
        app(cst("AbstractHeapPredicate"), type0()),
        arrow(
            app(cst("AbstractHeapPredicate"), type0()),
            app(cst("AbstractHeapPredicate"), type0()),
        ),
    )
}
/// `ShapeAnalysis : SeparationLogicAbsDomain → AbstractHeapPredicate`
///
/// Shape analysis result: maps an abstract domain element to the
/// corresponding heap shape predicate (list, tree, dag, etc.).
pub fn shape_analysis_ty() -> Expr {
    arrow(
        cst("SeparationLogicAbsDomain"),
        app(cst("AbstractHeapPredicate"), type0()),
    )
}
/// `HigherOrderAbstractDomain : (Type → Type) → Type`
///
/// Abstract domain lifted to higher-order types (functionals, closures).
/// Approximates sets of functions F : A → B by abstract function spaces.
pub fn higher_order_abstract_domain_ty() -> Expr {
    arrow(arrow(type0(), type0()), type0())
}
/// `AbstractClosure : AbstractState → (AbstractState → AbstractState) → Type`
///
/// Abstract closure: a pair of captured abstract environment and abstract
/// function body, used in higher-order program analysis.
pub fn abstract_closure_ty() -> Expr {
    arrow(
        cst("AbstractState"),
        arrow(arrow(cst("AbstractState"), cst("AbstractState")), type0()),
    )
}
/// `ClosureAbstraction : AbstractClosure → AbstractState → AbstractState`
///
/// Abstract application of a closure to an abstract argument.
pub fn closure_abstraction_ty() -> Expr {
    arrow(
        app2(
            cst("AbstractClosure"),
            cst("AbstractState"),
            arrow(cst("AbstractState"), cst("AbstractState")),
        ),
        arrow(cst("AbstractState"), cst("AbstractState")),
    )
}
/// `HigherOrderFixpoint : (AbstractState → AbstractState) → AbstractState`
///
/// Fixpoint of a higher-order transformer: least fixpoint of a functional
/// on the abstract domain, for recursive procedure analysis.
pub fn higher_order_fixpoint_ty() -> Expr {
    arrow(
        arrow(cst("AbstractState"), cst("AbstractState")),
        cst("AbstractState"),
    )
}
/// `InformationFlowLattice : Type`
///
/// Security lattice for information flow control: elements are security
/// levels (e.g., Low ⊑ High) with flow relation.
pub fn information_flow_lattice_ty() -> Expr {
    type0()
}
/// `NonInterference : AbstractState → InformationFlowLattice → Prop`
///
/// Non-interference: public outputs depend only on public inputs.
/// Formally: ∀ s1 s2, s1 =_L s2 → out(s1) =_L out(s2).
pub fn non_interference_ty() -> Expr {
    arrow(
        cst("AbstractState"),
        arrow(cst("InformationFlowLattice"), prop()),
    )
}
/// `DeclassificationPolicy : Type → InformationFlowLattice → Prop`
///
/// Declassification policy: specifies what secret information may be
/// intentionally revealed (e.g., checking a password hash).
pub fn declassification_policy_ty() -> Expr {
    arrow(type0(), arrow(cst("InformationFlowLattice"), prop()))
}
/// `SecureAbstractInterpreter : AbstractState → InformationFlowLattice → Prop`
///
/// A security-aware abstract interpreter that tracks security levels
/// alongside abstract values for combined functional+security analysis.
pub fn secure_abstract_interpreter_ty() -> Expr {
    arrow(
        cst("AbstractState"),
        arrow(cst("InformationFlowLattice"), prop()),
    )
}
/// `TypeLattice : Type`
///
/// A type lattice: types ordered by subtyping. Abstract interpretation
/// over the type lattice gives type inference as a special case.
pub fn type_lattice_ty() -> Expr {
    type0()
}
/// `TypeRefinement : TypeLattice → AbstractDomain → Type`
///
/// A type refinement: an abstract domain element that refines a base type.
/// E.g., {x : Int | x > 0} refines Int with a positivity predicate.
pub fn type_refinement_ty() -> Expr {
    arrow(
        cst("TypeLattice"),
        arrow(app(cst("AbstractDomain"), type0()), type0()),
    )
}
/// `TypedAbstractTransformer : TypeLattice → AbstractState → AbstractState`
///
/// Type-directed abstract transformer: uses type information to improve
/// precision of transfer functions (e.g., numeric types vs. pointer types).
pub fn typed_abstract_transformer_ty() -> Expr {
    arrow(
        cst("TypeLattice"),
        arrow(cst("AbstractState"), cst("AbstractState")),
    )
}
/// `DeepPolyDomain : Nat → Type`
///
/// The DeepPoly abstract domain for neural network verification: each
/// neuron's activation is over-approximated by a linear constraint.
pub fn deep_poly_domain_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// `AbstractRelu : DeepPolyDomain → DeepPolyDomain`
///
/// Abstract ReLU transformer in DeepPoly: propagates linear bounds through
/// the ReLU nonlinearity using case analysis on the sign of bounds.
pub fn abstract_relu_ty() -> Expr {
    arrow(
        app(cst("DeepPolyDomain"), nat_ty()),
        app(cst("DeepPolyDomain"), nat_ty()),
    )
}
/// `NeuralNetworkVerification : DeepPolyDomain → Prop → Bool`
///
/// Neural network verification: checks that for all inputs in the abstract
/// domain, the network output satisfies a given property (robustness, safety).
pub fn neural_network_verification_ty() -> Expr {
    arrow(
        app(cst("DeepPolyDomain"), nat_ty()),
        arrow(prop(), cst("Bool")),
    )
}
/// `AbstractAffineTransform : Nat → Nat → DeepPolyDomain → DeepPolyDomain`
///
/// Abstract affine transformation W·x + b over DeepPoly domain:
/// propagates linear bounds through a fully-connected layer exactly.
pub fn abstract_affine_transform_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(
            nat_ty(),
            arrow(
                app(cst("DeepPolyDomain"), nat_ty()),
                app(cst("DeepPolyDomain"), nat_ty()),
            ),
        ),
    )
}
/// `AssumeGuaranteeContract : AbstractState → AbstractState → Type`
///
/// An assume-guarantee contract (A, G): if the environment satisfies
/// assumption A, the component guarantees G. Enables compositional reasoning.
pub fn assume_guarantee_contract_ty() -> Expr {
    arrow(cst("AbstractState"), arrow(cst("AbstractState"), type0()))
}
/// `ContractComposition : AssumeGuaranteeContract → AssumeGuaranteeContract → AssumeGuaranteeContract`
///
/// Compose two contracts: the output guarantee of one becomes the input
/// assumption of the other. Sound if no circular dependencies.
pub fn contract_composition_ty() -> Expr {
    let ag = app2(
        cst("AssumeGuaranteeContract"),
        cst("AbstractState"),
        cst("AbstractState"),
    );
    arrow(ag.clone(), arrow(ag.clone(), ag))
}
/// `SummaryFunction : AbstractState → AbstractState`
///
/// Procedure summary: maps abstract pre-states to abstract post-states.
/// Enables modular interprocedural analysis without inlining.
pub fn summary_function_ty() -> Expr {
    arrow(cst("AbstractState"), cst("AbstractState"))
}
/// `CompositionSoundness : SummaryFunction → AssumeGuaranteeContract → Prop`
///
/// Soundness of compositional reasoning: a procedure satisfies its contract
/// whenever its summary function is sound with respect to the contract.
pub fn composition_soundness_ty() -> Expr {
    arrow(
        cst("SummaryFunction"),
        arrow(
            app2(
                cst("AssumeGuaranteeContract"),
                cst("AbstractState"),
                cst("AbstractState"),
            ),
            prop(),
        ),
    )
}
/// `ModularFixpoint : (AbstractState → AbstractState) → AbstractState → AbstractState`
///
/// Modular fixpoint computation: compute procedure fixpoints compositionally,
/// reusing summaries across call sites for efficiency.
pub fn modular_fixpoint_ty() -> Expr {
    arrow(
        arrow(cst("AbstractState"), cst("AbstractState")),
        arrow(cst("AbstractState"), cst("AbstractState")),
    )
}
/// Register the extended abstract interpretation axioms into the environment.
pub fn build_env_extended(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("EllipsoidDomain", ellipsoid_domain_ty()),
        ("ZonotopeDomain", zonotope_domain_ty()),
        ("TemplatePolyhedronDomain", template_polyhedron_domain_ty()),
        ("ReducedProductDomain", reduced_product_domain_ty()),
        ("SmashingDomain", smashing_domain_ty()),
        ("PowersetLiftingDomain", powerset_lifting_domain_ty()),
        ("TraceAbstraction", trace_abstraction_ty()),
        ("AbstractTrace", abstract_trace_ty()),
        ("TraceSemanticsMonotone", trace_semantics_monotone_ty()),
        ("ThreadModularAnalysis", thread_modular_analysis_ty()),
        ("InterferenceAbstraction", interference_abstraction_ty()),
        ("PartialOrderReductionAbs", partial_order_reduction_abs_ty()),
        ("LocksetAnalysis", lockset_analysis_ty()),
        ("WideningOperator", widening_operator_ty()),
        ("NarrowingOperator", narrowing_operator_ty()),
        ("ExtrapolationLemma", extrapolation_lemma_ty()),
        ("AcceleratedFixpoint", accelerated_fixpoint_ty()),
        (
            "ProbabilisticAbstractDomain",
            probabilistic_abstract_domain_ty(),
        ),
        ("AbstractDistribution", abstract_distribution_ty()),
        ("AbstractExpectation", abstract_expectation_ty()),
        ("ProbabilisticSoundness", probabilistic_soundness_ty()),
        ("BddAbstractDomain", bdd_abstract_domain_ty()),
        ("SatAbstractDomain", sat_abstract_domain_ty()),
        ("SymbolicTransformer", symbolic_transformer_ty()),
        ("AbstractModelChecking", abstract_model_checking_ty()),
        ("SeparationLogicAbsDomain", separation_logic_abs_domain_ty()),
        ("AbstractHeapPredicate", abstract_heap_predicate_ty()),
        ("SeparatingConjunction", separating_conjunction_ty()),
        ("ShapeAnalysis", shape_analysis_ty()),
        (
            "HigherOrderAbstractDomain",
            higher_order_abstract_domain_ty(),
        ),
        ("AbstractClosure", abstract_closure_ty()),
        ("ClosureAbstraction", closure_abstraction_ty()),
        ("HigherOrderFixpoint", higher_order_fixpoint_ty()),
        ("InformationFlowLattice", information_flow_lattice_ty()),
        ("NonInterference", non_interference_ty()),
        ("DeclassificationPolicy", declassification_policy_ty()),
        (
            "SecureAbstractInterpreter",
            secure_abstract_interpreter_ty(),
        ),
        ("TypeLattice", type_lattice_ty()),
        ("TypeRefinement", type_refinement_ty()),
        ("TypedAbstractTransformer", typed_abstract_transformer_ty()),
        ("DeepPolyDomain", deep_poly_domain_ty()),
        ("AbstractRelu", abstract_relu_ty()),
        (
            "NeuralNetworkVerification",
            neural_network_verification_ty(),
        ),
        ("AbstractAffineTransform", abstract_affine_transform_ty()),
        ("AssumeGuaranteeContract", assume_guarantee_contract_ty()),
        ("ContractComposition", contract_composition_ty()),
        ("SummaryFunction", summary_function_ty()),
        ("CompositionSoundness", composition_soundness_ty()),
        ("ModularFixpoint", modular_fixpoint_ty()),
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
    fn registered_env() -> Environment {
        let mut env = Environment::new();
        build_env(&mut env);
        env
    }
    #[test]
    fn test_abstract_domain_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("AbstractDomain")).is_some());
    }
    #[test]
    fn test_interval_domain_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("IntervalDomain")).is_some());
    }
    #[test]
    fn test_galois_connection_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("GaloisConnection")).is_some());
    }
    #[test]
    fn test_sign_domain_join() {
        assert_eq!(SignDomain::Pos.join(&SignDomain::Zero), SignDomain::NonNeg);
        assert_eq!(SignDomain::Neg.join(&SignDomain::Pos), SignDomain::Top);
        assert_eq!(SignDomain::Bottom.join(&SignDomain::Neg), SignDomain::Neg);
    }
    #[test]
    fn test_parity_domain_add() {
        assert_eq!(
            ParityDomain::Even.add(&ParityDomain::Even),
            ParityDomain::Even
        );
        assert_eq!(
            ParityDomain::Odd.add(&ParityDomain::Odd),
            ParityDomain::Even
        );
        assert_eq!(
            ParityDomain::Even.add(&ParityDomain::Odd),
            ParityDomain::Odd
        );
    }
    #[test]
    fn test_interval_join_widen() {
        let a = IntervalDomain::new(Bound::Finite(0), Bound::Finite(5));
        let b = IntervalDomain::new(Bound::Finite(3), Bound::Finite(10));
        let j = a.join(&b);
        assert_eq!(j.lower, Bound::Finite(0));
        assert_eq!(j.upper, Bound::Finite(10));
        let a2 = IntervalDomain::new(Bound::Finite(0), Bound::Finite(5));
        let b2 = IntervalDomain::new(Bound::Finite(0), Bound::Finite(20));
        let w = a2.widen(&b2);
        assert_eq!(w.upper, Bound::PosInf);
    }
    #[test]
    fn test_interval_contains() {
        let i = IntervalDomain::new(Bound::Finite(1), Bound::Finite(10));
        assert!(i.contains(5));
        assert!(!i.contains(0));
        assert!(!i.contains(11));
    }
    #[test]
    fn test_galois_connection() {
        let gc = GaloisConnection::interval_galois();
        let abs = gc.abstract_of(&[1, 3, 5, 7]);
        assert_eq!(abs.lower, Bound::Finite(1));
        assert_eq!(abs.upper, Bound::Finite(7));
    }
    #[test]
    fn test_abstract_state_widen() {
        let mut s1 = AbstractState::bottom();
        s1.set("x", IntervalDomain::new(Bound::Finite(0), Bound::Finite(5)));
        let mut s2 = AbstractState::bottom();
        s2.set(
            "x",
            IntervalDomain::new(Bound::Finite(0), Bound::Finite(10)),
        );
        let w = s1.widen(&s2);
        assert_eq!(w.get("x").upper, Bound::PosInf);
    }
    #[test]
    fn test_taint_analysis() {
        let mut ta = TaintAnalysis::new();
        ta.add_source("user_input");
        assert!(ta.is_tainted("user_input"));
        ta.propagate("result", &["user_input", "constant"]);
        assert!(ta.is_tainted("result"));
    }
    #[test]
    fn test_delayed_widening() {
        let a = IntervalDomain::new(Bound::Finite(0), Bound::Finite(5));
        let b = IntervalDomain::new(Bound::Finite(0), Bound::Finite(10));
        let mut dw = DelayedWidening::new(2);
        let r1 = dw.apply(&a, &b);
        assert_eq!(r1.upper, Bound::Finite(10));
        let r2 = dw.apply(&a, &b);
        assert_eq!(r2.upper, Bound::Finite(10));
        let r3 = dw.apply(&a, &b);
        assert_eq!(r3.upper, Bound::PosInf);
    }
    #[test]
    fn test_octagon_satisfiable() {
        let mut oct = OctagonDomain::top(2);
        oct.add_upper_bound(0, 10);
        assert!(oct.is_satisfiable());
    }
    #[test]
    fn test_polyhedral_contains() {
        let mut poly = PolyhedralDomain::top(2);
        poly.add_constraint(vec![1.0, 0.0], 5.0);
        poly.add_constraint(vec![0.0, 1.0], 3.0);
        assert!(poly.contains(&[4.0, 2.0]));
        assert!(!poly.contains(&[6.0, 2.0]));
    }
}
