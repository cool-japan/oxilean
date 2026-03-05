//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    BottomUpSynth, Candidate, CegisState, Component, ComponentLibrary, FlashFillSynth, FoilLearner,
    FuncProgram, Hole, ILPProblem, IOExample, OGISSynthesizer, OracleSynthLoop, ProgramSketch,
    RefinementType, Sketch, Spec, SyGuSProblem, SynthContext, SynthType, TableOracle,
    TypeDirectedSynth, VersionSpace, CFG,
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
pub(super) fn arrow(a: Expr, b: Expr) -> Expr {
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
pub fn option_ty(t: Expr) -> Expr {
    app(cst("Option"), t)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
/// An oracle that can answer membership queries about the target function.
pub trait Oracle {
    /// Given an input, return the expected output (consulting the oracle).
    fn query(&self, input: &[String]) -> Option<String>;
}
pub fn add_axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    let decl = Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    };
    env.add(decl).map_err(|e| format!("{:?}", e))
}
/// Build the kernel environment with program synthesis axioms.
///
/// Declares types and properties for:
/// - Specification satisfiability
/// - CEGIS completeness
/// - SyGuS soundness
/// - Type-directed synthesis completeness (Djinn)
/// - Component-based soundness and completeness
/// - Sketch-based correctness
/// - Inductive synthesis correctness
/// - Oracle-guided query complexity
pub fn build_program_synthesis_env(env: &mut Environment) -> Result<(), String> {
    add_axiom(
        env,
        "SpecSatisfied",
        arrow(cst("Spec"), arrow(cst("Program"), prop())),
    )?;
    add_axiom(env, "SpecConsistent", arrow(cst("Spec"), prop()))?;
    add_axiom(
        env,
        "SpecEquiv",
        arrow(cst("Spec"), arrow(cst("Spec"), prop())),
    )?;
    add_axiom(env, "CegisSpec", arrow(cst("Spec"), cst("Spec")))?;
    add_axiom(env, "CegisCandidate", cst("Spec"))?;
    add_axiom(
        env,
        "CegisComplete",
        arrow(cst("Spec"), arrow(prop(), prop())),
    )?;
    add_axiom(
        env,
        "CegisSound",
        arrow(cst("Spec"), arrow(cst("Program"), prop())),
    )?;
    add_axiom(
        env,
        "CegisCounterexample",
        arrow(cst("Program"), arrow(cst("Input"), prop())),
    )?;
    add_axiom(
        env,
        "SyGuSGrammarMembership",
        arrow(cst("Program"), arrow(cst("Grammar"), prop())),
    )?;
    add_axiom(
        env,
        "SyGuSSoundness",
        arrow(
            cst("Spec"),
            arrow(cst("Grammar"), arrow(cst("Program"), prop())),
        ),
    )?;
    add_axiom(
        env,
        "SyGuSCompleteness",
        arrow(cst("Spec"), arrow(cst("Grammar"), prop())),
    )?;
    add_axiom(
        env,
        "SyGuSEnumerationBound",
        arrow(nat_ty(), arrow(cst("Grammar"), nat_ty())),
    )?;
    add_axiom(
        env,
        "DeductiveDerivation",
        arrow(cst("Spec"), arrow(cst("Program"), prop())),
    )?;
    add_axiom(
        env,
        "DeductiveCorrect",
        arrow(cst("Spec"), arrow(cst("Program"), prop())),
    )?;
    add_axiom(
        env,
        "WeakestPrecondition",
        arrow(cst("Program"), arrow(cst("Assertion"), cst("Assertion"))),
    )?;
    add_axiom(
        env,
        "StrongestPostcondition",
        arrow(cst("Program"), arrow(cst("Assertion"), cst("Assertion"))),
    )?;
    add_axiom(env, "TypeInhabited", arrow(cst("SynthType"), prop()))?;
    add_axiom(
        env,
        "TypedTerm",
        arrow(cst("SynthType"), arrow(cst("SynthContext"), prop())),
    )?;
    add_axiom(
        env,
        "DjinnComplete",
        arrow(cst("SynthType"), arrow(cst("SynthContext"), prop())),
    )?;
    add_axiom(
        env,
        "DjinnSound",
        arrow(cst("SynthType"), arrow(cst("Program"), prop())),
    )?;
    add_axiom(
        env,
        "TypeDirectedSearchDepth",
        arrow(cst("SynthType"), nat_ty()),
    )?;
    add_axiom(
        env,
        "ComponentApplicable",
        arrow(cst("Component"), arrow(cst("Input"), prop())),
    )?;
    add_axiom(
        env,
        "ComponentCorrect",
        arrow(
            cst("Component"),
            arrow(cst("Input"), arrow(cst("Output"), prop())),
        ),
    )?;
    add_axiom(
        env,
        "ComponentComposition",
        arrow(cst("Component"), arrow(cst("Component"), cst("Component"))),
    )?;
    add_axiom(
        env,
        "ComponentSynthSound",
        arrow(
            cst("Spec"),
            arrow(cst("ComponentLibrary"), arrow(cst("Program"), prop())),
        ),
    )?;
    add_axiom(
        env,
        "SketchCompletion",
        arrow(cst("Sketch"), arrow(cst("Spec"), option_ty(cst("Program")))),
    )?;
    add_axiom(
        env,
        "SketchCorrect",
        arrow(
            cst("Sketch"),
            arrow(cst("Spec"), arrow(cst("Program"), prop())),
        ),
    )?;
    add_axiom(env, "HoleCount", arrow(cst("Sketch"), nat_ty()))?;
    add_axiom(
        env,
        "SketchSubsumes",
        arrow(cst("Sketch"), arrow(cst("Grammar"), prop())),
    )?;
    add_axiom(
        env,
        "ExampleConsistency",
        arrow(cst("Program"), arrow(list_ty(cst("IOExample")), prop())),
    )?;
    add_axiom(
        env,
        "VersionSpaceNonEmpty",
        arrow(list_ty(cst("IOExample")), prop()),
    )?;
    add_axiom(
        env,
        "PBEConvergence",
        arrow(nat_ty(), arrow(list_ty(cst("IOExample")), prop())),
    )?;
    add_axiom(
        env,
        "FlashFillCorrect",
        arrow(list_ty(cst("IOExample")), arrow(cst("Program"), prop())),
    )?;
    add_axiom(
        env,
        "OracleQuery",
        arrow(cst("Input"), option_ty(cst("Output"))),
    )?;
    add_axiom(
        env,
        "OracleGuided",
        arrow(cst("Oracle"), arrow(nat_ty(), option_ty(cst("Program")))),
    )?;
    add_axiom(env, "OracleQueryComplexity", arrow(nat_ty(), nat_ty()))?;
    add_axiom(
        env,
        "StructuralRecursion",
        arrow(cst("FuncProgram"), prop()),
    )?;
    add_axiom(
        env,
        "FuncSynthCorrect",
        arrow(cst("Spec"), arrow(cst("FuncProgram"), prop())),
    )?;
    add_axiom(
        env,
        "FuncSynthTerminating",
        arrow(cst("Spec"), arrow(nat_ty(), prop())),
    )?;
    add_axiom(
        env,
        "BottomUpEnumCorrect",
        arrow(nat_ty(), arrow(cst("Spec"), option_ty(cst("FuncProgram")))),
    )?;
    Ok(())
}
/// Inductive program synthesis: soundness of inductive generalization.
pub fn axiom_inductive_generalization_ty() -> Expr {
    arrow(list_ty(cst("IOExample")), arrow(cst("Program"), prop()))
}
/// Inductive program synthesis: completeness over a finite hypothesis space.
pub fn axiom_inductive_completeness_ty() -> Expr {
    arrow(
        cst("HypothesisSpace"),
        arrow(list_ty(cst("IOExample")), option_ty(cst("Program"))),
    )
}
/// Minimal description length principle for inductive synthesis.
pub fn axiom_mdl_synthesis_ty() -> Expr {
    arrow(cst("Program"), arrow(list_ty(cst("IOExample")), nat_ty()))
}
/// SyGuS multi-function: soundness when synthesising several functions jointly.
pub fn axiom_sygus_multi_function_ty() -> Expr {
    arrow(list_ty(cst("SyGuSProblem")), arrow(cst("Grammar"), prop()))
}
/// SyGuS separability: two grammars produce disjoint solution sets.
pub fn axiom_sygus_separability_ty() -> Expr {
    arrow(cst("Grammar"), arrow(cst("Grammar"), prop()))
}
/// CEGIS with bounded counterexample depth.
pub fn axiom_cegis_bounded_depth_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("Spec"), arrow(cst("Program"), prop())))
}
/// CEGIS acceleration: convergence rate under a strong verifier.
pub fn axiom_cegis_convergence_rate_ty() -> Expr {
    arrow(cst("Spec"), arrow(nat_ty(), nat_ty()))
}
/// Oracle-guided synthesis with membership queries only.
pub fn axiom_oracle_membership_query_ty() -> Expr {
    arrow(cst("Input"), bool_ty())
}
/// Oracle-guided synthesis with equivalence queries.
pub fn axiom_oracle_equivalence_query_ty() -> Expr {
    arrow(
        cst("Program"),
        arrow(cst("Oracle"), pair_ty(bool_ty(), option_ty(cst("Input")))),
    )
}
/// Oracle-guided synthesis: Angluin's L* query complexity bound.
pub fn axiom_lstar_query_complexity_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), nat_ty()))
}
/// Type-driven synthesis: refinement type checking decidability.
pub fn axiom_refinement_type_decidable_ty() -> Expr {
    arrow(cst("RefinementType"), arrow(cst("Program"), prop()))
}
/// Refinement type synthesis: soundness of liquid type inference.
pub fn axiom_liquid_type_soundness_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("RefinementType"), prop()))
}
/// Refinement type synthesis: completeness for subtyping.
pub fn axiom_refinement_subtype_complete_ty() -> Expr {
    arrow(cst("RefinementType"), arrow(cst("RefinementType"), prop()))
}
/// Manna–Waldinger deductive synthesis: resolution correctness.
pub fn axiom_mw_resolution_correct_ty() -> Expr {
    arrow(cst("Goal"), arrow(cst("Program"), prop()))
}
/// Manna–Waldinger synthesis: goal reduction termination.
pub fn axiom_mw_termination_ty() -> Expr {
    arrow(cst("Goal"), prop())
}
/// Synthesis from logical specifications: realizability.
pub fn axiom_spec_realizability_ty() -> Expr {
    arrow(cst("Spec"), prop())
}
/// Synthesis from specifications: Church's synthesis problem.
pub fn axiom_church_synthesis_ty() -> Expr {
    arrow(cst("LTLSpec"), option_ty(cst("ReactiveProgram")))
}
/// Neural program synthesis: soundness of learned synthesiser.
pub fn axiom_neural_synth_soundness_ty() -> Expr {
    arrow(
        cst("NeuralModel"),
        arrow(list_ty(cst("IOExample")), option_ty(cst("Program"))),
    )
}
/// Neural program synthesis: generalisation bound (PAC-style).
pub fn axiom_neural_synth_generalisation_ty() -> Expr {
    arrow(nat_ty(), arrow(cst("NeuralModel"), prop()))
}
/// Execution-guided synthesis: observable semantics consistency.
pub fn axiom_execution_guided_consistency_ty() -> Expr {
    arrow(
        cst("Program"),
        arrow(list_ty(cst("ExecutionTrace")), prop()),
    )
}
/// Execution-guided synthesis: trace completeness.
pub fn axiom_execution_trace_complete_ty() -> Expr {
    arrow(list_ty(cst("ExecutionTrace")), option_ty(cst("Program")))
}
/// Constraint-based synthesis: SAT encoding correctness.
pub fn axiom_constraint_encoding_correct_ty() -> Expr {
    arrow(cst("SynthConstraint"), arrow(cst("Program"), prop()))
}
/// Constraint-based synthesis: finite domain decidability.
pub fn axiom_finite_domain_decidable_ty() -> Expr {
    arrow(cst("SynthConstraint"), prop())
}
/// Superoptimisation: optimality of synthesised loop-free program.
pub fn axiom_superopt_optimal_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("CostModel"), prop()))
}
/// Superoptimisation: equivalence preservation.
pub fn axiom_superopt_equivalence_ty() -> Expr {
    arrow(cst("Program"), arrow(cst("Program"), prop()))
}
/// Loop invariant synthesis: inductive invariant correctness.
pub fn axiom_loop_invariant_correct_ty() -> Expr {
    arrow(cst("LoopBody"), arrow(cst("Assertion"), prop()))
}
/// Loop invariant synthesis: strongest inductive invariant existence.
pub fn axiom_strongest_invariant_exists_ty() -> Expr {
    arrow(cst("LoopBody"), option_ty(cst("Assertion")))
}
/// Sketch framework: hole satisfiability modulo theory.
pub fn axiom_sketch_hole_smt_ty() -> Expr {
    arrow(
        cst("Sketch"),
        arrow(cst("Theory"), option_ty(cst("Assignment"))),
    )
}
/// FlashMeta: DSL synthesis via version space algebra.
pub fn axiom_flashmeta_vsa_ty() -> Expr {
    arrow(
        list_ty(cst("IOExample")),
        arrow(cst("DSL"), cst("VersionSpace")),
    )
}
/// FlashFill generalization: number of examples sufficient for uniqueness.
pub fn axiom_flashfill_unique_threshold_ty() -> Expr {
    arrow(nat_ty(), arrow(list_ty(cst("IOExample")), prop()))
}
/// Abstraction-based synthesis: abstraction refinement correctness.
pub fn axiom_abstraction_refinement_correct_ty() -> Expr {
    arrow(cst("Abstraction"), arrow(cst("Spec"), prop()))
}
/// Abstraction-based synthesis: spurious counterexample detection.
pub fn axiom_spurious_cex_detection_ty() -> Expr {
    arrow(cst("Abstraction"), arrow(cst("Counterexample"), bool_ty()))
}
/// Learning from demonstrations: behavioral cloning correctness.
pub fn axiom_behavioral_cloning_correct_ty() -> Expr {
    arrow(list_ty(cst("Demonstration")), arrow(cst("Policy"), prop()))
}
/// Learning from demonstrations: inverse reinforcement learning soundness.
pub fn axiom_irl_soundness_ty() -> Expr {
    arrow(
        list_ty(cst("Demonstration")),
        arrow(cst("RewardFunction"), prop()),
    )
}
/// Program synthesis by A* search: admissible heuristic correctness.
pub fn axiom_astar_synth_admissible_ty() -> Expr {
    arrow(cst("Heuristic"), arrow(cst("SynthState"), nat_ty()))
}
/// Stochastic program synthesis: probabilistic soundness.
pub fn axiom_stochastic_synth_soundness_ty() -> Expr {
    arrow(cst("ProbDistribution"), arrow(cst("Spec"), prop()))
}
/// Synthesis with sketches: relative completeness wrt sketch language.
pub fn axiom_sketch_relative_complete_ty() -> Expr {
    arrow(cst("SketchLanguage"), arrow(cst("Spec"), prop()))
}
/// Build the extended program synthesis environment (new axioms).
///
/// Adds 30+ new axioms covering:
/// - Inductive synthesis (MDL, PBE generalization)
/// - CEGIS extensions (bounded depth, convergence rate)
/// - SyGuS extensions (multi-function, separability)
/// - Oracle-guided synthesis (membership, equivalence, L*)
/// - Refinement types (liquid types, subtyping)
/// - Manna–Waldinger synthesis
/// - Neural, execution-guided, and constraint-based synthesis
/// - Superoptimisation
/// - Loop invariant synthesis
/// - Sketch / FlashFill / FlashMeta
/// - Abstraction-based synthesis
/// - Learning from demonstrations
/// - Stochastic and search-based synthesis
pub fn build_program_synthesis_env_ext(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        (
            "InductiveGeneralization",
            axiom_inductive_generalization_ty(),
        ),
        ("InductiveCompleteness", axiom_inductive_completeness_ty()),
        ("MDLSynthesis", axiom_mdl_synthesis_ty()),
        ("SyGuSMultiFunction", axiom_sygus_multi_function_ty()),
        ("SyGuSSeparability", axiom_sygus_separability_ty()),
        ("CegisBoundedDepth", axiom_cegis_bounded_depth_ty()),
        ("CegisConvergenceRate", axiom_cegis_convergence_rate_ty()),
        ("OracleMembershipQuery", axiom_oracle_membership_query_ty()),
        (
            "OracleEquivalenceQuery",
            axiom_oracle_equivalence_query_ty(),
        ),
        ("LStarQueryComplexity", axiom_lstar_query_complexity_ty()),
        (
            "RefinementTypeDecidable",
            axiom_refinement_type_decidable_ty(),
        ),
        ("LiquidTypeSoundness", axiom_liquid_type_soundness_ty()),
        (
            "RefinementSubtypeComplete",
            axiom_refinement_subtype_complete_ty(),
        ),
        ("MWResolutionCorrect", axiom_mw_resolution_correct_ty()),
        ("MWTermination", axiom_mw_termination_ty()),
        ("SpecRealizability", axiom_spec_realizability_ty()),
        ("ChurchSynthesis", axiom_church_synthesis_ty()),
        ("NeuralSynthSoundness", axiom_neural_synth_soundness_ty()),
        (
            "NeuralSynthGeneralisation",
            axiom_neural_synth_generalisation_ty(),
        ),
        (
            "ExecutionGuidedConsistency",
            axiom_execution_guided_consistency_ty(),
        ),
        (
            "ExecutionTraceComplete",
            axiom_execution_trace_complete_ty(),
        ),
        (
            "ConstraintEncodingCorrect",
            axiom_constraint_encoding_correct_ty(),
        ),
        ("FiniteDomainDecidable", axiom_finite_domain_decidable_ty()),
        ("SuperoptOptimal", axiom_superopt_optimal_ty()),
        ("SuperoptEquivalence", axiom_superopt_equivalence_ty()),
        ("LoopInvariantCorrect", axiom_loop_invariant_correct_ty()),
        (
            "StrongestInvariantExists",
            axiom_strongest_invariant_exists_ty(),
        ),
        ("SketchHoleSMT", axiom_sketch_hole_smt_ty()),
        ("FlashMetaVSA", axiom_flashmeta_vsa_ty()),
        (
            "FlashFillUniqueThreshold",
            axiom_flashfill_unique_threshold_ty(),
        ),
        (
            "AbstractionRefinementCorrect",
            axiom_abstraction_refinement_correct_ty(),
        ),
        ("SpuriousCexDetection", axiom_spurious_cex_detection_ty()),
        (
            "BehavioralCloningCorrect",
            axiom_behavioral_cloning_correct_ty(),
        ),
        ("IRLSoundness", axiom_irl_soundness_ty()),
        ("AStarSynthAdmissible", axiom_astar_synth_admissible_ty()),
        (
            "StochasticSynthSoundness",
            axiom_stochastic_synth_soundness_ty(),
        ),
        (
            "SketchRelativeComplete",
            axiom_sketch_relative_complete_ty(),
        ),
    ];
    for (name, ty) in axioms {
        add_axiom(env, name, ty.clone())?;
    }
    Ok(())
}
/// Build a single `SpecSatisfied` axiom declaration.
pub fn decl_spec_satisfied() -> Declaration {
    Declaration::Axiom {
        name: Name::str("SpecSatisfied"),
        univ_params: vec![],
        ty: arrow(cst("Spec"), arrow(cst("Program"), prop())),
    }
}
/// Build the `CegisComplete` axiom declaration.
pub fn decl_cegis_complete() -> Declaration {
    Declaration::Axiom {
        name: Name::str("CegisComplete"),
        univ_params: vec![],
        ty: arrow(cst("Spec"), arrow(prop(), prop())),
    }
}
/// Build the `SyGuSSoundness` axiom declaration.
pub fn decl_sygus_soundness() -> Declaration {
    Declaration::Axiom {
        name: Name::str("SyGuSSoundness"),
        univ_params: vec![],
        ty: arrow(
            cst("Spec"),
            arrow(cst("Grammar"), arrow(cst("Program"), prop())),
        ),
    }
}
/// Build the `DjinnComplete` axiom declaration.
pub fn decl_djinn_complete() -> Declaration {
    Declaration::Axiom {
        name: Name::str("DjinnComplete"),
        univ_params: vec![],
        ty: arrow(cst("SynthType"), arrow(cst("SynthContext"), prop())),
    }
}
/// Build the `PBEConvergence` axiom declaration.
pub fn decl_pbe_convergence() -> Declaration {
    Declaration::Axiom {
        name: Name::str("PBEConvergence"),
        univ_params: vec![],
        ty: arrow(nat_ty(), arrow(list_ty(cst("IOExample")), prop())),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env() {
        let mut env = Environment::new();
        let result = build_program_synthesis_env(&mut env);
        assert!(
            result.is_ok(),
            "build_program_synthesis_env failed: {:?}",
            result
        );
    }
    #[test]
    fn test_spec_constraint_count() {
        let s1 = Spec::logic("y = x * 2");
        assert_eq!(s1.constraint_count(), 1);
        let s2 = Spec::from_examples(vec![
            (vec!["0".into()], "0".into()),
            (vec!["1".into()], "2".into()),
            (vec!["2".into()], "4".into()),
        ]);
        assert_eq!(s2.constraint_count(), 3);
        let conj = Spec::Conjunction(Box::new(s1), Box::new(s2.clone()));
        assert_eq!(conj.constraint_count(), 4);
        let disj = Spec::Disjunction(Box::new(s2.clone()), Box::new(Spec::logic("z = 0")));
        assert_eq!(disj.constraint_count(), 3);
    }
    #[test]
    fn test_cegis_state() {
        let spec = Spec::logic("y = x + 1");
        let mut state = CegisState::new(spec, 10);
        assert!(!state.is_solved());
        assert!(!state.is_exhausted());
        state.propose(Candidate::new("fun x -> x + 1"));
        assert!(state.is_solved());
        assert_eq!(state.iterations, 1);
        state.add_counterexample(vec!["0".into()]);
        assert!(!state.is_solved());
        assert_eq!(state.counterexamples.len(), 1);
    }
    #[test]
    fn test_cfg_productions() {
        let mut cfg = CFG::new("E");
        cfg.add_production("E", vec!["E".into(), "+".into(), "E".into()]);
        cfg.add_production("E", vec!["0".into()]);
        cfg.add_production("E", vec!["1".into()]);
        cfg.add_terminal("0");
        cfg.add_terminal("1");
        cfg.add_terminal("+");
        assert_eq!(cfg.productions_for("E").len(), 3);
        assert_eq!(cfg.terminals.len(), 3);
    }
    #[test]
    fn test_type_directed_synth_identity() {
        let alpha = SynthType::Var("α".into());
        let goal = SynthType::arrow(alpha.clone(), alpha.clone());
        let ctx = SynthContext::new();
        let mut synth = TypeDirectedSynth::new(3);
        let result = synth.synthesise(&ctx, &goal, 0);
        assert!(result.is_some(), "should synthesise identity: {:?}", result);
        let prog = result.expect("result should be valid");
        assert!(prog.contains("fun"), "should be a lambda: {}", prog);
    }
    #[test]
    fn test_sketch_fill_and_complete() {
        let holes = vec![Hole::new(0, "Nat"), Hole::new(1, "Nat")];
        let sketch = Sketch::new("fun x -> ??0 + ??1", holes);
        assert_eq!(sketch.num_holes(), 2);
        assert!(!sketch.is_complete());
        let s1 = sketch.fill_hole(0, "x");
        assert_eq!(s1.num_holes(), 1);
        let s2 = s1.fill_hole(1, "1");
        assert!(s2.is_complete());
        assert_eq!(s2.source, "fun x -> x + 1");
    }
    #[test]
    fn test_flashfill_identity() {
        let examples = vec![
            IOExample::new(vec!["hello".into()], "hello"),
            IOExample::new(vec!["world".into()], "world"),
        ];
        let synth = FlashFillSynth::new();
        let result = synth.synthesise(&examples);
        assert_eq!(result, Some("fun x -> x".into()));
    }
    #[test]
    fn test_oracle_table_query() {
        let mut oracle = TableOracle::new();
        oracle.insert(vec!["0".into()], "1");
        oracle.insert(vec!["1".into()], "2");
        let mut loop_ = OracleSynthLoop::new();
        let ans0 = loop_.query(&oracle, vec!["0".into()]);
        let ans1 = loop_.query(&oracle, vec!["1".into()]);
        let ans_miss = loop_.query(&oracle, vec!["99".into()]);
        assert_eq!(ans0, Some("1".into()));
        assert_eq!(ans1, Some("2".into()));
        assert_eq!(ans_miss, None);
        assert_eq!(loop_.num_queries(), 3);
    }
    #[test]
    fn test_bottom_up_enum_size1() {
        let synth = BottomUpSynth::new(
            5,
            vec!["x".into(), "y".into()],
            vec!["0".into(), "1".into()],
        );
        let progs = synth.enumerate_size(1);
        assert_eq!(progs.len(), 4);
    }
}
#[cfg(test)]
mod tests_program_synthesis_extended {
    use super::*;
    #[test]
    fn test_ilp_problem_examples() {
        let mut prob = ILPProblem::new("parent");
        prob.add_positive("parent(tom, bob)");
        prob.add_negative("parent(bob, ann)");
        prob.add_background("ancestor(X, Y) :- parent(X, Y)");
        assert_eq!(prob.total_examples(), 2);
        assert_eq!(prob.positive_examples.len(), 1);
    }
    #[test]
    fn test_foil_gain_calculation() {
        let learner = FoilLearner::new(5, 0.1);
        let gain = learner.foil_gain(10.0, 5.0, 5.0, 4.0, 1.0);
        assert!(gain > 0.0);
    }
    #[test]
    fn test_program_sketch_fill() {
        let sketch = ProgramSketch::new("x + {hole_0} * y + {hole_1}", 2);
        let filled = sketch.fill(&["2", "3"]);
        assert_eq!(filled, "x + 2 * y + 3");
        assert!(sketch.is_complete(&filled));
    }
    #[test]
    fn test_program_sketch_space_size() {
        let sketch = ProgramSketch::new("{hole_0} op {hole_1}", 2);
        assert_eq!(sketch.sketch_space_size(5), 25);
    }
    #[test]
    fn test_ogis_cegis_convergence() {
        let mut synth = OGISSynthesizer::new(10);
        synth.add_counter_example(vec![1, 2], 3);
        synth.add_counter_example(vec![2, 3], 5);
        assert_eq!(synth.cegis_iterations(), 2);
        assert!(!synth.has_converged());
        for i in 0..8 {
            synth.add_counter_example(vec![i], i * 2);
        }
        assert!(synth.has_converged());
    }
    #[test]
    fn test_foil_covering_clauses() {
        let learner = FoilLearner::new(10, 0.1);
        let clauses = learner.covering_clauses_needed(100);
        assert!(clauses <= 10);
    }
}
