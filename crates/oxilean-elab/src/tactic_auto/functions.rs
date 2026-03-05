//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tactic::{Goal, TacticResult, TacticState};

use super::types::{
    AutoAnnotation, AutoConfig, AutoGoalQueue, AutoHintFilterChain, AutoLemmaScorer, AutoResult,
    AutoSearchBudgetTracker, AutoTactic, AutoTacticBuilder, AutoTacticChain,
    AutoTacticExtensionMarker, AutoTacticRegistry, AutoTacticSession, BestFirstSearchNode,
    ExhaustiveSearch, HintDatabase, MinLengthFilter, NamePrefixFilter, ProofStep, ProofTrace,
    SearchBudget, SearchFrontier, SearchNode, SearchResult, SearchStatistics, SearchStrategy,
    TacticAutoProfile, TacticAutoReport, TautoConfig, TautoTactic,
};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

/// Check if `target` is a reflexivity goal `Eq T a a`.
pub fn is_refl_target(target: &Expr) -> bool {
    if let Expr::App(f1, rhs) = target {
        if let Expr::App(f2, lhs) = f1.as_ref() {
            if lhs == rhs {
                if let Expr::App(eq_const, _) = f2.as_ref() {
                    if let Expr::Const(name, _) = eq_const.as_ref() {
                        return name == &Name::str("Eq");
                    }
                }
            }
        }
    }
    false
}
/// Produce a short string representation of an Expr for debug output.
pub fn expr_to_summary(expr: &Expr) -> String {
    match expr {
        Expr::Const(name, _) => name.to_string(),
        Expr::App(f, _a) => format!("App({},...)", expr_to_summary(f)),
        Expr::Pi(_, name, _, _) => format!("Pi({name},...)",),
        Expr::Lam(_, name, _, _) => format!("Lam({name},...)",),
        Expr::Sort(_) => "Sort".to_string(),
        Expr::BVar(i) => format!("BVar({i})"),
        Expr::FVar(id) => format!("FVar({id:?})"),
        Expr::Lit(lit) => format!("Lit({lit:?})"),
        Expr::Let(name, _, _, _) => format!("Let({name},...)"),
        Expr::Proj(name, idx, _) => format!("Proj({name},{idx})"),
    }
}
/// Try to split a target string of form "App(App(And,A),B)" into (A,B).
pub fn split_and(target: &str) -> Option<(&str, &str)> {
    if !target.starts_with("App(App(And,") {
        return None;
    }
    let rest = &target["App(App(And,".len()..];
    let (a_part, remainder) = find_balanced_content(rest, ')', ',')?;
    let remainder = remainder.strip_prefix(',')?;
    let b_part = remainder.strip_suffix("))")?;
    Some((a_part, b_part))
}
/// Try to split a target string of form "App(App(Or,A),B)" into (A,B).
pub fn split_or(target: &str) -> Option<(&str, &str)> {
    if !target.starts_with("App(App(Or,") {
        return None;
    }
    let rest = &target["App(App(Or,".len()..];
    let (a_part, remainder) = find_balanced_content(rest, ')', ',')?;
    let remainder = remainder.strip_prefix(',')?;
    let b_part = remainder.strip_suffix("))")?;
    Some((a_part, b_part))
}
/// Find content in `s` up to the first unbalanced stop char or separator.
fn find_balanced_content(s: &str, _stop: char, sep: char) -> Option<(&str, &str)> {
    let mut depth = 0i32;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return Some((&s[..i], &s[i..]));
                }
                depth -= 1;
            }
            c if c == sep && depth == 0 => {
                return Some((&s[..i], &s[i..]));
            }
            _ => {}
        }
    }
    None
}
/// Run the auto tactic on the current state with the given config.
///
/// Returns `Ok(new_state)` with all goals solved, or an error if any goal
/// could not be solved within the search budget.
pub fn eval_auto(state: &TacticState, config: AutoConfig) -> TacticResult {
    use crate::tactic::TacticError;
    let mut auto = AutoTactic::new(config);
    match auto.search(state) {
        SearchResult::Solved => {
            let mut new_state = state.clone();
            let goal_names: Vec<Name> = new_state.goals().iter().map(|g| g.name.clone()).collect();
            for name in &goal_names {
                new_state.solve_goal(name);
            }
            Ok(new_state)
        }
        SearchResult::Failed => Err(TacticError::InternalError(
            "auto: proof search failed".to_string(),
        )),
        SearchResult::Partial(n) => Err(TacticError::InternalError(format!(
            "auto: {n} goal(s) remain unsolved after search"
        ))),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic_auto::*;
    fn make_true_expr() -> Expr {
        Expr::Const(Name::str("True"), vec![])
    }
    fn make_false_expr() -> Expr {
        Expr::Const(Name::str("False"), vec![])
    }
    fn make_prop_expr() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_auto_config_default() {
        let cfg = AutoConfig::default();
        assert_eq!(cfg.max_depth, 5);
        assert_eq!(cfg.max_steps, 1000);
        assert!(cfg.use_assumptions);
        assert!(cfg.use_simp);
        assert!(cfg.use_constructor);
        assert!(cfg.use_apply);
        assert!(cfg.lemma_hints.is_empty());
    }
    #[test]
    fn test_auto_tactic_new() {
        let cfg = AutoConfig {
            max_depth: 3,
            max_steps: 50,
            ..AutoConfig::default()
        };
        let auto = AutoTactic::new(cfg.clone());
        assert_eq!(auto.config.max_depth, 3);
        assert_eq!(auto.config.max_steps, 50);
        assert_eq!(auto.steps_taken, 0);
    }
    #[test]
    fn test_trivial_true_goal() {
        let auto = AutoTactic::with_defaults();
        let goal = Goal::new(Name::str("g"), make_true_expr());
        assert!(auto.is_trivial(&goal));
    }
    #[test]
    fn test_trivial_false_goal_not_trivial() {
        let auto = AutoTactic::with_defaults();
        let goal = Goal::new(Name::str("g"), make_false_expr());
        assert!(!auto.is_trivial(&goal));
    }
    #[test]
    fn test_goal_summary() {
        let auto = AutoTactic::with_defaults();
        let goal = Goal::new(Name::str("main"), make_true_expr());
        let summary = auto.goal_summary(&goal);
        assert!(summary.contains("main"));
        assert!(summary.contains("True"));
    }
    #[test]
    fn test_search_result_display() {
        let solved = SearchResult::Solved;
        let failed = SearchResult::Failed;
        let partial = SearchResult::Partial(3);
        assert!(format!("{solved:?}").contains("Solved"));
        assert!(format!("{failed:?}").contains("Failed"));
        assert!(format!("{partial:?}").contains("Partial"));
        assert!(format!("{partial:?}").contains('3'));
    }
    #[test]
    fn test_tauto_new() {
        let tauto = TautoTactic::new();
        let goal = Goal::new(Name::str("g"), make_true_expr());
        assert!(tauto.check(&goal));
    }
    #[test]
    fn test_auto_max_depth_config() {
        let cfg = AutoConfig {
            max_depth: 10,
            ..AutoConfig::default()
        };
        let auto = AutoTactic::new(cfg);
        assert_eq!(auto.config.max_depth, 10);
    }
    #[test]
    fn test_auto_with_hints() {
        let cfg = AutoConfig {
            lemma_hints: vec!["h1".to_string(), "h2".to_string()],
            ..AutoConfig::default()
        };
        assert_eq!(cfg.lemma_hints.len(), 2);
        assert_eq!(cfg.lemma_hints[0], "h1");
    }
    #[test]
    fn test_auto_search_empty_state() {
        let mut auto = AutoTactic::with_defaults();
        let state = TacticState::new();
        let result = auto.search(&state);
        assert!(matches!(result, SearchResult::Solved));
    }
    #[test]
    fn test_auto_search_true_goal() {
        let mut auto = AutoTactic::with_defaults();
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g"), make_true_expr()));
        let result = auto.search(&state);
        assert!(matches!(result, SearchResult::Solved));
    }
    #[test]
    fn test_auto_assumption() {
        let mut auto = AutoTactic::with_defaults();
        let mut state = TacticState::new();
        let prop = make_prop_expr();
        let mut goal = Goal::new(Name::str("g"), prop.clone());
        goal.add_hypothesis(Name::str("h"), prop);
        state.add_goal(goal);
        let result = auto.search(&state);
        assert!(matches!(result, SearchResult::Solved));
    }
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::tactic_auto::*;
    use oxilean_kernel::{BinderInfo, Expr, Level, Name};
    fn make_true() -> Expr {
        Expr::Const(Name::str("True"), vec![])
    }
    fn make_false() -> Expr {
        Expr::Const(Name::str("False"), vec![])
    }
    fn make_nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_auto_config_custom() {
        let cfg = AutoConfig {
            max_depth: 2,
            max_steps: 10,
            use_assumptions: false,
            use_simp: false,
            use_constructor: false,
            use_apply: false,
            lemma_hints: vec!["myLemma".to_string()],
        };
        assert_eq!(cfg.max_depth, 2);
        assert_eq!(cfg.lemma_hints[0], "myLemma");
    }
    #[test]
    fn test_auto_config_clone() {
        let cfg = AutoConfig::default();
        let cfg2 = cfg.clone();
        assert_eq!(cfg2.max_depth, cfg.max_depth);
    }
    #[test]
    fn test_auto_with_defaults_steps_zero() {
        let auto = AutoTactic::with_defaults();
        assert_eq!(auto.steps_taken, 0);
    }
    #[test]
    fn test_auto_search_false_goal_not_solved() {
        let mut auto = AutoTactic::with_defaults();
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g"), make_false()));
        let result = auto.search(&state);
        assert!(
            matches!(result, SearchResult::Partial(_)) || matches!(result, SearchResult::Failed)
        );
    }
    #[test]
    fn test_auto_search_multiple_true_goals() {
        let mut auto = AutoTactic::with_defaults();
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), make_true()));
        state.add_goal(Goal::new(Name::str("g2"), make_true()));
        assert!(matches!(auto.search(&state), SearchResult::Solved));
    }
    #[test]
    fn test_is_trivial_via_assumption() {
        let auto = AutoTactic::with_defaults();
        let nat = make_nat();
        let mut goal = Goal::new(Name::str("g"), nat.clone());
        goal.add_hypothesis(Name::str("h"), nat);
        assert!(auto.is_trivial(&goal));
    }
    #[test]
    fn test_is_trivial_refl_eq() {
        let auto = AutoTactic::with_defaults();
        let eq = Expr::Const(Name::str("Eq"), vec![]);
        let nat = make_nat();
        let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let app1 = Expr::App(Box::new(eq), Box::new(nat));
        let app2 = Expr::App(Box::new(app1), Box::new(zero.clone()));
        let app3 = Expr::App(Box::new(app2), Box::new(zero));
        let goal = Goal::new(Name::str("g"), app3);
        assert!(auto.is_trivial(&goal));
    }
    #[test]
    fn test_goal_summary_includes_hyp_names() {
        let auto = AutoTactic::with_defaults();
        let mut goal = Goal::new(Name::str("myGoal"), make_true());
        goal.add_hypothesis(Name::str("h1"), make_nat());
        let summary = auto.goal_summary(&goal);
        assert!(summary.contains("h1"));
        assert!(summary.contains("myGoal"));
    }
    #[test]
    fn test_tauto_false_goal_not_tautology() {
        let tauto = TautoTactic::new();
        let goal = Goal::new(Name::str("g"), make_false());
        assert!(!tauto.check(&goal));
    }
    #[test]
    fn test_tauto_assumption_is_tautology() {
        let tauto = TautoTactic::new();
        let nat = make_nat();
        let mut goal = Goal::new(Name::str("g"), nat.clone());
        goal.add_hypothesis(Name::str("h"), nat);
        assert!(tauto.check(&goal));
    }
    #[test]
    fn test_tauto_default_equals_new() {
        let t1 = TautoTactic::new();
        let t2 = TautoTactic;
        let goal = Goal::new(Name::str("g"), make_true());
        assert_eq!(t1.check(&goal), t2.check(&goal));
    }
    #[test]
    fn test_eval_auto_empty_state_ok() {
        let state = TacticState::new();
        assert!(eval_auto(&state, AutoConfig::default()).is_ok());
    }
    #[test]
    fn test_eval_auto_true_goal_ok() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g"), make_true()));
        assert!(eval_auto(&state, AutoConfig::default()).is_ok());
    }
    #[test]
    fn test_eval_auto_false_goal_err() {
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g"), make_false()));
        assert!(eval_auto(&state, AutoConfig::default()).is_err());
    }
    #[test]
    fn test_expr_to_summary_all_variants() {
        let c = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(expr_to_summary(&c), "Nat");
        let s = Expr::Sort(Level::zero());
        assert_eq!(expr_to_summary(&s), "Sort");
        let b = Expr::BVar(3);
        assert_eq!(expr_to_summary(&b), "BVar(3)");
        let app = Expr::App(Box::new(c.clone()), Box::new(b.clone()));
        assert!(expr_to_summary(&app).starts_with("App(Nat"));
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(c.clone()),
            Box::new(b.clone()),
        );
        assert!(expr_to_summary(&pi).starts_with("Pi(x"));
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(c.clone()),
            Box::new(c.clone()),
        );
        assert!(expr_to_summary(&lam).starts_with("Lam(y"));
        let lit = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        assert!(expr_to_summary(&lit).starts_with("Lit("));
    }
    #[test]
    fn test_is_refl_target_non_refl() {
        assert!(!is_refl_target(&Expr::Const(Name::str("Nat"), vec![])));
    }
    #[test]
    fn test_is_refl_target_neq_args() {
        let eq = Expr::Const(Name::str("Eq"), vec![]);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let one = Expr::Lit(oxilean_kernel::Literal::Nat(1));
        let app1 = Expr::App(Box::new(eq), Box::new(nat));
        let app2 = Expr::App(Box::new(app1), Box::new(zero));
        let app3 = Expr::App(Box::new(app2), Box::new(one));
        assert!(!is_refl_target(&app3));
    }
    #[test]
    fn test_split_and_returns_none_for_non_and() {
        assert!(split_and("App(App(Or,A),B)").is_none());
        assert!(split_and("True").is_none());
        assert!(split_and("").is_none());
    }
    #[test]
    fn test_split_or_returns_none_for_non_or() {
        assert!(split_or("App(App(And,A),B)").is_none());
        assert!(split_or("").is_none());
        assert!(split_or("True").is_none());
    }
    #[test]
    fn test_find_balanced_content_simple_comma() {
        let result = find_balanced_content("A,rest", ')', ',');
        assert_eq!(result, Some(("A", ",rest")));
    }
    #[test]
    fn test_find_balanced_content_nested_parens() {
        let result = find_balanced_content("App(X,Y),rest", ')', ',');
        assert_eq!(result, Some(("App(X,Y)", ",rest")));
    }
}
/// Score a lemma for the auto tactic based on heuristic relevance.
#[allow(dead_code)]
pub fn score_lemma(lemma_name: &str, goal_target: &str) -> f64 {
    let mut score = 1.0f64;
    let goal_words: Vec<&str> = goal_target
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect();
    for word in &goal_words {
        if lemma_name.contains(word) {
            score += 2.0;
        }
    }
    score -= lemma_name.len() as f64 * 0.01;
    if lemma_name.ends_with("_comm") || lemma_name.ends_with("_assoc") {
        score += 0.5;
    }
    score
}
/// Sort a list of lemma hints by relevance score (descending).
#[allow(dead_code)]
pub fn sort_hints_by_relevance(hints: &[String], goal_target: &str) -> Vec<String> {
    let mut scored: Vec<(f64, &String)> = hints
        .iter()
        .map(|h| (score_lemma(h, goal_target), h))
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored.into_iter().map(|(_, h)| h.clone()).collect()
}
#[cfg(test)]
mod extended_tactic_auto_tests {
    use super::*;
    use crate::tactic_auto::*;
    #[test]
    fn test_hint_database_add_get() {
        let mut db = HintDatabase::new();
        db.add("nat", "Nat.add_comm");
        db.add("nat", "Nat.add_zero");
        let hints = db.get("nat");
        assert_eq!(hints.len(), 2);
        assert_eq!(db.get("logic").len(), 0);
    }
    #[test]
    fn test_hint_database_all_lemmas() {
        let mut db = HintDatabase::new();
        db.add("a", "lemma_x");
        db.add("b", "lemma_y");
        db.add("a", "lemma_x");
        let all = db.all_lemmas();
        assert_eq!(all.len(), 2);
    }
    #[test]
    fn test_hint_database_standard() {
        let db = HintDatabase::standard();
        assert!(db.num_categories() >= 3);
        assert!(db.total_entries() >= 10);
        let nat_hints = db.get("nat");
        assert!(nat_hints.iter().any(|s| s == "Nat.add_comm"));
    }
    #[test]
    fn test_hint_database_merge() {
        let mut db1 = HintDatabase::new();
        db1.add("cat", "lemma_a");
        let mut db2 = HintDatabase::new();
        db2.add("cat", "lemma_b");
        db1.merge(db2);
        assert_eq!(db1.get("cat").len(), 2);
    }
    #[test]
    fn test_proof_step_is_closing() {
        let step = ProofStep::new("exact", "True", vec![], 0);
        assert!(step.is_closing());
        let step2 = ProofStep::new("intro", "Pi(x,True)", vec!["True".to_string()], 0);
        assert!(!step2.is_closing());
    }
    #[test]
    fn test_proof_trace_record_when_enabled() {
        let mut trace = ProofTrace::new();
        let step = ProofStep::new("trivial", "True", vec![], 0);
        trace.record(step);
        assert!(trace.is_empty());
        trace.enable();
        let step2 = ProofStep::new("trivial", "True", vec![], 0);
        trace.record(step2);
        assert_eq!(trace.len(), 1);
    }
    #[test]
    fn test_proof_trace_format() {
        let mut trace = ProofTrace::new();
        trace.enable();
        trace.record(ProofStep::new("assumption", "Nat", vec![], 1));
        let s = trace.format();
        assert!(s.contains("assumption"));
    }
    #[test]
    fn test_proof_trace_clear() {
        let mut trace = ProofTrace::new();
        trace.enable();
        trace.record(ProofStep::new("trivial", "True", vec![], 0));
        trace.clear();
        assert!(trace.is_empty());
    }
    #[test]
    fn test_search_budget_consume() {
        let config = AutoConfig {
            max_steps: 5,
            ..AutoConfig::default()
        };
        let mut budget = SearchBudget::from_config(&config);
        for _ in 0..5 {
            assert!(budget.consume_step());
        }
        assert!(!budget.consume_step());
        assert!(budget.is_exhausted());
        assert_eq!(budget.remaining(), 0);
    }
    #[test]
    fn test_search_budget_depth() {
        let config = AutoConfig {
            max_depth: 3,
            ..AutoConfig::default()
        };
        let budget = SearchBudget::from_config(&config);
        assert!(budget.within_depth(3));
        assert!(!budget.within_depth(4));
    }
    #[test]
    fn test_search_budget_reset() {
        let config = AutoConfig {
            max_steps: 3,
            ..AutoConfig::default()
        };
        let mut budget = SearchBudget::from_config(&config);
        budget.consume_step();
        budget.consume_step();
        budget.reset();
        assert_eq!(budget.remaining(), 3);
    }
    #[test]
    fn test_score_lemma_relevance() {
        let s1 = score_lemma("Nat.add_comm", "App(App(Nat.add,x),y)");
        let s2 = score_lemma("Bool.true_and", "App(App(Nat.add,x),y)");
        assert!(s1 > s2);
    }
    #[test]
    fn test_sort_hints_by_relevance() {
        let hints = vec!["Bool.true_and".to_string(), "Nat.add_comm".to_string()];
        let sorted = sort_hints_by_relevance(&hints, "Nat.add");
        assert_eq!(sorted[0], "Nat.add_comm");
    }
    #[test]
    fn test_auto_result_solved_summary() {
        let r = AutoResult::solved(42, 3);
        let s = r.summary();
        assert!(s.contains("Solved"));
        assert!(s.contains("42"));
    }
    #[test]
    fn test_auto_result_failed_summary() {
        let r = AutoResult::failed(100, 5);
        let s = r.summary();
        assert!(s.contains("Failed"));
    }
    #[test]
    fn test_auto_result_with_trace() {
        let mut trace = ProofTrace::new();
        trace.enable();
        trace.record(ProofStep::new("trivial", "True", vec![], 0));
        let r = AutoResult::solved(1, 0).with_trace(trace);
        assert!(r.trace.is_some());
        assert_eq!(
            r.trace
                .as_ref()
                .expect("type conversion should succeed")
                .len(),
            1
        );
    }
    #[test]
    fn test_search_frontier_push_pop() {
        let mut frontier = SearchFrontier::new();
        frontier.push(SearchNode::new(1.0, 0, "True", "trivial"));
        frontier.push(SearchNode::new(5.0, 0, "False", "exfalso"));
        frontier.push(SearchNode::new(3.0, 1, "And", "constructor"));
        assert_eq!(frontier.len(), 3);
        let top = frontier.pop().expect("collection should not be empty");
        assert!((top.priority - 5.0).abs() < 1e-10);
    }
    #[test]
    fn test_search_frontier_empty() {
        let mut frontier = SearchFrontier::new();
        assert!(frontier.is_empty());
        assert!(frontier.pop().is_none());
    }
    #[test]
    fn test_search_frontier_clear() {
        let mut frontier = SearchFrontier::new();
        frontier.push(SearchNode::new(1.0, 0, "True", "trivial"));
        frontier.clear();
        assert!(frontier.is_empty());
    }
    #[test]
    fn test_tauto_config_default() {
        let cfg = TautoConfig::default();
        assert_eq!(cfg.max_depth, 10);
        assert!(cfg.use_disj_syllogism);
        assert!(cfg.use_hypo_syllogism);
        assert!(cfg.use_modus_ponens);
    }
}
pub trait ProofSearch: Send + Sync {
    fn search_name(&self) -> &'static str;
    fn search(&self, goal: &Goal, config: &AutoConfig) -> SearchResult;
    fn max_depth(&self) -> u32 {
        10
    }
}
pub trait HintFilter: Send + Sync {
    fn filter_name(&self) -> &'static str;
    fn accept(&self, hint: &str, goal: &Goal) -> bool;
}
#[cfg(test)]
mod tactic_auto_extended_tests {
    use super::*;
    use crate::tactic_auto::*;
    #[test]
    fn test_tactic_auto_profile() {
        let mut profile = TacticAutoProfile::new();
        profile.record_node(3);
        profile.record_node(5);
        profile.record_backtrack();
        profile.record_success("lemma1");
        profile.record_failure("lemma2");
        assert_eq!(profile.node_count, 2);
        assert_eq!(profile.max_depth_reached, 5);
        assert_eq!(profile.backtrack_count, 1);
        assert!((profile.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_search_strategy_name() {
        assert_eq!(SearchStrategy::Exhaustive.name(), "exhaustive");
        let h = SearchStrategy::Heuristic(5);
        assert_eq!(h.name(), "heuristic");
        assert_eq!(h.beam_width(), Some(5));
        assert_eq!(SearchStrategy::BestFirst.name(), "best-first");
    }
    #[test]
    fn test_hint_filter_name_prefix() {
        let filter = NamePrefixFilter::new("Nat.");
        let dummy_goal = Goal::new(Name::str("g"), Expr::BVar(0));
        assert!(filter.accept("Nat.add_comm", &dummy_goal));
        assert!(!filter.accept("List.length", &dummy_goal));
    }
    #[test]
    fn test_hint_filter_min_length() {
        let filter = MinLengthFilter::new(5);
        let dummy_goal = Goal::new(Name::str("g"), Expr::BVar(0));
        assert!(filter.accept("hello_world", &dummy_goal));
        assert!(!filter.accept("hi", &dummy_goal));
    }
    #[test]
    fn test_auto_tactic_builder() {
        let tac = AutoTacticBuilder::new()
            .max_depth(20)
            .hint(Name::str("Nat.add_comm"))
            .strategy(SearchStrategy::Heuristic(3))
            .build();
        let _ = tac;
    }
    #[test]
    fn test_search_statistics() {
        let mut stats = SearchStatistics::new();
        stats.record_iteration();
        stats.record_iteration();
        stats.record_lemma_application();
        stats.mark_success();
        assert_eq!(stats.iterations, 2);
        assert!(stats.success);
        assert!((stats.efficiency() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_tactic_auto_report_success() {
        let report = TacticAutoReport::new("exhaustive").with_proof(3);
        assert!(report.proof_found);
        assert_eq!(report.proof_length, Some(3));
        let summary = report.summary();
        assert!(summary.contains("success"));
    }
    #[test]
    fn test_tactic_auto_report_failure() {
        let report = TacticAutoReport::new("heuristic");
        assert!(!report.proof_found);
        let summary = report.summary();
        assert!(summary.contains("failed"));
    }
    #[test]
    fn test_exhaustive_search() {
        let search = ExhaustiveSearch;
        let goal = Goal::new(Name::str("g"), Expr::BVar(0));
        let config = AutoConfig::default();
        let result = search.search(&goal, &config);
        assert_eq!(result, SearchResult::Failed);
    }
}
#[cfg(test)]
mod auto_tactic_extended2 {
    use super::*;
    use crate::tactic_auto::*;
    #[test]
    fn test_auto_tactic_registry() {
        let mut reg = AutoTacticRegistry::new();
        let mut cfg_fast = AutoConfig::default();
        cfg_fast.max_depth = 5;
        reg.register("fast", cfg_fast);
        let mut cfg_thorough = AutoConfig::default();
        cfg_thorough.max_depth = 50;
        reg.register("thorough", cfg_thorough);
        assert_eq!(reg.count(), 2);
        let fast = reg.lookup("fast").expect("test operation should succeed");
        assert_eq!(fast.max_depth, 5);
        assert!(reg.lookup("missing").is_none());
    }
    #[test]
    fn test_auto_tactic_chain_count() {
        let mut cfg2 = AutoConfig::default();
        cfg2.max_depth = 20;
        let chain = AutoTacticChain::new().add(AutoConfig::default()).add(cfg2);
        assert_eq!(chain.count(), 2);
    }
    #[test]
    fn test_auto_annotation_apply() {
        let mut config = AutoConfig::default();
        let ann = AutoAnnotation::MaxDepth(25);
        ann.apply_to_config(&mut config);
        assert_eq!(config.max_depth, 25);
        let ann2 = AutoAnnotation::UseClassical;
        ann2.apply_to_config(&mut config);
    }
    #[test]
    fn test_auto_annotation_names() {
        let anns = vec![
            AutoAnnotation::MaxDepth(10),
            AutoAnnotation::UseClassical,
            AutoAnnotation::Strategy("bfs".to_string()),
        ];
        let names: Vec<_> = anns.iter().map(|a| a.annotation_name()).collect();
        assert_eq!(names, vec!["max_depth", "classical", "strategy"]);
    }
    #[test]
    fn test_best_first_search_node() {
        let goal = Goal::new(Name::str("g"), Expr::BVar(0));
        let node =
            BestFirstSearchNode::new(goal, 0.9, 3).with_parent_lemma(Name::str("Nat.add_comm"));
        assert!((node.score - 0.9).abs() < 1e-9);
        assert_eq!(node.depth, 3);
        assert!(node.parent_lemma.is_some());
    }
    #[test]
    fn test_search_stats_efficiency() {
        let mut stats = SearchStatistics::new();
        for _ in 0..10 {
            stats.record_iteration();
        }
        for _ in 0..5 {
            stats.record_lemma_application();
        }
        assert!((stats.efficiency() - 0.5).abs() < 1e-9);
    }
}
#[cfg(test)]
mod auto_tactic_extended3 {
    use super::*;
    use crate::tactic_auto::*;
    #[test]
    fn test_hint_filter_chain() {
        let chain = AutoHintFilterChain::new()
            .add(NamePrefixFilter::new("Nat."))
            .add(MinLengthFilter::new(5));
        let goal = Goal::new(Name::str("g"), Expr::BVar(0));
        let hints = vec![
            "Nat.add_comm".to_string(),
            "Nat.x".to_string(),
            "List.length".to_string(),
        ];
        let filtered = chain.filter_hints(&hints, &goal);
        assert!(filtered.iter().all(|h| h.starts_with("Nat.")));
    }
    #[test]
    fn test_budget_tracker() {
        let mut tracker = AutoSearchBudgetTracker::new(5, 1000);
        assert!(!tracker.is_exhausted());
        for _ in 0..5 {
            tracker.consume_node();
        }
        assert!(tracker.is_exhausted());
        assert!((tracker.utilization() - 1.0).abs() < 1e-9);
        tracker.reset();
        assert!(!tracker.is_exhausted());
    }
    #[test]
    fn test_auto_goal_queue() {
        let mut q = AutoGoalQueue::new(10);
        q.enqueue(Goal::new(Name::str("g"), Expr::BVar(0)), 0.5);
        q.enqueue(Goal::new(Name::str("g"), Expr::BVar(1)), 0.9);
        q.enqueue(Goal::new(Name::str("g"), Expr::BVar(2)), 0.3);
        assert_eq!(q.len(), 3);
        let (_, p) = q.dequeue().expect("test operation should succeed");
        assert!((p - 0.9).abs() < 1e-9);
    }
    #[test]
    fn test_lemma_scorer() {
        let mut scorer = AutoLemmaScorer::new();
        scorer.set_score("Nat.add_comm", 0.9);
        scorer.set_score("Nat.zero_add", 0.7);
        scorer.set_score("List.length_nil", 0.3);
        assert!((scorer.score("Nat.add_comm") - 0.9).abs() < 1e-9);
        assert!((scorer.score("unknown") - 0.0).abs() < 1e-9);
        let top = scorer.top_lemmas(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "Nat.add_comm");
    }
    #[test]
    fn test_auto_goal_queue_capacity() {
        let mut q = AutoGoalQueue::new(2);
        q.enqueue(Goal::new(Name::str("g"), Expr::BVar(0)), 0.5);
        q.enqueue(Goal::new(Name::str("g"), Expr::BVar(1)), 0.8);
        q.enqueue(Goal::new(Name::str("g"), Expr::BVar(2)), 0.2);
        assert_eq!(q.len(), 2);
    }
    #[test]
    fn test_auto_tactic_registry_names() {
        let mut reg = AutoTacticRegistry::new();
        let mut cfg1 = AutoConfig::default();
        cfg1.max_depth = 5;
        let mut cfg2 = AutoConfig::default();
        cfg2.max_depth = 10;
        reg.register("quick", cfg1);
        reg.register("deep", cfg2);
        let mut names: Vec<&String> = reg.names();
        names.sort();
        assert_eq!(names, vec!["deep", "quick"]);
    }
}
#[cfg(test)]
mod auto_marker_test {
    use super::*;
    use crate::tactic_auto::*;
    #[test]
    fn test_auto_marker() {
        let _m = AutoTacticExtensionMarker::new();
        assert!(!AutoTacticExtensionMarker::description().is_empty());
    }
}
#[cfg(test)]
mod auto_session_tests {
    use super::*;
    use crate::tactic_auto::*;
    #[test]
    fn test_auto_tactic_session_creation() {
        let session = AutoTacticSession::new(AutoConfig::default());
        assert!(!session.is_budget_exhausted());
        assert!(session.report().is_none());
    }
    #[test]
    fn test_auto_tactic_session_lemma_score() {
        let mut session = AutoTacticSession::new(AutoConfig::default());
        session.set_lemma_score("Nat.add_comm", 0.95);
        assert!((session.scorer.score("Nat.add_comm") - 0.95).abs() < 1e-9);
    }
    #[test]
    fn test_auto_hint_filter_chain_empty() {
        let chain = AutoHintFilterChain::new();
        assert_eq!(chain.filter_count(), 0);
        let goal = Goal::new(Name::str("g"), Expr::BVar(0));
        assert!(chain.accept_all("anything", &goal));
    }
    #[test]
    fn test_auto_budget_tracker_steps() {
        let mut tracker = AutoSearchBudgetTracker::new(100, 1000);
        for _ in 0..50 {
            tracker.consume_node();
        }
        assert!((tracker.utilization() - 0.5).abs() < 1e-9);
    }
}
pub fn tactic_auto_version() -> u32 {
    1
}
pub fn tactic_auto_max_depth_limit() -> u32 {
    10000
}
pub fn tactic_auto_default_beam_width() -> usize {
    8
}
pub fn tactic_auto_search_algorithms() -> &'static [&'static str] {
    &["exhaustive", "heuristic", "best-first", "bidirectional"]
}
pub fn tactic_auto_supports_hints() -> bool {
    true
}
pub fn tactic_auto_supports_budget() -> bool {
    true
}
pub fn tactic_auto_supports_profiling() -> bool {
    true
}
pub fn tactic_auto_supports_annotation() -> bool {
    true
}
#[cfg(test)]
mod auto_version_tests {
    use super::*;
    use crate::tactic_auto::*;
    #[test]
    fn test_version_functions() {
        assert_eq!(tactic_auto_version(), 1);
        assert!(tactic_auto_max_depth_limit() > 0);
        assert!(tactic_auto_supports_hints());
        assert!(!tactic_auto_search_algorithms().is_empty());
    }
}
pub fn tactic_auto_default_time_limit_ms() -> u64 {
    5000
}
pub fn tactic_auto_default_node_limit() -> u64 {
    100_000
}
pub fn tactic_auto_supports_classical() -> bool {
    false
}
pub fn tactic_auto_supports_intuitionistic() -> bool {
    true
}
pub fn tactic_auto_supports_constructive() -> bool {
    true
}
pub fn tactic_auto_hint_database_max_size() -> usize {
    10_000
}
pub fn tactic_auto_default_strategy() -> &'static str {
    "exhaustive"
}
pub fn tactic_auto_supports_backward_chaining() -> bool {
    true
}
pub fn tactic_auto_supports_forward_chaining() -> bool {
    false
}
pub fn tactic_auto_goal_queue_default_capacity() -> usize {
    256
}
pub fn tactic_auto_lemma_score_threshold() -> f64 {
    0.1
}
pub fn tactic_auto_filter_chain_max_filters() -> usize {
    32
}
pub fn tactic_auto_registry_max_entries() -> usize {
    1000
}
pub fn tactic_auto_report_verbosity_default() -> u32 {
    1
}
pub fn tactic_auto_chain_max_length() -> usize {
    16
}
pub fn tactic_auto_goal_priority_default() -> f64 {
    0.5
}
pub fn tactic_auto_backtrack_penalty() -> f64 {
    0.1
}
pub fn tactic_auto_lemma_freshness_bonus() -> f64 {
    0.05
}
pub fn tactic_auto_depth_penalty() -> f64 {
    0.02
}
pub fn tactic_auto_success_reward() -> f64 {
    1.0
}
pub fn tactic_auto_simp_pre_enabled() -> bool {
    true
}
pub fn tactic_auto_decide_fallback() -> bool {
    false
}
pub fn tactic_auto_omega_fallback() -> bool {
    true
}
pub fn tactic_auto_ring_fallback() -> bool {
    true
}
pub fn tactic_auto_trace_enabled_by_default() -> bool {
    false
}
pub fn tactic_auto_proof_irrelevance_check() -> bool {
    false
}
pub fn tactic_auto_type_class_search() -> bool {
    true
}
pub fn tactic_auto_coercion_search() -> bool {
    true
}
pub fn tactic_auto_universe_polymorphism() -> bool {
    true
}
pub fn tactic_auto_definitional_equality() -> bool {
    true
}
pub fn tactic_auto_beta_reduction_in_search() -> bool {
    true
}
pub fn tactic_auto_eta_reduction_in_search() -> bool {
    false
}
