//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use super::types::{
    AbstractInterpreter, BasicBlock, Closure, CompilerCorrectness, ContextFreeGrammar,
    DataFlowAnalysis, DataflowSolver, GrammarType, HMType, HindleyMilnerInference, LRParser,
    PushdownAutomaton, RegisterAllocation, RegisterColoringSimple, SSAConstructor, SSAForm,
    SignValue, TypedLambdaCalculus,
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
pub fn option_ty(elem: Expr) -> Expr {
    app(cst("Option"), elem)
}
pub fn pair_ty(a: Expr, b: Expr) -> Expr {
    app2(cst("Prod"), a, b)
}
pub fn grammar_type_ty() -> Expr {
    type0()
}
pub fn context_free_grammar_ty() -> Expr {
    type0()
}
pub fn pushdown_automaton_ty() -> Expr {
    type0()
}
pub fn lr_parser_ty() -> Expr {
    type0()
}
pub fn typed_lambda_calculus_ty() -> Expr {
    type0()
}
pub fn closure_ty() -> Expr {
    type0()
}
pub fn register_allocation_ty() -> Expr {
    type0()
}
pub fn data_flow_analysis_ty() -> Expr {
    type0()
}
pub fn ssa_form_ty() -> Expr {
    type0()
}
pub fn compiler_correctness_ty() -> Expr {
    type0()
}
pub fn chomsky_hierarchy_ty() -> Expr {
    arrow(grammar_type_ty(), nat_ty())
}
pub fn cfg_is_ambiguous_ty() -> Expr {
    arrow(context_free_grammar_ty(), bool_ty())
}
pub fn cfg_cnf_ty() -> Expr {
    arrow(context_free_grammar_ty(), context_free_grammar_ty())
}
pub fn cyk_parse_ty() -> Expr {
    arrow(
        context_free_grammar_ty(),
        arrow(list_ty(nat_ty()), bool_ty()),
    )
}
pub fn pda_accepts_ty() -> Expr {
    arrow(pushdown_automaton_ty(), arrow(list_ty(nat_ty()), bool_ty()))
}
pub fn lr_is_lr1_ty() -> Expr {
    arrow(lr_parser_ty(), bool_ty())
}
pub fn tlc_is_normalizing_ty() -> Expr {
    arrow(typed_lambda_calculus_ty(), bool_ty())
}
pub fn closure_convert_ty() -> Expr {
    arrow(closure_ty(), closure_ty())
}
pub fn reg_alloc_graph_color_ty() -> Expr {
    arrow(register_allocation_ty(), option_ty(list_ty(nat_ty())))
}
pub fn dataflow_worklist_ty() -> Expr {
    arrow(data_flow_analysis_ty(), bool_ty())
}
pub fn ssa_convert_ty() -> Expr {
    arrow(ssa_form_ty(), ssa_form_ty())
}
pub fn compiler_correctness_sim_ty() -> Expr {
    arrow(compiler_correctness_ty(), prop())
}
pub fn type_scheme_ty() -> Expr {
    type0()
}
pub fn substitution_ty() -> Expr {
    type0()
}
pub fn unification_problem_ty() -> Expr {
    type0()
}
pub fn type_env_ty() -> Expr {
    type0()
}
/// Hindley-Milner type inference: TypeEnv → Expr → Option TypeScheme
pub fn hindley_milner_infer_ty() -> Expr {
    arrow(type_env_ty(), arrow(type0(), option_ty(type_scheme_ty())))
}
/// Algorithm W: TypeEnv → Expr → Option (Substitution × Type)
pub fn algorithm_w_ty() -> Expr {
    arrow(
        type_env_ty(),
        arrow(type0(), option_ty(pair_ty(substitution_ty(), type0()))),
    )
}
/// Unification: two types → Option Substitution
pub fn unification_ty() -> Expr {
    arrow(type0(), arrow(type0(), option_ty(substitution_ty())))
}
/// Most general unifier: unification is complete
pub fn mgu_complete_ty() -> Expr {
    arrow(unification_problem_ty(), option_ty(substitution_ty()))
}
pub fn system_f_term_ty() -> Expr {
    type0()
}
pub fn rank_n_type_ty() -> Expr {
    type0()
}
/// System F type-checking: SystemFTerm → Bool
pub fn system_f_type_check_ty() -> Expr {
    arrow(system_f_term_ty(), bool_ty())
}
/// Rank-n polymorphism check: RankNType → Nat → Bool
pub fn rank_n_check_ty() -> Expr {
    arrow(rank_n_type_ty(), arrow(nat_ty(), bool_ty()))
}
/// Impredicativity: System F allows instantiation with polymorphic types
pub fn system_f_impredicative_ty() -> Expr {
    arrow(system_f_term_ty(), arrow(type0(), system_f_term_ty()))
}
pub fn domain_equation_ty() -> Expr {
    type0()
}
pub fn fixpoint_sem_ty() -> Expr {
    type0()
}
/// Domain equation solver: DomainEquation → Type
pub fn domain_eq_solve_ty() -> Expr {
    arrow(domain_equation_ty(), type0())
}
/// Fixpoint semantics: FixpointSem → (Type → Type) → Type
pub fn fixpoint_semantics_ty() -> Expr {
    arrow(fixpoint_sem_ty(), arrow(arrow(type0(), type0()), type0()))
}
/// Full abstraction: observational equivalence = denotational equality
pub fn full_abstraction_ty() -> Expr {
    arrow(fixpoint_sem_ty(), prop())
}
pub fn small_step_rel_ty() -> Expr {
    type0()
}
pub fn big_step_rel_ty() -> Expr {
    type0()
}
pub fn bisimulation_ty() -> Expr {
    type0()
}
/// Small-step step: SmallStepRel → Expr → Option Expr
pub fn small_step_ty() -> Expr {
    arrow(small_step_rel_ty(), arrow(type0(), option_ty(type0())))
}
/// Big-step eval: BigStepRel → Expr → Option Value
pub fn big_step_ty() -> Expr {
    arrow(big_step_rel_ty(), arrow(type0(), option_ty(type0())))
}
/// Bisimulation check: Bisimulation → Bool
pub fn bisimulation_check_ty() -> Expr {
    arrow(bisimulation_ty(), bool_ty())
}
/// Logical relation: two terms are logically related at a type
pub fn logical_relation_ty() -> Expr {
    arrow(type0(), arrow(type0(), arrow(type0(), prop())))
}
pub fn abstract_domain_ty() -> Expr {
    type0()
}
pub fn galois_connection_ty() -> Expr {
    type0()
}
/// Galois connection: AbstractDomain → (Expr → Expr) → Bool (monotone)
pub fn galois_connection_monotone_ty() -> Expr {
    arrow(galois_connection_ty(), prop())
}
/// Widening operator: AbstractDomain → AbstractDomain → AbstractDomain
pub fn widening_ty() -> Expr {
    arrow(
        abstract_domain_ty(),
        arrow(abstract_domain_ty(), abstract_domain_ty()),
    )
}
/// Narrowing operator: AbstractDomain → AbstractDomain → AbstractDomain
pub fn narrowing_ty() -> Expr {
    arrow(
        abstract_domain_ty(),
        arrow(abstract_domain_ty(), abstract_domain_ty()),
    )
}
/// Abstract fixpoint: (AbstractDomain → AbstractDomain) → AbstractDomain
pub fn abstract_fixpoint_ty() -> Expr {
    arrow(
        arrow(abstract_domain_ty(), abstract_domain_ty()),
        abstract_domain_ty(),
    )
}
pub fn call_graph_ty() -> Expr {
    type0()
}
pub fn cfa_result_ty() -> Expr {
    type0()
}
/// 0-CFA analysis: Program → CFAResult
pub fn zero_cfa_ty() -> Expr {
    arrow(type0(), cfa_result_ty())
}
/// k-CFA analysis: Nat → Program → CFAResult
pub fn k_cfa_ty() -> Expr {
    arrow(nat_ty(), arrow(type0(), cfa_result_ty()))
}
/// Points-to analysis: Program → (Var → Set Addr)
pub fn points_to_analysis_ty() -> Expr {
    arrow(type0(), arrow(type0(), list_ty(nat_ty())))
}
/// Call graph construction: Program → CallGraph
pub fn call_graph_build_ty() -> Expr {
    arrow(type0(), call_graph_ty())
}
pub fn reaching_def_ty() -> Expr {
    type0()
}
pub fn live_var_ty() -> Expr {
    type0()
}
pub fn avail_expr_ty() -> Expr {
    type0()
}
/// Reaching definitions: BasicBlock → ReachingDef
pub fn reaching_definitions_ty() -> Expr {
    arrow(type0(), reaching_def_ty())
}
/// Live variable analysis: BasicBlock → LiveVar
pub fn live_variables_ty() -> Expr {
    arrow(type0(), live_var_ty())
}
/// Available expressions: BasicBlock → AvailExpr
pub fn available_expressions_ty() -> Expr {
    arrow(type0(), avail_expr_ty())
}
pub fn phi_function_ty() -> Expr {
    type0()
}
pub fn dom_frontier_ty() -> Expr {
    type0()
}
/// Dominance frontier: CFG → DomFrontier
pub fn dominance_frontier_ty() -> Expr {
    arrow(type0(), dom_frontier_ty())
}
/// SSA construction: CFG → DomFrontier → SSAForm
pub fn ssa_construction_ty() -> Expr {
    arrow(type0(), arrow(dom_frontier_ty(), ssa_form_ty()))
}
/// Phi function placement: DomFrontier → List PhiFunction
pub fn phi_placement_ty() -> Expr {
    arrow(dom_frontier_ty(), list_ty(phi_function_ty()))
}
pub fn interference_graph_ty() -> Expr {
    type0()
}
pub fn spill_code_ty() -> Expr {
    type0()
}
/// Linear scan allocation: List LiveRange → Nat → Option (List Register)
pub fn linear_scan_alloc_ty() -> Expr {
    arrow(
        list_ty(pair_ty(nat_ty(), nat_ty())),
        arrow(nat_ty(), option_ty(list_ty(nat_ty()))),
    )
}
/// Spill code insertion: InterferenceGraph → SpillCode
pub fn spill_code_insert_ty() -> Expr {
    arrow(interference_graph_ty(), spill_code_ty())
}
pub fn tree_pattern_ty() -> Expr {
    type0()
}
pub fn instruction_ty() -> Expr {
    type0()
}
/// Tree tiling (BURS): TreePattern → Instruction
pub fn tree_tiling_ty() -> Expr {
    arrow(tree_pattern_ty(), instruction_ty())
}
/// Instruction selection: IR → List Instruction
pub fn instruction_selection_ty() -> Expr {
    arrow(type0(), list_ty(instruction_ty()))
}
pub fn basic_block_ty() -> Expr {
    type0()
}
pub fn dag_ty() -> Expr {
    type0()
}
/// Basic block scheduling: BasicBlock → BasicBlock
pub fn basic_block_schedule_ty() -> Expr {
    arrow(basic_block_ty(), basic_block_ty())
}
/// DAG scheduling: DAG → List Instruction
pub fn dag_schedule_ty() -> Expr {
    arrow(dag_ty(), list_ty(instruction_ty()))
}
pub fn inline_candidate_ty() -> Expr {
    type0()
}
pub fn loop_nest_ty() -> Expr {
    type0()
}
/// Inlining: InlineCandidate → IR → IR
pub fn inlining_ty() -> Expr {
    arrow(inline_candidate_ty(), arrow(type0(), type0()))
}
/// Loop transformation (unroll/tile): LoopNest → LoopNest
pub fn loop_transform_ty() -> Expr {
    arrow(loop_nest_ty(), loop_nest_ty())
}
/// Vectorization: LoopNest → Option (List Instruction)
pub fn vectorization_ty() -> Expr {
    arrow(loop_nest_ty(), option_ty(list_ty(instruction_ty())))
}
pub fn heap_ty() -> Expr {
    type0()
}
pub fn gc_roots_ty() -> Expr {
    type0()
}
/// Mark-sweep GC: Heap → GCRoots → Heap
pub fn mark_sweep_ty() -> Expr {
    arrow(heap_ty(), arrow(gc_roots_ty(), heap_ty()))
}
/// Copying GC: Heap → GCRoots → Heap
pub fn copying_gc_ty() -> Expr {
    arrow(heap_ty(), arrow(gc_roots_ty(), heap_ty()))
}
/// Generational GC: Nat (generations) → Heap → GCRoots → Heap
pub fn generational_gc_ty() -> Expr {
    arrow(nat_ty(), arrow(heap_ty(), arrow(gc_roots_ty(), heap_ty())))
}
/// Incremental GC: Bool (pause-free) → Heap → GCRoots → Heap
pub fn incremental_gc_ty() -> Expr {
    arrow(bool_ty(), arrow(heap_ty(), arrow(gc_roots_ty(), heap_ty())))
}
pub fn jit_state_ty() -> Expr {
    type0()
}
pub fn deopt_info_ty() -> Expr {
    type0()
}
/// On-stack replacement: JITState → Nat (point) → JITState
pub fn on_stack_replacement_ty() -> Expr {
    arrow(jit_state_ty(), arrow(nat_ty(), jit_state_ty()))
}
/// Deoptimization: JITState → DeoptInfo → JITState
pub fn deoptimization_ty() -> Expr {
    arrow(jit_state_ty(), arrow(deopt_info_ty(), jit_state_ty()))
}
/// Tracing JIT: JITState → List Instruction → JITState
pub fn tracing_jit_ty() -> Expr {
    arrow(
        jit_state_ty(),
        arrow(list_ty(instruction_ty()), jit_state_ty()),
    )
}
pub fn simulation_rel_ty() -> Expr {
    type0()
}
/// CompCert-style forward simulation: SimulationRel → Prop
pub fn forward_simulation_ty() -> Expr {
    arrow(simulation_rel_ty(), prop())
}
/// CakeML-style verified semantics preservation: IR → IR → Prop
pub fn semantics_preservation_ty() -> Expr {
    arrow(type0(), arrow(type0(), prop()))
}
pub fn build_compiler_theory_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("GrammarType", grammar_type_ty()),
        ("ContextFreeGrammar", context_free_grammar_ty()),
        ("PushdownAutomaton", pushdown_automaton_ty()),
        ("LRParser", lr_parser_ty()),
        ("TypedLambdaCalculus", typed_lambda_calculus_ty()),
        ("Closure", closure_ty()),
        ("RegisterAllocation", register_allocation_ty()),
        ("DataFlowAnalysis", data_flow_analysis_ty()),
        ("SSAForm", ssa_form_ty()),
        ("CompilerCorrectness", compiler_correctness_ty()),
        ("ChomskyHierarchy", chomsky_hierarchy_ty()),
        ("CFG_IsAmbiguous", cfg_is_ambiguous_ty()),
        ("CFG_ChomskyNormalForm", cfg_cnf_ty()),
        ("CYKParse", cyk_parse_ty()),
        ("PDA_Accepts", pda_accepts_ty()),
        ("LR_IsLR1", lr_is_lr1_ty()),
        ("TLC_IsNormalizing", tlc_is_normalizing_ty()),
        ("ClosureConvert", closure_convert_ty()),
        ("RegAlloc_GraphColor", reg_alloc_graph_color_ty()),
        ("DataFlow_Worklist", dataflow_worklist_ty()),
        ("SSA_Convert", ssa_convert_ty()),
        ("CompilerCorrectness_Sim", compiler_correctness_sim_ty()),
        ("TypeScheme", type_scheme_ty()),
        ("Substitution", substitution_ty()),
        ("UnificationProblem", unification_problem_ty()),
        ("TypeEnv", type_env_ty()),
        ("HindleyMilner_Infer", hindley_milner_infer_ty()),
        ("AlgorithmW", algorithm_w_ty()),
        ("Unification", unification_ty()),
        ("MGU_Complete", mgu_complete_ty()),
        ("SystemFTerm", system_f_term_ty()),
        ("RankNType", rank_n_type_ty()),
        ("SystemF_TypeCheck", system_f_type_check_ty()),
        ("RankN_Check", rank_n_check_ty()),
        ("SystemF_Impredicative", system_f_impredicative_ty()),
        ("DomainEquation", domain_equation_ty()),
        ("FixpointSem", fixpoint_sem_ty()),
        ("DomainEq_Solve", domain_eq_solve_ty()),
        ("Fixpoint_Semantics", fixpoint_semantics_ty()),
        ("FullAbstraction", full_abstraction_ty()),
        ("SmallStepRel", small_step_rel_ty()),
        ("BigStepRel", big_step_rel_ty()),
        ("Bisimulation", bisimulation_ty()),
        ("SmallStep", small_step_ty()),
        ("BigStep", big_step_ty()),
        ("Bisimulation_Check", bisimulation_check_ty()),
        ("LogicalRelation", logical_relation_ty()),
        ("AbstractDomain", abstract_domain_ty()),
        ("GaloisConnection", galois_connection_ty()),
        ("GaloisConnection_Monotone", galois_connection_monotone_ty()),
        ("Widening", widening_ty()),
        ("Narrowing", narrowing_ty()),
        ("AbstractFixpoint", abstract_fixpoint_ty()),
        ("CallGraph", call_graph_ty()),
        ("CFAResult", cfa_result_ty()),
        ("ZeroCFA", zero_cfa_ty()),
        ("KCFA", k_cfa_ty()),
        ("PointsTo_Analysis", points_to_analysis_ty()),
        ("CallGraph_Build", call_graph_build_ty()),
        ("ReachingDef", reaching_def_ty()),
        ("LiveVar", live_var_ty()),
        ("AvailExpr", avail_expr_ty()),
        ("ReachingDefinitions", reaching_definitions_ty()),
        ("LiveVariables", live_variables_ty()),
        ("AvailableExpressions", available_expressions_ty()),
        ("PhiFunction", phi_function_ty()),
        ("DomFrontier", dom_frontier_ty()),
        ("DominanceFrontier", dominance_frontier_ty()),
        ("SSA_Construction", ssa_construction_ty()),
        ("PhiPlacement", phi_placement_ty()),
        ("InterferenceGraph", interference_graph_ty()),
        ("SpillCode", spill_code_ty()),
        ("LinearScan_Alloc", linear_scan_alloc_ty()),
        ("SpillCode_Insert", spill_code_insert_ty()),
        ("TreePattern", tree_pattern_ty()),
        ("Instruction", instruction_ty()),
        ("TreeTiling", tree_tiling_ty()),
        ("InstructionSelection", instruction_selection_ty()),
        ("BasicBlock", basic_block_ty()),
        ("DAG", dag_ty()),
        ("BasicBlock_Schedule", basic_block_schedule_ty()),
        ("DAG_Schedule", dag_schedule_ty()),
        ("InlineCandidate", inline_candidate_ty()),
        ("LoopNest", loop_nest_ty()),
        ("Inlining", inlining_ty()),
        ("LoopTransform", loop_transform_ty()),
        ("Vectorization", vectorization_ty()),
        ("Heap", heap_ty()),
        ("GCRoots", gc_roots_ty()),
        ("MarkSweep", mark_sweep_ty()),
        ("CopyingGC", copying_gc_ty()),
        ("GenerationalGC", generational_gc_ty()),
        ("IncrementalGC", incremental_gc_ty()),
        ("JITState", jit_state_ty()),
        ("DeoptInfo", deopt_info_ty()),
        ("OnStackReplacement", on_stack_replacement_ty()),
        ("Deoptimization", deoptimization_ty()),
        ("TracingJIT", tracing_jit_ty()),
        ("SimulationRel", simulation_rel_ty()),
        ("ForwardSimulation", forward_simulation_ty()),
        ("SemanticPreservation", semantics_preservation_ty()),
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_env_nonempty() {
        let env = build_compiler_theory_env();
        let names = [
            "GrammarType",
            "ContextFreeGrammar",
            "PushdownAutomaton",
            "LRParser",
            "TypedLambdaCalculus",
            "Closure",
            "RegisterAllocation",
            "DataFlowAnalysis",
            "SSAForm",
            "CompilerCorrectness",
        ];
        let count = names
            .iter()
            .filter(|&&n| env.get(&Name::str(n)).is_some())
            .count();
        assert_eq!(count, 10);
    }
    #[test]
    fn test_grammar_type_chomsky() {
        assert_eq!(GrammarType::Regular.chomsky_hierarchy_level(), 3);
        assert_eq!(GrammarType::CFL.chomsky_hierarchy_level(), 2);
        assert_eq!(GrammarType::CSL.chomsky_hierarchy_level(), 1);
        assert_eq!(
            GrammarType::RecursivelyEnumerable.chomsky_hierarchy_level(),
            0
        );
    }
    #[test]
    fn test_grammar_type_closure_regular() {
        let props = GrammarType::Regular.closure_properties();
        assert!(props.contains(&"complement"));
        assert!(props.contains(&"union"));
    }
    #[test]
    fn test_cfg_cyk_parse_simple() {
        let cfg = ContextFreeGrammar::new(
            vec!['S', 'A', 'B'],
            vec!['a', 'b'],
            'S',
            vec![
                ('S', "AB".to_string()),
                ('A', "a".to_string()),
                ('B', "b".to_string()),
            ],
        );
        assert!(cfg.cyk_parse(&['a', 'b']));
        assert!(!cfg.cyk_parse(&['b', 'a']));
        assert!(!cfg.cyk_parse(&['a']));
    }
    #[test]
    fn test_cfg_is_ambiguous_false() {
        let cfg = ContextFreeGrammar::new(
            vec!['S'],
            vec!['a'],
            'S',
            vec![('S', "a".to_string()), ('S', "aa".to_string())],
        );
        assert!(!cfg.is_ambiguous());
    }
    #[test]
    fn test_cfg_is_ambiguous_true() {
        let cfg = ContextFreeGrammar::new(
            vec!['S'],
            vec!['a'],
            'S',
            vec![('S', "a".to_string()), ('S', "a".to_string())],
        );
        assert!(cfg.is_ambiguous());
    }
    #[test]
    fn test_cfg_cnf() {
        let cfg = ContextFreeGrammar::new(
            vec!['S'],
            vec!['a', 'b', 'c'],
            'S',
            vec![('S', "abc".to_string())],
        );
        let cnf = cfg.chomsky_normal_form();
        for (_, rhs) in &cnf.rules {
            assert!(rhs.chars().count() <= 2);
        }
    }
    #[test]
    fn test_pda_accepts() {
        let pda = PushdownAutomaton::new(2, vec!['Z'], vec![]);
        assert!(pda.accepts(&[]));
    }
    #[test]
    fn test_lr_parser_is_lr1_clean() {
        let action = vec![
            vec!["s1".to_string(), "".to_string()],
            vec!["acc".to_string(), "r1".to_string()],
        ];
        let goto = vec![vec![1usize], vec![0usize]];
        let parser = LRParser::new(2, action, goto);
        assert!(parser.is_lr1());
    }
    #[test]
    fn test_lr_parser_conflict() {
        let action = vec![vec!["s1,r2".to_string()]];
        let goto = vec![vec![0usize]];
        let parser = LRParser::new(1, action, goto);
        assert!(!parser.is_lr1());
    }
    #[test]
    fn test_typed_lambda_calculus() {
        assert!(TypedLambdaCalculus::SimplyTyped.is_normalizing());
        assert!(TypedLambdaCalculus::SimplyTyped.is_decidable());
        assert!(TypedLambdaCalculus::SystemF.is_normalizing());
        assert!(!TypedLambdaCalculus::SystemF.is_decidable());
        assert!(TypedLambdaCalculus::CoC.is_decidable());
    }
    #[test]
    fn test_closure_convert() {
        let c = Closure::new(
            vec!["x".to_string(), "y".to_string()],
            vec![("z".to_string(), "42".to_string())],
        );
        let converted = c.closure_convert();
        assert!(converted.free_vars.is_empty());
        assert!(converted.env.iter().any(|(k, _)| k == "x"));
        assert!(converted.env.iter().any(|(k, _)| k == "y"));
    }
    #[test]
    fn test_closure_lambda_lift() {
        let c = Closure::new(
            vec!["x".to_string()],
            vec![("y".to_string(), "1".to_string())],
        );
        let params = c.lambda_lift();
        assert!(params.contains(&"x".to_string()));
        assert!(params.contains(&"y".to_string()));
    }
    #[test]
    fn test_register_allocation_graph_color() {
        let ra = RegisterAllocation::new(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec![(0, 2), (3, 5), (6, 8)],
            2,
        );
        let coloring = ra.graph_color();
        assert!(coloring.is_some());
        let c = coloring.expect("coloring should be valid");
        assert_eq!(c.len(), 3);
    }
    #[test]
    fn test_register_allocation_spill_needed() {
        let ra = RegisterAllocation::new(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec![(0, 10), (0, 10), (0, 10)],
            2,
        );
        let coloring = ra.graph_color();
        assert!(coloring.is_none());
    }
    #[test]
    fn test_dataflow_fixed_point() {
        let df = DataFlowAnalysis::new("forward", "powerset");
        let result = df.fixed_point(0u32, |&v| if v < 5 { v + 1 } else { v });
        assert_eq!(result, 5);
    }
    #[test]
    fn test_dataflow_worklist() {
        let df = DataFlowAnalysis::new("forward", "interval");
        let edges = vec![vec![1usize], vec![2], vec![]];
        let init = vec![1u32, 0, 0];
        let result = df.worklist_algorithm(3, &edges, init, |_, v| *v, |a, b| (*a).max(*b));
        assert_eq!(result[2], 1);
    }
    #[test]
    fn test_ssa_form_convert() {
        let ssa = SSAForm::new(
            vec!["x".to_string(), "x".to_string(), "y".to_string()],
            vec![("x".to_string(), vec!["x0".to_string(), "x1".to_string()])],
        );
        let converted = ssa.convert_to_ssa();
        assert!(converted.variables.iter().any(|v| v.contains('_')));
        let frontier = converted.dominance_frontier();
        assert!(!frontier.is_empty());
    }
    #[test]
    fn test_compiler_correctness() {
        let cc = CompilerCorrectness::new("denotational_sem", "operational_sem");
        assert!(cc.observable_equivalence());
        let sim = cc.simulation_relation();
        assert!(sim.contains("denotational_sem"));
        assert!(sim.contains("operational_sem"));
    }
    #[test]
    fn test_spill_cost() {
        let ra = RegisterAllocation::new(
            vec!["a".to_string(), "b".to_string()],
            vec![(0, 4), (1, 3)],
            4,
        );
        let costs = ra.spill_cost();
        assert_eq!(costs[0], 5.0);
        assert_eq!(costs[1], 3.0);
    }
    #[test]
    fn test_build_env_new_axioms() {
        let env = build_compiler_theory_env();
        let new_names = [
            "TypeScheme",
            "Substitution",
            "HindleyMilner_Infer",
            "AlgorithmW",
            "Unification",
            "MGU_Complete",
            "SystemFTerm",
            "RankNType",
            "SystemF_TypeCheck",
            "DomainEquation",
            "FixpointSem",
            "FullAbstraction",
            "SmallStepRel",
            "BigStepRel",
            "Bisimulation",
            "LogicalRelation",
            "GaloisConnection",
            "Widening",
            "Narrowing",
            "AbstractFixpoint",
            "ZeroCFA",
            "KCFA",
            "PointsTo_Analysis",
            "ReachingDefinitions",
            "LiveVariables",
            "AvailableExpressions",
            "DominanceFrontier",
            "SSA_Construction",
            "PhiPlacement",
            "LinearScan_Alloc",
            "TreeTiling",
            "InstructionSelection",
            "BasicBlock_Schedule",
            "DAG_Schedule",
            "Inlining",
            "LoopTransform",
            "Vectorization",
            "MarkSweep",
            "CopyingGC",
            "GenerationalGC",
            "IncrementalGC",
            "OnStackReplacement",
            "Deoptimization",
            "TracingJIT",
            "ForwardSimulation",
            "SemanticPreservation",
        ];
        let count = new_names
            .iter()
            .filter(|&&n| env.get(&Name::str(n)).is_some())
            .count();
        assert_eq!(count, new_names.len());
    }
    #[test]
    fn test_hm_infer_simple_types() {
        let mut hm = HindleyMilnerInference::new();
        assert_eq!(hm.infer_simple("int"), HMType::Int);
        assert_eq!(hm.infer_simple("bool"), HMType::Bool);
        match hm.infer_simple("fun") {
            HMType::Fun(_, _) => {}
            other => panic!("expected Fun, got {:?}", other),
        }
    }
    #[test]
    fn test_hm_unify_base_types() {
        let mut hm = HindleyMilnerInference::new();
        assert!(hm.unify(&HMType::Int, &HMType::Int).is_ok());
        assert!(hm.unify(&HMType::Bool, &HMType::Bool).is_ok());
        assert!(hm.unify(&HMType::Int, &HMType::Bool).is_err());
    }
    #[test]
    fn test_hm_unify_var() {
        let mut hm = HindleyMilnerInference::new();
        let v = hm.fresh();
        assert!(hm.unify(&v, &HMType::Int).is_ok());
        let resolved = hm.resolve(&v);
        assert_eq!(resolved, HMType::Int);
    }
    #[test]
    fn test_hm_unify_fun() {
        let mut hm = HindleyMilnerInference::new();
        let t1 = HMType::Fun(Box::new(HMType::Int), Box::new(HMType::Bool));
        let t2 = HMType::Fun(Box::new(HMType::Int), Box::new(HMType::Bool));
        assert!(hm.unify(&t1, &t2).is_ok());
    }
    #[test]
    fn test_hm_occurs_check() {
        let mut hm = HindleyMilnerInference::new();
        let v = hm.fresh();
        let v_idx = match &v {
            HMType::Var(i) => *i,
            _ => panic!(),
        };
        let recursive_ty = HMType::Fun(Box::new(v.clone()), Box::new(v.clone()));
        assert!(hm.unify(&HMType::Var(v_idx), &recursive_ty).is_err());
    }
    #[test]
    fn test_abstract_interpreter_assign_lookup() {
        let mut ai = AbstractInterpreter::new();
        ai.assign("x", SignValue::Positive);
        assert_eq!(ai.lookup("x"), SignValue::Positive);
        assert_eq!(ai.lookup("y"), SignValue::Top);
    }
    #[test]
    fn test_sign_join() {
        assert_eq!(
            SignValue::Positive.join(&SignValue::Positive),
            SignValue::Positive
        );
        assert_eq!(
            SignValue::Bottom.join(&SignValue::Negative),
            SignValue::Negative
        );
        assert_eq!(
            SignValue::Positive.join(&SignValue::Negative),
            SignValue::Top
        );
        assert_eq!(SignValue::Zero.join(&SignValue::Bottom), SignValue::Zero);
    }
    #[test]
    fn test_sign_add() {
        assert_eq!(
            SignValue::Positive.add(&SignValue::Positive),
            SignValue::Positive
        );
        assert_eq!(
            SignValue::Negative.add(&SignValue::Negative),
            SignValue::Negative
        );
        assert_eq!(
            SignValue::Zero.add(&SignValue::Positive),
            SignValue::Positive
        );
        assert_eq!(
            SignValue::Positive.add(&SignValue::Negative),
            SignValue::Top
        );
    }
    #[test]
    fn test_sign_mul() {
        assert_eq!(
            SignValue::Positive.mul(&SignValue::Positive),
            SignValue::Positive
        );
        assert_eq!(
            SignValue::Negative.mul(&SignValue::Negative),
            SignValue::Positive
        );
        assert_eq!(
            SignValue::Positive.mul(&SignValue::Negative),
            SignValue::Negative
        );
        assert_eq!(SignValue::Zero.mul(&SignValue::Top), SignValue::Zero);
    }
    #[test]
    fn test_abstract_interpreter_join_state() {
        let mut ai1 = AbstractInterpreter::new();
        ai1.assign("x", SignValue::Positive);
        let mut ai2 = AbstractInterpreter::new();
        ai2.assign("x", SignValue::Negative);
        ai2.assign("y", SignValue::Zero);
        let joined = ai1.join_state(&ai2);
        assert_eq!(joined.lookup("x"), SignValue::Top);
        assert_eq!(joined.lookup("y"), SignValue::Zero);
    }
    #[test]
    fn test_abstract_interpreter_widen() {
        let mut ai1 = AbstractInterpreter::new();
        ai1.assign("x", SignValue::Positive);
        let mut ai2 = AbstractInterpreter::new();
        ai2.assign("x", SignValue::Negative);
        let widened = ai1.widen(&ai2);
        assert_eq!(widened.lookup("x"), SignValue::Top);
    }
    #[test]
    fn test_dataflow_solver_forward_simple() {
        let solver = DataflowSolver::new(
            3,
            vec![vec![1], vec![2], vec![]],
            vec![
                [0].iter().copied().collect(),
                [1].iter().copied().collect(),
                HashSet::new(),
            ],
            vec![
                HashSet::new(),
                [0].iter().copied().collect(),
                HashSet::new(),
            ],
        );
        let (_, out) = solver.solve_forward();
        assert!(out[0].contains(&0));
        assert!(out[1].contains(&1));
        assert!(!out[1].contains(&0));
        assert!(out[2].contains(&1));
    }
    #[test]
    fn test_dataflow_solver_backward_simple() {
        let solver = DataflowSolver::new(
            2,
            vec![vec![1], vec![]],
            vec![HashSet::new(), [5].iter().copied().collect()],
            vec![HashSet::new(), HashSet::new()],
        );
        let (in_sets, _) = solver.solve_backward();
        assert!(in_sets[0].contains(&5));
    }
    #[test]
    fn test_reg_coloring_no_interference() {
        let rc = RegisterColoringSimple::new(3, &[], 1);
        let coloring = rc.color();
        assert!(coloring.is_some());
        let c = coloring.expect("coloring should be valid");
        assert_eq!(c, vec![0, 0, 0]);
    }
    #[test]
    fn test_reg_coloring_triangle_3regs() {
        let rc = RegisterColoringSimple::new(3, &[(0, 1), (1, 2), (0, 2)], 3);
        let coloring = rc.color();
        assert!(coloring.is_some());
        let c = coloring.expect("coloring should be valid");
        assert_ne!(c[0], c[1]);
        assert_ne!(c[1], c[2]);
        assert_ne!(c[0], c[2]);
    }
    #[test]
    fn test_reg_coloring_triangle_2regs_impossible() {
        let rc = RegisterColoringSimple::new(3, &[(0, 1), (1, 2), (0, 2)], 2);
        assert!(rc.color().is_none());
    }
    #[test]
    fn test_reg_coloring_clique_lower_bound() {
        let rc =
            RegisterColoringSimple::new(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)], 4);
        assert!(rc.clique_lower_bound() >= 4);
    }
    #[test]
    fn test_ssa_constructor_all_vars() {
        let blocks = vec![
            BasicBlock::new(0, vec!["x".to_string(), "y".to_string()], vec![], vec![1]),
            BasicBlock::new(1, vec!["z".to_string()], vec!["x".to_string()], vec![]),
        ];
        let ctor = SSAConstructor::new(blocks);
        let vars = ctor.all_vars();
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert!(vars.contains("z"));
    }
    #[test]
    fn test_ssa_constructor_rename() {
        let blocks = vec![BasicBlock::new(
            0,
            vec!["x".to_string(), "x".to_string()],
            vec![],
            vec![],
        )];
        let ctor = SSAConstructor::new(blocks);
        let renaming = ctor.rename_variables();
        assert_eq!(renaming.get(&(0, "x".to_string())), Some(&1));
    }
    #[test]
    fn test_ssa_constructor_phi_insertion() {
        let blocks = vec![
            BasicBlock::new(0, vec!["x".to_string()], vec![], vec![1, 2]),
            BasicBlock::new(1, vec![], vec!["x".to_string()], vec![3]),
            BasicBlock::new(2, vec![], vec!["x".to_string()], vec![3]),
            BasicBlock::new(3, vec![], vec!["x".to_string()], vec![]),
        ];
        let ctor = SSAConstructor::new(blocks);
        let phi_pts = ctor.phi_insertion_points();
        let _ = phi_pts;
    }
    #[test]
    fn test_ssa_constructor_dominance_frontier() {
        let blocks = vec![
            BasicBlock::new(0, vec![], vec![], vec![1]),
            BasicBlock::new(1, vec![], vec![], vec![2]),
            BasicBlock::new(2, vec![], vec![], vec![]),
        ];
        let ctor = SSAConstructor::new(blocks);
        let df = ctor.dominance_frontier_map();
        let _ = df;
    }
}
