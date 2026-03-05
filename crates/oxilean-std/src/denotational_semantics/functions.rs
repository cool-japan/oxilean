//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    FinitePartialOrder, FiniteValuation, InnocentStrategy, KleenePCA, LambdaTerm, LogicalRelation,
    MaybeInterp, MonotoneMap, PCFTerm, PCFType, PCFValue, ScottOpen, Trace,
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
pub fn list_ty(elem: Expr) -> Expr {
    app(cst("List"), elem)
}
pub fn string_ty() -> Expr {
    cst("String")
}
pub fn option_ty(a: Expr) -> Expr {
    app(cst("Option"), a)
}
/// CPO: a set with a partial order and least upper bounds of directed sets.
/// Type: Type → Prop
pub fn cpo_ty() -> Expr {
    arrow(type0(), prop())
}
/// PointedCPO: CPO with a least element ⊥.
/// Type: Type → Prop
pub fn pointed_cpo_ty() -> Expr {
    arrow(type0(), prop())
}
/// ScottDomain: bounded-complete pointed CPO.
/// Type: Type → Prop
pub fn scott_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// BoundedLattice: lattice with top and bottom.
/// Type: Type → Prop
pub fn bounded_lattice_ty() -> Expr {
    arrow(type0(), prop())
}
/// CompleteLattice: every subset has a least upper bound.
/// Type: Type → Prop
pub fn complete_lattice_ty() -> Expr {
    arrow(type0(), prop())
}
/// IsDirected: a subset S of a CPO is directed iff it is nonempty and every
/// finite subset has an upper bound in S.
/// Type: {D : Type} → (D → Prop) → Prop
pub fn is_directed_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// DirectedSup: the least upper bound of a directed set.
/// Type: {D : Type} → (D → Prop) → D
pub fn directed_sup_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), prop()), bvar(0)),
    )
}
/// LeastElement: the bottom element ⊥ of a pointed CPO.
/// Type: {D : Type} → D
pub fn least_element_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), bvar(0))
}
/// IsCompact: x is a compact element of D iff ∀ directed S, x ≤ sup S → ∃ s ∈ S, x ≤ s.
/// Type: {D : Type} → D → Prop
pub fn is_compact_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), arrow(bvar(0), prop()))
}
/// WayBelow: x ≪ y (x is way below y).
/// Type: {D : Type} → D → D → Prop
pub fn way_below_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// ScottContinuous: f : D → E is Scott-continuous iff it preserves directed sups.
/// Type: {D E : Type} → (D → E) → Prop
pub fn scott_continuous_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "E",
            type0(),
            arrow(arrow(bvar(1), bvar(1)), prop()),
        ),
    )
}
/// ContinuousFunctionSpace: [D →_c E] — the CPO of continuous maps from D to E.
/// Type: Type → Type → Type
pub fn continuous_function_space_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// ContinuousId: the identity function on a CPO is continuous.
/// Type: {D : Type} → ScottContinuous (id : D → D)
pub fn continuous_id_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), prop())
}
/// ContinuousCompose: composition of continuous functions is continuous.
/// Type: {D E F} → ScottContinuous f → ScottContinuous g → ScottContinuous (g ∘ f)
pub fn continuous_compose_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "E",
            type0(),
            pi(BinderInfo::Default, "F", type0(), prop()),
        ),
    )
}
/// IsMonotone: f is monotone iff x ≤ y → f x ≤ f y.
/// Type: {D E : Type} → (D → E) → Prop
pub fn is_monotone_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "E",
            type0(),
            arrow(arrow(bvar(1), bvar(1)), prop()),
        ),
    )
}
/// ContinuousImpliesMonotone: every Scott-continuous function is monotone.
/// Type: {D E} → ScottContinuous f → IsMonotone f
pub fn continuous_implies_monotone_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(BinderInfo::Default, "E", type0(), prop()),
    )
}
/// KnasterTarski: for a complete lattice L and monotone f : L → L,
/// the set of fixed points forms a complete lattice; in particular f has a
/// least fixed point lfp(f) = inf { x | f(x) ≤ x }.
/// Type: {L : Type} → CompleteLattice L → IsMonotone f → Prop
pub fn knaster_tarski_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(prop(), arrow(prop(), prop())),
    )
}
/// LeastFixedPoint: lfp f — least fixed point of a monotone function on a complete lattice.
/// Type: {L : Type} → (L → L) → L
pub fn least_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(arrow(bvar(0), bvar(0)), bvar(0)),
    )
}
/// GreatestFixedPoint: gfp f — greatest fixed point.
/// Type: {L : Type} → (L → L) → L
pub fn greatest_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "L",
        type0(),
        arrow(arrow(bvar(0), bvar(0)), bvar(0)),
    )
}
/// KleeneFixedPoint: for a pointed CPO D and Scott-continuous f : D → D,
/// the Kleene chain ⊥ ≤ f(⊥) ≤ f²(⊥) ≤ … has a sup which is lfp(f).
/// Type: {D : Type} → PointedCPO D → ScottContinuous f → lfp f = sup (KleeneChain f)
pub fn kleene_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(prop(), arrow(prop(), prop())),
    )
}
/// KleeneChain: the ascending Kleene chain ⊥, f⊥, f²⊥, …
/// Type: {D : Type} → (D → D) → Nat → D
pub fn kleene_chain_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), bvar(0)), arrow(nat_ty(), bvar(1))),
    )
}
/// FixedPointInduction: if P is admissible (closed under directed sups) and
/// P(⊥) and P is preserved by f, then P(lfp f).
/// Type: {D : Type} → IsAdmissible P → P ⊥ → (∀ x, P x → P (f x)) → P (lfp f)
pub fn fixed_point_induction_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(prop(), arrow(prop(), arrow(prop(), prop()))),
    )
}
/// IsAdmissible: P : D → Prop is admissible iff it is closed under directed sups.
/// Type: {D : Type} → (D → Prop) → Prop
pub fn is_admissible_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// PCFType: the syntactic types of PCF: nat, bool, T1 → T2.
/// Type: Type
pub fn pcf_type_ty() -> Expr {
    type0()
}
/// PCFTerm: a term of PCF (variable, abstraction, application, fix, zero, succ, pred,
/// iszero, true, false, if-then-else, Y combinator).
/// Type: Type
pub fn pcf_term_ty() -> Expr {
    type0()
}
/// SemType: the denotation of a PCF type as a Scott domain.
/// Type: PCFType → Type
pub fn sem_type_ty() -> Expr {
    arrow(type0(), type0())
}
/// SemTerm: the denotation of a PCF term in an environment.
/// Type: {Γ : Ctx} → {τ : PCFType} → Γ ⊢ t : τ → SemCtx Γ → SemType τ
pub fn sem_term_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "Gamma",
        type0(),
        pi(
            BinderInfo::Default,
            "tau",
            type0(),
            arrow(prop(), arrow(type0(), type0())),
        ),
    )
}
/// PCFCtx: a typing context, mapping variables to PCF types.
/// Type: Type
pub fn pcf_ctx_ty() -> Expr {
    type0()
}
/// SemCtx: denotation of a typing context as a product of domains.
/// Type: PCFCtx → Type
pub fn sem_ctx_ty() -> Expr {
    arrow(type0(), type0())
}
/// PCFFix: the fixed-point combinator Y_τ of type (τ → τ) → τ.
/// Type: {τ : PCFType} → SemType ((τ → τ) → τ)
pub fn pcf_fix_ty() -> Expr {
    pi(BinderInfo::Default, "tau", type0(), type0())
}
/// AdequacyThm: denotational semantics of PCF is computationally adequate:
/// if ⟦t⟧ ≠ ⊥ then t evaluates to a value.
/// Type: Prop
pub fn adequacy_thm_ty() -> Expr {
    prop()
}
/// SoundnessThm: if t →* v then ⟦t⟧ = ⟦v⟧.
/// Type: Prop
pub fn soundness_thm_ty() -> Expr {
    prop()
}
/// OperationalEquivalence: t ≅_op s iff ∀ context C, C[t] ⇓ ⟺ C[s] ⇓.
/// Type: PCFTerm → PCFTerm → Prop
pub fn operational_equivalence_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// DenotationalEquivalence: t ≅_den s iff ⟦t⟧ = ⟦s⟧.
/// Type: PCFTerm → PCFTerm → Prop
pub fn denotational_equivalence_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// FullAbstraction: the model is fully abstract iff
/// t ≅_op s ⟺ t ≅_den s for all terms t, s.
/// Type: Prop
pub fn full_abstraction_ty() -> Expr {
    prop()
}
/// Compositionality: if t ≅_den s then C[t] ≅_den C[s] for every context C.
/// Type: Prop
pub fn compositionality_ty() -> Expr {
    prop()
}
/// FullyAbstractModel: a semantic model that is both sound and fully abstract.
/// Type: Type
pub fn fully_abstract_model_ty() -> Expr {
    type0()
}
/// Arena: consists of moves, polarity (O/P), and enabling relation.
/// Type: Type
pub fn arena_ty() -> Expr {
    type0()
}
/// Move: a single interaction event in a game.
/// Type: Type
pub fn move_ty() -> Expr {
    type0()
}
/// Strategy: a prefix-closed, deterministic set of plays for Proponent.
/// Type: Arena → Type
pub fn strategy_ty() -> Expr {
    arrow(type0(), type0())
}
/// InnocentStrategy: a strategy that depends only on P-views.
/// Type: {A : Arena} → Strategy A → Prop
pub fn innocent_strategy_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(type0(), prop()))
}
/// WellBracketed: a strategy respects the call-return nesting discipline.
/// Type: {A : Arena} → Strategy A → Prop
pub fn well_bracketed_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(type0(), prop()))
}
/// StrategyCompose: sequential composition of strategies.
/// Type: Strategy A → Strategy B → Strategy (A → B)
pub fn strategy_compose_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// Play: a sequence of moves consistent with the enabling relation and alternation.
/// Type: Arena → Type
pub fn play_ty() -> Expr {
    arrow(type0(), type0())
}
/// PView: the P-view of a play (the last unanswered P-question and all its justifiers).
/// Type: {A : Arena} → Play A → Play A
pub fn p_view_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(type0(), type0()))
}
/// OView: the O-view of a play.
/// Type: {A : Arena} → Play A → Play A
pub fn o_view_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(type0(), type0()))
}
/// GameModel: interpretation of PCF types as arenas and terms as strategies.
/// Type: Type
pub fn game_model_ty() -> Expr {
    type0()
}
/// GameFullAbstraction: the game semantics of PCF is fully abstract.
/// Type: Prop
pub fn game_full_abstraction_ty() -> Expr {
    prop()
}
/// Trace: a finite or infinite sequence of observable actions.
/// Type: Type → Type  (parameterized by action alphabet)
pub fn trace_ty() -> Expr {
    arrow(type0(), type0())
}
/// TraceSemantics: the set of traces of a program.
/// Type: {A : Type} → Program → PowerSet (Trace A)
pub fn trace_semantics_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(type0(), type0()))
}
/// TraceEquivalence: two programs are trace-equivalent iff they have the same trace set.
/// Type: {A : Type} → Program → Program → Prop
pub fn trace_equivalence_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(type0(), arrow(type0(), prop())),
    )
}
/// TraceInclusion: trace inclusion (refinement).
/// Type: {A : Type} → Program → Program → Prop
pub fn trace_inclusion_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(type0(), arrow(type0(), prop())),
    )
}
/// InfiniteTrace: coinductive infinite trace.
/// Type: {A : Type} → Type
pub fn infinite_trace_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), type0())
}
/// TracePrefix: one trace is a prefix of another.
/// Type: {A : Type} → Trace A → Trace A → Prop
pub fn trace_prefix_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(list_ty(bvar(0)), arrow(list_ty(bvar(1)), prop())),
    )
}
/// TraceConcat: concatenation of two finite traces.
/// Type: {A : Type} → Trace A → Trace A → Trace A
pub fn trace_concat_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(list_ty(bvar(0)), arrow(list_ty(bvar(1)), list_ty(bvar(2)))),
    )
}
/// PlotkinPowerDomain: the convex (Plotkin) power domain over D.
/// Type: Type → Type
pub fn plotkin_power_domain_ty() -> Expr {
    arrow(type0(), type0())
}
/// SmythPowerDomain: the upper (Smyth) power domain — modelling
/// angelic nondeterminism, ordered by reverse-Hoare (Smyth) order.
/// Type: Type → Type
pub fn smyth_power_domain_ty() -> Expr {
    arrow(type0(), type0())
}
/// HoarePowerDomain: the lower (Hoare) power domain — modelling
/// demonic nondeterminism, ordered by Hoare order.
/// Type: Type → Type
pub fn hoare_power_domain_ty() -> Expr {
    arrow(type0(), type0())
}
/// PowerDomainUnit: η : D → P(D), the unit of the power domain monad.
/// Type: {D : Type} → D → PlotkinPowerDomain D
pub fn power_domain_unit_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), arrow(bvar(0), type0()))
}
/// PowerDomainBind: monadic bind for power domains.
/// Type: {D E : Type} → PlotkinPowerDomain D → (D → PlotkinPowerDomain E) → PlotkinPowerDomain E
pub fn power_domain_bind_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "E",
            type0(),
            arrow(type0(), arrow(arrow(bvar(1), type0()), type0())),
        ),
    )
}
/// HoareOrder: the subset ordering on the lower power domain.
/// Type: {D : Type} → HoarePowerDomain D → HoarePowerDomain D → Prop
pub fn hoare_order_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(type0(), arrow(type0(), prop())),
    )
}
/// SmythOrder: the superset ordering on the upper power domain.
/// Type: {D : Type} → SmythPowerDomain D → SmythPowerDomain D → Prop
pub fn smyth_order_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(type0(), arrow(type0(), prop())),
    )
}
/// Valuation: a Scott-continuous map v : Open(D) → [0,1] representing a
/// sub-probability measure on a domain D.
/// Type: Type → Type
pub fn valuation_ty() -> Expr {
    arrow(type0(), type0())
}
/// ProbabilisticPowerDomain: the probabilistic (Jones–Plotkin) power domain.
/// Type: Type → Type
pub fn probabilistic_power_domain_ty() -> Expr {
    arrow(type0(), type0())
}
/// DiracValuation: δ_x — the point mass at x.
/// Type: {D : Type} → D → Valuation D
pub fn dirac_valuation_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), arrow(bvar(0), type0()))
}
/// ValuationBind: monadic bind for the probabilistic power domain.
/// Type: {D E : Type} → Valuation D → (D → Valuation E) → Valuation E
pub fn valuation_bind_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "E",
            type0(),
            arrow(type0(), arrow(arrow(bvar(1), type0()), type0())),
        ),
    )
}
/// ValuationLeq: the stochastic order on valuations (ν ≤ μ iff ∀ U open, ν(U) ≤ μ(U)).
/// Type: {D : Type} → Valuation D → Valuation D → Prop
pub fn valuation_leq_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(type0(), arrow(type0(), prop())),
    )
}
/// ExpectedValue: integration of a bounded continuous function against a valuation.
/// Type: {D : Type} → Valuation D → (D → Nat) → Nat
pub fn expected_value_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(type0(), arrow(arrow(bvar(0), nat_ty()), nat_ty())),
    )
}
/// SubProbabilityValuation: a valuation with total mass ≤ 1.
/// Type: {D : Type} → Valuation D → Prop
pub fn sub_probability_valuation_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), arrow(type0(), prop()))
}
/// DomainFunctor: a functor F : CPO → CPO used to state a domain equation D ≅ F(D).
/// Type: (Type → Type)
pub fn domain_functor_ty() -> Expr {
    arrow(type0(), type0())
}
/// DomainEquation: the recursive specification D ≅ F(D).
/// Type: (Type → Type) → Prop
pub fn domain_equation_ty() -> Expr {
    arrow(arrow(type0(), type0()), prop())
}
/// BilimitSolution: the canonical solution of D ≅ F(D) as a bilimit of
/// the initial F-chain in the category of CPOs and continuous maps.
/// Type: {F : Type → Type} → DomainEquation F → Type
pub fn bilimit_solution_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        arrow(type0(), type0()),
        arrow(prop(), type0()),
    )
}
/// SolutionUnique: the solution of a locally contractive domain equation is unique
/// up to isomorphism.
/// Type: {F : Type → Type} → DomainEquation F → Prop
pub fn solution_unique_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "F",
        arrow(type0(), type0()),
        arrow(prop(), prop()),
    )
}
/// EmbeddingProjectionPair: an e-p pair (e : D → E, p : E → D) with p ∘ e = id and
/// e ∘ p ≤ id, used to build bilimits.
/// Type: Type → Type → Prop
pub fn embedding_projection_pair_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
/// Bilimit: the colimit of a directed diagram of e-p pairs.
/// Type: Type
pub fn bilimit_ty() -> Expr {
    type0()
}
/// UniversalDomain: a CPO U that contains every countably-based Scott domain as a retract.
/// Type: Prop
pub fn universal_domain_ty() -> Expr {
    prop()
}
/// ScottTopUniversal: Scott's D_∞ is the universal Scott domain.
/// Type: Prop
pub fn scott_top_universal_ty() -> Expr {
    prop()
}
/// AJArena: Abramsky-Jagadeesan arena with moves, polarity, and enabling.
/// Type: Type
pub fn aj_arena_ty() -> Expr {
    type0()
}
/// AJGame: a two-player game defined by an arena and an interaction protocol.
/// Type: Type
pub fn aj_game_ty() -> Expr {
    type0()
}
/// InnocentComposition: composition of innocent strategies via hiding.
/// Type: {A B C : Arena} -> Prop
pub fn innocent_composition_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "C",
                type0(),
                arrow(type0(), arrow(type0(), type0())),
            ),
        ),
    )
}
/// CopyingLemma: innocent strategies are determined by their P-views.
/// Type: {A : Arena} -> InnocentStrategy A -> Prop
pub fn copying_lemma_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(type0(), prop()))
}
/// WellBracketedCompose: composition preserves well-bracketedness.
/// Type: {A B C : Arena} -> WellBracketed sigma -> WellBracketed tau -> Prop
pub fn well_bracketed_compose_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        pi(
            BinderInfo::Default,
            "B",
            type0(),
            pi(
                BinderInfo::Default,
                "C",
                type0(),
                arrow(prop(), arrow(prop(), prop())),
            ),
        ),
    )
}
/// LogicalRelation: a family of relations R_tau indexed by types tau.
/// Type: (Type -> Type -> Prop) -> Prop
pub fn logical_relation_ty() -> Expr {
    arrow(arrow(type0(), arrow(type0(), prop())), prop())
}
/// FundamentalTheoremLR: the fundamental theorem of logical relations.
/// Type: Prop
pub fn fundamental_theorem_lr_ty() -> Expr {
    prop()
}
/// ReynoldsAbstraction: Reynolds abstraction theorem for polymorphic lambda-calculus.
/// Type: Prop
pub fn reynolds_abstraction_ty() -> Expr {
    prop()
}
/// ParametricityRelation: the relation witnessing parametricity for a term.
/// Type: {tau : Type} -> tau -> tau -> Prop
pub fn parametricity_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "tau",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// LogRelPreservation: logical relations are preserved under beta-reduction.
/// Type: Prop
pub fn log_rel_preservation_ty() -> Expr {
    prop()
}
/// IdealCompletion: the ideal completion (rounded ideal completion) of a basis.
/// Type: Type -> Type
pub fn ideal_completion_ty() -> Expr {
    arrow(type0(), type0())
}
/// RoundedIdeal: a rounded ideal: downward closed, directed, non-empty subset.
/// Type: {B : Type} -> (B -> Prop) -> Prop
pub fn rounded_ideal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "B",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// ScottOpenSet: a Scott-open set: upward closed and inaccessible by directed joins.
/// Type: {D : Type} -> (D -> Prop) -> Prop
pub fn scott_open_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// ScottTopology: the collection of Scott-open sets forms a topology.
/// Type: {D : Type} -> Prop
pub fn scott_topology_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), prop())
}
/// BasisScottTopology: the way-below neighborhoods form a basis for the Scott topology.
/// Type: {D : Type} -> Prop
pub fn basis_scott_topology_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), prop())
}
/// CCCModel: a cartesian closed category (CCC) modeling typed lambda-calculus.
/// Type: Type
pub fn ccc_model_ty() -> Expr {
    type0()
}
/// CCCInterpretation: interpretation of types as objects and terms as morphisms in a CCC.
/// Type: Type
pub fn ccc_interpretation_ty() -> Expr {
    type0()
}
/// LCCCModel: locally cartesian closed category modeling dependent type theory.
/// Type: Type
pub fn lccc_model_ty() -> Expr {
    type0()
}
/// PCFFullAbstraction: the game-semantic model of PCF is fully abstract.
/// Type: Prop
pub fn pcf_full_abstraction_ty() -> Expr {
    prop()
}
/// SoundnessCCC: every equation provable in the typed lambda-calculus holds in all CCCs.
/// Type: Prop
pub fn soundness_ccc_ty() -> Expr {
    prop()
}
/// CompletenessCCC: every equation valid in all CCCs is provable in the calculus.
/// Type: Prop
pub fn completeness_ccc_ty() -> Expr {
    prop()
}
/// ComputationalMonad: Moggi's computational monad T modeling effectful computations.
/// Type: (Type -> Type) -> Prop
pub fn computational_monad_ty() -> Expr {
    arrow(arrow(type0(), type0()), prop())
}
/// MonadUnit: the unit (return) of a computational monad.
/// Type: {T : Type -> Type} -> {A : Type} -> A -> T A
pub fn monad_unit_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        arrow(type0(), type0()),
        pi(BinderInfo::Default, "A", type0(), arrow(bvar(0), type0())),
    )
}
/// MonadBind: the bind (>>=) of a computational monad.
/// Type: {T A B} -> T A -> (A -> T B) -> T B
pub fn monad_bind_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        arrow(type0(), type0()),
        pi(
            BinderInfo::Default,
            "A",
            type0(),
            pi(
                BinderInfo::Default,
                "B",
                type0(),
                arrow(type0(), arrow(arrow(bvar(1), type0()), type0())),
            ),
        ),
    )
}
/// MonadLaws: the three monad laws (left unit, right unit, associativity).
/// Type: {T : Type -> Type} -> ComputationalMonad T -> Prop
pub fn monad_laws_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "T",
        arrow(type0(), type0()),
        arrow(prop(), prop()),
    )
}
/// MoggiInterpretation: denotational interpretation using Moggi's monadic metalanguage.
/// Type: Type
pub fn moggi_interpretation_ty() -> Expr {
    type0()
}
/// IsoRecursiveType: a recursive type muX.F(X) with explicit fold/unfold.
/// Type: (Type -> Type) -> Type
pub fn iso_recursive_type_ty() -> Expr {
    arrow(arrow(type0(), type0()), type0())
}
/// FoldIso: fold : F(muX.F(X)) -> muX.F(X).
/// Type: {F : Type -> Type} -> Prop
pub fn fold_iso_ty() -> Expr {
    pi(BinderInfo::Default, "F", arrow(type0(), type0()), prop())
}
/// UnfoldIso: unfold : muX.F(X) -> F(muX.F(X)).
/// Type: {F : Type -> Type} -> Prop
pub fn unfold_iso_ty() -> Expr {
    pi(BinderInfo::Default, "F", arrow(type0(), type0()), prop())
}
/// EquiRecursiveType: a recursive type identified up to structural unfolding.
/// Type: (Type -> Type) -> Type
pub fn equi_recursive_type_ty() -> Expr {
    arrow(arrow(type0(), type0()), type0())
}
/// MixedVarianceFunctor: a functor with mixed covariant/contravariant parameters.
/// Type: (Type -> Type -> Type) -> Prop
pub fn mixed_variance_functor_ty() -> Expr {
    arrow(arrow(type0(), arrow(type0(), type0())), prop())
}
/// ContType: the continuation type (tau -> R) where R is the answer type.
/// Type: Type -> Type -> Type
pub fn cont_type_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// CPSTransform: the CPS transformation of a term.
/// Type: {tau R : Type} -> tau -> ContType tau R
pub fn cps_transform_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "tau",
        type0(),
        pi(
            BinderInfo::Default,
            "R",
            type0(),
            arrow(bvar(1), arrow(arrow(bvar(2), bvar(2)), bvar(2))),
        ),
    )
}
/// DoubleNegationTranslation: the double-negation translation for classical/constructive bridge.
/// Type: Type -> Type
pub fn double_negation_translation_ty() -> Expr {
    arrow(type0(), type0())
}
/// CallccOperator: the call/cc operator: ((tau -> R) -> R) -> R.
/// Type: {tau R : Type} -> ((tau -> R) -> R) -> R
pub fn callcc_operator_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "tau",
        type0(),
        pi(
            BinderInfo::Default,
            "R",
            type0(),
            arrow(arrow(arrow(bvar(1), bvar(1)), bvar(1)), bvar(1)),
        ),
    )
}
/// CPSAdequacy: CPS-transformed programs have the same observational behavior.
/// Type: Prop
pub fn cps_adequacy_ty() -> Expr {
    prop()
}
/// ComputationalAdequacy: denotation is non-bottom iff the term has an operational value.
/// Type: Prop
pub fn computational_adequacy_ty() -> Expr {
    prop()
}
/// OperationalSoundness: operational equivalence implies denotational equivalence.
/// Type: Prop
pub fn operational_soundness_ty() -> Expr {
    prop()
}
/// BiOrthogonality: the biorthogonal characterization of observational equivalence.
/// Type: {tau : Type} -> tau -> tau -> Prop
pub fn bi_orthogonality_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "tau",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// ReflexiveGraphModel: logical relations model based on reflexive graph interpretation.
/// Type: Type
pub fn reflexive_graph_model_ty() -> Expr {
    type0()
}
/// PCA: a partial combinatory algebra with K and S combinators.
/// Type: Type -> Prop
pub fn pca_ty() -> Expr {
    arrow(type0(), prop())
}
/// PCACombinatorK: the K combinator in a PCA.
/// Type: {A : Type} -> PCA A -> A
pub fn pca_combinator_k_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(prop(), bvar(1)))
}
/// PCACombinatorS: the S combinator in a PCA.
/// Type: {A : Type} -> PCA A -> A
pub fn pca_combinator_s_ty() -> Expr {
    pi(BinderInfo::Default, "A", type0(), arrow(prop(), bvar(1)))
}
/// RealizabilityInterp: a realizability interpretation of types over a PCA.
/// Type: {A : Type} -> PCA A -> Type -> (A -> Prop)
pub fn realizability_interp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(prop(), arrow(type0(), arrow(bvar(2), prop()))),
    )
}
/// KleeneRealizability: Kleene number realizability.
/// Type: Prop
pub fn kleene_realizability_ty() -> Expr {
    prop()
}
/// ModifiedRealizability: Kreisel modified realizability for HA.
/// Type: Prop
pub fn modified_realizability_ty() -> Expr {
    prop()
}
/// CPMap: a completely positive (CP) map between operator algebras.
/// Type: Type -> Type -> Type
pub fn cp_map_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// CPTPMap: a completely positive trace-preserving (CPTP) map (quantum channel).
/// Type: Type -> Type -> Type
pub fn cptp_map_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// QuantumChannel: synonym for CPTP map used in quantum denotational semantics.
/// Type: Type -> Type -> Type
pub fn quantum_channel_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// DensityMatrix: a positive semidefinite matrix of trace at most 1.
/// Type: Type
pub fn density_matrix_ty() -> Expr {
    type0()
}
/// QuantumDenotation: denotational interpretation of quantum programs as CPTP maps.
/// Type: Type
pub fn quantum_denotation_ty() -> Expr {
    type0()
}
/// Register all denotational semantics axioms in the kernel environment.
pub fn build_denotational_semantics_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("CPO", cpo_ty()),
        ("PointedCPO", pointed_cpo_ty()),
        ("ScottDomain", scott_domain_ty()),
        ("BoundedLattice", bounded_lattice_ty()),
        ("CompleteLattice", complete_lattice_ty()),
        ("IsDirected", is_directed_ty()),
        ("DirectedSup", directed_sup_ty()),
        ("LeastElement", least_element_ty()),
        ("IsCompact", is_compact_ty()),
        ("WayBelow", way_below_ty()),
        ("ScottContinuous", scott_continuous_ty()),
        ("ContinuousFunctionSpace", continuous_function_space_ty()),
        ("ContinuousId", continuous_id_ty()),
        ("ContinuousCompose", continuous_compose_ty()),
        ("IsMonotone", is_monotone_ty()),
        (
            "ContinuousImpliesMonotone",
            continuous_implies_monotone_ty(),
        ),
        ("KnasterTarski", knaster_tarski_ty()),
        ("LeastFixedPoint", least_fixed_point_ty()),
        ("GreatestFixedPoint", greatest_fixed_point_ty()),
        ("KleeneFixedPoint", kleene_fixed_point_ty()),
        ("KleeneChain", kleene_chain_ty()),
        ("FixedPointInduction", fixed_point_induction_ty()),
        ("IsAdmissible", is_admissible_ty()),
        ("PCFType", pcf_type_ty()),
        ("PCFTerm", pcf_term_ty()),
        ("SemType", sem_type_ty()),
        ("SemTerm", sem_term_ty()),
        ("PCFCtx", pcf_ctx_ty()),
        ("SemCtx", sem_ctx_ty()),
        ("PCFFix", pcf_fix_ty()),
        ("AdequacyThm", adequacy_thm_ty()),
        ("SoundnessThm", soundness_thm_ty()),
        ("OperationalEquivalence", operational_equivalence_ty()),
        ("DenotationalEquivalence", denotational_equivalence_ty()),
        ("FullAbstraction", full_abstraction_ty()),
        ("Compositionality", compositionality_ty()),
        ("FullyAbstractModel", fully_abstract_model_ty()),
        ("Arena", arena_ty()),
        ("Move", move_ty()),
        ("Strategy", strategy_ty()),
        ("InnocentStrategy", innocent_strategy_ty()),
        ("WellBracketed", well_bracketed_ty()),
        ("StrategyCompose", strategy_compose_ty()),
        ("Play", play_ty()),
        ("PView", p_view_ty()),
        ("OView", o_view_ty()),
        ("GameModel", game_model_ty()),
        ("GameFullAbstraction", game_full_abstraction_ty()),
        ("Trace", trace_ty()),
        ("TraceSemantics", trace_semantics_ty()),
        ("TraceEquivalence", trace_equivalence_ty()),
        ("TraceInclusion", trace_inclusion_ty()),
        ("InfiniteTrace", infinite_trace_ty()),
        ("TracePrefix", trace_prefix_ty()),
        ("TraceConcat", trace_concat_ty()),
        ("PlotkinPowerDomain", plotkin_power_domain_ty()),
        ("SmythPowerDomain", smyth_power_domain_ty()),
        ("HoarePowerDomain", hoare_power_domain_ty()),
        ("PowerDomainUnit", power_domain_unit_ty()),
        ("PowerDomainBind", power_domain_bind_ty()),
        ("HoareOrder", hoare_order_ty()),
        ("SmythOrder", smyth_order_ty()),
        ("Valuation", valuation_ty()),
        ("ProbabilisticPowerDomain", probabilistic_power_domain_ty()),
        ("DiracValuation", dirac_valuation_ty()),
        ("ValuationBind", valuation_bind_ty()),
        ("ValuationLeq", valuation_leq_ty()),
        ("ExpectedValue", expected_value_ty()),
        ("SubProbabilityValuation", sub_probability_valuation_ty()),
        ("DomainFunctor", domain_functor_ty()),
        ("DomainEquation", domain_equation_ty()),
        ("BilimitSolution", bilimit_solution_ty()),
        ("SolutionUnique", solution_unique_ty()),
        ("EmbeddingProjectionPair", embedding_projection_pair_ty()),
        ("Bilimit", bilimit_ty()),
        ("UniversalDomain", universal_domain_ty()),
        ("ScottTopUniversal", scott_top_universal_ty()),
        ("AJArena", aj_arena_ty()),
        ("AJGame", aj_game_ty()),
        ("InnocentComposition", innocent_composition_ty()),
        ("CopyingLemma", copying_lemma_ty()),
        ("WellBracketedCompose", well_bracketed_compose_ty()),
        ("LogicalRelation", logical_relation_ty()),
        ("FundamentalTheoremLR", fundamental_theorem_lr_ty()),
        ("ReynoldsAbstraction", reynolds_abstraction_ty()),
        ("ParametricityRelation", parametricity_relation_ty()),
        ("LogRelPreservation", log_rel_preservation_ty()),
        ("IdealCompletion", ideal_completion_ty()),
        ("RoundedIdeal", rounded_ideal_ty()),
        ("ScottOpenSet", scott_open_set_ty()),
        ("ScottTopology", scott_topology_ty()),
        ("BasisScottTopology", basis_scott_topology_ty()),
        ("CCCModel", ccc_model_ty()),
        ("CCCInterpretation", ccc_interpretation_ty()),
        ("LCCCModel", lccc_model_ty()),
        ("PCFFullAbstraction", pcf_full_abstraction_ty()),
        ("SoundnessCCC", soundness_ccc_ty()),
        ("CompletenessCCC", completeness_ccc_ty()),
        ("ComputationalMonad", computational_monad_ty()),
        ("MonadUnit", monad_unit_ty()),
        ("MonadBind", monad_bind_ty()),
        ("MonadLaws", monad_laws_ty()),
        ("MoggiInterpretation", moggi_interpretation_ty()),
        ("IsoRecursiveType", iso_recursive_type_ty()),
        ("FoldIso", fold_iso_ty()),
        ("UnfoldIso", unfold_iso_ty()),
        ("EquiRecursiveType", equi_recursive_type_ty()),
        ("MixedVarianceFunctor", mixed_variance_functor_ty()),
        ("ContType", cont_type_ty()),
        ("CPSTransform", cps_transform_ty()),
        (
            "DoubleNegationTranslation",
            double_negation_translation_ty(),
        ),
        ("CallccOperator", callcc_operator_ty()),
        ("CPSAdequacy", cps_adequacy_ty()),
        ("ComputationalAdequacy", computational_adequacy_ty()),
        ("OperationalSoundness", operational_soundness_ty()),
        ("BiOrthogonality", bi_orthogonality_ty()),
        ("ReflexiveGraphModel", reflexive_graph_model_ty()),
        ("PCA", pca_ty()),
        ("PCACombinatorK", pca_combinator_k_ty()),
        ("PCACombinatorS", pca_combinator_s_ty()),
        ("RealizabilityInterp", realizability_interp_ty()),
        ("KleeneRealizability", kleene_realizability_ty()),
        ("ModifiedRealizability", modified_realizability_ty()),
        ("CPMap", cp_map_ty()),
        ("CPTPMap", cptp_map_ty()),
        ("QuantumChannel", quantum_channel_ty()),
        ("DensityMatrix", density_matrix_ty()),
        ("QuantumDenotation", quantum_denotation_ty()),
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
/// Compute the least fixed point of `f` on a finite flat domain {⊥=0, 1, …, n}
/// using Kleene iteration starting from ⊥=0.
pub fn kleene_lfp(f: &MonotoneMap, n: usize) -> usize {
    let mut x = 0usize;
    for _ in 0..=n {
        let fx = f.apply(x);
        if fx == x {
            return x;
        }
        x = fx;
    }
    x
}
/// Compute the Kleene chain [⊥, f(⊥), f²(⊥), …] until a fixed point.
pub fn kleene_chain(f: &MonotoneMap, max_steps: usize) -> Vec<usize> {
    let mut chain = vec![0usize];
    let mut x = 0usize;
    for _ in 0..max_steps {
        let fx = f.apply(x);
        chain.push(fx);
        if fx == x {
            break;
        }
        x = fx;
    }
    chain
}
/// A simple big-step evaluator for ground PCF terms (no free variables).
/// Returns `PCFValue::Bottom` for non-terminating or ill-typed terms.
pub fn pcf_eval(term: &PCFTerm, fuel: u64) -> PCFValue {
    if fuel == 0 {
        return PCFValue::Bottom;
    }
    match term {
        PCFTerm::Zero => PCFValue::Num(0),
        PCFTerm::True => PCFValue::Bool(true),
        PCFTerm::False => PCFValue::Bool(false),
        PCFTerm::Succ(t) => match pcf_eval(t, fuel - 1) {
            PCFValue::Num(n) => PCFValue::Num(n + 1),
            _ => PCFValue::Bottom,
        },
        PCFTerm::Pred(t) => match pcf_eval(t, fuel - 1) {
            PCFValue::Num(0) => PCFValue::Num(0),
            PCFValue::Num(n) => PCFValue::Num(n - 1),
            _ => PCFValue::Bottom,
        },
        PCFTerm::IsZero(t) => match pcf_eval(t, fuel - 1) {
            PCFValue::Num(0) => PCFValue::Bool(true),
            PCFValue::Num(_) => PCFValue::Bool(false),
            _ => PCFValue::Bottom,
        },
        PCFTerm::If(cond, tb, fb) => match pcf_eval(cond, fuel - 1) {
            PCFValue::Bool(true) => pcf_eval(tb, fuel - 1),
            PCFValue::Bool(false) => pcf_eval(fb, fuel - 1),
            _ => PCFValue::Bottom,
        },
        PCFTerm::Lam(body) => PCFValue::Closure(body.clone(), 0),
        _ => PCFValue::Bottom,
    }
}
/// CPS-transform a term with a depth-indexed continuation variable.
pub fn cps_transform_to_string(term: &LambdaTerm, depth: usize) -> String {
    let k = format!("k{depth}");
    match term {
        LambdaTerm::Var(n) => format!("({k} x{n})"),
        LambdaTerm::Const(s) => format!("({k} {s})"),
        LambdaTerm::Lam(body) => {
            let inner = cps_transform_to_string(body, depth + 2);
            format!("({k} (lam x{}. lam k{}. {inner}))", depth + 1, depth + 2)
        }
        LambdaTerm::App(f, x) => {
            let x_cps = cps_transform_to_string(x, depth + 2);
            let f_cont = format!(
                "(lam f{}. (lam v{}. (f{} v{} {k})))",
                depth + 1,
                depth + 2,
                depth + 1,
                depth + 2
            );
            format!(
                "{} /* arg={x_cps} */",
                cps_transform_with_cont(f, depth + 1, &f_cont)
            )
        }
    }
}
pub fn cps_transform_with_cont(term: &LambdaTerm, depth: usize, cont: &str) -> String {
    match term {
        LambdaTerm::Var(n) => format!("({cont} x{n})"),
        LambdaTerm::Const(s) => format!("({cont} {s})"),
        LambdaTerm::Lam(body) => {
            let inner = cps_transform_to_string(body, depth + 2);
            format!("({cont} (lam x{}. lam k{}. {inner}))", depth + 1, depth + 2)
        }
        LambdaTerm::App(f, x) => {
            let x_cps = cps_transform_to_string(x, depth + 2);
            let f_cont = format!(
                "(lam f{}. (lam v{}. (f{} v{} {cont})))",
                depth + 1,
                depth + 2,
                depth + 1,
                depth + 2
            );
            format!(
                "{} /* arg={x_cps} */",
                cps_transform_with_cont(f, depth + 1, &f_cont)
            )
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_denotational_semantics_env() {
        let env = build_denotational_semantics_env();
        assert!(env.get(&Name::str("CPO")).is_some());
        assert!(env.get(&Name::str("KnasterTarski")).is_some());
        assert!(env.get(&Name::str("KleeneFixedPoint")).is_some());
        assert!(env.get(&Name::str("FullAbstraction")).is_some());
        assert!(env.get(&Name::str("Arena")).is_some());
        assert!(env.get(&Name::str("Valuation")).is_some());
        assert!(env.get(&Name::str("DomainEquation")).is_some());
        assert!(env.get(&Name::str("BilimitSolution")).is_some());
    }
    #[test]
    fn test_partial_order_validity() {
        let po = FinitePartialOrder::discrete(4);
        assert!(po.is_valid());
        let flat = FinitePartialOrder::flat(3);
        assert!(flat.is_valid());
    }
    #[test]
    fn test_partial_order_lub() {
        let flat = FinitePartialOrder::flat(3);
        assert_eq!(flat.lub(0, 1), Some(1));
        assert_eq!(flat.lub(1, 1), Some(1));
    }
    #[test]
    fn test_kleene_chain_and_lfp() {
        let f = MonotoneMap::new(vec![1, 1]);
        let chain = kleene_chain(&f, 10);
        assert_eq!(chain[0], 0);
        assert_eq!(chain[1], 1);
        assert_eq!(chain[2], 1);
        let lfp = kleene_lfp(&f, 10);
        assert_eq!(lfp, 1);
    }
    #[test]
    fn test_pcf_type_names() {
        assert_eq!(PCFType::Nat.name(), "Nat");
        assert_eq!(PCFType::Bool.name(), "Bool");
        let arr = PCFType::arrow(PCFType::Nat, PCFType::Bool);
        assert_eq!(arr.name(), "(Nat → Bool)");
    }
    #[test]
    fn test_pcf_eval_basic() {
        let t = PCFTerm::Succ(Box::new(PCFTerm::Succ(Box::new(PCFTerm::Zero))));
        assert_eq!(pcf_eval(&t, 100), PCFValue::Num(2));
        let pred_zero = PCFTerm::Pred(Box::new(PCFTerm::Zero));
        assert_eq!(pcf_eval(&pred_zero, 100), PCFValue::Num(0));
        let iszero = PCFTerm::IsZero(Box::new(PCFTerm::Zero));
        assert_eq!(pcf_eval(&iszero, 100), PCFValue::Bool(true));
    }
    #[test]
    fn test_trace_operations() {
        let t1: Trace<&str> = Trace::new(vec!["a", "b"]);
        let t2: Trace<&str> = Trace::new(vec!["c"]);
        let t3 = t1.concat(&t2);
        assert_eq!(t3.actions, vec!["a", "b", "c"]);
        assert!(t1.is_prefix_of(&t3));
        assert!(!t2.is_prefix_of(&t3));
        let empty: Trace<&str> = Trace::empty();
        assert!(empty.is_prefix_of(&t1));
    }
    #[test]
    fn test_valuation_sub_probability() {
        let v = FiniteValuation::dirac("x");
        assert!(v.is_sub_probability());
        assert!((v.total_mass() - 1.0).abs() < 1e-9);
        let w = FiniteValuation::dirac("y");
        let mixed = v.mix(&w, 0.5);
        assert!(mixed.is_sub_probability());
        assert!((mixed.total_mass() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_innocent_strategy() {
        let mut strat = InnocentStrategy::new();
        strat.add_response(vec![0], 1);
        strat.add_response(vec![0, 2], 3);
        assert_eq!(strat.respond(&[0]), Some(1));
        assert_eq!(strat.respond(&[0, 2]), Some(3));
        assert_eq!(strat.respond(&[99]), None);
        assert_eq!(strat.size(), 2);
    }
    #[test]
    fn test_new_axioms_registered() {
        let env = build_denotational_semantics_env();
        assert!(env.get(&Name::str("AJArena")).is_some());
        assert!(env.get(&Name::str("AJGame")).is_some());
        assert!(env.get(&Name::str("InnocentComposition")).is_some());
        assert!(env.get(&Name::str("CopyingLemma")).is_some());
        assert!(env.get(&Name::str("WellBracketedCompose")).is_some());
        assert!(env.get(&Name::str("LogicalRelation")).is_some());
        assert!(env.get(&Name::str("FundamentalTheoremLR")).is_some());
        assert!(env.get(&Name::str("ReynoldsAbstraction")).is_some());
        assert!(env.get(&Name::str("ParametricityRelation")).is_some());
        assert!(env.get(&Name::str("LogRelPreservation")).is_some());
        assert!(env.get(&Name::str("IdealCompletion")).is_some());
        assert!(env.get(&Name::str("RoundedIdeal")).is_some());
        assert!(env.get(&Name::str("ScottOpenSet")).is_some());
        assert!(env.get(&Name::str("ScottTopology")).is_some());
        assert!(env.get(&Name::str("BasisScottTopology")).is_some());
        assert!(env.get(&Name::str("CCCModel")).is_some());
        assert!(env.get(&Name::str("CCCInterpretation")).is_some());
        assert!(env.get(&Name::str("LCCCModel")).is_some());
        assert!(env.get(&Name::str("PCFFullAbstraction")).is_some());
        assert!(env.get(&Name::str("SoundnessCCC")).is_some());
        assert!(env.get(&Name::str("CompletenessCCC")).is_some());
        assert!(env.get(&Name::str("ComputationalMonad")).is_some());
        assert!(env.get(&Name::str("MonadUnit")).is_some());
        assert!(env.get(&Name::str("MonadBind")).is_some());
        assert!(env.get(&Name::str("MonadLaws")).is_some());
        assert!(env.get(&Name::str("MoggiInterpretation")).is_some());
        assert!(env.get(&Name::str("IsoRecursiveType")).is_some());
        assert!(env.get(&Name::str("FoldIso")).is_some());
        assert!(env.get(&Name::str("UnfoldIso")).is_some());
        assert!(env.get(&Name::str("EquiRecursiveType")).is_some());
        assert!(env.get(&Name::str("MixedVarianceFunctor")).is_some());
        assert!(env.get(&Name::str("ContType")).is_some());
        assert!(env.get(&Name::str("CPSTransform")).is_some());
        assert!(env.get(&Name::str("DoubleNegationTranslation")).is_some());
        assert!(env.get(&Name::str("CallccOperator")).is_some());
        assert!(env.get(&Name::str("CPSAdequacy")).is_some());
        assert!(env.get(&Name::str("ComputationalAdequacy")).is_some());
        assert!(env.get(&Name::str("OperationalSoundness")).is_some());
        assert!(env.get(&Name::str("BiOrthogonality")).is_some());
        assert!(env.get(&Name::str("ReflexiveGraphModel")).is_some());
        assert!(env.get(&Name::str("PCA")).is_some());
        assert!(env.get(&Name::str("PCACombinatorK")).is_some());
        assert!(env.get(&Name::str("PCACombinatorS")).is_some());
        assert!(env.get(&Name::str("RealizabilityInterp")).is_some());
        assert!(env.get(&Name::str("KleeneRealizability")).is_some());
        assert!(env.get(&Name::str("ModifiedRealizability")).is_some());
        assert!(env.get(&Name::str("CPMap")).is_some());
        assert!(env.get(&Name::str("CPTPMap")).is_some());
        assert!(env.get(&Name::str("QuantumChannel")).is_some());
        assert!(env.get(&Name::str("DensityMatrix")).is_some());
        assert!(env.get(&Name::str("QuantumDenotation")).is_some());
    }
    #[test]
    fn test_logical_relation_basic() {
        let mut lr = LogicalRelation::new();
        lr.add("Nat", 0, 0);
        lr.add("Nat", 1, 1);
        lr.add("Nat", 2, 2);
        assert!(lr.relates("Nat", 0, 0));
        assert!(!lr.relates("Nat", 0, 1));
        assert_eq!(lr.size("Nat"), 3);
        assert!(lr.is_reflexive_on("Nat", 3));
        assert!(lr.is_symmetric_on("Nat"));
    }
    #[test]
    fn test_logical_relation_asymmetric() {
        let mut lr = LogicalRelation::new();
        lr.add("Bool", 0, 1);
        assert!(!lr.is_symmetric_on("Bool"));
    }
    #[test]
    fn test_lambda_term_size_and_free_vars() {
        let id = LambdaTerm::lam(LambdaTerm::var(0));
        assert_eq!(id.size(), 2);
        let ap = LambdaTerm::app(LambdaTerm::cst("f"), LambdaTerm::var(0));
        assert_eq!(ap.size(), 3);
        assert!(LambdaTerm::var(0).has_free_var(0, 0));
        assert!(!id.has_free_var(0, 0));
    }
    #[test]
    fn test_cps_transform_produces_output() {
        let v = LambdaTerm::var(0);
        let s = cps_transform_to_string(&v, 0);
        assert!(!s.is_empty());
        assert!(s.contains("k0") && s.contains("x0"));
        let c = LambdaTerm::cst("zero");
        let sc = cps_transform_to_string(&c, 1);
        assert!(sc.contains("k1") && sc.contains("zero"));
        assert!(!cps_transform_to_string(&LambdaTerm::lam(LambdaTerm::var(0)), 0).is_empty());
    }
    #[test]
    fn test_kleene_pca_laws() {
        let pca = KleenePCA::with_ks();
        assert!(pca.check_k_law(), "K law must hold");
        assert!(pca.check_i_law(), "I law must hold");
        assert!(pca.lookup("K").is_some());
        assert!(pca.lookup("S").is_some());
        assert!(pca.lookup("I").is_some());
    }
    #[test]
    fn test_kleene_pca_add_element() {
        let mut pca = KleenePCA::with_ks();
        let idx = pca.add_element("custom");
        assert!(idx >= 3);
        assert_eq!(pca.lookup("custom"), Some(idx));
        pca.define_app(idx, 0, 1);
        assert_eq!(pca.apply(idx, 0), Some(1));
        assert_eq!(pca.apply(idx, 1), None);
    }
    #[test]
    fn test_scott_open_set_operations() {
        let open = ScottOpen::new(5, [1, 2, 3]);
        assert!(open.is_scott_open());
        assert!(open.contains(1) && open.contains(2));
        assert!(!open.contains(0) && !open.contains(4));
        let top = ScottOpen::top(5);
        assert!(top.is_scott_open() && top.elements.len() == 4);
        let empty = ScottOpen::empty(5);
        assert!(empty.is_scott_open() && empty.elements.is_empty());
        assert_eq!(open.union(&empty).elements, vec![1, 2, 3]);
        assert_eq!(open.intersection(&top).elements, vec![1, 2, 3]);
        assert!(open.characteristic(2) && !open.characteristic(4));
    }
    #[test]
    fn test_maybe_interp_monad_laws() {
        let interp = MaybeInterp::new(100);
        assert_eq!(interp.eval(&PCFTerm::Zero), Some(PCFValue::Num(0)));
        assert_eq!(MaybeInterp::new(0).eval(&PCFTerm::Zero), None);
        let v = MaybeInterp::ret(PCFValue::Num(7));
        assert_eq!(v, Some(PCFValue::Num(7)));
        let bound = MaybeInterp::bind(v, |x| match x {
            PCFValue::Num(n) => Some(PCFValue::Num(n * 2)),
            _ => None,
        });
        assert_eq!(bound, Some(PCFValue::Num(14)));
        assert_eq!(MaybeInterp::bind(None, |_| Some(PCFValue::Num(99))), None);
        assert_eq!(MaybeInterp::guard(true), Some(()));
        assert_eq!(MaybeInterp::guard(false), None);
        let mapped = MaybeInterp::map(Some(PCFValue::Num(3)), |v| match v {
            PCFValue::Num(n) => PCFValue::Num(n + 1),
            other => other,
        });
        assert_eq!(mapped, Some(PCFValue::Num(4)));
        let succ_zero = PCFTerm::Succ(Box::new(PCFTerm::Zero));
        assert_eq!(
            interp.seq(&PCFTerm::Zero, &succ_zero),
            Some(PCFValue::Num(1))
        );
    }
}
