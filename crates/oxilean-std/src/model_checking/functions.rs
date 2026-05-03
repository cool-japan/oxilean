//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    AbstractDomain, AbstractTransformer, AtomicProposition, BDDManager, BDDModelChecker,
    BuchiAutomaton, CounterExample, CounterExampleGuidedRefinement, CtlFormula, CtlModelChecker,
    CtlStarFormula, KripkeStructure, LtlFormula, LtlModelChecker, MuCalculusEvaluator, MuFormula,
    ParityGameZielonka, ProbabilisticMCVerifier, SpuriousCounterexample, StateLabel,
    SymbolicTransitionRelation, BDD,
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
/// KripkeStructure: (S, S_0, R, L)
pub fn kripke_structure_ty() -> Expr {
    type0()
}
/// AtomicProposition: boolean-valued proposition on states
pub fn atomic_proposition_ty() -> Expr {
    type0()
}
/// StateLabel: set of atomic propositions true in a state
pub fn state_label_ty() -> Expr {
    arrow(cst("State"), type0())
}
/// reachable_states: the set of states reachable from initial states
pub fn reachable_states_ty() -> Expr {
    arrow(cst("KripkeStructure"), app(cst("List"), cst("State")))
}
/// is_connected: every state is reachable from some initial state
pub fn is_connected_ty() -> Expr {
    arrow(cst("KripkeStructure"), prop())
}
/// compute_scc: Tarjan's SCC decomposition
pub fn compute_scc_ty() -> Expr {
    arrow(
        cst("KripkeStructure"),
        app(cst("List"), app(cst("List"), cst("State"))),
    )
}
/// LTL formula type
pub fn ltl_formula_ty() -> Expr {
    type0()
}
/// CTL formula type
pub fn ctl_formula_ty() -> Expr {
    type0()
}
/// CTL* formula type
pub fn ctl_star_formula_ty() -> Expr {
    type0()
}
/// ltl_is_safety: φ is a safety property
pub fn ltl_is_safety_ty() -> Expr {
    arrow(cst("LtlFormula"), prop())
}
/// ltl_is_liveness: φ is a liveness property
pub fn ltl_is_liveness_ty() -> Expr {
    arrow(cst("LtlFormula"), prop())
}
/// ltl_is_fairness: φ is a fairness constraint
pub fn ltl_is_fairness_ty() -> Expr {
    arrow(cst("LtlFormula"), prop())
}
/// LtlModelChecker: automaton-theoretic LTL checking
pub fn ltl_model_checker_ty() -> Expr {
    type0()
}
/// CtlModelChecker: fixpoint computation for CTL
pub fn ctl_model_checker_ty() -> Expr {
    type0()
}
/// CounterExample: trace witnessing formula violation
pub fn counter_example_ty() -> Expr {
    type0()
}
/// BuchiAutomaton: (Q, Σ, δ, q_0, F) ω-automaton
pub fn buchi_automaton_ty() -> Expr {
    type0()
}
/// check_ltl: check whether K ⊨ φ (LTL)
pub fn check_ltl_ty() -> Expr {
    arrow(cst("KripkeStructure"), arrow(cst("LtlFormula"), bool_ty()))
}
/// check_ctl: check whether K ⊨ φ (CTL)
pub fn check_ctl_ty() -> Expr {
    arrow(cst("KripkeStructure"), arrow(cst("CtlFormula"), bool_ty()))
}
/// find_counterexample: produce a counterexample trace if formula fails
pub fn find_counterexample_ty() -> Expr {
    arrow(
        cst("KripkeStructure"),
        arrow(cst("LtlFormula"), app(cst("Option"), cst("CounterExample"))),
    )
}
/// BDD: Binary Decision Diagram node
pub fn bdd_ty() -> Expr {
    type0()
}
/// BDDManager: unique table + apply cache
pub fn bdd_manager_ty() -> Expr {
    type0()
}
/// SymbolicTransitionRelation: T(s,s') as BDD
pub fn symbolic_transition_relation_ty() -> Expr {
    type0()
}
/// image: compute the forward image of a set of states
pub fn image_ty() -> Expr {
    arrow(
        cst("BDDManager"),
        arrow(
            cst("BDD"),
            arrow(cst("SymbolicTransitionRelation"), cst("BDD")),
        ),
    )
}
/// pre_image: compute the backward image of a set of states
pub fn pre_image_ty() -> Expr {
    arrow(
        cst("BDDManager"),
        arrow(
            cst("BDD"),
            arrow(cst("SymbolicTransitionRelation"), cst("BDD")),
        ),
    )
}
/// AbstractDomain: predicate abstraction / interval / octagon domain
pub fn abstract_domain_ty() -> Expr {
    type0()
}
/// AbstractTransformer: post[τ](α(S))
pub fn abstract_transformer_ty() -> Expr {
    arrow(
        cst("AbstractDomain"),
        arrow(cst("AbstractDomain"), cst("AbstractDomain")),
    )
}
/// CounterExampleGuidedRefinement: CEGAR loop
pub fn cegar_ty() -> Expr {
    type0()
}
/// SpuriousCounterexample: infeasible concrete path
pub fn spurious_counterexample_ty() -> Expr {
    type0()
}
/// abstract_states: map concrete states to abstract domain
pub fn abstract_states_ty() -> Expr {
    arrow(app(cst("List"), cst("State")), cst("AbstractDomain"))
}
/// refine_abstraction: refine abstract domain using a spurious counterexample
pub fn refine_abstraction_ty() -> Expr {
    arrow(
        cst("AbstractDomain"),
        arrow(cst("SpuriousCounterexample"), cst("AbstractDomain")),
    )
}
/// check_feasibility: determine if a counterexample is spurious
pub fn check_feasibility_ty() -> Expr {
    arrow(cst("CounterExample"), bool_ty())
}
/// MuFormula: a μ-calculus formula type
pub fn mu_formula_ty() -> Expr {
    type0()
}
/// mu_fixpoint: the least fixpoint operator μX.φ(X)
/// mu_fixpoint : (State → Prop) → (State → Prop)
pub fn mu_fixpoint_ty() -> Expr {
    arrow(arrow(cst("State"), prop()), arrow(cst("State"), prop()))
}
/// nu_fixpoint: the greatest fixpoint operator νX.φ(X)
/// nu_fixpoint : (State → Prop) → (State → Prop)
pub fn nu_fixpoint_ty() -> Expr {
    arrow(arrow(cst("State"), prop()), arrow(cst("State"), prop()))
}
/// check_mu: model check a μ-calculus formula
/// check_mu : KripkeStructure → MuFormula → Bool
pub fn check_mu_ty() -> Expr {
    arrow(cst("KripkeStructure"), arrow(cst("MuFormula"), bool_ty()))
}
/// AlternatingTuringMachine: ATM used in alternation-based μ-calculus model checking
pub fn alternating_turing_machine_ty() -> Expr {
    type0()
}
/// ALC: the description logic ALC (attribute language with complement)
pub fn alc_concept_ty() -> Expr {
    type0()
}
/// ParityGame: a game graph with priority function
pub fn parity_game_ty() -> Expr {
    type0()
}
/// ParityCondition: ω-winning condition: the highest priority seen inf-often is even
/// ParityCondition : (Nat → Bool) → Prop
pub fn parity_condition_ty() -> Expr {
    arrow(arrow(nat_ty(), bool_ty()), prop())
}
/// ZielonkaSolver: Zielonka's recursive parity game algorithm
/// ZielonkaSolver : ParityGame → Bool
pub fn zielonka_solver_ty() -> Expr {
    arrow(cst("ParityGame"), bool_ty())
}
/// parity_game_winner: which player wins from a given vertex?
/// parity_game_winner : ParityGame → Nat → Bool
pub fn parity_game_winner_ty() -> Expr {
    arrow(cst("ParityGame"), arrow(nat_ty(), bool_ty()))
}
/// mu_calculus_parity_reduction: reduction of μ-calc. to parity games
/// mu_calculus_parity_reduction : MuFormula → ParityGame
pub fn mu_calculus_parity_reduction_ty() -> Expr {
    arrow(cst("MuFormula"), cst("ParityGame"))
}
/// SatSolver: a SAT/SMT oracle used in bounded model checking
pub fn sat_solver_ty() -> Expr {
    type0()
}
/// BoundedMCQuery: a bounded model checking problem (BMC)
/// BoundedMCQuery : KripkeStructure → LtlFormula → Nat → Bool
pub fn bounded_mc_query_ty() -> Expr {
    arrow(
        cst("KripkeStructure"),
        arrow(cst("LtlFormula"), arrow(nat_ty(), bool_ty())),
    )
}
/// KInductionResult: result of a k-induction proof step
pub fn k_induction_result_ty() -> Expr {
    type0()
}
/// k_induction_check: run k-induction for LTL safety properties
/// k_induction_check : KripkeStructure → LtlFormula → Nat → KInductionResult
pub fn k_induction_check_ty() -> Expr {
    arrow(
        cst("KripkeStructure"),
        arrow(cst("LtlFormula"), arrow(nat_ty(), cst("KInductionResult"))),
    )
}
/// ProbabilisticKripke: a Markov Decision Process / Markov chain
pub fn probabilistic_kripke_ty() -> Expr {
    type0()
}
/// PCTLFormula: a PCTL (probabilistic CTL) formula type
pub fn pctl_formula_ty() -> Expr {
    type0()
}
/// check_pctl: check whether M ⊨ φ (PCTL)
/// check_pctl : ProbabilisticKripke → PCTLFormula → Bool
pub fn check_pctl_ty() -> Expr {
    arrow(
        cst("ProbabilisticKripke"),
        arrow(cst("PCTLFormula"), bool_ty()),
    )
}
/// reachability_probability: P\[reach(T) from s\] in a Markov chain
/// reachability_probability : ProbabilisticKripke → State → Set State → Real
pub fn reachability_probability_ty() -> Expr {
    arrow(
        cst("ProbabilisticKripke"),
        arrow(
            cst("State"),
            arrow(app(cst("Set"), cst("State")), cst("Real")),
        ),
    )
}
/// TimedAutomaton: automaton with clock variables
pub fn timed_automaton_ty() -> Expr {
    type0()
}
/// TCTLFormula: a timed CTL formula
pub fn tctl_formula_ty() -> Expr {
    type0()
}
/// ZoneGraph: zone-based abstract state space for timed systems
pub fn zone_graph_ty() -> Expr {
    type0()
}
/// check_tctl: verify a timed CTL formula over a timed automaton
/// check_tctl : TimedAutomaton → TCTLFormula → Bool
pub fn check_tctl_ty() -> Expr {
    arrow(cst("TimedAutomaton"), arrow(cst("TCTLFormula"), bool_ty()))
}
/// zone_reachability: compute the reachable zone graph of a timed automaton
/// zone_reachability : TimedAutomaton → ZoneGraph
pub fn zone_reachability_ty() -> Expr {
    arrow(cst("TimedAutomaton"), cst("ZoneGraph"))
}
/// HybridAutomaton: automaton with continuous flow conditions
pub fn hybrid_automaton_ty() -> Expr {
    type0()
}
/// FlowCondition: ODE describing continuous evolution in a mode
/// FlowCondition : Type → Prop
pub fn flow_condition_ty() -> Expr {
    arrow(type0(), prop())
}
/// GuardRegion: a polyhedral region enabling a discrete transition
/// GuardRegion : Type → Prop
pub fn guard_region_ty() -> Expr {
    arrow(type0(), prop())
}
/// HybridReachability: reachable set of a hybrid system
/// HybridReachability : HybridAutomaton → Set Type
pub fn hybrid_reachability_ty() -> Expr {
    arrow(cst("HybridAutomaton"), app(cst("Set"), type0()))
}
/// PushdownSystem: a recursive program modeled as a pushdown automaton
pub fn pushdown_system_ty() -> Expr {
    type0()
}
/// ContextFreeLTL: an LTL formula interpreted over pushdown system runs
pub fn context_free_ltl_ty() -> Expr {
    type0()
}
/// check_pushdown_ltl: model check a pushdown system against an LTL formula
/// check_pushdown_ltl : PushdownSystem → LtlFormula → Bool
pub fn check_pushdown_ltl_ty() -> Expr {
    arrow(cst("PushdownSystem"), arrow(cst("LtlFormula"), bool_ty()))
}
/// pushdown_reachability: backwards reachability in a pushdown system
/// pushdown_reachability : PushdownSystem → Set State
pub fn pushdown_reachability_ty() -> Expr {
    arrow(cst("PushdownSystem"), app(cst("Set"), cst("State")))
}
/// HigherOrderRecursionScheme: a HORS defining a tree language
pub fn hors_ty() -> Expr {
    type0()
}
/// HORSModelChecking: model check a HORS against an MSO property
/// HORSModelChecking : HigherOrderRecursionScheme → MuFormula → Bool
pub fn hors_model_checking_ty() -> Expr {
    arrow(
        cst("HigherOrderRecursionScheme"),
        arrow(cst("MuFormula"), bool_ty()),
    )
}
/// CraigInterpolant: a formula I with A ⊨ I and I ∧ B unsatisfiable
/// CraigInterpolant : LtlFormula → LtlFormula → LtlFormula → Prop
pub fn craig_interpolant_ty() -> Expr {
    arrow(
        cst("LtlFormula"),
        arrow(cst("LtlFormula"), arrow(cst("LtlFormula"), prop())),
    )
}
/// lazy_cegar: CEGAR with lazy abstraction refinement
/// lazy_cegar : KripkeStructure → LtlFormula → Bool
pub fn lazy_cegar_ty() -> Expr {
    arrow(cst("KripkeStructure"), arrow(cst("LtlFormula"), bool_ty()))
}
/// AssumeGuaranteeContract: an AG specification (A, G)
pub fn assume_guarantee_contract_ty() -> Expr {
    type0()
}
/// ag_decomposition: decompose a verification task using A-G reasoning
/// ag_decomposition : KripkeStructure → AssumeGuaranteeContract → Bool
pub fn ag_decomposition_ty() -> Expr {
    arrow(
        cst("KripkeStructure"),
        arrow(cst("AssumeGuaranteeContract"), bool_ty()),
    )
}
/// interface_verification: check compatibility of component interfaces
/// interface_verification : List AssumeGuaranteeContract → Bool
pub fn interface_verification_ty() -> Expr {
    arrow(app(cst("List"), cst("AssumeGuaranteeContract")), bool_ty())
}
/// MazurkiewiczTrace: an equivalence class of runs under independence
pub fn mazurkiewicz_trace_ty() -> Expr {
    type0()
}
/// PersistentSet: a persistent set for partial order reduction
/// PersistentSet : KripkeStructure → State → Set (State → State) → Prop
pub fn persistent_set_ty() -> Expr {
    arrow(
        cst("KripkeStructure"),
        arrow(
            cst("State"),
            arrow(app(cst("Set"), arrow(cst("State"), cst("State"))), prop()),
        ),
    )
}
/// AmpleSet: an ample set satisfying C0–C3 for POR
/// AmpleSet : KripkeStructure → State → Set (State → State) → Prop
pub fn ample_set_ty() -> Expr {
    arrow(
        cst("KripkeStructure"),
        arrow(
            cst("State"),
            arrow(app(cst("Set"), arrow(cst("State"), cst("State"))), prop()),
        ),
    )
}
/// por_reduction: apply partial order reduction to a Kripke structure
/// por_reduction : KripkeStructure → KripkeStructure
pub fn por_reduction_ty() -> Expr {
    arrow(cst("KripkeStructure"), cst("KripkeStructure"))
}
/// PSLFormula: a PSL (Property Specification Language) formula
pub fn psl_formula_ty() -> Expr {
    type0()
}
/// SVAFormula: a SystemVerilog assertion formula
pub fn sva_formula_ty() -> Expr {
    type0()
}
/// check_psl: check a PSL formula against a Kripke structure
/// check_psl : KripkeStructure → PSLFormula → Bool
pub fn check_psl_ty() -> Expr {
    arrow(cst("KripkeStructure"), arrow(cst("PSLFormula"), bool_ty()))
}
/// TemporalLogicPattern: a reusable specification pattern (Dwyer patterns)
pub fn temporal_logic_pattern_ty() -> Expr {
    type0()
}
/// Register all model checking axioms into the kernel environment.
pub fn build_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("KripkeStructure", kripke_structure_ty()),
        ("AtomicProposition", atomic_proposition_ty()),
        ("StateLabel", state_label_ty()),
        ("State", type0()),
        ("reachable_states", reachable_states_ty()),
        ("is_connected", is_connected_ty()),
        ("compute_scc", compute_scc_ty()),
        ("LtlFormula", ltl_formula_ty()),
        ("CtlFormula", ctl_formula_ty()),
        ("CtlStarFormula", ctl_star_formula_ty()),
        ("ltl_is_safety", ltl_is_safety_ty()),
        ("ltl_is_liveness", ltl_is_liveness_ty()),
        ("ltl_is_fairness", ltl_is_fairness_ty()),
        ("LtlModelChecker", ltl_model_checker_ty()),
        ("CtlModelChecker", ctl_model_checker_ty()),
        ("CounterExample", counter_example_ty()),
        ("BuchiAutomaton", buchi_automaton_ty()),
        ("Option", arrow(type0(), type0())),
        ("check_ltl", check_ltl_ty()),
        ("check_ctl", check_ctl_ty()),
        ("find_counterexample", find_counterexample_ty()),
        ("BDD", bdd_ty()),
        ("BDDManager", bdd_manager_ty()),
        (
            "SymbolicTransitionRelation",
            symbolic_transition_relation_ty(),
        ),
        ("image", image_ty()),
        ("pre_image", pre_image_ty()),
        ("AbstractDomain", abstract_domain_ty()),
        ("AbstractTransformer", abstract_transformer_ty()),
        ("CounterExampleGuidedRefinement", cegar_ty()),
        ("SpuriousCounterexample", spurious_counterexample_ty()),
        ("abstract_states", abstract_states_ty()),
        ("refine_abstraction", refine_abstraction_ty()),
        ("check_feasibility", check_feasibility_ty()),
        ("MuFormula", mu_formula_ty()),
        ("mu_fixpoint", mu_fixpoint_ty()),
        ("nu_fixpoint", nu_fixpoint_ty()),
        ("check_mu", check_mu_ty()),
        ("AlternatingTuringMachine", alternating_turing_machine_ty()),
        ("ALCConcept", alc_concept_ty()),
        ("ParityGame", parity_game_ty()),
        ("ParityCondition", parity_condition_ty()),
        ("ZielonkaSolver", zielonka_solver_ty()),
        ("parity_game_winner", parity_game_winner_ty()),
        (
            "mu_calculus_parity_reduction",
            mu_calculus_parity_reduction_ty(),
        ),
        ("SatSolver", sat_solver_ty()),
        ("BoundedMCQuery", bounded_mc_query_ty()),
        ("KInductionResult", k_induction_result_ty()),
        ("k_induction_check", k_induction_check_ty()),
        ("ProbabilisticKripke", probabilistic_kripke_ty()),
        ("PCTLFormula", pctl_formula_ty()),
        ("check_pctl", check_pctl_ty()),
        ("reachability_probability", reachability_probability_ty()),
        ("TimedAutomaton", timed_automaton_ty()),
        ("TCTLFormula", tctl_formula_ty()),
        ("ZoneGraph", zone_graph_ty()),
        ("check_tctl", check_tctl_ty()),
        ("zone_reachability", zone_reachability_ty()),
        ("HybridAutomaton", hybrid_automaton_ty()),
        ("FlowCondition", flow_condition_ty()),
        ("GuardRegion", guard_region_ty()),
        ("HybridReachability", hybrid_reachability_ty()),
        ("PushdownSystem", pushdown_system_ty()),
        ("ContextFreeLTL", context_free_ltl_ty()),
        ("check_pushdown_ltl", check_pushdown_ltl_ty()),
        ("pushdown_reachability", pushdown_reachability_ty()),
        ("HigherOrderRecursionScheme", hors_ty()),
        ("HORSModelChecking", hors_model_checking_ty()),
        ("CraigInterpolant", craig_interpolant_ty()),
        ("lazy_cegar", lazy_cegar_ty()),
        ("AssumeGuaranteeContract", assume_guarantee_contract_ty()),
        ("ag_decomposition", ag_decomposition_ty()),
        ("interface_verification", interface_verification_ty()),
        ("MazurkiewiczTrace", mazurkiewicz_trace_ty()),
        ("PersistentSet", persistent_set_ty()),
        ("AmpleSet", ample_set_ty()),
        ("por_reduction", por_reduction_ty()),
        ("PSLFormula", psl_formula_ty()),
        ("SVAFormula", sva_formula_ty()),
        ("check_psl", check_psl_ty()),
        ("TemporalLogicPattern", temporal_logic_pattern_ty()),
        ("Real", cst("Real")),
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
mod new_impl_tests {
    use super::*;
    fn small_kripke() -> KripkeStructure {
        let mut k = KripkeStructure::new(3);
        k.add_initial(0);
        k.add_transition(0, 1);
        k.add_transition(1, 2);
        k.add_transition(2, 0);
        k.label_state(0, "p");
        k.label_state(1, "q");
        k
    }
    #[test]
    fn test_mu_calculus_evaluator_true() {
        let k = small_kripke();
        let mc = MuCalculusEvaluator::new(k);
        let f = MuFormula::True_;
        assert!(mc.check(&f));
    }
    #[test]
    fn test_mu_calculus_evaluator_prop() {
        let k = small_kripke();
        let mc = MuCalculusEvaluator::new(k);
        let f = MuFormula::Prop("p".to_string());
        assert!(mc.check(&f));
    }
    #[test]
    fn test_mu_calculus_evaluator_diamond() {
        let k = small_kripke();
        let mc = MuCalculusEvaluator::new(k);
        let f = MuFormula::Diamond(Box::new(MuFormula::Prop("q".to_string())));
        assert!(mc.check(&f));
    }
    #[test]
    fn test_mu_calculus_evaluator_nu_box() {
        let k = small_kripke();
        let mc = MuCalculusEvaluator::new(k.clone());
        let f = MuFormula::Nu("X".to_string(), Box::new(MuFormula::True_));
        let mut env = HashMap::new();
        let sat = mc.eval(&f, &mut env);
        assert_eq!(sat.len(), k.num_states);
    }
    #[test]
    fn test_parity_game_zielonka_trivial() {
        let mut pg = ParityGameZielonka::new(1);
        pg.set_priority(0, 0);
        pg.set_owner(0, 1);
        pg.add_edge(0, 0);
        let (w0, _w1) = pg.solve();
        assert!(w0.contains(&0));
    }
    #[test]
    fn test_parity_game_zielonka_two_nodes() {
        let mut pg = ParityGameZielonka::new(2);
        pg.set_priority(0, 1);
        pg.set_priority(1, 2);
        pg.set_owner(0, 0);
        pg.set_owner(1, 1);
        pg.add_edge(0, 1);
        pg.add_edge(1, 0);
        let (w0, _w1) = pg.solve();
        assert!(!w0.is_empty() || pg.player0_wins(0));
    }
    #[test]
    fn test_bdd_model_checker_new() {
        let mut bmc = BDDModelChecker::new(2);
        let t = bmc.mgr.true_node();
        let f = bmc.mgr.false_node();
        bmc.set_init(t);
        bmc.set_trans(f);
        let reach = bmc.reachable();
        assert_eq!(reach, t);
        assert!(bmc.check_ag_safe(t));
        assert!(!bmc.check_ef(f));
    }
    #[test]
    fn test_bdd_model_checker_variable() {
        let mut bmc = BDDModelChecker::new(2);
        let v0 = bmc.mgr.var(0);
        let v1 = bmc.mgr.var(1);
        let combined = bmc.mgr.bdd_and(v0, v1);
        assert!(combined < bmc.mgr.nodes.len());
    }
    #[test]
    fn test_probabilistic_mc_verifier() {
        let mut mc = ProbabilisticMCVerifier::new(3);
        mc.set_initial(0, 1.0);
        mc.add_transition(0, 1, 0.6);
        mc.add_transition(0, 2, 0.4);
        mc.add_transition(1, 1, 1.0);
        mc.add_transition(2, 2, 1.0);
        mc.label_state(1, "good");
        mc.label_state(2, "bad");
        let target: HashSet<usize> = [1].iter().copied().collect();
        let prob = mc.reachability_prob(&target);
        assert!((prob[0] - 0.6).abs() < 1e-6);
        assert!(mc.check_prob_reach("good", 0.5));
        assert!(!mc.check_prob_reach("good", 0.7));
    }
    #[test]
    fn test_mc_env_new_axioms() {
        let mut env = Environment::new();
        build_env(&mut env);
        assert!(env.get(&Name::str("MuFormula")).is_some());
        assert!(env.get(&Name::str("mu_fixpoint")).is_some());
        assert!(env.get(&Name::str("check_mu")).is_some());
        assert!(env.get(&Name::str("ParityGame")).is_some());
        assert!(env.get(&Name::str("ZielonkaSolver")).is_some());
        assert!(env
            .get(&Name::str("mu_calculus_parity_reduction"))
            .is_some());
        assert!(env.get(&Name::str("BoundedMCQuery")).is_some());
        assert!(env.get(&Name::str("k_induction_check")).is_some());
        assert!(env.get(&Name::str("PCTLFormula")).is_some());
        assert!(env.get(&Name::str("reachability_probability")).is_some());
        assert!(env.get(&Name::str("TimedAutomaton")).is_some());
        assert!(env.get(&Name::str("zone_reachability")).is_some());
        assert!(env.get(&Name::str("HybridAutomaton")).is_some());
        assert!(env.get(&Name::str("HybridReachability")).is_some());
        assert!(env.get(&Name::str("PushdownSystem")).is_some());
        assert!(env.get(&Name::str("check_pushdown_ltl")).is_some());
        assert!(env.get(&Name::str("HigherOrderRecursionScheme")).is_some());
        assert!(env.get(&Name::str("CraigInterpolant")).is_some());
        assert!(env.get(&Name::str("lazy_cegar")).is_some());
        assert!(env.get(&Name::str("AssumeGuaranteeContract")).is_some());
        assert!(env.get(&Name::str("interface_verification")).is_some());
        assert!(env.get(&Name::str("MazurkiewiczTrace")).is_some());
        assert!(env.get(&Name::str("AmpleSet")).is_some());
        assert!(env.get(&Name::str("por_reduction")).is_some());
        assert!(env.get(&Name::str("PSLFormula")).is_some());
        assert!(env.get(&Name::str("SVAFormula")).is_some());
        assert!(env.get(&Name::str("check_psl")).is_some());
    }
}
