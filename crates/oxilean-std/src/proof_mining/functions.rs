//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BarRecursion, BoundType, BoundedArithmetic, CantorNormalForm, Clause, ComplexityBound,
    CookReckhowThm, CurryHoward, DialecticaFormula, DialecticaInterp, EffectiveBound, EmptyClause,
    ExtractedProgram, Finitization, FunctionalInterpretation, GentzenNormalization,
    HerbrandSequenceBuilder, HerbrandTerm, HeuristicFn, MajorizabilityChecker, MetastabilityBound,
    MetricFixedPointMining, ModelCheckingBound, MonotoneFunctionalInterpretation,
    OrdinalTermination, PhpPrinciple, ProofComplexityMeasure, ProofSearcher, ProofState,
    ProofSystem, ProofSystemNew, PropositionalProof, ProverData, QuantitativeCauchy, RamseyBound,
    RealizabilityInterpretation, RealizedFormula, ResolutionProof, ResolutionRefutation,
    ResolutionStep, SearchStrategy, TerminationProof, UniformConvexityModulus, WeakKoenigsLemma,
    WitnessExtractor,
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
/// WitnessExtractor: extracts computational content from a proof
pub fn witness_extractor_ty() -> Expr {
    type0()
}
/// RealizedFormula: realizability type for a formula
pub fn realized_formula_ty() -> Expr {
    type0()
}
/// RealizabilityInterpretation: Kleene's or Kreisel's modified realizability
pub fn realizability_interpretation_ty() -> Expr {
    arrow(cst("Formula"), type0())
}
/// HerbrandTerm: explicit witness extracted from ∃x.P(x)
pub fn herbrand_term_ty() -> Expr {
    type0()
}
/// extract_witness: given a proof of ∃x.P(x), produce a term
/// extract_witness : ∀ (P : Nat → Prop), (∃ x, P x) → HerbrandTerm
pub fn extract_witness_ty() -> Expr {
    impl_pi(
        "P",
        arrow(nat_ty(), prop()),
        arrow(app2(cst("Exists"), nat_ty(), bvar(0)), cst("HerbrandTerm")),
    )
}
/// compute_bound: extract a uniform bound from a proof
/// compute_bound : WitnessExtractor → Nat
pub fn compute_bound_ty() -> Expr {
    arrow(cst("WitnessExtractor"), nat_ty())
}
/// is_realizable: check realizability of a formula
/// is_realizable : Formula → Prop
pub fn is_realizable_ty() -> Expr {
    arrow(cst("Formula"), prop())
}
/// DialecticaFormula: Gödel's Dialectica translation A^D as (∃u.∀x. A_D(u,x))
pub fn dialectica_formula_ty() -> Expr {
    type0()
}
/// FunctionalInterpretation: computable functional realizing ∀∃ statements
pub fn functional_interpretation_ty() -> Expr {
    arrow(cst("DialecticaFormula"), type0())
}
/// WeakKoenigsLemma: WKL and its Dialectica interpretation
pub fn weak_koenighs_lemma_ty() -> Expr {
    prop()
}
/// BoundedArithmetic: PA^ω conservativity results
pub fn bounded_arithmetic_ty() -> Expr {
    type0()
}
/// dialectica_soundness: if PA proves A then A^D is realized
/// dialectica_soundness : ∀ (A : Formula), Provable A → Realized (Dialectica A)
pub fn dialectica_soundness_ty() -> Expr {
    impl_pi(
        "A",
        cst("Formula"),
        arrow(
            app(cst("Provable"), bvar(0)),
            app(cst("Realized"), app(cst("Dialectica"), bvar(1))),
        ),
    )
}
/// ProofSystem enum type
pub fn proof_system_ty() -> Expr {
    type0()
}
/// ProofComplexityMeasure: size, depth, width, degree
pub fn proof_complexity_measure_ty() -> Expr {
    type0()
}
/// PropositionalProof: sequence of lines with justifications
pub fn propositional_proof_ty() -> Expr {
    type0()
}
/// ResolutionProof: DAG of resolution steps
pub fn resolution_proof_ty() -> Expr {
    type0()
}
/// CookReckhowThm: NP-completeness ↔ no efficient proof system
/// CookReckhow : (∃ sys : ProofSystem, Efficient sys) ↔ NP_equals_CoNP
pub fn cook_reckhow_thm_ty() -> Expr {
    app2(
        cst("Iff"),
        app2(
            cst("Exists"),
            cst("ProofSystem"),
            app(cst("Efficient"), bvar(0)),
        ),
        cst("NP_equals_CoNP"),
    )
}
/// Clause: disjunction of literals
pub fn clause_ty() -> Expr {
    type0()
}
/// ResolutionStep: C_1 ∨ x, C_2 ∨ ¬x ⊢ C_1 ∨ C_2
pub fn resolution_step_ty() -> Expr {
    arrow(
        cst("Clause"),
        arrow(cst("Clause"), arrow(cst("Clause"), prop())),
    )
}
/// EmptyClause: ⊥ (contradiction)
pub fn empty_clause_ty() -> Expr {
    prop()
}
/// ResolutionRefutation: proof of unsatisfiability
pub fn resolution_refutation_ty() -> Expr {
    arrow(app(cst("List"), cst("Clause")), prop())
}
/// resolution_completeness: unsatisfiable CNF has a resolution refutation
/// resolution_completeness : ∀ (F : CNFFormula), ¬Satisfiable F → ResolutionRefutation F
pub fn resolution_completeness_ty() -> Expr {
    impl_pi(
        "F",
        cst("CNFFormula"),
        arrow(
            arrow(app(cst("Satisfiable"), bvar(0)), cst("False")),
            app(cst("ResolutionRefutation"), bvar(1)),
        ),
    )
}
/// SearchStrategy type
pub fn search_strategy_ty() -> Expr {
    type0()
}
/// ProofState: current goals, applied tactics, remaining budget
pub fn proof_state_ty() -> Expr {
    type0()
}
/// HeuristicFn: estimate distance to proof
pub fn heuristic_fn_ty() -> Expr {
    arrow(cst("ProofState"), nat_ty())
}
/// ProofSearcher: systematic proof search with backtracking
pub fn proof_searcher_ty() -> Expr {
    type0()
}
/// ModelCheckingBound: bounded model checking with BMC encoding
pub fn model_checking_bound_ty() -> Expr {
    type0()
}
/// CurryHoward: proof-as-program correspondence
pub fn curry_howard_ty() -> Expr {
    impl_pi(
        "P",
        prop(),
        arrow(app(cst("Proof"), bvar(0)), cst("Program")),
    )
}
/// ExtractedProgram: ML-like program extracted from constructive proof
pub fn extracted_program_ty() -> Expr {
    type0()
}
/// TerminationProof: well-founded induction certificate
pub fn termination_proof_ty() -> Expr {
    type0()
}
/// ComplexityBound: polynomial/exponential bound extracted
pub fn complexity_bound_ty() -> Expr {
    type0()
}
/// program_extraction_soundness: extracted program computes the right function
/// program_extraction_soundness : ∀ (P : Prop), Proof P → Realizes (Extract P) P
pub fn program_extraction_soundness_ty() -> Expr {
    impl_pi(
        "P",
        prop(),
        arrow(
            app(cst("Proof"), bvar(0)),
            app2(cst("Realizes"), app(cst("Extract"), bvar(1)), bvar(1)),
        ),
    )
}
/// Register all proof mining and realizability axioms into the kernel environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("WitnessExtractor", witness_extractor_ty()),
        ("RealizedFormula", realized_formula_ty()),
        (
            "RealizabilityInterpretation",
            realizability_interpretation_ty(),
        ),
        ("HerbrandTerm", herbrand_term_ty()),
        ("Formula", type0()),
        (
            "Exists",
            arrow(type0(), arrow(arrow(type0(), prop()), prop())),
        ),
        ("Provable", arrow(cst("Formula"), prop())),
        ("Realized", arrow(type0(), prop())),
        ("Dialectica", arrow(cst("Formula"), type0())),
        ("extract_witness", extract_witness_ty()),
        ("compute_bound", compute_bound_ty()),
        ("is_realizable", is_realizable_ty()),
        ("DialecticaFormula", dialectica_formula_ty()),
        ("FunctionalInterpretation", functional_interpretation_ty()),
        ("WeakKoenigsLemma", weak_koenighs_lemma_ty()),
        ("BoundedArithmetic", bounded_arithmetic_ty()),
        ("dialectica_soundness", dialectica_soundness_ty()),
        ("ProofSystem", proof_system_ty()),
        ("ProofComplexityMeasure", proof_complexity_measure_ty()),
        ("PropositionalProof", propositional_proof_ty()),
        ("ResolutionProof", resolution_proof_ty()),
        ("Efficient", arrow(cst("ProofSystem"), prop())),
        ("NP_equals_CoNP", prop()),
        ("cook_reckhow_thm", cook_reckhow_thm_ty()),
        ("Clause", clause_ty()),
        ("ResolutionStep", resolution_step_ty()),
        ("EmptyClause", empty_clause_ty()),
        ("ResolutionRefutation", resolution_refutation_ty()),
        ("CNFFormula", type0()),
        ("Satisfiable", arrow(cst("CNFFormula"), prop())),
        ("resolution_completeness", resolution_completeness_ty()),
        ("SearchStrategy", search_strategy_ty()),
        ("ProofState", proof_state_ty()),
        ("HeuristicFn", heuristic_fn_ty()),
        ("ProofSearcher", proof_searcher_ty()),
        ("ModelCheckingBound", model_checking_bound_ty()),
        ("Program", type0()),
        ("Proof", arrow(prop(), type0())),
        ("Extract", arrow(prop(), cst("Program"))),
        ("Realizes", arrow(cst("Program"), arrow(prop(), prop()))),
        ("ExtractedProgram", extracted_program_ty()),
        ("TerminationProof", termination_proof_ty()),
        ("ComplexityBound", complexity_bound_ty()),
        ("curry_howard", curry_howard_ty()),
        (
            "program_extraction_soundness",
            program_extraction_soundness_ty(),
        ),
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
/// `MonotoneFunctionalInterp : DialecticaFormula → Type`
/// Kohlenbach's bounded / monotone functional interpretation.
pub fn monotone_functional_interp_ty() -> Expr {
    arrow(cst("DialecticaFormula"), type0())
}
/// `BoundedPrimitiveRecursor : (Nat → Nat → Nat) → Nat → Nat`
/// Bounded primitive recursion (Kohlenbach's T^ω_b).
pub fn bounded_primitive_recursor_ty() -> Expr {
    arrow(
        arrow(nat_ty(), arrow(nat_ty(), nat_ty())),
        arrow(nat_ty(), nat_ty()),
    )
}
/// `WE_HPCA : Prop` — WE-HPCA metatheorem (Kohlenbach): uniform bound extraction in metric spaces.
pub fn we_hpca_ty() -> Expr {
    prop()
}
/// `WE_HRCA : Prop` — WE-HRCA metatheorem: system for real-closed arithmetic.
pub fn we_hrca_ty() -> Expr {
    prop()
}
/// `ModelTheoreticMining : Prop` — model-theoretic metatheorem for proof mining.
pub fn model_theoretic_mining_ty() -> Expr {
    prop()
}
/// `KreiselModifiedRealizability : Formula → Type`
/// Kreisel's modified realizability interpretation.
pub fn kreisel_modified_realizability_ty() -> Expr {
    arrow(cst("Formula"), type0())
}
/// `TroelstraModifiedRealizability : Formula → Type`
/// Troelstra's variant of modified realizability.
pub fn troelstra_modified_realizability_ty() -> Expr {
    arrow(cst("Formula"), type0())
}
/// `BezemRealizability : Formula → Type`
/// Bezem's strong computability realizability.
pub fn bezem_realizability_ty() -> Expr {
    arrow(cst("Formula"), type0())
}
/// `modified_realizability_soundness : ∀ A, Provable A → KreiselMR A`
pub fn modified_realizability_soundness_ty() -> Expr {
    impl_pi(
        "A",
        cst("Formula"),
        arrow(
            app(cst("Provable"), bvar(0)),
            app(cst("KreiselModifiedRealizability"), bvar(1)),
        ),
    )
}
/// `HowardMajorizability : (Nat → Nat) → (Nat → Nat) → Prop`
/// Howard's majorizability: f majorizes g if ∀n, g n ≤ f n.
pub fn howard_majorizability_ty() -> Expr {
    arrow(
        arrow(nat_ty(), nat_ty()),
        arrow(arrow(nat_ty(), nat_ty()), prop()),
    )
}
/// `BezemMajorizability : (Nat → Nat) → (Nat → Nat) → Prop`
/// Bezem's strong majorizability.
pub fn bezem_majorizability_ty() -> Expr {
    arrow(
        arrow(nat_ty(), nat_ty()),
        arrow(arrow(nat_ty(), nat_ty()), prop()),
    )
}
/// `majorizability_closure : ∀ f g h, HowardMaj f g → HowardMaj g h → HowardMaj f h`
pub fn majorizability_closure_ty() -> Expr {
    impl_pi(
        "f",
        arrow(nat_ty(), nat_ty()),
        impl_pi(
            "g",
            arrow(nat_ty(), nat_ty()),
            impl_pi(
                "h",
                arrow(nat_ty(), nat_ty()),
                arrow(
                    app2(cst("HowardMajorizability"), bvar(2), bvar(1)),
                    arrow(
                        app2(cst("HowardMajorizability"), bvar(2), bvar(1)),
                        app2(cst("HowardMajorizability"), bvar(3), bvar(1)),
                    ),
                ),
            ),
        ),
    )
}
/// `HerbrandBound : Nat → Nat`
/// A Herbrand-style bound on the number of instances needed.
pub fn herbrand_bound_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `MetastabilityBoundTy : (Nat → Nat) → Prop`
/// Terence Tao's metastability: ∀ ε k, ∃ n ≤ Φ(ε,k), ...
pub fn metastability_bound_ty_axiom() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), prop())
}
/// `CauchyRate : (Nat → Nat) → Prop`
/// A modulus of convergence (Cauchy rate) for a sequence.
pub fn cauchy_rate_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), prop())
}
/// `FluctuationRate : Nat → Nat`
/// Upper bound on the number of ε-fluctuations of a sequence.
pub fn fluctuation_rate_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// `herbrand_bound_extraction : ∀ (f : Nat → Nat), MetastabilityBound f → ∃ n, Φ n ≤ HerbrandBound n`
pub fn herbrand_bound_extraction_ty() -> Expr {
    impl_pi(
        "f",
        arrow(nat_ty(), nat_ty()),
        arrow(
            app(cst("MetastabilityBoundTy"), bvar(0)),
            app(
                app(cst("Exists"), nat_ty()),
                app(cst("HerbrandBoundBound"), bvar(0)),
            ),
        ),
    )
}
/// `NoCounterexampleInterpretation : Formula → Type`
/// Kreisel's no-counterexample interpretation (nci) of PA.
pub fn no_counterexample_interp_ty() -> Expr {
    arrow(cst("Formula"), type0())
}
/// `dialectica_no_counterexample_equiv : ∀ A, Dialectica A ↔ NCI A`
pub fn dialectica_nci_equiv_ty() -> Expr {
    impl_pi(
        "A",
        cst("Formula"),
        app2(
            cst("Iff"),
            app(cst("Dialectica"), bvar(0)),
            app(cst("NoCounterexampleInterpretation"), bvar(1)),
        ),
    )
}
/// `GodelFunctionalInterp : Formula → Type`
/// Full Gödel functional interpretation (combining Dialectica with T).
pub fn godel_functional_interp_ty() -> Expr {
    arrow(cst("Formula"), type0())
}
/// `UniformContinuityModulus : (Nat → Nat) → Nat → Nat`
/// ω(f, ε) = modulus of uniform continuity of f at precision ε.
pub fn uniform_continuity_modulus_ty() -> Expr {
    arrow(arrow(nat_ty(), nat_ty()), arrow(nat_ty(), nat_ty()))
}
/// `weak_compactness_mining : Prop`
/// Mining uniform bounds from weak compactness arguments.
pub fn weak_compactness_mining_ty() -> Expr {
    prop()
}
/// `ergodic_theorem_mining : Prop`
/// Quantitative bounds extracted from the ergodic theorem (Avigad-Rute).
pub fn ergodic_theorem_mining_ty() -> Expr {
    prop()
}
/// `uniform_continuity_extraction :
///   ∀ (f : Nat → Nat), WeakCompact f → ∃ ω, UniformContinuityModulus f ω`
pub fn uniform_continuity_extraction_ty() -> Expr {
    impl_pi(
        "f",
        arrow(nat_ty(), nat_ty()),
        arrow(
            app(cst("WeakCompact"), bvar(0)),
            app(
                app(cst("Exists"), arrow(nat_ty(), nat_ty())),
                app(cst("IsModulus"), bvar(0)),
            ),
        ),
    )
}
/// `HerbrandSequence : Formula → Type`
/// A Herbrand sequence (finite disjunction of ground instances).
pub fn herbrand_sequence_ty() -> Expr {
    arrow(cst("Formula"), type0())
}
/// `herbrand_theorem : ∀ (A : Formula), Valid A → ∃ H, HerbrandSequence A H ∧ TautologyGround H`
pub fn herbrand_theorem_ty() -> Expr {
    impl_pi(
        "A",
        cst("Formula"),
        arrow(
            app(cst("Valid"), bvar(0)),
            app(
                app(cst("Exists"), cst("HerbrandSequenceObj")),
                app(cst("HerbrandWitness"), bvar(0)),
            ),
        ),
    )
}
/// `herbrand_complexity : HerbrandSequence F → Nat`
/// The size (number of ground instances) in a Herbrand sequence.
pub fn herbrand_complexity_ty() -> Expr {
    arrow(cst("HerbrandSequenceObj"), nat_ty())
}
/// `ShoenfieldCompleteness : Prop`
/// Shoenfield's completeness theorem for realizability.
pub fn shoenfield_completeness_ty() -> Expr {
    prop()
}
/// `BarwiseCompactness : Prop`
/// Barwise compactness for admissible sets.
pub fn barwise_compactness_ty() -> Expr {
    prop()
}
/// `ChoiceFunctionRealization : (Nat → Prop) → (Nat → Nat)`
/// Realizing a choice function from a provability assumption.
pub fn choice_function_realization_ty() -> Expr {
    arrow(arrow(nat_ty(), prop()), arrow(nat_ty(), nat_ty()))
}
/// `Ordinal : Type` — the type of proof-theoretic ordinals.
pub fn ordinal_ty() -> Expr {
    type0()
}
/// `Epsilon0 : Ordinal` — ε_0, the proof-theoretic ordinal of PA.
pub fn epsilon0_ty() -> Expr {
    cst("Ordinal")
}
/// `Gamma0 : Ordinal` — Γ_0, the Feferman-Schütte ordinal (ATR_0).
pub fn gamma0_ty() -> Expr {
    cst("Ordinal")
}
/// `VeblenFunction : Ordinal → Ordinal → Ordinal`
/// φ(α, β) — the Veblen φ-function for ordinal analysis.
pub fn veblen_function_ty() -> Expr {
    arrow(cst("Ordinal"), arrow(cst("Ordinal"), cst("Ordinal")))
}
/// `OrdinalAnalysis : ProofSystem → Ordinal → Prop`
/// The proof-theoretic ordinal of a system S is α.
pub fn ordinal_analysis_ty() -> Expr {
    arrow(cst("ProofSystem"), arrow(cst("Ordinal"), prop()))
}
/// `pa_ordinal_epsilon0 : OrdinalAnalysis PA Epsilon0`
pub fn pa_ordinal_epsilon0_ty() -> Expr {
    app2(cst("OrdinalAnalysis"), cst("PA"), cst("Epsilon0"))
}
/// `atr0_ordinal_gamma0 : OrdinalAnalysis ATR0 Gamma0`
pub fn atr0_ordinal_gamma0_ty() -> Expr {
    app2(cst("OrdinalAnalysis"), cst("ATR0"), cst("Gamma0"))
}
/// Extend the proof mining environment with the new §7–§15 axioms.
pub fn build_env_extended(env: &mut Environment) {
    build_env(env);
    let axioms: &[(&str, Expr)] = &[
        ("MonotoneFunctionalInterp", monotone_functional_interp_ty()),
        ("BoundedPrimitiveRecursor", bounded_primitive_recursor_ty()),
        ("WE_HPCA", we_hpca_ty()),
        ("WE_HRCA", we_hrca_ty()),
        ("ModelTheoreticMining", model_theoretic_mining_ty()),
        (
            "KreiselModifiedRealizability",
            kreisel_modified_realizability_ty(),
        ),
        (
            "TroelstraModifiedRealizability",
            troelstra_modified_realizability_ty(),
        ),
        ("BezemRealizability", bezem_realizability_ty()),
        (
            "modified_realizability_soundness",
            modified_realizability_soundness_ty(),
        ),
        ("HowardMajorizability", howard_majorizability_ty()),
        ("BezemMajorizability", bezem_majorizability_ty()),
        ("majorizability_closure", majorizability_closure_ty()),
        ("HerbrandBound", herbrand_bound_ty()),
        ("MetastabilityBoundTy", metastability_bound_ty_axiom()),
        ("CauchyRate", cauchy_rate_ty()),
        ("FluctuationRate", fluctuation_rate_ty()),
        (
            "HerbrandBoundBound",
            arrow(nat_ty(), arrow(nat_ty(), prop())),
        ),
        ("herbrand_bound_extraction", herbrand_bound_extraction_ty()),
        (
            "NoCounterexampleInterpretation",
            no_counterexample_interp_ty(),
        ),
        ("dialectica_nci_equiv", dialectica_nci_equiv_ty()),
        ("GodelFunctionalInterp", godel_functional_interp_ty()),
        ("UniformContinuityModulus", uniform_continuity_modulus_ty()),
        ("WeakCompact", arrow(arrow(nat_ty(), nat_ty()), prop())),
        (
            "IsModulus",
            arrow(
                arrow(nat_ty(), nat_ty()),
                arrow(arrow(nat_ty(), nat_ty()), prop()),
            ),
        ),
        ("weak_compactness_mining", weak_compactness_mining_ty()),
        ("ergodic_theorem_mining", ergodic_theorem_mining_ty()),
        (
            "uniform_continuity_extraction",
            uniform_continuity_extraction_ty(),
        ),
        ("HerbrandSequence", herbrand_sequence_ty()),
        ("HerbrandSequenceObj", type0()),
        ("Valid", arrow(cst("Formula"), prop())),
        (
            "HerbrandWitness",
            arrow(cst("Formula"), arrow(cst("HerbrandSequenceObj"), prop())),
        ),
        ("TautologyGround", arrow(cst("HerbrandSequenceObj"), prop())),
        ("herbrand_theorem", herbrand_theorem_ty()),
        ("herbrand_complexity", herbrand_complexity_ty()),
        ("ShoenfieldCompleteness", shoenfield_completeness_ty()),
        ("BarwiseCompactness", barwise_compactness_ty()),
        (
            "ChoiceFunctionRealization",
            choice_function_realization_ty(),
        ),
        ("Ordinal", ordinal_ty()),
        ("Epsilon0", epsilon0_ty()),
        ("Gamma0", gamma0_ty()),
        ("VeblenFunction", veblen_function_ty()),
        ("OrdinalAnalysis", ordinal_analysis_ty()),
        ("PA", cst("ProofSystem")),
        ("ATR0", cst("ProofSystem")),
        ("pa_ordinal_epsilon0", pa_ordinal_epsilon0_ty()),
        ("atr0_ordinal_gamma0", atr0_ordinal_gamma0_ty()),
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
mod proof_mining_tests {
    use super::*;
    #[test]
    fn test_build_env_extended() {
        let mut env = oxilean_kernel::Environment::new();
        build_env_extended(&mut env);
        assert!(env.get(&Name::str("WE_HPCA")).is_some());
        assert!(env.get(&Name::str("Epsilon0")).is_some());
        assert!(env.get(&Name::str("VeblenFunction")).is_some());
    }
    #[test]
    fn test_herbrand_sequence_builder() {
        let mut hsb = HerbrandSequenceBuilder::new("∃x, P(x)", 5);
        assert!(hsb.add_instance("P(0)"));
        assert!(hsb.add_instance("P(True)"));
        assert_eq!(hsb.complexity(), 2);
        assert!(hsb.is_tautology());
        let disj = hsb.disjunction();
        assert!(disj.contains("P(0)"));
    }
    #[test]
    fn test_metastability_bound() {
        let mb = MetastabilityBound::constant("ergodic", 100);
        assert!(mb.is_finite());
        assert_eq!(mb.evaluate(0, 0), 100);
        assert_eq!(mb.evaluate(3, 5), 100);
    }
    #[test]
    fn test_majorizability_checker_howard() {
        let checker = MajorizabilityChecker::new(vec![10, 10, 10], vec![1, 2, 3]);
        assert!(checker.howard_majorizes());
    }
    #[test]
    fn test_majorizability_checker_bezem() {
        let checker = MajorizabilityChecker::new(vec![5, 5, 5], vec![1, 2, 3]);
        assert!(checker.bezem_majorizes());
    }
    #[test]
    fn test_majorizability_checker_not_bezem() {
        let checker = MajorizabilityChecker::new(vec![1, 2, 3], vec![0, 0, 5]);
        assert!(!checker.howard_majorizes());
    }
    #[test]
    fn test_cantor_normal_form() {
        let zero = CantorNormalForm::zero();
        let one = CantorNormalForm::one();
        let omega = CantorNormalForm::omega();
        assert!(zero.is_zero());
        assert!(zero.less_than(&one));
        assert!(one.less_than(&omega));
        let sum = one.add(&omega);
        assert_eq!(sum, omega);
    }
    #[test]
    fn test_monotone_functional_interpretation() {
        let interp = MonotoneFunctionalInterpretation::new("∀x ∃y, P x y", "λf. f 0");
        assert!(interp.is_bounded);
        assert!(interp.check_bound());
    }
    #[test]
    fn test_majorizability_pointwise_max() {
        let checker = MajorizabilityChecker::new(vec![1, 5, 2], vec![3, 2, 4]);
        let mx = checker.pointwise_max();
        assert_eq!(mx, vec![3, 5, 4]);
    }
}
/// Functional interpretation of arithmetic.
#[allow(dead_code)]
pub fn functional_interpretations() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Godel Dialectica",
            "System T",
            "Realizes HA+AC by finite-type functionals",
        ),
        (
            "Shoenfield",
            "System T",
            "Alternative to Dialectica for classical arithmetic",
        ),
        ("Diller-Nahm", "System T", "Non-deterministic Dialectica"),
        (
            "Refined A-translation",
            "HA",
            "Transforms classical to constructive arithmetic",
        ),
        (
            "Modified realizability",
            "HA",
            "Sound for HA; extract witnesses",
        ),
        ("q-realizability", "HA", "Quantitative variant for bounds"),
        (
            "Monotone Dialectica",
            "System T",
            "Kohlenbach: for metric structures",
        ),
        (
            "Bounded functional interp",
            "T_0",
            "Ferreira-Oliva: bounded type theory",
        ),
    ]
}
#[cfg(test)]
mod proof_mining_ext_tests {
    use super::*;
    #[test]
    fn test_effective_bound() {
        let mut eb = EffectiveBound::new("Cauchy completeness", "O(n^2)", BoundType::Polynomial);
        eb.add_dependency("metric space axioms");
        assert!(eb.is_feasible());
    }
    #[test]
    fn test_metric_fixed_point() {
        let mfp = MetricFixedPointMining::new(0.5, 1.0);
        let n = mfp.iterations_to_epsilon(0.001);
        assert!(n > 0);
        assert!(n >= 9);
    }
    #[test]
    fn test_ramsey_bound() {
        let r33 = RamseyBound::r33();
        assert!(r33.is_exact());
        assert_eq!(r33.lower_bound, 6);
    }
    #[test]
    fn test_bar_recursion() {
        let br = BarRecursion::spector();
        assert!(br.models_comprehension);
        assert!(!br.description().is_empty());
    }
    #[test]
    fn test_functional_interpretations_nonempty() {
        let interps = functional_interpretations();
        assert!(!interps.is_empty());
    }
}
#[cfg(test)]
mod finitization_tests {
    use super::*;
    #[test]
    fn test_finitization() {
        let bw = Finitization::bolzano_weierstrass();
        assert!(!bw.description().is_empty());
    }
    #[test]
    fn test_ordinal_termination() {
        let ot = OrdinalTermination::new("Knuth-Bendix", "omega^omega", true);
        assert!(!ot.termination_proof().is_empty());
    }
}
#[cfg(test)]
mod quantitative_convergence_tests {
    use super::*;
    #[test]
    fn test_uniform_convexity() {
        let h = UniformConvexityModulus::hilbert_space();
        assert!(!h.modulus.is_empty());
        let lp = UniformConvexityModulus::l_p_space(2.0);
        assert!(!lp.bound_on_iterations_for_mann(0.01).is_empty());
    }
    #[test]
    fn test_quantitative_cauchy() {
        let qc = QuantitativeCauchy::new("CAT(0)", "omega^omega");
        assert!(!qc.leustean_bound_for_cat0().is_empty());
    }
}
#[allow(dead_code)]
pub fn saturation_strategy_name() -> &'static str {
    "Saturation"
}
#[cfg(test)]
mod tests_proof_mining_ext {
    use super::*;
    #[test]
    fn test_dialectica_interpretation() {
        let di = DialecticaInterp::new("∀x.∃y. x + y = 0", "f: N→N", "x: N");
        let transl = di.godel_t_translation();
        assert!(transl.contains("Dialectica"));
        assert!(di.is_sound_for_classical());
        let sound = di.soundness_theorem();
        assert!(sound.contains("sound"));
    }
    #[test]
    fn test_gentzen_normalization() {
        let gn = GentzenNormalization::new(10, 3);
        assert!(gn.reduction_terminates());
        let desc = gn.cut_elimination_theorem();
        assert!(desc.contains("Gentzen"));
        let ord = gn.ordinal_analysis_connection();
        assert!(ord.contains("ε₀"));
    }
    #[test]
    fn test_proof_system() {
        let res = ProofSystemNew::resolution();
        let sep = res.separating_tautologies();
        assert!(sep.contains("Pigeonhole"));
        let ef = ProofSystemNew::extended_frege();
        let cook = ef.cook_reckhow_theorem();
        assert!(cook.contains("Cook-Reckhow"));
    }
    #[test]
    fn test_php_principle() {
        let php = PhpPrinciple::new(5, 4);
        assert!(php.is_valid_php());
        let haken = php.haken_lower_bound_description();
        assert!(haken.contains("Haken"));
        let ba = php.bounded_arithmetic_connection();
        assert!(ba.contains("S^1_2"));
    }
    #[test]
    fn test_prover_data() {
        let tab = ProverData::tableau_prover();
        assert!(tab.is_complete && tab.is_sound);
        let comp = tab.completeness_theorem();
        assert!(comp.contains("complete"));
        let heuristic = ProverData::new_heuristic("E prover", "FOL+Eq", "age+weight");
        assert!(!heuristic.is_complete);
        let super_desc = heuristic.superposition_calculus_description();
        assert!(super_desc.contains("Superposition"));
    }
}
/// Survey of proof mining results in analysis.
#[allow(dead_code)]
pub fn proof_mining_results_survey() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Bolzano-Weierstrass",
            "Omega(n, M) prim. rec.",
            "Dialectica",
        ),
        ("Monotone convergence", "n * 2^(n*M)", "A-translation"),
        ("Cauchy completeness", "O(1/eps)", "Bar recursion"),
        ("Banach fixed point", "ceil(log_q(eps))", "Dialectica"),
        (
            "Mann iteration convergence",
            "Omega(eps, lambda, k)",
            "Monotone Dialectica",
        ),
        (
            "Halpern iteration (Hilbert)",
            "primitive recursive",
            "Monotone bar recursion",
        ),
        (
            "Ergodic theorem (Avigad-Rute)",
            "f(eps, M)",
            "A-translation",
        ),
        (
            "Ramsey's theorem (RT^2_2)",
            "Ackermannian",
            "Weihrauch reduction",
        ),
        (
            "KCF theorem",
            "non-primitive recursive",
            "Finite axioms of choice",
        ),
        (
            "CAT(0) fixed points (Leustean)",
            "omega^omega",
            "Modified Dialectica",
        ),
    ]
}
#[cfg(test)]
mod survey_test {
    use super::*;
    #[test]
    fn test_survey_nonempty() {
        let r = proof_mining_results_survey();
        assert!(!r.is_empty());
    }
}
