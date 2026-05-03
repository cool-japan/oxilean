//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AdditiveConj, AlgebraicDomain, BanachFixedPoint, BifiniteApproximation, CoherenceSpace,
    ContinuousDomain, ContinuousLattice, Denotation, DenotationalSoundness, DomainEqn,
    DomainEquation, EnvironmentModel, ExponentialModality, IdealCompletion, InformationSystem,
    KleeneFixedPoint, LinearArrow, LinearType, MultiplicativeConj, OperationalEquivalence,
    Powerdomain, PrimeEventStructure, ProofNet, ScottContinuousFunction, ScottDomain, ScottOpenSet,
    SemanticDomain, StableFunction, DCPO,
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
pub fn app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    app(app3(f, a, b, c), d)
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
/// DCPO: directed-complete partial order.
/// Type: Type → Prop (given a carrier type, asserts it forms a DCPO).
pub fn dcpo_ty() -> Expr {
    arrow(type0(), prop())
}
/// ScottDomain: bounded-complete DCPO with least element ⊥.
pub fn scott_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// AlgebraicDomain: DCPO where every element is the sup of compact elements below it.
pub fn algebraic_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// ContinuousDomain: DCPO where every element is the sup of elements way-below it.
pub fn continuous_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// WayBelow relation: x ≪ y.
/// Type: {D : Type} → D → D → Prop
pub fn way_below_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "x",
            bvar(0),
            pi(BinderInfo::Default, "y", bvar(1), prop()),
        ),
    )
}
/// Directed sup: sup of a directed set exists.
pub fn directed_sup_ty() -> Expr {
    prop()
}
/// ScottOpenSet: upper set closed under directed sups.
pub fn scott_open_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// ScottContinuousFunction: preserves directed sups.
pub fn scott_continuous_fn_ty() -> Expr {
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
/// LawsonTopology: coarsest topology making both identity and complement maps continuous.
pub fn lawson_topology_ty() -> Expr {
    arrow(type0(), prop())
}
/// SpectralSpace: sober T₀ space with compact open sets closed under finite intersection.
pub fn spectral_space_ty() -> Expr {
    arrow(type0(), prop())
}
/// KleeneFixedPoint: ⊔_{n≥0} fⁿ(⊥) for Scott-continuous f on a DCPO.
pub fn kleene_fixed_point_ty() -> Expr {
    prop()
}
/// BanachFixedPoint: unique fixed point of a contractive map on a complete metric space.
pub fn banach_fixed_point_ty() -> Expr {
    prop()
}
/// DomainEquation: D ≅ F(D) for a functor F.
pub fn domain_equation_ty() -> Expr {
    prop()
}
/// SolutionByPointed: bifinite / SFP domain solving D ≅ F(D).
pub fn solution_by_pointed_ty() -> Expr {
    prop()
}
/// ScottContinuousF: functor on Cpo category.
pub fn scott_continuous_functor_ty() -> Expr {
    prop()
}
/// SemanticDomain: type → domain interpreting it.
pub fn semantic_domain_ty() -> Expr {
    arrow(type0(), type0())
}
/// Denotation: ⟦e⟧ρ = semantic value.
pub fn denotation_ty() -> Expr {
    prop()
}
/// EnvironmentModel: type contexts → domains.
pub fn environment_model_ty() -> Expr {
    prop()
}
/// OperationalEquivalence: e₁ ≡ e₂ iff ∀C: C\[e₁\]↓ ↔ C\[e₂\]↓.
pub fn operational_equivalence_ty() -> Expr {
    prop()
}
/// DenotationalSoundness: ⟦e₁⟧ = ⟦e₂⟧ ⟹ e₁ ≡ e₂.
pub fn denotational_soundness_ty() -> Expr {
    prop()
}
/// LinearType: must be used exactly once.
pub fn linear_type_ty() -> Expr {
    arrow(type0(), prop())
}
/// ExponentialModality: !A — can be used any number of times.
pub fn exponential_modality_ty() -> Expr {
    arrow(type0(), type0())
}
/// MultiplicativeConj: A ⊗ B — tensor product of linear types.
pub fn multiplicative_conj_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// AdditiveConj: A & B — "with", shared resources.
pub fn additive_conj_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// LinearArrow: A ⊸ B — linear function using A exactly once.
pub fn linear_arrow_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// ProofNetShortcut: multiplicative proof net (combinatorial proof structure).
pub fn proof_net_ty() -> Expr {
    prop()
}
/// CPO: complete partial order (has sups for all directed subsets, but may lack ⊥).
pub fn cpo_ty() -> Expr {
    arrow(type0(), prop())
}
/// DirectedSet: a non-empty set where any two elements have an upper bound in the set.
pub fn directed_set_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// IsDirected: predicate asserting a subset is directed.
pub fn is_directed_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// UpperBound: x is an upper bound of a subset S in a poset.
pub fn upper_bound_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "x",
            bvar(0),
            arrow(arrow(bvar(1), prop()), prop()),
        ),
    )
}
/// IsLeastUpperBound: x is the least upper bound (supremum) of S.
pub fn is_lub_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "x",
            bvar(0),
            arrow(arrow(bvar(1), prop()), prop()),
        ),
    )
}
/// OmegaCPO: CPO where all omega-chains (indexed by Nat) have sups.
pub fn omega_cpo_ty() -> Expr {
    arrow(type0(), prop())
}
/// ChainComplete: every chain has a supremum.
pub fn chain_complete_ty() -> Expr {
    arrow(type0(), prop())
}
/// TarskiFixedPoint: every monotone function on a complete lattice has a fixed point.
pub fn tarski_fixed_point_ty() -> Expr {
    prop()
}
/// KnasterTarskiLeastFixedPoint: least pre-fixed-point of a monotone map on a CPO.
pub fn knaster_tarski_lfp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), bvar(0)), bvar(0)),
    )
}
/// KnasterTarskiGreatestFixedPoint: greatest post-fixed-point of a monotone map.
pub fn knaster_tarski_gfp_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(arrow(bvar(0), bvar(0)), bvar(0)),
    )
}
/// PreFixedPoint: x is a pre-fixed-point if f(x) ≤ x.
pub fn pre_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "f",
            arrow(bvar(0), bvar(0)),
            arrow(bvar(1), prop()),
        ),
    )
}
/// PostFixedPoint: x is a post-fixed-point if x ≤ f(x).
pub fn post_fixed_point_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "f",
            arrow(bvar(0), bvar(0)),
            arrow(bvar(1), prop()),
        ),
    )
}
/// LiftedDomain: D_⊥ — adds a new bottom element to a type.
pub fn lifted_domain_ty() -> Expr {
    arrow(type0(), type0())
}
/// ProductDomain: D × E — the product of two domains (with pointwise order).
pub fn product_domain_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// FunctionSpaceDomain: \[D → E\] — the domain of Scott-continuous functions.
pub fn function_space_domain_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// SumDomain: D + E — the coalesced sum domain.
pub fn sum_domain_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// PointedDomain: D with a specified bottom element ⊥.
pub fn pointed_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// InformationSystem: (A, Con, ⊢) — Scott's information systems.
/// Type: encodes the three-component structure as a Prop predicate.
pub fn information_system_ty() -> Expr {
    prop()
}
/// IdealCompletion: the ideal completion of a preorder gives a domain.
pub fn ideal_completion_ty() -> Expr {
    arrow(type0(), type0())
}
/// IsIdeal: a downward-closed directed subset (a Scott ideal).
pub fn is_ideal_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "P",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// ConsistentSubset: a subset S where ∃ upper bound in the ambient system.
pub fn consistent_subset_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        type0(),
        arrow(arrow(bvar(0), prop()), prop()),
    )
}
/// EntailmentRelation: a ⊢ x in an information system.
pub fn entailment_relation_ty() -> Expr {
    prop()
}
/// PlotkinPowerdomain: the Plotkin (convex) powerdomain of D.
pub fn plotkin_powerdomain_ty() -> Expr {
    arrow(type0(), type0())
}
/// SmythPowerdomain: the Smyth (upper) powerdomain — over-approximations.
pub fn smyth_powerdomain_ty() -> Expr {
    arrow(type0(), type0())
}
/// HoarePowerdomain: the Hoare (lower) powerdomain — under-approximations.
pub fn hoare_powerdomain_ty() -> Expr {
    arrow(type0(), type0())
}
/// PowerdomainInclusion: embedding of the Hoare into the Plotkin powerdomain.
pub fn powerdomain_inclusion_ty() -> Expr {
    prop()
}
/// AngularBisimulation: bisimulation for powerdomain semantics.
pub fn angular_bisimulation_ty() -> Expr {
    prop()
}
/// SFPDomain: strongly finite projection domain (a class of bifinite domains).
pub fn sfp_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// BifDomain: bifinite domain — directed colimit of finite posets.
pub fn bif_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// StableFunction: a Scott-continuous function that also preserves greatest lower bounds
/// of compatible pairs (Berry's stability condition).
pub fn stable_function_ty() -> Expr {
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
/// BerryOrder: the stable order f ≤_s g iff ∀x y: x≤y ∧ f y defined → f x = g x.
pub fn berry_order_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "E",
            type0(),
            pi(
                BinderInfo::Default,
                "f",
                arrow(bvar(1), bvar(1)),
                pi(BinderInfo::Default, "g", arrow(bvar(2), bvar(2)), prop()),
            ),
        ),
    )
}
/// StronglyStableFunction: stable AND satisfies the extra coherence condition.
pub fn strongly_stable_function_ty() -> Expr {
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
/// SequentialAlgorithm: a Berry-Curien sequential algorithm (stable function representation).
pub fn sequential_algorithm_ty() -> Expr {
    prop()
}
/// EventStructure: (E, ≤, #) — events, causality, conflict relation.
pub fn event_structure_ty() -> Expr {
    prop()
}
/// StableEventStructure: event structure with stability axiom.
pub fn stable_event_structure_ty() -> Expr {
    prop()
}
/// PrimeEventStructure: event structure where causality is a forest.
pub fn prime_event_structure_ty() -> Expr {
    prop()
}
/// ConflictRelation: irreflexive symmetric relation # on events.
pub fn conflict_relation_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "E",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// CoherenceSpace: a set of cliques (tokens + coherence relation).
pub fn coherence_space_ty() -> Expr {
    prop()
}
/// WebOfCoherenceSpace: the underlying set of tokens.
pub fn web_of_coherence_space_ty() -> Expr {
    arrow(type0(), type0())
}
/// CliqueFunctionSpace: A → B for coherence spaces A and B.
pub fn clique_function_space_ty() -> Expr {
    arrow(type0(), arrow(type0(), type0()))
}
/// GameArena: (P, O, λ, ⊢) — a two-player game for game semantics.
pub fn game_arena_ty() -> Expr {
    prop()
}
/// GameStrategy: a deterministic strategy in a game arena.
pub fn game_strategy_ty() -> Expr {
    prop()
}
/// InnocentStrategy: a strategy that only depends on P-view (locally determined).
pub fn innocent_strategy_ty() -> Expr {
    prop()
}
/// WellBracketedStrategy: a strategy respecting call-return matching.
pub fn well_bracketed_strategy_ty() -> Expr {
    prop()
}
/// PCFType: a type in the language PCF (parallel control flow).
pub fn pcf_type_ty() -> Expr {
    arrow(nat_ty(), prop())
}
/// PCFDenotation: semantic interpretation [\[e\]] of a PCF term.
pub fn pcf_denotation_ty() -> Expr {
    prop()
}
/// FullAbstractionPCF: Scott's model of PCF is not fully abstract.
pub fn full_abstraction_pcf_ty() -> Expr {
    prop()
}
/// UniversalDomain: a domain D such that every domain embeds into D.
pub fn universal_domain_ty() -> Expr {
    prop()
}
/// ComputabilityInDomains: a domain-theoretic model of TTE computability.
pub fn computability_in_domains_ty() -> Expr {
    prop()
}
/// QuasimetricSpace: an asymmetric metric space (d(x,y) ≠ d(y,x) allowed).
pub fn quasimetric_space_ty() -> Expr {
    arrow(type0(), prop())
}
/// PartialEquivalenceRelation: PER used for domain-theoretic semantics of types.
pub fn per_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        arrow(bvar(0), arrow(bvar(1), prop())),
    )
}
/// SoberSpace: a topological space where every completely prime filter is a point.
pub fn sober_space_ty() -> Expr {
    arrow(type0(), prop())
}
/// TopologicalDomain: a domain with a compatible topology (T_0 sober space).
pub fn topological_domain_ty() -> Expr {
    arrow(type0(), prop())
}
/// DomainRetract: D is a retract of E (r∘s = id_D, s∘r ≤ id_E).
pub fn domain_retract_ty() -> Expr {
    pi(BinderInfo::Default, "D", type0(), arrow(type0(), prop()))
}
/// EmbeddingProjectionPair: (e, p) with p∘e = id and e∘p ≤ id.
pub fn embedding_projection_pair_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "D",
        type0(),
        pi(
            BinderInfo::Default,
            "E",
            type0(),
            arrow(
                arrow(bvar(1), bvar(1)),
                arrow(arrow(bvar(2), bvar(2)), prop()),
            ),
        ),
    )
}
/// DomainInverseLimit: solution of a domain equation via inverse limit construction.
pub fn domain_inverse_limit_ty() -> Expr {
    prop()
}
/// Register all domain theory axioms in the kernel environment.
pub fn build_domain_theory_env(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("DCPO", dcpo_ty()),
        ("ScottDomain", scott_domain_ty()),
        ("AlgebraicDomain", algebraic_domain_ty()),
        ("ContinuousDomain", continuous_domain_ty()),
        ("WayBelow", way_below_ty()),
        ("DirectedSup", directed_sup_ty()),
        ("IsCompactElement", arrow(type0(), arrow(type0(), prop()))),
        ("HasBottom", arrow(type0(), prop())),
        ("HasTop", arrow(type0(), prop())),
        ("Sup", arrow(list_ty(type0()), type0())),
        ("ScottOpenSet", scott_open_set_ty()),
        ("ScottContinuousFunction", scott_continuous_fn_ty()),
        ("LawsonTopology", lawson_topology_ty()),
        ("SpectralSpace", spectral_space_ty()),
        ("IsScottOpen", prop()),
        ("IsScottClosed", prop()),
        ("KleeneFixedPoint", kleene_fixed_point_ty()),
        ("BanachFixedPoint", banach_fixed_point_ty()),
        ("DomainEquation", domain_equation_ty()),
        ("SolutionByPointed", solution_by_pointed_ty()),
        ("ScottContinuousFunctor", scott_continuous_functor_ty()),
        ("KleeneChain", prop()),
        ("DomainIsomorphism", prop()),
        ("SemanticDomain", semantic_domain_ty()),
        ("Denotation", denotation_ty()),
        ("EnvironmentModel", environment_model_ty()),
        ("OperationalEquivalence", operational_equivalence_ty()),
        ("DenotationalSoundness", denotational_soundness_ty()),
        ("DenotationalAdequacy", prop()),
        ("FullAbstraction", prop()),
        ("LinearType", linear_type_ty()),
        ("ExponentialModality", exponential_modality_ty()),
        ("MultiplicativeConj", multiplicative_conj_ty()),
        ("AdditiveConj", additive_conj_ty()),
        ("LinearArrow", linear_arrow_ty()),
        ("ProofNet", proof_net_ty()),
        (
            "MultiplicativeDisj",
            arrow(type0(), arrow(type0(), type0())),
        ),
        ("AdditiveDisj", arrow(type0(), arrow(type0(), type0()))),
        ("LinearNeg", arrow(type0(), type0())),
        ("CutElimination", prop()),
        ("CPO", cpo_ty()),
        ("DirectedSet", directed_set_ty()),
        ("IsDirected", is_directed_ty()),
        ("UpperBound", upper_bound_ty()),
        ("IsLeastUpperBound", is_lub_ty()),
        ("OmegaCPO", omega_cpo_ty()),
        ("ChainComplete", chain_complete_ty()),
        ("TarskiFixedPoint", tarski_fixed_point_ty()),
        ("KnasterTarskiLFP", knaster_tarski_lfp_ty()),
        ("KnasterTarskiGFP", knaster_tarski_gfp_ty()),
        ("PreFixedPoint", pre_fixed_point_ty()),
        ("PostFixedPoint", post_fixed_point_ty()),
        ("LiftedDomain", lifted_domain_ty()),
        ("ProductDomain", product_domain_ty()),
        ("FunctionSpaceDomain", function_space_domain_ty()),
        ("SumDomain", sum_domain_ty()),
        ("PointedDomain", pointed_domain_ty()),
        ("InformationSystem", information_system_ty()),
        ("IdealCompletion", ideal_completion_ty()),
        ("IsIdeal", is_ideal_ty()),
        ("ConsistentSubset", consistent_subset_ty()),
        ("EntailmentRelation", entailment_relation_ty()),
        ("PlotkinPowerdomain", plotkin_powerdomain_ty()),
        ("SmythPowerdomain", smyth_powerdomain_ty()),
        ("HoarePowerdomain", hoare_powerdomain_ty()),
        ("PowerdomainInclusion", powerdomain_inclusion_ty()),
        ("AngularBisimulation", angular_bisimulation_ty()),
        ("SFPDomain", sfp_domain_ty()),
        ("BifDomain", bif_domain_ty()),
        ("StableFunction", stable_function_ty()),
        ("BerryOrder", berry_order_ty()),
        ("StronglyStableFunction", strongly_stable_function_ty()),
        ("SequentialAlgorithm", sequential_algorithm_ty()),
        ("EventStructure", event_structure_ty()),
        ("StableEventStructure", stable_event_structure_ty()),
        ("PrimeEventStructure", prime_event_structure_ty()),
        ("ConflictRelation", conflict_relation_ty()),
        ("CoherenceSpace", coherence_space_ty()),
        ("WebOfCoherenceSpace", web_of_coherence_space_ty()),
        ("CliqueFunctionSpace", clique_function_space_ty()),
        ("GameArena", game_arena_ty()),
        ("GameStrategy", game_strategy_ty()),
        ("InnocentStrategy", innocent_strategy_ty()),
        ("WellBracketedStrategy", well_bracketed_strategy_ty()),
        ("PCFType", pcf_type_ty()),
        ("PCFDenotation", pcf_denotation_ty()),
        ("FullAbstractionPCF", full_abstraction_pcf_ty()),
        ("UniversalDomain", universal_domain_ty()),
        ("ComputabilityInDomains", computability_in_domains_ty()),
        ("QuasimetricSpace", quasimetric_space_ty()),
        ("PartialEquivalenceRelation", per_ty()),
        ("SoberSpace", sober_space_ty()),
        ("TopologicalDomain", topological_domain_ty()),
        ("DomainRetract", domain_retract_ty()),
        ("EmbeddingProjectionPair", embedding_projection_pair_ty()),
        ("DomainInverseLimit", domain_inverse_limit_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
    Ok(())
}
#[cfg(test)]
mod tests_domain_theory_ext {
    use super::*;
    #[test]
    fn test_continuous_lattice() {
        let cl = ContinuousLattice::new("L", true);
        assert!(cl.is_algebraic);
        assert!(cl.interpolation_property());
        let desc = cl.way_below_description();
        assert!(desc.contains("≪"));
        let ir = ContinuousLattice::real_interval_domain();
        assert!(!ir.is_algebraic);
        let scott = ir.scott_topology_description();
        assert!(scott.contains("Scott"));
    }
    #[test]
    fn test_information_system() {
        let mut is =
            InformationSystem::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        is.add_consistent(0, 1);
        is.add_consistent(1, 2);
        is.add_entailment(vec![0, 1], 2);
        assert!(is.is_consistent_set(&[0, 1]));
        assert!(!is.is_consistent_set(&[0, 2]));
        let desc = is.scott_domain_from_is();
        assert!(desc.contains("consistent"));
    }
    #[test]
    fn test_powerdomains() {
        let plotkin = Powerdomain::plotkin("D");
        let sem = plotkin.semantics_for();
        assert!(sem.contains("nondeterminism"));
        let smyth = Powerdomain::smyth("D");
        let ord = smyth.order_description();
        assert!(ord.contains("Smyth"));
        let hoare = Powerdomain::hoare("D");
        let hord = hoare.order_description();
        assert!(hord.contains("Hoare"));
    }
    #[test]
    fn test_domain_equation() {
        let uc = DomainEqn::untyped_lambda_calculus();
        assert!(uc.solution_name.contains("Scott"));
        let desc = uc.banach_iteration_description();
        assert!(desc.contains("Banach"));
        let stream = DomainEqn::recursive_stream();
        assert!(stream.variable == "S");
        let pitts = stream.pitts_theorem();
        assert!(pitts.contains("Pitts"));
    }
    #[test]
    fn test_bifinite_approximation() {
        let mut ba = BifiniteApproximation::new("D∞");
        ba.add_level("D0");
        ba.add_level("D1");
        ba.add_level("D2");
        assert!(ba.is_sfp_domain());
        let col = ba.colimit_description();
        assert!(col.contains("D0 → D1 → D2"));
    }
}
