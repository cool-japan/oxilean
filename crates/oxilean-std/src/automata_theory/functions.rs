//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{BTreeSet, HashSet};

use super::types::{
    BuchiAutomatonSimulator, BuchiNba, CellularAutomataRule, ClockRegion, LtlFormula, ParityAut,
    ParityGameSolver, TimedAutomatonChecker, WeightedAut, WeightedAutomaton,
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
pub fn option_ty(elem: Expr) -> Expr {
    app(cst("Option"), elem)
}
pub fn set_ty(elem: Expr) -> Expr {
    app(cst("Set"), elem)
}
/// OmegaWord: an infinite word over an alphabet (Nat → State)
pub fn omega_word_ty() -> Expr {
    arrow(nat_ty(), type0())
}
/// InfSet: the set of states visited infinitely often in a run
pub fn inf_set_ty() -> Expr {
    arrow(omega_word_ty(), set_ty(cst("State")))
}
/// BuchiAutomaton: (Q, Σ, δ, q₀, F) where F ⊆ Q is accepting set
pub fn buchi_automaton_ty() -> Expr {
    type0()
}
/// BuchiRun: an infinite run of a Büchi automaton
pub fn buchi_run_ty() -> Expr {
    type0()
}
/// BuchiAccepting: a run ρ is accepting iff Inf(ρ) ∩ F ≠ ∅
pub fn buchi_accepting_ty() -> Expr {
    arrow(cst("BuchiAutomaton"), arrow(cst("BuchiRun"), prop()))
}
/// BuchiLanguage: the omega-language accepted by a Büchi automaton
pub fn buchi_language_ty() -> Expr {
    arrow(cst("BuchiAutomaton"), set_ty(omega_word_ty()))
}
/// NondetBuchiAutomaton: nondeterministic Büchi (NBA)
pub fn nba_ty() -> Expr {
    type0()
}
/// DetBuchiAutomaton: deterministic Büchi (DBA)  — strictly weaker than NBA
pub fn dba_ty() -> Expr {
    type0()
}
/// RabinPair: (E_i, F_i) pair used in Rabin acceptance
pub fn rabin_pair_ty() -> Expr {
    type0()
}
/// RabinAutomaton: accepts iff some pair (E,F) has Inf(ρ)∩E=∅, Inf(ρ)∩F≠∅
pub fn rabin_automaton_ty() -> Expr {
    type0()
}
/// MullerAutomaton: accepts iff Inf(ρ) ∈ ℱ for some accepting-set family ℱ
pub fn muller_automaton_ty() -> Expr {
    type0()
}
/// ParityAutomaton: accepts iff min priority visited inf-often is even
pub fn parity_automaton_ty() -> Expr {
    type0()
}
/// StreettAutomaton: dual of Rabin — accepts iff all (E,F) pairs satisfy
/// Inf(ρ)∩E≠∅ → Inf(ρ)∩F≠∅
pub fn streett_automaton_ty() -> Expr {
    type0()
}
/// rabin_to_muller: Rabin automaton → equivalent Muller automaton
pub fn rabin_to_muller_ty() -> Expr {
    arrow(rabin_automaton_ty(), muller_automaton_ty())
}
/// muller_to_rabin: Muller automaton → equivalent Rabin automaton
pub fn muller_to_rabin_ty() -> Expr {
    arrow(muller_automaton_ty(), rabin_automaton_ty())
}
/// parity_to_rabin: Parity automaton → equivalent Rabin automaton
pub fn parity_to_rabin_ty() -> Expr {
    arrow(parity_automaton_ty(), rabin_automaton_ty())
}
/// Equivalence of acceptance conditions: Rabin ≡ Muller ≡ Parity
pub fn omega_acceptance_equivalence_ty() -> Expr {
    prop()
}
/// BinaryTree: inductively defined binary tree type
pub fn binary_tree_ty() -> Expr {
    arrow(type0(), type0())
}
/// TreeAutomaton: processes finite or infinite trees
pub fn tree_automaton_ty() -> Expr {
    type0()
}
/// TopDownTreeAutomaton: processes tree top-down
pub fn top_down_tree_automaton_ty() -> Expr {
    type0()
}
/// BottomUpTreeAutomaton: processes tree bottom-up
pub fn bottom_up_tree_automaton_ty() -> Expr {
    type0()
}
/// TreeLanguage: a set of trees accepted by a tree automaton
pub fn tree_language_ty() -> Expr {
    arrow(tree_automaton_ty(), set_ty(binary_tree_ty()))
}
/// topDown_to_bottomUp: conversion between tree automaton representations
pub fn top_to_bottom_ty() -> Expr {
    arrow(top_down_tree_automaton_ty(), bottom_up_tree_automaton_ty())
}
/// TreeAutomataIntersection: closure under intersection
pub fn tree_automata_intersection_ty() -> Expr {
    arrow(
        tree_automaton_ty(),
        arrow(tree_automaton_ty(), tree_automaton_ty()),
    )
}
/// TreeAutomataComplementation: closure under complement
pub fn tree_automata_complementation_ty() -> Expr {
    arrow(tree_automaton_ty(), tree_automaton_ty())
}
/// TreeAutomataDeterminization: every NTA has an equivalent DTA
pub fn tree_automata_determinization_ty() -> Expr {
    arrow(tree_automaton_ty(), tree_automaton_ty())
}
/// Semiring: algebraic structure (S, +, ·, 0, 1) for weights
pub fn semiring_ty() -> Expr {
    type0()
}
/// WeightedAutomaton: automaton with semiring-valued transitions
pub fn weighted_automaton_ty() -> Expr {
    type0()
}
/// WeightedLanguage: a series (function from words to semiring elements)
pub fn weighted_language_ty() -> Expr {
    arrow(semiring_ty(), arrow(list_ty(cst("Symbol")), semiring_ty()))
}
/// WeightedBehavior: the behavior (series) of a weighted automaton
pub fn weighted_behavior_ty() -> Expr {
    arrow(weighted_automaton_ty(), weighted_language_ty())
}
/// LinearRecognizable: a series is linearly recognizable iff it's
/// the behavior of some weighted automaton
pub fn linear_recognizable_ty() -> Expr {
    arrow(weighted_language_ty(), prop())
}
/// SchützenbergerTheorem: a series is recognizable iff it has a
/// morphism into a finite monoid ring
pub fn schutzenberger_theorem_ty() -> Expr {
    prop()
}
/// Clock: a real-valued variable ranging over ℝ≥0
pub fn clock_ty() -> Expr {
    type0()
}
/// ClockValuation: assignment of values to clocks
pub fn clock_valuation_ty() -> Expr {
    arrow(set_ty(clock_ty()), arrow(clock_ty(), cst("Real")))
}
/// ClockConstraint: a guard (conjunction of clock difference constraints)
pub fn clock_constraint_ty() -> Expr {
    type0()
}
/// TimedAutomaton: (Q, Σ, C, q₀, E, Inv, F) with clock constraints
pub fn timed_automaton_ty() -> Expr {
    type0()
}
/// TimedWord: finite/infinite word with timestamps
pub fn timed_word_ty() -> Expr {
    type0()
}
/// TimedLanguage: the language of a timed automaton
pub fn timed_language_ty() -> Expr {
    arrow(timed_automaton_ty(), set_ty(timed_word_ty()))
}
/// RegionEquivalence: finite partition of clock valuations
pub fn region_equivalence_ty() -> Expr {
    arrow(timed_automaton_ty(), prop())
}
/// RegionGraph: finite abstraction via region equivalence
pub fn region_graph_ty() -> Expr {
    arrow(timed_automaton_ty(), cst("KripkeStructure"))
}
/// ZoneGraph: symbolic reachability graph using clock zones (DBMs)
pub fn zone_graph_ty() -> Expr {
    arrow(timed_automaton_ty(), type0())
}
/// TimedAutomataReachability: reachability is PSPACE-complete
pub fn timed_reachability_ty() -> Expr {
    prop()
}
/// AlternatingAutomaton: δ : Q×Σ → BoolComb(Q)  (positive boolean combinations)
pub fn alternating_automaton_ty() -> Expr {
    type0()
}
/// BoolComb: positive boolean combinations of states
pub fn bool_comb_ty() -> Expr {
    arrow(type0(), type0())
}
/// alternating_to_nba: alternating Büchi → nondeterministic Büchi (exponential)
pub fn alternating_to_nba_ty() -> Expr {
    arrow(alternating_automaton_ty(), nba_ty())
}
/// AlternatingBuchiLanguage: the language of an alternating Büchi automaton
pub fn alternating_buchi_language_ty() -> Expr {
    arrow(alternating_automaton_ty(), set_ty(omega_word_ty()))
}
/// ZielonkaTree: the Zielonka tree of a Muller automaton
pub fn zielonka_tree_ty() -> Expr {
    type0()
}
/// ZielonkaNode: a node in the Zielonka tree (a set of accepting colors)
pub fn zielonka_node_ty() -> Expr {
    type0()
}
/// build_zielonka_tree: construct the Zielonka tree from a Muller condition
pub fn build_zielonka_tree_ty() -> Expr {
    arrow(muller_automaton_ty(), zielonka_tree_ty())
}
/// ZielonkaTheorem: every Muller automaton can be determinized; the Zielonka tree
/// gives a parity automaton of minimal index
pub fn zielonka_theorem_ty() -> Expr {
    pi(
        BinderInfo::Default,
        "A",
        muller_automaton_ty(),
        arrow(parity_automaton_ty(), prop()),
    )
}
/// muller_to_parity: witness of Zielonka theorem — direct conversion
pub fn muller_to_parity_ty() -> Expr {
    arrow(muller_automaton_ty(), parity_automaton_ty())
}
/// LtlFormula: linear temporal logic formula type
pub fn ltl_formula_ty() -> Expr {
    type0()
}
/// CtlFormula: computation tree logic formula type
pub fn ctl_formula_ty() -> Expr {
    type0()
}
/// KripkeStructure: transition system for model checking
pub fn kripke_structure_ty() -> Expr {
    type0()
}
/// ltl_to_buchi: translate an LTL formula to an equivalent NBA
pub fn ltl_to_buchi_ty() -> Expr {
    arrow(ltl_formula_ty(), nba_ty())
}
/// product_automaton: Kripke structure × NBA → NBA for model checking
pub fn product_automaton_ty() -> Expr {
    arrow(kripke_structure_ty(), arrow(nba_ty(), nba_ty()))
}
/// check_emptiness: decide emptiness of an NBA (reachable accepting SCC)
pub fn check_emptiness_ty() -> Expr {
    arrow(nba_ty(), bool_ty())
}
/// ModelCheckingLTL: K ⊨ φ iff L(K) ∩ L(A_¬φ) = ∅
pub fn model_checking_ltl_ty() -> Expr {
    arrow(kripke_structure_ty(), arrow(ltl_formula_ty(), bool_ty()))
}
/// CounterExampleTrace: a lasso witness to model checking failure
pub fn counter_example_trace_ty() -> Expr {
    type0()
}
/// find_counterexample: extract a lasso counterexample
pub fn find_counterexample_ty() -> Expr {
    arrow(
        kripke_structure_ty(),
        arrow(ltl_formula_ty(), option_ty(counter_example_trace_ty())),
    )
}
/// on_the_fly_model_checking: nested DFS emptiness check
pub fn on_the_fly_ty() -> Expr {
    arrow(kripke_structure_ty(), arrow(ltl_formula_ty(), bool_ty()))
}
/// SynthesisProblem: (φ, I, O) where φ is spec, I/O are input/output alphabets
pub fn synthesis_problem_ty() -> Expr {
    type0()
}
/// Transducer: a Mealy machine realizing a strategy
pub fn transducer_ty() -> Expr {
    type0()
}
/// LTLSynthesis: compute a transducer realizing an LTL spec (Church's problem)
pub fn ltl_synthesis_ty() -> Expr {
    arrow(synthesis_problem_ty(), option_ty(transducer_ty()))
}
/// CTLSynthesis: fixpoint computation for CTL strategy synthesis
pub fn ctl_synthesis_ty() -> Expr {
    arrow(synthesis_problem_ty(), option_ty(transducer_ty()))
}
/// PariGameSolver: parity game solver for synthesis (Zielonka / progress measures)
pub fn parity_game_solver_ty() -> Expr {
    type0()
}
/// SolvePariGame: solve a parity game and extract winning strategy
pub fn solve_parity_game_ty() -> Expr {
    arrow(type0(), type0())
}
/// RealizabilityCheck: decide if an LTL spec is realizable
pub fn realizability_check_ty() -> Expr {
    arrow(synthesis_problem_ty(), bool_ty())
}
/// BuchiGame: 2-player game on a Büchi objective
pub fn buchi_game_ty() -> Expr {
    type0()
}
/// solve_buchi_game: attractor computation for Büchi games
pub fn solve_buchi_game_ty() -> Expr {
    arrow(buchi_game_ty(), option_ty(transducer_ty()))
}
/// OmegaRegularLanguage: a language recognizable by an NBA
pub fn omega_regular_language_ty() -> Expr {
    set_ty(omega_word_ty())
}
/// omega_closure: L^ω closure of a regular language
pub fn omega_closure_ty() -> Expr {
    arrow(set_ty(list_ty(cst("Symbol"))), omega_regular_language_ty())
}
/// OmegaRegularClosed_Union: ω-regular languages closed under union
pub fn omega_regular_union_ty() -> Expr {
    arrow(
        omega_regular_language_ty(),
        arrow(omega_regular_language_ty(), omega_regular_language_ty()),
    )
}
/// OmegaRegularClosed_Intersection: closed under intersection
pub fn omega_regular_intersection_ty() -> Expr {
    arrow(
        omega_regular_language_ty(),
        arrow(omega_regular_language_ty(), omega_regular_language_ty()),
    )
}
/// OmegaRegularClosed_Complement: closed under complement
pub fn omega_regular_complement_ty() -> Expr {
    arrow(omega_regular_language_ty(), omega_regular_language_ty())
}
/// BuchiComplementation: NBA → NBA for complement (via Safra / Piterman)
pub fn buchi_complementation_ty() -> Expr {
    arrow(nba_ty(), nba_ty())
}
/// TropicalSemiring: the (min,+) semiring ℝ∪{+∞} used in shortest-path automata
pub fn tropical_semiring_ty() -> Expr {
    type0()
}
/// ProbabilisticAutomaton: transition probabilities sum to 1 per state/symbol
pub fn probabilistic_automaton_ty() -> Expr {
    type0()
}
/// WeightedNFA: nondeterministic automaton with semiring-weighted transitions
pub fn weighted_nfa_ty() -> Expr {
    arrow(semiring_ty(), type0())
}
/// tropical_shortest_path: compute shortest-path weight in tropical automaton
pub fn tropical_shortest_path_ty() -> Expr {
    arrow(
        tropical_semiring_ty(),
        arrow(list_ty(cst("Symbol")), cst("Real")),
    )
}
/// probabilistic_word_probability: probability that a word is accepted
pub fn probabilistic_word_prob_ty() -> Expr {
    arrow(
        probabilistic_automaton_ty(),
        arrow(list_ty(cst("Symbol")), cst("Real")),
    )
}
/// SemiringHomomorphism: structure-preserving map between semirings
pub fn semiring_hom_ty() -> Expr {
    arrow(semiring_ty(), arrow(semiring_ty(), prop()))
}
/// SequentialTransducer: a deterministic transducer (input → output words)
pub fn sequential_transducer_ty() -> Expr {
    type0()
}
/// SubsequentialTransducer: sequential + final output function
pub fn subsequential_transducer_ty() -> Expr {
    type0()
}
/// RationalRelation: a binary relation on words computed by a finite transducer
pub fn rational_relation_ty() -> Expr {
    type0()
}
/// Bimachine: pair of deterministic automata computing a rational function
pub fn bimachine_ty() -> Expr {
    type0()
}
/// transducer_compose: composition of two transducers
pub fn transducer_compose_ty() -> Expr {
    arrow(
        sequential_transducer_ty(),
        arrow(sequential_transducer_ty(), sequential_transducer_ty()),
    )
}
/// rational_relation_closure: rational relations closed under composition
pub fn rational_relation_closure_ty() -> Expr {
    arrow(
        rational_relation_ty(),
        arrow(rational_relation_ty(), rational_relation_ty()),
    )
}
/// bimachine_to_transducer: every bimachine computes a subsequential function
pub fn bimachine_to_transducer_ty() -> Expr {
    arrow(bimachine_ty(), subsequential_transducer_ty())
}
/// GoodForGames: an automaton is good-for-games if it can be used for synthesis
pub fn good_for_games_ty() -> Expr {
    arrow(nba_ty(), prop())
}
/// HistoryDeterministic: weaker than deterministic, stronger than nondeterministic
pub fn history_deterministic_ty() -> Expr {
    arrow(nba_ty(), prop())
}
/// LimitLinear: Streett pair acceptance index
pub fn streett_index_ty() -> Expr {
    nat_ty()
}
/// RabinIndex: minimum number of pairs in a Rabin condition
pub fn rabin_index_ty() -> Expr {
    nat_ty()
}
/// parity_to_streett: parity automaton → Streett automaton
pub fn parity_to_streett_ty() -> Expr {
    arrow(parity_automaton_ty(), streett_automaton_ty())
}
/// streett_to_parity: Streett automaton → parity (via index conversion)
pub fn streett_to_parity_ty() -> Expr {
    arrow(streett_automaton_ty(), parity_automaton_ty())
}
/// AlternatingTreeAutomaton: ATA over ranked trees with conjunctive/disjunctive transitions
pub fn alternating_tree_automaton_ty() -> Expr {
    type0()
}
/// RankedAlphabet: a finite-ranked alphabet (symbol → arity)
pub fn ranked_alphabet_ty() -> Expr {
    arrow(cst("Symbol"), nat_ty())
}
/// TreeTransducer: a transducer operating on trees
pub fn tree_transducer_ty() -> Expr {
    type0()
}
/// ata_to_nta: alternating tree automaton → nondeterministic tree automaton
pub fn ata_to_nta_ty() -> Expr {
    arrow(alternating_tree_automaton_ty(), tree_automaton_ty())
}
/// TreeAutomataUnion: tree-recognizable languages closed under union
pub fn tree_automata_union_ty() -> Expr {
    arrow(
        tree_automaton_ty(),
        arrow(tree_automaton_ty(), tree_automaton_ty()),
    )
}
/// MSO_to_TreeAutomaton: Rabin's theorem — MSO ↔ tree-recognizable
pub fn mso_to_tree_automaton_ty() -> Expr {
    prop()
}
/// PushdownAutomaton: (Q, Σ, Γ, δ, q₀, Z₀, F) with stack alphabet Γ
pub fn pushdown_automaton_ty() -> Expr {
    type0()
}
/// DPDA: deterministic pushdown automaton
pub fn dpda_ty() -> Expr {
    type0()
}
/// PushdownTransducer: pushdown automaton with output
pub fn pushdown_transducer_ty() -> Expr {
    type0()
}
/// ContextFreeLanguage: language accepted by a PDA
pub fn context_free_language_ty() -> Expr {
    arrow(pushdown_automaton_ty(), set_ty(list_ty(cst("Symbol"))))
}
/// cfl_union_closure: CFLs closed under union
pub fn cfl_union_closure_ty() -> Expr {
    arrow(
        context_free_language_ty(),
        arrow(context_free_language_ty(), context_free_language_ty()),
    )
}
/// cfl_concatenation_closure: CFLs closed under concatenation
pub fn cfl_concat_closure_ty() -> Expr {
    arrow(
        context_free_language_ty(),
        arrow(context_free_language_ty(), context_free_language_ty()),
    )
}
/// cfl_not_closed_intersection: intersection of two CFLs may not be CFL (Prop = evidence of witness)
pub fn cfl_not_closed_intersection_ty() -> Expr {
    prop()
}
/// NestedWord: a word with a matching relation on positions
pub fn nested_word_ty() -> Expr {
    type0()
}
/// VisiblyPushdownAutomaton: pushdown automaton where call/return is alphabet-driven
pub fn visibly_pushdown_automaton_ty() -> Expr {
    type0()
}
/// NestedWordAutomaton: automaton recognizing nested words
pub fn nested_word_automaton_ty() -> Expr {
    type0()
}
/// vpa_to_nwa: VPA ↔ NWA equivalence
pub fn vpa_to_nwa_ty() -> Expr {
    arrow(visibly_pushdown_automaton_ty(), nested_word_automaton_ty())
}
/// NestedWordLanguage: closed under Boolean operations
pub fn nested_word_language_closure_ty() -> Expr {
    arrow(
        nested_word_automaton_ty(),
        arrow(nested_word_automaton_ty(), nested_word_automaton_ty()),
    )
}
/// CounterAutomaton: finite automaton augmented with integer counters
pub fn counter_automaton_ty() -> Expr {
    type0()
}
/// ReversalBoundedCounter: counter automaton with bounded reversal count
pub fn reversal_bounded_counter_ty() -> Expr {
    arrow(counter_automaton_ty(), arrow(nat_ty(), prop()))
}
/// PresburgerFormula: a formula in Presburger arithmetic (linear arithmetic over ℤ)
pub fn presburger_formula_ty() -> Expr {
    type0()
}
/// presburger_decidable: Presburger arithmetic is decidable
pub fn presburger_decidable_ty() -> Expr {
    arrow(presburger_formula_ty(), bool_ty())
}
/// reversal_bounded_decidability: reversal-bounded counter automata have decidable reachability
pub fn reversal_bounded_decidability_ty() -> Expr {
    prop()
}
/// TimedBuchiAutomaton: timed automaton with Büchi acceptance condition
pub fn timed_buchi_automaton_ty() -> Expr {
    type0()
}
/// TimedOmegaWord: an omega-word with timestamps
pub fn timed_omega_word_ty() -> Expr {
    type0()
}
/// AlurDillTheorem: timed automata reachability is PSPACE-complete
pub fn alur_dill_theorem_ty() -> Expr {
    prop()
}
/// DBM: difference bound matrix for clock zone representation
pub fn dbm_ty() -> Expr {
    type0()
}
/// dbm_intersection: intersection of two DBMs
pub fn dbm_intersection_ty() -> Expr {
    arrow(dbm_ty(), arrow(dbm_ty(), option_ty(dbm_ty())))
}
/// dbm_reset: reset a clock in a DBM
pub fn dbm_reset_ty() -> Expr {
    arrow(dbm_ty(), arrow(nat_ty(), dbm_ty()))
}
/// CellularAutomaton1D: 1D binary cellular automaton with local rule
pub fn cellular_automaton_1d_ty() -> Expr {
    arrow(
        arrow(bool_ty(), arrow(bool_ty(), arrow(bool_ty(), bool_ty()))),
        type0(),
    )
}
/// WolframRule: a Wolfram elementary CA rule (0–255)
pub fn wolfram_rule_ty() -> Expr {
    nat_ty()
}
/// TotalisticRule: a totalistic CA rule based on neighborhood sums
pub fn totalistic_rule_ty() -> Expr {
    type0()
}
/// ca_universality: Rule 110 is Turing-complete (Cook's theorem)
pub fn ca_universality_ty() -> Expr {
    prop()
}
/// ca_garden_of_eden: existence of configurations with no predecessor
pub fn ca_garden_of_eden_ty() -> Expr {
    prop()
}
/// MeasureOnceQFA: quantum finite automaton with measurement at end
pub fn measure_once_qfa_ty() -> Expr {
    type0()
}
/// MeasureManyQFA: quantum finite automaton with intermediate measurements
pub fn measure_many_qfa_ty() -> Expr {
    type0()
}
/// QFALanguage: the set of words accepted with probability > 1/2
pub fn qfa_language_ty() -> Expr {
    arrow(measure_many_qfa_ty(), set_ty(list_ty(cst("Symbol"))))
}
/// qfa_not_regular: measure-once QFAs recognize strictly fewer languages than DFA
pub fn qfa_not_regular_ty() -> Expr {
    prop()
}
/// qfa_measure_many_regular: measure-many QFAs can recognize all regular languages
pub fn qfa_measure_many_regular_ty() -> Expr {
    prop()
}
/// TwoWayDFA: 2DFA — reads input tape in both directions
pub fn two_way_dfa_ty() -> Expr {
    type0()
}
/// TwoWayNFA: 2NFA — nondeterministic two-way automaton
pub fn two_way_nfa_ty() -> Expr {
    type0()
}
/// two_way_dfa_to_dfa: every 2DFA is equivalent to a 1DFA (Shepherdson)
pub fn two_way_dfa_to_dfa_ty() -> Expr {
    arrow(two_way_dfa_ty(), cst("DFA"))
}
/// two_way_nfa_to_dfa: every 2NFA is equivalent to a 1DFA (polynomial states)
pub fn two_way_nfa_to_dfa_ty() -> Expr {
    arrow(two_way_nfa_ty(), cst("DFA"))
}
/// TwoWayPolySimulation: 2NFA → 1DFA uses at most polynomial states
pub fn two_way_poly_simulation_ty() -> Expr {
    prop()
}
/// RegisterAutomaton: finite automaton with registers for data values
pub fn register_automaton_ty() -> Expr {
    type0()
}
/// FreshName: a data value that has not been seen before (global freshness)
pub fn fresh_name_ty() -> Expr {
    type0()
}
/// GlobalFreshness: a register automaton model with global freshness tests
pub fn global_freshness_ty() -> Expr {
    type0()
}
/// register_automaton_decidability: emptiness of register automata is decidable
pub fn register_automaton_decidability_ty() -> Expr {
    arrow(register_automaton_ty(), bool_ty())
}
/// LocalFreshness: freshness relative to current configuration only
pub fn local_freshness_ty() -> Expr {
    type0()
}
/// Populate an `Environment` with automata-theory axioms and theorem stubs.
pub fn build_automata_theory_env() -> Environment {
    let mut env = Environment::new();
    let axioms: &[(&str, Expr)] = &[
        ("OmegaWord", omega_word_ty()),
        ("InfSet", inf_set_ty()),
        ("BuchiAutomaton", buchi_automaton_ty()),
        ("BuchiRun", buchi_run_ty()),
        ("BuchiAccepting", buchi_accepting_ty()),
        ("BuchiLanguage", buchi_language_ty()),
        ("NBA", nba_ty()),
        ("DBA", dba_ty()),
        ("RabinPair", rabin_pair_ty()),
        ("RabinAutomaton", rabin_automaton_ty()),
        ("MullerAutomaton", muller_automaton_ty()),
        ("ParityAutomaton", parity_automaton_ty()),
        ("StreettAutomaton", streett_automaton_ty()),
        ("rabin_to_muller", rabin_to_muller_ty()),
        ("muller_to_rabin", muller_to_rabin_ty()),
        ("parity_to_rabin", parity_to_rabin_ty()),
        (
            "OmegaAcceptanceEquivalence",
            omega_acceptance_equivalence_ty(),
        ),
        ("BinaryTree", binary_tree_ty()),
        ("TreeAutomaton", tree_automaton_ty()),
        ("TopDownTreeAutomaton", top_down_tree_automaton_ty()),
        ("BottomUpTreeAutomaton", bottom_up_tree_automaton_ty()),
        ("TreeLanguage", tree_language_ty()),
        ("topDown_to_bottomUp", top_to_bottom_ty()),
        ("TreeAutomataIntersection", tree_automata_intersection_ty()),
        (
            "TreeAutomataComplementation",
            tree_automata_complementation_ty(),
        ),
        (
            "TreeAutomataDeterminization",
            tree_automata_determinization_ty(),
        ),
        ("Semiring", semiring_ty()),
        ("WeightedAutomaton", weighted_automaton_ty()),
        ("WeightedLanguage", weighted_language_ty()),
        ("WeightedBehavior", weighted_behavior_ty()),
        ("LinearRecognizable", linear_recognizable_ty()),
        ("SchutzenbergerTheorem", schutzenberger_theorem_ty()),
        ("Clock", clock_ty()),
        ("ClockValuation", clock_valuation_ty()),
        ("ClockConstraint", clock_constraint_ty()),
        ("TimedAutomaton", timed_automaton_ty()),
        ("TimedWord", timed_word_ty()),
        ("TimedLanguage", timed_language_ty()),
        ("RegionEquivalence", region_equivalence_ty()),
        ("RegionGraph", region_graph_ty()),
        ("ZoneGraph", zone_graph_ty()),
        ("TimedReachability", timed_reachability_ty()),
        ("AlternatingAutomaton", alternating_automaton_ty()),
        ("BoolComb", bool_comb_ty()),
        ("alternating_to_nba", alternating_to_nba_ty()),
        ("AlternatingBuchiLanguage", alternating_buchi_language_ty()),
        ("ZielonkaTree", zielonka_tree_ty()),
        ("ZielonkaNode", zielonka_node_ty()),
        ("build_zielonka_tree", build_zielonka_tree_ty()),
        ("ZielonkaTheorem", zielonka_theorem_ty()),
        ("muller_to_parity", muller_to_parity_ty()),
        ("LtlFormula", ltl_formula_ty()),
        ("CtlFormula", ctl_formula_ty()),
        ("KripkeStructure", kripke_structure_ty()),
        ("ltl_to_buchi", ltl_to_buchi_ty()),
        ("product_automaton", product_automaton_ty()),
        ("check_emptiness", check_emptiness_ty()),
        ("ModelCheckingLTL", model_checking_ltl_ty()),
        ("CounterExampleTrace", counter_example_trace_ty()),
        ("find_counterexample", find_counterexample_ty()),
        ("on_the_fly_model_checking", on_the_fly_ty()),
        ("SynthesisProblem", synthesis_problem_ty()),
        ("Transducer", transducer_ty()),
        ("LTLSynthesis", ltl_synthesis_ty()),
        ("CTLSynthesis", ctl_synthesis_ty()),
        ("ParityGameSolver", parity_game_solver_ty()),
        ("solve_parity_game", solve_parity_game_ty()),
        ("RealizabilityCheck", realizability_check_ty()),
        ("BuchiGame", buchi_game_ty()),
        ("solve_buchi_game", solve_buchi_game_ty()),
        ("OmegaRegularLanguage", omega_regular_language_ty()),
        ("omega_closure", omega_closure_ty()),
        ("OmegaRegularClosed_Union", omega_regular_union_ty()),
        (
            "OmegaRegularClosed_Intersection",
            omega_regular_intersection_ty(),
        ),
        (
            "OmegaRegularClosed_Complement",
            omega_regular_complement_ty(),
        ),
        ("BuchiComplementation", buchi_complementation_ty()),
        ("TropicalSemiring", tropical_semiring_ty()),
        ("ProbabilisticAutomaton", probabilistic_automaton_ty()),
        ("WeightedNFA", weighted_nfa_ty()),
        ("tropical_shortest_path", tropical_shortest_path_ty()),
        ("probabilistic_word_prob", probabilistic_word_prob_ty()),
        ("SemiringHomomorphism", semiring_hom_ty()),
        ("SequentialTransducer", sequential_transducer_ty()),
        ("SubsequentialTransducer", subsequential_transducer_ty()),
        ("RationalRelation", rational_relation_ty()),
        ("Bimachine", bimachine_ty()),
        ("transducer_compose", transducer_compose_ty()),
        ("rational_relation_closure", rational_relation_closure_ty()),
        ("bimachine_to_transducer", bimachine_to_transducer_ty()),
        ("GoodForGames", good_for_games_ty()),
        ("HistoryDeterministic", history_deterministic_ty()),
        ("StreettIndex", streett_index_ty()),
        ("RabinIndex", rabin_index_ty()),
        ("parity_to_streett", parity_to_streett_ty()),
        ("streett_to_parity", streett_to_parity_ty()),
        ("AlternatingTreeAutomaton", alternating_tree_automaton_ty()),
        ("RankedAlphabet", ranked_alphabet_ty()),
        ("TreeTransducer", tree_transducer_ty()),
        ("ata_to_nta", ata_to_nta_ty()),
        ("TreeAutomataUnion", tree_automata_union_ty()),
        ("MSO_to_TreeAutomaton", mso_to_tree_automaton_ty()),
        ("PushdownAutomaton", pushdown_automaton_ty()),
        ("DPDA", dpda_ty()),
        ("PushdownTransducer", pushdown_transducer_ty()),
        ("ContextFreeLanguage", context_free_language_ty()),
        ("cfl_union_closure", cfl_union_closure_ty()),
        ("cfl_concat_closure", cfl_concat_closure_ty()),
        (
            "cfl_not_closed_intersection",
            cfl_not_closed_intersection_ty(),
        ),
        ("NestedWord", nested_word_ty()),
        ("VisiblyPushdownAutomaton", visibly_pushdown_automaton_ty()),
        ("NestedWordAutomaton", nested_word_automaton_ty()),
        ("vpa_to_nwa", vpa_to_nwa_ty()),
        (
            "NestedWordLanguageClosure",
            nested_word_language_closure_ty(),
        ),
        ("CounterAutomaton", counter_automaton_ty()),
        ("ReversalBoundedCounter", reversal_bounded_counter_ty()),
        ("PresburgerFormula", presburger_formula_ty()),
        ("presburger_decidable", presburger_decidable_ty()),
        (
            "reversal_bounded_decidability",
            reversal_bounded_decidability_ty(),
        ),
        ("TimedBuchiAutomaton", timed_buchi_automaton_ty()),
        ("TimedOmegaWord", timed_omega_word_ty()),
        ("AlurDillTheorem", alur_dill_theorem_ty()),
        ("DBM", dbm_ty()),
        ("dbm_intersection", dbm_intersection_ty()),
        ("dbm_reset", dbm_reset_ty()),
        ("CellularAutomaton1D", cellular_automaton_1d_ty()),
        ("WolframRule", wolfram_rule_ty()),
        ("TotalisticRule", totalistic_rule_ty()),
        ("ca_universality", ca_universality_ty()),
        ("ca_garden_of_eden", ca_garden_of_eden_ty()),
        ("MeasureOnceQFA", measure_once_qfa_ty()),
        ("MeasureManyQFA", measure_many_qfa_ty()),
        ("QFALanguage", qfa_language_ty()),
        ("qfa_not_regular", qfa_not_regular_ty()),
        ("qfa_measure_many_regular", qfa_measure_many_regular_ty()),
        ("TwoWayDFA", two_way_dfa_ty()),
        ("TwoWayNFA", two_way_nfa_ty()),
        ("two_way_dfa_to_dfa", two_way_dfa_to_dfa_ty()),
        ("two_way_nfa_to_dfa", two_way_nfa_to_dfa_ty()),
        ("TwoWayPolySimulation", two_way_poly_simulation_ty()),
        ("RegisterAutomaton", register_automaton_ty()),
        ("FreshName", fresh_name_ty()),
        ("GlobalFreshness", global_freshness_ty()),
        (
            "register_automaton_decidability",
            register_automaton_decidability_ty(),
        ),
        ("LocalFreshness", local_freshness_ty()),
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
    /// Verify that the environment builds without errors and contains key axioms.
    #[test]
    fn test_build_automata_theory_env() {
        let env = build_automata_theory_env();
        assert!(env.get(&Name::str("BuchiAutomaton")).is_some());
        assert!(env.get(&Name::str("ZielonkaTheorem")).is_some());
        assert!(env.get(&Name::str("LTLSynthesis")).is_some());
        assert!(env.get(&Name::str("ParityAutomaton")).is_some());
        assert!(env.get(&Name::str("TimedAutomaton")).is_some());
        assert!(env.get(&Name::str("WeightedAutomaton")).is_some());
        assert!(env.get(&Name::str("AlternatingAutomaton")).is_some());
    }
    /// Test NBA emptiness check — automaton accepting (ab)^ω is not empty.
    #[test]
    fn test_buchi_nba_nonempty() {
        let mut nba = BuchiNba::new(2, vec!['a', 'b']);
        nba.add_transition(0, 'a', 1);
        nba.add_transition(1, 'b', 0);
        nba.set_accepting(0);
        assert!(nba.accepts_lasso("", "ab"));
        assert!(!nba.is_empty());
    }
    /// Test NBA emptiness check — trivial automaton with no accepting states is empty.
    #[test]
    fn test_buchi_nba_empty() {
        let mut nba = BuchiNba::new(2, vec!['a', 'b']);
        nba.add_transition(0, 'a', 1);
        nba.add_transition(1, 'b', 0);
        assert!(nba.is_empty());
    }
    /// Test LTL formula NNF conversion.
    #[test]
    fn test_ltl_nnf() {
        let phi = LtlFormula::Not(Box::new(LtlFormula::Globally(Box::new(LtlFormula::Atom(
            "p".into(),
        )))));
        let nnf = phi.nnf();
        assert_eq!(
            nnf,
            LtlFormula::Finally(Box::new(LtlFormula::Not(Box::new(LtlFormula::Atom(
                "p".into()
            )))))
        );
    }
    /// Test LTL formula evaluation on a finite trace.
    #[test]
    fn test_ltl_eval_finite() {
        let trace: Vec<HashSet<String>> = vec![
            ["p".to_string()].into(),
            ["q".to_string()].into(),
            HashSet::new(),
        ];
        let f_q = LtlFormula::Finally(Box::new(LtlFormula::Atom("q".into())));
        assert!(f_q.eval_finite(&trace, 0));
        let g_p = LtlFormula::Globally(Box::new(LtlFormula::Atom("p".into())));
        assert!(!g_p.eval_finite(&trace, 0));
        let p_until_q = LtlFormula::Until(
            Box::new(LtlFormula::Atom("p".into())),
            Box::new(LtlFormula::Atom("q".into())),
        );
        assert!(p_until_q.eval_finite(&trace, 0));
    }
    /// Test parity automaton lasso acceptance.
    #[test]
    fn test_parity_aut_accepts() {
        let mut pa = ParityAut::new(2, vec!['a'], vec![1, 0]);
        pa.set_transition(0, 'a', 1);
        pa.set_transition(1, 'a', 0);
        pa.init = 0;
        assert!(pa.accepts_lasso("", "a"));
    }
    /// Test weighted automaton (max,+) computation.
    #[test]
    fn test_weighted_aut_run() {
        let mut wa = WeightedAut::new(1, vec!['a']);
        wa.init_weights[0] = 0;
        wa.final_weights[0] = 0;
        wa.add_transition(0, 'a', 0, 3);
        assert_eq!(wa.run_weight("aaa"), 9);
        assert_eq!(wa.run_weight("a"), 3);
        assert_eq!(wa.run_weight(""), 0);
    }
    /// Test clock region computation.
    #[test]
    fn test_clock_region() {
        assert_eq!(ClockRegion::of(0.0, 3), ClockRegion::Exact(0));
        assert_eq!(ClockRegion::of(0.5, 3), ClockRegion::Open(0));
        assert_eq!(ClockRegion::of(1.0, 3), ClockRegion::Exact(1));
        assert_eq!(ClockRegion::of(2.7, 3), ClockRegion::Open(2));
        assert_eq!(ClockRegion::of(3.0, 3), ClockRegion::Above);
        assert_eq!(ClockRegion::of(5.0, 3), ClockRegion::Above);
        assert_eq!(ClockRegion::Exact(1).time_succ(), ClockRegion::Open(1));
        assert_eq!(ClockRegion::Open(1).time_succ(), ClockRegion::Exact(2));
        assert_eq!(ClockRegion::Above.time_succ(), ClockRegion::Above);
    }
    /// Test NBA that accepts all words ending in repeated 'b' (lasso with cycle "b").
    #[test]
    fn test_buchi_lasso_acceptance() {
        let mut nba = BuchiNba::new(2, vec!['a', 'b']);
        nba.add_transition(0, 'a', 0);
        nba.add_transition(0, 'b', 1);
        nba.add_transition(1, 'b', 1);
        nba.set_accepting(1);
        assert!(nba.accepts_lasso("aaa", "b"));
        assert!(nba.accepts_lasso("", "b"));
        assert!(!nba.accepts_lasso("", "a"));
        assert!(!nba.is_empty());
    }
    /// Test extended environment contains new axioms.
    #[test]
    fn test_extended_env_axioms() {
        let env = build_automata_theory_env();
        assert!(env.get(&Name::str("TropicalSemiring")).is_some());
        assert!(env.get(&Name::str("ProbabilisticAutomaton")).is_some());
        assert!(env.get(&Name::str("SemiringHomomorphism")).is_some());
        assert!(env.get(&Name::str("SequentialTransducer")).is_some());
        assert!(env.get(&Name::str("Bimachine")).is_some());
        assert!(env.get(&Name::str("RationalRelation")).is_some());
        assert!(env.get(&Name::str("GoodForGames")).is_some());
        assert!(env.get(&Name::str("HistoryDeterministic")).is_some());
        assert!(env.get(&Name::str("AlternatingTreeAutomaton")).is_some());
        assert!(env.get(&Name::str("MSO_to_TreeAutomaton")).is_some());
        assert!(env.get(&Name::str("PushdownAutomaton")).is_some());
        assert!(env.get(&Name::str("ContextFreeLanguage")).is_some());
        assert!(env.get(&Name::str("VisiblyPushdownAutomaton")).is_some());
        assert!(env.get(&Name::str("NestedWordAutomaton")).is_some());
        assert!(env.get(&Name::str("CounterAutomaton")).is_some());
        assert!(env.get(&Name::str("PresburgerFormula")).is_some());
        assert!(env.get(&Name::str("TimedBuchiAutomaton")).is_some());
        assert!(env.get(&Name::str("DBM")).is_some());
        assert!(env.get(&Name::str("CellularAutomaton1D")).is_some());
        assert!(env.get(&Name::str("ca_universality")).is_some());
        assert!(env.get(&Name::str("MeasureOnceQFA")).is_some());
        assert!(env.get(&Name::str("MeasureManyQFA")).is_some());
        assert!(env.get(&Name::str("TwoWayDFA")).is_some());
        assert!(env.get(&Name::str("TwoWayPolySimulation")).is_some());
        assert!(env.get(&Name::str("RegisterAutomaton")).is_some());
        assert!(env.get(&Name::str("GlobalFreshness")).is_some());
    }
    /// Test WeightedAutomaton semiring evaluation.
    #[test]
    fn test_weighted_automaton_eval() {
        let mut wa = WeightedAutomaton::new(2, vec!['a', 'b']);
        wa.init_vec[0] = 1.0;
        wa.final_vec[1] = 1.0;
        wa.add_transition(0, 'a', 0, 1.0);
        wa.add_transition(0, 'b', 1, 2.0);
        wa.add_transition(1, 'a', 1, 1.0);
        assert!((wa.evaluate("ba") - 2.0).abs() < 1e-9);
        assert!(wa.accepts("ba"));
        assert!((wa.evaluate("aa") - 0.0).abs() < 1e-9);
        assert!(!wa.accepts("aa"));
        assert!((wa.evaluate("") - 0.0).abs() < 1e-9);
    }
    /// Test BuchiAutomatonSimulator.
    #[test]
    fn test_buchi_automaton_simulator() {
        let mut sim = BuchiAutomatonSimulator::new(2, vec!['a', 'b']);
        sim.add_transition(0, 'a', 1);
        sim.add_transition(1, 'b', 0);
        sim.mark_accepting(0);
        assert!(sim.accepts_lasso("", "ab"));
        assert!(!sim.is_empty());
        let sim_empty = BuchiAutomatonSimulator::new(2, vec!['a', 'b']);
        assert!(sim_empty.is_empty());
    }
    /// Test TimedAutomatonChecker simple reachability.
    #[test]
    fn test_timed_automaton_checker() {
        let mut ta = TimedAutomatonChecker::new(2, 0, 3);
        ta.add_edge(0, 1, 'a', 1.0, 2.0, true);
        assert!(ta.is_reachable(1));
        assert!(ta.is_reachable(0));
    }
    /// Test CellularAutomataRule (Rule 110).
    #[test]
    fn test_cellular_automata_rule110() {
        let ca = CellularAutomataRule::new(110);
        assert_eq!(ca.rule_number(), 110);
        let init = vec![false, false, true, false];
        let next = ca.step(&init);
        assert_eq!(next.len(), 4);
        assert!(!next[0]);
        assert!(next[1]);
        assert!(next[2]);
        assert!(!next[3]);
        let history = ca.run(&init, 3);
        assert_eq!(history.len(), 4);
    }
    /// Test ParityGameSolver on a trivial game.
    #[test]
    fn test_parity_game_solver() {
        let mut solver = ParityGameSolver::new(3);
        solver.set_node(0, 0, 0, vec![1]);
        solver.set_node(1, 1, 1, vec![0, 2]);
        solver.set_node(2, 2, 0, vec![2]);
        let (w0, _w1) = solver.solve();
        assert!(w0.contains(&2));
    }
    /// Test ParityGameSolver attractor computation.
    #[test]
    fn test_parity_game_attractor() {
        let mut solver = ParityGameSolver::new(3);
        solver.set_node(0, 0, 0, vec![1]);
        solver.set_node(1, 0, 0, vec![2]);
        solver.set_node(2, 2, 0, vec![]);
        let mut target = BTreeSet::new();
        target.insert(2usize);
        let attr = solver.attractor(0, &target);
        assert!(attr.contains(&0));
        assert!(attr.contains(&1));
        assert!(attr.contains(&2));
    }
}
