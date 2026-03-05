//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    BddNode, BuchiAutomaton, CtlChecker, CtlFormula, FairnessConstraint, LtlFormula, MuFormula,
    ParityGame, StreettAutomaton, TransitionSystem,
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
/// TransitionSystem: (S, S_0, R, AP, L)
pub fn transition_system_ty() -> Expr {
    type0()
}
/// State: an element of the state space.
pub fn state_ty() -> Expr {
    type0()
}
/// TransitionRelation: S → S → Prop
pub fn transition_relation_ty() -> Expr {
    arrow(cst("State"), arrow(cst("State"), prop()))
}
/// Labeling: S → 2^AP (atomic propositions true at a state)
pub fn labeling_ty() -> Expr {
    arrow(cst("State"), arrow(nat_ty(), bool_ty()))
}
/// Path: an infinite sequence of states (ω-path)
pub fn path_ty() -> Expr {
    arrow(nat_ty(), cst("State"))
}
/// FairPath: a path satisfying a fairness condition
pub fn fair_path_ty() -> Expr {
    arrow(cst("FairnessConstraint"), arrow(path_ty(), prop()))
}
/// LtlFormula: syntax of LTL
pub fn ltl_formula_ty() -> Expr {
    type0()
}
/// LtlNext: X φ — φ holds at the next position
pub fn ltl_next_ty() -> Expr {
    arrow(cst("LtlFormula"), cst("LtlFormula"))
}
/// LtlFinally: F φ — φ holds at some future position
pub fn ltl_finally_ty() -> Expr {
    arrow(cst("LtlFormula"), cst("LtlFormula"))
}
/// LtlGlobally: G φ — φ holds at all future positions
pub fn ltl_globally_ty() -> Expr {
    arrow(cst("LtlFormula"), cst("LtlFormula"))
}
/// LtlUntil: φ U ψ — φ holds until ψ
pub fn ltl_until_ty() -> Expr {
    arrow(
        cst("LtlFormula"),
        arrow(cst("LtlFormula"), cst("LtlFormula")),
    )
}
/// LtlRelease: φ R ψ — ψ holds until/unless φ (dual of Until)
pub fn ltl_release_ty() -> Expr {
    arrow(
        cst("LtlFormula"),
        arrow(cst("LtlFormula"), cst("LtlFormula")),
    )
}
/// LtlWeakUntil: φ W ψ — weak until (φ U ψ ∨ G φ)
pub fn ltl_weak_until_ty() -> Expr {
    arrow(
        cst("LtlFormula"),
        arrow(cst("LtlFormula"), cst("LtlFormula")),
    )
}
/// LtlSat: π, i ⊨ φ (LTL satisfaction at position i)
pub fn ltl_sat_ty() -> Expr {
    arrow(path_ty(), arrow(nat_ty(), arrow(cst("LtlFormula"), prop())))
}
/// LtlModelSat: M ⊨ φ (all paths of M satisfy φ)
pub fn ltl_model_sat_ty() -> Expr {
    arrow(cst("TransitionSystem"), arrow(cst("LtlFormula"), prop()))
}
/// CtlFormula: syntax of CTL
pub fn ctl_formula_ty() -> Expr {
    type0()
}
/// CTL path quantifiers + temporal operators
pub fn ctl_ex_ty() -> Expr {
    arrow(cst("CtlFormula"), cst("CtlFormula"))
}
pub fn ctl_ef_ty() -> Expr {
    arrow(cst("CtlFormula"), cst("CtlFormula"))
}
pub fn ctl_eg_ty() -> Expr {
    arrow(cst("CtlFormula"), cst("CtlFormula"))
}
pub fn ctl_ax_ty() -> Expr {
    arrow(cst("CtlFormula"), cst("CtlFormula"))
}
pub fn ctl_af_ty() -> Expr {
    arrow(cst("CtlFormula"), cst("CtlFormula"))
}
pub fn ctl_ag_ty() -> Expr {
    arrow(cst("CtlFormula"), cst("CtlFormula"))
}
/// CTL EU: E[φ U ψ]
pub fn ctl_eu_ty() -> Expr {
    arrow(
        cst("CtlFormula"),
        arrow(cst("CtlFormula"), cst("CtlFormula")),
    )
}
/// CTL AU: A[φ U ψ]
pub fn ctl_au_ty() -> Expr {
    arrow(
        cst("CtlFormula"),
        arrow(cst("CtlFormula"), cst("CtlFormula")),
    )
}
/// CtlSat: M, s ⊨ φ
pub fn ctl_sat_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("State"), arrow(cst("CtlFormula"), prop())),
    )
}
/// CtlStarFormula: unifies LTL (path formulas) and CTL (state formulas)
pub fn ctl_star_formula_ty() -> Expr {
    type0()
}
/// CtlStarPathFormula: formula evaluated over paths
pub fn ctl_star_path_ty() -> Expr {
    type0()
}
/// CtlStarSat: M, π, i ⊨ f (state formula) / M, π, i ⊨ p (path formula)
pub fn ctl_star_sat_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("CtlStarFormula"), prop()),
    )
}
/// BuchiAutomaton: (Q, Σ, δ, q_0, F) — nondeterministic Büchi automaton
pub fn buchi_automaton_ty() -> Expr {
    type0()
}
/// BuchiRun: an infinite run of a Büchi automaton
pub fn buchi_run_ty() -> Expr {
    arrow(nat_ty(), cst("State"))
}
/// BuchiAccepting: a run is accepting if it visits F infinitely often
pub fn buchi_accepting_ty() -> Expr {
    arrow(cst("BuchiAutomaton"), arrow(buchi_run_ty(), prop()))
}
/// BuchiLanguage: the ω-language recognized by a Büchi automaton
pub fn buchi_language_ty() -> Expr {
    arrow(cst("BuchiAutomaton"), arrow(path_ty(), prop()))
}
/// GeneralizedBuchi: accepting condition is a set of sets of states
pub fn generalized_buchi_ty() -> Expr {
    type0()
}
/// DeterministicRabin: deterministic Rabin automaton (for complementation)
pub fn rabin_automaton_ty() -> Expr {
    type0()
}
/// CtlModelChecker: explicit-state CTL checking via fixpoints
pub fn ctl_model_checker_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("CtlFormula"), arrow(cst("State"), bool_ty())),
    )
}
/// LtlModelChecker: automata-based LTL model checking
pub fn ltl_model_checker_ty() -> Expr {
    arrow(cst("TransitionSystem"), arrow(cst("LtlFormula"), bool_ty()))
}
/// Counterexample: a witness path violating a property
pub fn counterexample_ty() -> Expr {
    type0()
}
/// PreImage: pre_R(S) = {s | ∃ s', sRs' ∧ s' ∈ S}
pub fn pre_image_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(
            arrow(cst("State"), bool_ty()),
            arrow(cst("State"), bool_ty()),
        ),
    )
}
/// PostImage: post_R(S) = {s' | ∃ s, sRs' ∧ s ∈ S}
pub fn post_image_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(
            arrow(cst("State"), bool_ty()),
            arrow(cst("State"), bool_ty()),
        ),
    )
}
/// BDD: binary decision diagram
pub fn bdd_ty() -> Expr {
    type0()
}
/// BDDManager: manages BDD variables and operations
pub fn bdd_manager_ty() -> Expr {
    type0()
}
/// SymbolicTransitionRelation: R(s, s') encoded as a BDD
pub fn symbolic_relation_ty() -> Expr {
    arrow(cst("BDDManager"), cst("BDD"))
}
/// SymbolicReach: symbolic reachability — lfp (λS. S_0 ∪ post(S))
pub fn symbolic_reach_ty() -> Expr {
    arrow(cst("TransitionSystem"), cst("BDD"))
}
/// MuFormula: formula in the modal mu-calculus
pub fn mu_formula_ty() -> Expr {
    type0()
}
/// LeastFixpoint: μX.φ(X)
pub fn least_fixpoint_ty() -> Expr {
    arrow(arrow(cst("MuFormula"), cst("MuFormula")), cst("MuFormula"))
}
/// GreatestFixpoint: νX.φ(X)
pub fn greatest_fixpoint_ty() -> Expr {
    arrow(arrow(cst("MuFormula"), cst("MuFormula")), cst("MuFormula"))
}
/// MuSat: M, s ⊨ f (mu-calculus satisfaction)
pub fn mu_sat_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("State"), arrow(cst("MuFormula"), prop())),
    )
}
/// AlternationDepth: the alternation depth of a mu-formula
pub fn alternation_depth_ty() -> Expr {
    arrow(cst("MuFormula"), nat_ty())
}
/// AfMuCalculus: alternation-free fragment
pub fn af_mu_calculus_ty() -> Expr {
    arrow(cst("MuFormula"), prop())
}
/// FairnessConstraint: a set of states that must be visited infinitely often
pub fn fairness_constraint_ty() -> Expr {
    type0()
}
/// StrongFairness: ∀ fair c, inf-often enabled → inf-often taken
pub fn strong_fairness_ty() -> Expr {
    arrow(cst("FairnessConstraint"), prop())
}
/// WeakFairness: ∀ fair c, eventually always enabled → inf-often taken
pub fn weak_fairness_ty() -> Expr {
    arrow(cst("FairnessConstraint"), prop())
}
/// FairCtlSat: M ⊨_fair φ (CTL under fairness constraints)
pub fn fair_ctl_sat_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("FairnessConstraint"), arrow(cst("CtlFormula"), prop())),
    )
}
/// ParityGame: (V, V_0, V_1, E, Ω) — two-player parity game
pub fn parity_game_ty() -> Expr {
    type0()
}
/// ParityCondition: Ω : V → ℕ — priority function
pub fn parity_condition_ty() -> Expr {
    arrow(cst("State"), nat_ty())
}
/// Player0Wins: player 0 wins from a vertex v
pub fn player0_wins_ty() -> Expr {
    arrow(cst("ParityGame"), arrow(cst("State"), prop()))
}
/// PositionalDeterminacy: parity games are determined by positional strategies
pub fn positional_determinacy_ty() -> Expr {
    prop()
}
/// Strategy: σ : V_0 → V — positional strategy for player 0
pub fn strategy_ty() -> Expr {
    arrow(cst("State"), cst("State"))
}
/// WinningStrategy: a strategy that guarantees winning from all vertices in W
pub fn winning_strategy_ty() -> Expr {
    arrow(cst("ParityGame"), arrow(strategy_ty(), prop()))
}
/// ConcurrentGameStructure: (Ag, S, Act, d, δ, L)
pub fn concurrent_game_ty() -> Expr {
    type0()
}
/// AtlFormula: formula of ATL
pub fn atl_formula_ty() -> Expr {
    type0()
}
/// AtlCoopX: ⟪A⟫X φ — coalition A can enforce X φ
pub fn atl_coop_x_ty() -> Expr {
    arrow(
        cst("AgentCoalition"),
        arrow(cst("AtlFormula"), cst("AtlFormula")),
    )
}
/// AtlCoopF: ⟪A⟫F φ — coalition A can enforce F φ
pub fn atl_coop_f_ty() -> Expr {
    arrow(
        cst("AgentCoalition"),
        arrow(cst("AtlFormula"), cst("AtlFormula")),
    )
}
/// AtlCoopG: ⟪A⟫G φ — coalition A can enforce G φ
pub fn atl_coop_g_ty() -> Expr {
    arrow(
        cst("AgentCoalition"),
        arrow(cst("AtlFormula"), cst("AtlFormula")),
    )
}
/// AtlCoopU: ⟪A⟫[φ U ψ] — coalition A can enforce φ U ψ
pub fn atl_coop_u_ty() -> Expr {
    arrow(
        cst("AgentCoalition"),
        arrow(
            cst("AtlFormula"),
            arrow(cst("AtlFormula"), cst("AtlFormula")),
        ),
    )
}
/// AtlSat: CGS, s ⊨ φ (ATL satisfaction)
pub fn atl_sat_ty() -> Expr {
    arrow(
        cst("ConcurrentGameStructure"),
        arrow(cst("State"), arrow(cst("AtlFormula"), prop())),
    )
}
/// SafetyProperty: a property P is a safety property if every violation has a finite prefix
pub fn safety_property_ty() -> Expr {
    arrow(arrow(path_ty(), prop()), prop())
}
/// LivenessProperty: every finite prefix can be extended to satisfy P
pub fn liveness_property_ty() -> Expr {
    arrow(arrow(path_ty(), prop()), prop())
}
/// BuchiSafety: P is safety iff its complement is recognized by a reachability automaton
pub fn buchi_safety_ty() -> Expr {
    arrow(cst("BuchiAutomaton"), prop())
}
/// BuchiLiveness: P is liveness iff every prefix can be extended to an accepting run
pub fn buchi_liveness_ty() -> Expr {
    arrow(cst("BuchiAutomaton"), prop())
}
/// FairnessLiveness: liveness under fairness = fair liveness
pub fn fairness_liveness_ty() -> Expr {
    arrow(cst("FairnessConstraint"), arrow(cst("LtlFormula"), prop()))
}
/// Populate an `Environment` with all temporal logic axioms and theorems.
pub fn build_temporal_logic_env(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("TransitionSystem", transition_system_ty()),
        ("State", state_ty()),
        ("TransitionRelation", transition_relation_ty()),
        ("Labeling", labeling_ty()),
        ("Path", path_ty()),
        ("FairnessConstraint", fairness_constraint_ty()),
        ("FairPath", fair_path_ty()),
        ("LtlFormula", ltl_formula_ty()),
        ("LtlNext", ltl_next_ty()),
        ("LtlFinally", ltl_finally_ty()),
        ("LtlGlobally", ltl_globally_ty()),
        ("LtlUntil", ltl_until_ty()),
        ("LtlRelease", ltl_release_ty()),
        ("LtlWeakUntil", ltl_weak_until_ty()),
        ("LtlSat", ltl_sat_ty()),
        ("LtlModelSat", ltl_model_sat_ty()),
        ("CtlFormula", ctl_formula_ty()),
        ("CtlEX", ctl_ex_ty()),
        ("CtlEF", ctl_ef_ty()),
        ("CtlEG", ctl_eg_ty()),
        ("CtlAX", ctl_ax_ty()),
        ("CtlAF", ctl_af_ty()),
        ("CtlAG", ctl_ag_ty()),
        ("CtlEU", ctl_eu_ty()),
        ("CtlAU", ctl_au_ty()),
        ("CtlSat", ctl_sat_ty()),
        ("CtlStarFormula", ctl_star_formula_ty()),
        ("CtlStarPath", ctl_star_path_ty()),
        ("CtlStarSat", ctl_star_sat_ty()),
        ("BuchiAutomaton", buchi_automaton_ty()),
        ("BuchiRun", buchi_run_ty()),
        ("BuchiAccepting", buchi_accepting_ty()),
        ("BuchiLanguage", buchi_language_ty()),
        ("GeneralizedBuchi", generalized_buchi_ty()),
        ("RabinAutomaton", rabin_automaton_ty()),
        ("CtlModelChecker", ctl_model_checker_ty()),
        ("LtlModelChecker", ltl_model_checker_ty()),
        ("Counterexample", counterexample_ty()),
        ("PreImage", pre_image_ty()),
        ("PostImage", post_image_ty()),
        ("BDD", bdd_ty()),
        ("BDDManager", bdd_manager_ty()),
        ("SymbolicRelation", symbolic_relation_ty()),
        ("SymbolicReach", symbolic_reach_ty()),
        ("MuFormula", mu_formula_ty()),
        ("LeastFixpoint", least_fixpoint_ty()),
        ("GreatestFixpoint", greatest_fixpoint_ty()),
        ("MuSat", mu_sat_ty()),
        ("AlternationDepth", alternation_depth_ty()),
        ("AfMuCalculus", af_mu_calculus_ty()),
        ("StrongFairness", strong_fairness_ty()),
        ("WeakFairness", weak_fairness_ty()),
        ("FairCtlSat", fair_ctl_sat_ty()),
        ("ParityGame", parity_game_ty()),
        ("ParityCondition", parity_condition_ty()),
        ("Player0Wins", player0_wins_ty()),
        ("PositionalDeterminacy", positional_determinacy_ty()),
        ("Strategy", strategy_ty()),
        ("WinningStrategy", winning_strategy_ty()),
        ("ConcurrentGameStructure", concurrent_game_ty()),
        ("AtlFormula", atl_formula_ty()),
        ("AgentCoalition", type0()),
        ("AtlCoopX", atl_coop_x_ty()),
        ("AtlCoopF", atl_coop_f_ty()),
        ("AtlCoopG", atl_coop_g_ty()),
        ("AtlCoopU", atl_coop_u_ty()),
        ("AtlSat", atl_sat_ty()),
        ("SafetyProperty", safety_property_ty()),
        ("LivenessProperty", liveness_property_ty()),
        ("BuchiSafety", buchi_safety_ty()),
        ("BuchiLiveness", buchi_liveness_ty()),
        ("FairnessLiveness", fairness_liveness_ty()),
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
/// Atomic proposition identifier.
pub type AtomId = u32;
/// A nondeterministic Büchi automaton state.
pub type BuchiState = u32;
/// Variable in the mu-calculus.
pub type MuVar = String;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ctl_checker_ef() {
        let mut ts = TransitionSystem::new(3);
        ts.add_initial(0);
        ts.add_transition(0, 1);
        ts.add_transition(1, 2);
        ts.add_label(2, 0);
        let checker = CtlChecker::new(&ts);
        let ef_p = CtlFormula::ef(CtlFormula::Atom(0));
        let sat = checker.sat(&ef_p);
        assert!(sat.contains(&0));
        assert!(sat.contains(&1));
        assert!(sat.contains(&2));
        assert!(checker.check(&ef_p));
    }
    #[test]
    fn test_ctl_checker_ag() {
        let mut ts = TransitionSystem::new(2);
        ts.add_initial(0);
        ts.add_transition(0, 1);
        ts.add_transition(1, 0);
        ts.add_label(0, 0);
        ts.add_label(1, 0);
        let checker = CtlChecker::new(&ts);
        let ag_p = CtlFormula::ag(CtlFormula::Atom(0));
        assert!(checker.check(&ag_p));
    }
    #[test]
    fn test_ctl_checker_au() {
        let mut ts = TransitionSystem::new(3);
        ts.add_initial(0);
        ts.add_transition(0, 1);
        ts.add_transition(1, 2);
        ts.add_label(0, 0);
        ts.add_label(1, 0);
        ts.add_label(2, 1);
        let checker = CtlChecker::new(&ts);
        let au = CtlFormula::AU(Box::new(CtlFormula::Atom(0)), Box::new(CtlFormula::Atom(1)));
        assert!(checker.check(&au));
    }
    #[test]
    fn test_ltl_formula_nnf() {
        let gp = LtlFormula::globally(LtlFormula::Atom(0));
        let neg_gp = LtlFormula::Not(Box::new(gp));
        let nnf = neg_gp.nnf();
        assert_eq!(
            nnf,
            LtlFormula::Finally(Box::new(LtlFormula::Not(Box::new(LtlFormula::Atom(0)))))
        );
    }
    #[test]
    fn test_transition_system_reachability() {
        let mut ts = TransitionSystem::new(4);
        ts.add_initial(0);
        ts.add_transition(0, 1);
        ts.add_transition(1, 2);
        let reachable = ts.reachable_states();
        assert!(reachable.contains(&0));
        assert!(reachable.contains(&1));
        assert!(reachable.contains(&2));
        assert!(!reachable.contains(&3));
    }
    #[test]
    fn test_mu_formula_alternation_depth() {
        let ag_true = MuFormula::ag(MuFormula::True);
        assert_eq!(ag_true.alternation_depth(), 0);
        let ef_true = MuFormula::ef(MuFormula::True);
        assert_eq!(ef_true.alternation_depth(), 0);
    }
    #[test]
    fn test_bdd_eval() {
        let bdd = BddNode::Node(
            0,
            Box::new(BddNode::Zero),
            Box::new(BddNode::Node(
                1,
                Box::new(BddNode::Zero),
                Box::new(BddNode::One),
            )),
        );
        let mut assign_tt = HashMap::new();
        assign_tt.insert(0u32, true);
        assign_tt.insert(1u32, true);
        assert!(bdd.eval(&assign_tt));
        let mut assign_tf = HashMap::new();
        assign_tf.insert(0u32, true);
        assign_tf.insert(1u32, false);
        assert!(!bdd.eval(&assign_tf));
    }
    #[test]
    fn test_parity_game_solver() {
        let mut game = ParityGame::new();
        let v0 = game.add_vertex(0, 0);
        game.add_edge(v0, v0);
        let (w0, w1) = game.solve();
        assert!(w0.contains(&v0));
        assert!(!w1.contains(&v0));
    }
    #[test]
    fn test_build_temporal_logic_env() {
        let mut env = Environment::new();
        build_temporal_logic_env(&mut env);
        assert!(env.get(&Name::str("LtlFormula")).is_some());
        assert!(env.get(&Name::str("CtlFormula")).is_some());
        assert!(env.get(&Name::str("BuchiAutomaton")).is_some());
        assert!(env.get(&Name::str("MuFormula")).is_some());
        assert!(env.get(&Name::str("ParityGame")).is_some());
        assert!(env.get(&Name::str("AtlFormula")).is_some());
    }
    #[test]
    fn test_register_temporal_logic_extended() {
        let mut env = Environment::new();
        build_temporal_logic_env(&mut env);
        let result = register_temporal_logic_extended(&mut env);
        assert!(result.is_ok());
        assert!(env.get(&Name::str("TctlFormula")).is_some());
        assert!(env.get(&Name::str("MtlFormula")).is_some());
        assert!(env.get(&Name::str("BmcInstance")).is_some());
    }
}
/// TctlFormula: Timed CTL formula (with time bounds on path quantifiers)
pub fn tctl_formula_ty() -> Expr {
    type0()
}
/// TctlEFBounded: EF_[a,b] φ — exists path reaching φ within interval [a,b]
pub fn tl_ext_tctl_ef_bounded_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(cst("TctlFormula"), cst("TctlFormula"))),
    )
}
/// TctlAGBounded: AG_[a,b] φ — all paths satisfy φ throughout interval [a,b]
pub fn tl_ext_tctl_ag_bounded_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(cst("TctlFormula"), cst("TctlFormula"))),
    )
}
/// TctlSat: M, s, t ⊨ φ (TCTL satisfaction at real time t)
pub fn tl_ext_tctl_sat_ty() -> Expr {
    arrow(
        cst("TimedTransitionSystem"),
        arrow(
            cst("State"),
            arrow(nat_ty(), arrow(cst("TctlFormula"), prop())),
        ),
    )
}
/// MtlFormula: Metric Temporal Logic formula
pub fn mtl_formula_ty() -> Expr {
    type0()
}
/// MtlUntilBounded: φ U_[a,b] ψ — metric bounded until
pub fn tl_ext_mtl_until_bounded_ty() -> Expr {
    arrow(
        cst("MtlFormula"),
        arrow(
            cst("MtlFormula"),
            arrow(nat_ty(), arrow(nat_ty(), cst("MtlFormula"))),
        ),
    )
}
/// MtlFinally: F_[a,b] φ — metric finally
pub fn tl_ext_mtl_finally_bounded_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(cst("MtlFormula"), cst("MtlFormula"))),
    )
}
/// MtlGlobally: G_[a,b] φ — metric globally
pub fn tl_ext_mtl_globally_bounded_ty() -> Expr {
    arrow(
        nat_ty(),
        arrow(nat_ty(), arrow(cst("MtlFormula"), cst("MtlFormula"))),
    )
}
/// StlFormula: Signal Temporal Logic formula (over continuous signals)
pub fn tl_ext_stl_formula_ty() -> Expr {
    type0()
}
/// StlSignal: a real-valued signal (time → ℝ, approximated as Nat → Nat)
pub fn tl_ext_stl_signal_ty() -> Expr {
    arrow(nat_ty(), nat_ty())
}
/// StlSat: (x, t) ⊨ φ — signal x at time t satisfies STL formula φ
pub fn tl_ext_stl_sat_ty() -> Expr {
    arrow(
        tl_ext_stl_signal_ty(),
        arrow(nat_ty(), arrow(cst("StlFormula"), prop())),
    )
}
/// StlRobustness: ρ(φ, x, t) — quantitative robustness of φ w.r.t. x at t
pub fn tl_ext_stl_robustness_ty() -> Expr {
    arrow(
        cst("StlFormula"),
        arrow(tl_ext_stl_signal_ty(), arrow(nat_ty(), nat_ty())),
    )
}
/// ItlFormula: Interval Temporal Logic formula
pub fn tl_ext_itl_formula_ty() -> Expr {
    type0()
}
/// ItlChop: φ ; ψ — interval split (chop operator)
pub fn tl_ext_itl_chop_ty() -> Expr {
    arrow(
        cst("ItlFormula"),
        arrow(cst("ItlFormula"), cst("ItlFormula")),
    )
}
/// ItlProjection: φ ↓ — projection to sub-interval
pub fn tl_ext_itl_projection_ty() -> Expr {
    arrow(cst("ItlFormula"), cst("ItlFormula"))
}
/// ItlSat: [i,j] ⊨ φ — interval [i,j] satisfies φ
pub fn tl_ext_itl_sat_ty() -> Expr {
    arrow(nat_ty(), arrow(nat_ty(), arrow(cst("ItlFormula"), prop())))
}
/// LtlToGba: translate LTL formula to Generalized Büchi Automaton
pub fn tl_ext_ltl_to_gba_ty() -> Expr {
    arrow(cst("LtlFormula"), cst("GeneralizedBuchi"))
}
/// GbaToNba: degeneralize GBA to NBA (standard Büchi)
pub fn tl_ext_gba_to_nba_ty() -> Expr {
    arrow(cst("GeneralizedBuchi"), cst("BuchiAutomaton"))
}
/// ProductAutomaton: M × A_¬φ for LTL model checking
pub fn tl_ext_product_automaton_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("BuchiAutomaton"), cst("BuchiAutomaton")),
    )
}
/// LtlModelCheckCorrectness: M ⊭ φ iff L(M × A_¬φ) ≠ ∅
pub fn tl_ext_ltl_check_correctness_ty() -> Expr {
    arrow(cst("TransitionSystem"), arrow(cst("LtlFormula"), prop()))
}
/// StreettAutomaton: deterministic Streett automaton
pub fn tl_ext_streett_automaton_ty() -> Expr {
    type0()
}
/// MullerAutomaton: deterministic Muller automaton with acceptance table F
pub fn tl_ext_muller_automaton_ty() -> Expr {
    type0()
}
/// RabinCondition: list of (E_i, F_i) pairs — infinitely often E_i ∧ finitely often F_i
pub fn tl_ext_rabin_condition_ty() -> Expr {
    arrow(cst("RabinAutomaton"), prop())
}
/// StreettCondition: list of (E_i, F_i) pairs — finitely often E_i ∨ infinitely often F_i
pub fn tl_ext_streett_condition_ty() -> Expr {
    arrow(cst("StreettAutomaton"), prop())
}
/// BuchiToRabin: convert Büchi to deterministic Rabin via Safra construction
pub fn tl_ext_buchi_to_rabin_ty() -> Expr {
    arrow(cst("BuchiAutomaton"), cst("RabinAutomaton"))
}
/// RabinToStreett: Rabin and Streett are duals
pub fn tl_ext_rabin_streett_dual_ty() -> Expr {
    arrow(cst("RabinAutomaton"), cst("StreettAutomaton"))
}
/// OmegaRegularLanguage: recognized by a Büchi automaton
pub fn tl_ext_omega_regular_ty() -> Expr {
    arrow(cst("BuchiAutomaton"), arrow(path_ty(), prop()))
}
/// OmegaRegularClosed: ω-regular languages are closed under Boolean operations
pub fn tl_ext_omega_regular_closed_ty() -> Expr {
    arrow(
        cst("BuchiAutomaton"),
        arrow(cst("BuchiAutomaton"), cst("BuchiAutomaton")),
    )
}
/// LtlIsOmegaRegular: every LTL formula defines an ω-regular language
pub fn tl_ext_ltl_omega_regular_ty() -> Expr {
    arrow(cst("LtlFormula"), cst("BuchiAutomaton"))
}
/// BmcInstance: a bounded model checking instance (M, φ, k)
pub fn bmc_instance_ty() -> Expr {
    type0()
}
/// BmcUnrolling: k-step unrolling of transition system
pub fn tl_ext_bmc_unrolling_ty() -> Expr {
    arrow(cst("TransitionSystem"), arrow(nat_ty(), cst("BmcFormula")))
}
/// BmcFormula: propositional formula generated by BMC unrolling
pub fn tl_ext_bmc_formula_ty() -> Expr {
    type0()
}
/// BmcSoundness: if BMC finds no counterexample of length ≤ k, then no lasso exists up to k
pub fn tl_ext_bmc_soundness_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(nat_ty(), arrow(cst("LtlFormula"), prop())),
    )
}
/// BmcCompleteness: BMC is complete for safety properties given sufficient bound
pub fn tl_ext_bmc_completeness_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("LtlFormula"), arrow(nat_ty(), prop())),
    )
}
/// Ic3Frame: an inductive frame in IC3/PDR
pub fn tl_ext_ic3_frame_ty() -> Expr {
    type0()
}
/// Ic3Invariant: an inductively strengthened invariant
pub fn tl_ext_ic3_invariant_ty() -> Expr {
    arrow(cst("TransitionSystem"), arrow(cst("LtlFormula"), prop()))
}
/// Ic3Termination: IC3 terminates on finite transition systems
pub fn tl_ext_ic3_termination_ty() -> Expr {
    arrow(cst("TransitionSystem"), prop())
}
/// Ic3Correctness: IC3 returns SAFE iff the safety property holds
pub fn tl_ext_ic3_correctness_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("LtlFormula"), arrow(cst("Ic3Frame"), prop())),
    )
}
/// CtlStarEmbedding: CTL embeds into CTL*
pub fn tl_ext_ctl_embeds_ctl_star_ty() -> Expr {
    arrow(cst("CtlFormula"), cst("CtlStarFormula"))
}
/// LtlEmbedding: LTL embeds into CTL*
pub fn tl_ext_ltl_embeds_ctl_star_ty() -> Expr {
    arrow(cst("LtlFormula"), cst("CtlStarFormula"))
}
/// LinearMuFormula: linear-time μ-calculus formula
pub fn tl_ext_linear_mu_formula_ty() -> Expr {
    type0()
}
/// LinearMuSat: π, i ⊨ φ in the linear μ-calculus
pub fn tl_ext_linear_mu_sat_ty() -> Expr {
    arrow(
        path_ty(),
        arrow(nat_ty(), arrow(cst("LinearMuFormula"), prop())),
    )
}
/// PnueliCompleteness: Pnueli's completeness theorem for LTL
pub fn tl_ext_pnueli_completeness_ty() -> Expr {
    arrow(cst("LtlFormula"), prop())
}
/// AbstractionRefinement: CEGAR loop for symbolic model checking
pub fn tl_ext_cegar_ty() -> Expr {
    arrow(
        cst("TransitionSystem"),
        arrow(cst("CtlFormula"), arrow(cst("TransitionSystem"), prop())),
    )
}
/// PartialOrderReduction: stubborn/persistent sets for state space reduction
pub fn tl_ext_por_ty() -> Expr {
    arrow(cst("TransitionSystem"), cst("TransitionSystem"))
}
/// SymmetryReduction: exploit symmetry in the state space
pub fn tl_ext_symmetry_reduction_ty() -> Expr {
    arrow(cst("TransitionSystem"), cst("TransitionSystem"))
}
/// VariableOrdering: BDD variable ordering for symbolic model checking
pub fn tl_ext_var_ordering_ty() -> Expr {
    arrow(cst("BDDManager"), arrow(nat_ty(), cst("BDDManager")))
}
/// Register all extended temporal logic axioms into the given environment.
pub fn register_temporal_logic_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, Expr)] = &[
        ("TctlFormula", tctl_formula_ty()),
        ("TctlEFBounded", tl_ext_tctl_ef_bounded_ty()),
        ("TctlAGBounded", tl_ext_tctl_ag_bounded_ty()),
        ("TimedTransitionSystem", type0()),
        ("TctlSat", tl_ext_tctl_sat_ty()),
        ("MtlFormula", mtl_formula_ty()),
        ("MtlUntilBounded", tl_ext_mtl_until_bounded_ty()),
        ("MtlFinallyBounded", tl_ext_mtl_finally_bounded_ty()),
        ("MtlGloballyBounded", tl_ext_mtl_globally_bounded_ty()),
        ("StlFormula", tl_ext_stl_formula_ty()),
        ("StlSat", tl_ext_stl_sat_ty()),
        ("StlRobustness", tl_ext_stl_robustness_ty()),
        ("ItlFormula", tl_ext_itl_formula_ty()),
        ("ItlChop", tl_ext_itl_chop_ty()),
        ("ItlProjection", tl_ext_itl_projection_ty()),
        ("ItlSat", tl_ext_itl_sat_ty()),
        ("LtlToGba", tl_ext_ltl_to_gba_ty()),
        ("GbaToNba", tl_ext_gba_to_nba_ty()),
        ("ProductAutomaton", tl_ext_product_automaton_ty()),
        (
            "LtlModelCheckCorrectness",
            tl_ext_ltl_check_correctness_ty(),
        ),
        ("StreettAutomaton", tl_ext_streett_automaton_ty()),
        ("MullerAutomaton", tl_ext_muller_automaton_ty()),
        ("RabinCondition", tl_ext_rabin_condition_ty()),
        ("StreettCondition", tl_ext_streett_condition_ty()),
        ("BuchiToRabin", tl_ext_buchi_to_rabin_ty()),
        ("RabinStreettDual", tl_ext_rabin_streett_dual_ty()),
        ("OmegaRegularLanguage", tl_ext_omega_regular_ty()),
        ("OmegaRegularClosed", tl_ext_omega_regular_closed_ty()),
        ("LtlIsOmegaRegular", tl_ext_ltl_omega_regular_ty()),
        ("BmcInstance", bmc_instance_ty()),
        ("BmcFormula", tl_ext_bmc_formula_ty()),
        ("BmcUnrolling", tl_ext_bmc_unrolling_ty()),
        ("BmcSoundness", tl_ext_bmc_soundness_ty()),
        ("BmcCompleteness", tl_ext_bmc_completeness_ty()),
        ("Ic3Frame", tl_ext_ic3_frame_ty()),
        ("Ic3Invariant", tl_ext_ic3_invariant_ty()),
        ("Ic3Termination", tl_ext_ic3_termination_ty()),
        ("Ic3Correctness", tl_ext_ic3_correctness_ty()),
        ("CtlEmbedsCtlStar", tl_ext_ctl_embeds_ctl_star_ty()),
        ("LtlEmbedsCtlStar", tl_ext_ltl_embeds_ctl_star_ty()),
        ("LinearMuFormula", tl_ext_linear_mu_formula_ty()),
        ("LinearMuSat", tl_ext_linear_mu_sat_ty()),
        ("PnueliCompleteness", tl_ext_pnueli_completeness_ty()),
        ("CegarLoop", tl_ext_cegar_ty()),
        ("PartialOrderReduction", tl_ext_por_ty()),
        ("SymmetryReduction", tl_ext_symmetry_reduction_ty()),
        ("VariableOrdering", tl_ext_var_ordering_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
