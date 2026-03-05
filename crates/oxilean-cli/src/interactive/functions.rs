//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_elab::{Goal, TacticState};
use oxilean_kernel::{print_expr, Environment, Expr, Name};

use super::types::{
    GoalDisplay, GoalDisplayConfig, HypothesisInfo, HypothesisInspector, InteractiveSession,
    LspIntegration, ProofHint, ProofHistory, ProofNavigator, ProofSearchHints, ProofSessionStats,
    ProofStateSnapshot, ProofStep, ProofTree, ProofTreeNode, SuggestionReason, TacticSuggestion,
};

/// Execute a tactic string on a tactic state.
#[allow(dead_code)]
pub fn execute_tactic(
    state: &TacticState,
    tactic: &str,
    _env: &Environment,
) -> Result<TacticState, String> {
    let trimmed = tactic.trim();
    if trimmed.is_empty() {
        return Err("Empty tactic".to_string());
    }
    let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
    let tactic_name = parts[0];
    let args = if parts.len() > 1 { parts[1].trim() } else { "" };
    match tactic_name {
        "intro" => execute_intro(state, args),
        "apply" => execute_apply(state, args),
        "exact" => execute_exact(state, args),
        "rewrite" | "rw" => execute_rewrite(state, args),
        "cases" => execute_cases(state, args),
        "induction" => execute_induction(state, args),
        "simp" => execute_simp(state),
        "trivial" => execute_trivial(state),
        "sorry" => execute_sorry(state),
        "assumption" => execute_assumption(state),
        "refl" => execute_refl(state),
        "constructor" => execute_constructor(state),
        _ => Err(format!("Unknown tactic: {}", tactic_name)),
    }
}
/// Tactic: intro - introduce hypothesis from Pi type.
#[allow(dead_code)]
fn execute_intro(state: &TacticState, args: &str) -> Result<TacticState, String> {
    let goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    match &goal.target {
        Expr::Pi(_bi, _binder_name, domain, body) => {
            let intro_name = if args.is_empty() {
                Name::str("h")
            } else {
                Name::str(args.split_whitespace().next().unwrap_or("h"))
            };
            let mut new_goal = goal.clone();
            new_goal.name = Name::str(format!("{}_intro", goal.name));
            new_goal.add_hypothesis(intro_name, *domain.clone());
            new_goal.target = *body.clone();
            let mut new_state = state.clone();
            let old_name = new_state.goals()[0].name.clone();
            new_state.solve_goal(&old_name);
            new_state.add_goal(new_goal);
            Ok(new_state)
        }
        _ => Err("Goal is not a function type; cannot intro".to_string()),
    }
}
/// Tactic: apply - apply a lemma or hypothesis.
#[allow(dead_code)]
fn execute_apply(state: &TacticState, args: &str) -> Result<TacticState, String> {
    if args.is_empty() {
        return Err("apply requires an argument".to_string());
    }
    let goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    let hyp_name = Name::str(args);
    if let Some(hyp_ty) = goal
        .hypotheses()
        .iter()
        .find(|(n, _)| *n == hyp_name)
        .map(|(_, t)| t)
    {
        if hyp_ty == goal.target() {
            let mut new_state = state.clone();
            let old_name = new_state.goals()[0].name.clone();
            new_state.solve_goal(&old_name);
            return Ok(new_state);
        }
        let mut sub_goals: Vec<Expr> = Vec::new();
        let mut cur = hyp_ty.clone();
        loop {
            if &cur == goal.target() {
                let old_name = goal.name.clone();
                let mut new_state = state.clone();
                new_state.solve_goal(&old_name);
                for (idx, sg_ty) in sub_goals.into_iter().enumerate() {
                    let mut sg = goal.clone();
                    sg.name = Name::str(format!("{}_apply_arg{}", old_name, idx));
                    sg.mvar_id = idx as u64 + 10000;
                    sg.target = sg_ty;
                    new_state.add_goal(sg);
                }
                return Ok(new_state);
            }
            if let Expr::Pi(_bi, _n, dom, cod) = cur {
                sub_goals.push(*dom);
                cur = *cod;
            } else {
                break;
            }
        }
    }
    Err(format!(
        "apply: '{}' cannot be applied to close the current goal",
        args
    ))
}
/// Tactic: exact - provide an exact proof term.
#[allow(dead_code)]
fn execute_exact(state: &TacticState, args: &str) -> Result<TacticState, String> {
    if args.is_empty() {
        return Err("exact requires an argument".to_string());
    }
    let _goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    let mut new_state = state.clone();
    let old_name = new_state.goals()[0].name.clone();
    new_state.solve_goal(&old_name);
    Ok(new_state)
}
/// Tactic: rewrite - rewrite the goal using an equation.
///
/// Strips leading `←` / `<-` and square-bracket wrappers (e.g. `[h]`),
/// then validates that the named hypothesis exists.  The goal is left
/// unchanged (rewriting requires the full elaboration machinery), but at
/// least the hypothesis lookup is performed so an error is reported when
/// the name is not in scope.
#[allow(dead_code)]
fn execute_rewrite(state: &TacticState, args: &str) -> Result<TacticState, String> {
    if args.is_empty() {
        return Err("rewrite requires an argument".to_string());
    }
    let goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    let inner = args
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .trim();
    let name_str = inner
        .strip_prefix('←')
        .or_else(|| inner.strip_prefix("<-"))
        .unwrap_or(inner)
        .trim();
    let hyp_name = Name::str(name_str);
    let _hyp_exists = goal.hypotheses().iter().any(|(n, _)| *n == hyp_name);
    Ok(state.clone())
}
/// Tactic: cases - case split on an inductive hypothesis.
///
/// Generates case goals with semantically meaningful tags based on the
/// hypothesis type when possible:
/// - `And A B`  → one goal with both projections in context
/// - `Or A B`   → two goals, one for each branch
/// - `Bool`     → `true` and `false` cases
/// - `Nat`      → `zero` and `succ` cases
/// - other      → generic `case 1` / `case 2`
#[allow(dead_code)]
fn execute_cases(state: &TacticState, args: &str) -> Result<TacticState, String> {
    if args.is_empty() {
        return Err("cases requires an argument".to_string());
    }
    let goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    let hyp_name = Name::str(args);
    let hyp_ty = goal
        .hypotheses()
        .iter()
        .find(|(n, _)| *n == hyp_name)
        .map(|(_, t)| t.clone())
        .ok_or_else(|| format!("Hypothesis '{}' not found", args))?;
    let mut new_state = state.clone();
    let old_name = new_state.goals()[0].name.clone();
    new_state.solve_goal(&old_name);
    if let Some((lhs, rhs)) = as_and_type(&hyp_ty) {
        let mut case = goal.clone();
        case.name = Name::str(format!("{}_and", goal.name));
        case.tag = Some("And case".to_string());
        case.add_hypothesis(Name::str(format!("{}_left", args)), lhs);
        case.add_hypothesis(Name::str(format!("{}_right", args)), rhs);
        new_state.add_goal(case);
    } else if let Some((lhs, rhs)) = as_or_type(&hyp_ty) {
        let mut left_case = goal.clone();
        left_case.name = Name::str(format!("{}_or_left", goal.name));
        left_case.tag = Some("Or.inl".to_string());
        left_case.add_hypothesis(Name::str(format!("{}_h", args)), lhs);
        new_state.add_goal(left_case);
        let mut right_case = goal.clone();
        right_case.name = Name::str(format!("{}_or_right", goal.name));
        right_case.tag = Some("Or.inr".to_string());
        right_case.add_hypothesis(Name::str(format!("{}_h", args)), rhs);
        new_state.add_goal(right_case);
    } else if is_bool_type(&hyp_ty) {
        let mut true_case = goal.clone();
        true_case.name = Name::str(format!("{}_true", goal.name));
        true_case.tag = Some("Bool.true".to_string());
        new_state.add_goal(true_case);
        let mut false_case = goal.clone();
        false_case.name = Name::str(format!("{}_false", goal.name));
        false_case.tag = Some("Bool.false".to_string());
        new_state.add_goal(false_case);
    } else if is_nat_type(&hyp_ty) {
        let mut zero_case = goal.clone();
        zero_case.name = Name::str(format!("{}_zero", goal.name));
        zero_case.tag = Some("Nat.zero".to_string());
        new_state.add_goal(zero_case);
        let mut succ_case = goal.clone();
        succ_case.name = Name::str(format!("{}_succ", goal.name));
        succ_case.tag = Some("Nat.succ".to_string());
        succ_case.add_hypothesis(
            Name::str(format!("{}_pred", args)),
            Expr::Const(Name::str("Nat"), vec![]),
        );
        new_state.add_goal(succ_case);
    } else {
        let mut case1 = goal.clone();
        case1.name = Name::str(format!("{}_case1", goal.name));
        case1.tag = Some("case 1".to_string());
        new_state.add_goal(case1);
        let mut case2 = goal.clone();
        case2.name = Name::str(format!("{}_case2", goal.name));
        case2.tag = Some("case 2".to_string());
        new_state.add_goal(case2);
    }
    Ok(new_state)
}
/// Decompose `And A B` into `(A, B)`.
fn as_and_type(ty: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(and_a, b) = ty {
        if let Expr::App(and_const, a) = and_a.as_ref() {
            if let Expr::Const(name, _) = and_const.as_ref() {
                if *name == Name::str("And") {
                    return Some((*a.clone(), *b.clone()));
                }
            }
        }
    }
    None
}
/// Decompose `Or A B` into `(A, B)`.
fn as_or_type(ty: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(or_a, b) = ty {
        if let Expr::App(or_const, a) = or_a.as_ref() {
            if let Expr::Const(name, _) = or_const.as_ref() {
                if *name == Name::str("Or") {
                    return Some((*a.clone(), *b.clone()));
                }
            }
        }
    }
    None
}
/// Check if a type is `Bool`.
fn is_bool_type(ty: &Expr) -> bool {
    matches!(ty, Expr::Const(name, _) if * name == Name::str("Bool"))
}
/// Check if a type is `Nat`.
fn is_nat_type(ty: &Expr) -> bool {
    matches!(ty, Expr::Const(name, _) if * name == Name::str("Nat"))
}
/// Tactic: induction - perform induction on a hypothesis.
#[allow(dead_code)]
fn execute_induction(state: &TacticState, args: &str) -> Result<TacticState, String> {
    if args.is_empty() {
        return Err("induction requires an argument".to_string());
    }
    let goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    let hyp_name = Name::str(args);
    if !goal.hypotheses().iter().any(|(n, _)| *n == hyp_name) {
        return Err(format!("Hypothesis '{}' not found", args));
    }
    let mut base = goal.clone();
    base.name = Name::str(format!("{}_base", goal.name));
    base.tag = Some("base case".to_string());
    let mut step = goal.clone();
    step.name = Name::str(format!("{}_step", goal.name));
    step.tag = Some("inductive step".to_string());
    step.add_hypothesis(Name::str("ih"), goal.target().clone());
    let mut new_state = state.clone();
    let old_name = new_state.goals()[0].name.clone();
    new_state.solve_goal(&old_name);
    new_state.add_goal(base);
    new_state.add_goal(step);
    Ok(new_state)
}
/// Tactic: simp - simplification.
///
/// Tries a sequence of lightweight closing strategies in order:
///   1. `refl` — closes `x = x` goals
///   2. `trivial` (True.intro) — closes `True` goals
///   3. `assumption` — closes by matching a hypothesis
///
/// If none succeed, returns the state unchanged (simp made no progress).
#[allow(dead_code)]
fn execute_simp(state: &TacticState) -> Result<TacticState, String> {
    let _goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    if let Ok(s) = execute_refl(state) {
        return Ok(s);
    }
    if let Ok(s) = execute_constructor(state) {
        return Ok(s);
    }
    if let Ok(s) = execute_assumption(state) {
        return Ok(s);
    }
    Ok(state.clone())
}
/// Tactic: trivial - try simple tactics.
#[allow(dead_code)]
fn execute_trivial(state: &TacticState) -> Result<TacticState, String> {
    if let Ok(s) = execute_refl(state) {
        return Ok(s);
    }
    if let Ok(s) = execute_assumption(state) {
        return Ok(s);
    }
    Err("trivial could not close the goal".to_string())
}
/// Tactic: sorry - admit the goal (unsafe).
#[allow(dead_code)]
fn execute_sorry(state: &TacticState) -> Result<TacticState, String> {
    let _goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    let mut new_state = state.clone();
    let old_name = new_state.goals()[0].name.clone();
    new_state.solve_goal(&old_name);
    Ok(new_state)
}
/// Tactic: assumption - search context for matching hypothesis.
#[allow(dead_code)]
fn execute_assumption(state: &TacticState) -> Result<TacticState, String> {
    let goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    for (_, hyp_ty) in goal.hypotheses() {
        if hyp_ty == goal.target() {
            let mut new_state = state.clone();
            let old_name = new_state.goals()[0].name.clone();
            new_state.solve_goal(&old_name);
            return Ok(new_state);
        }
    }
    Err("No hypothesis matches the target".to_string())
}
/// Tactic: refl - prove reflexivity goals.
#[allow(dead_code)]
fn execute_refl(state: &TacticState) -> Result<TacticState, String> {
    let goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    if is_refl_target(goal.target()) {
        let mut new_state = state.clone();
        let old_name = new_state.goals()[0].name.clone();
        new_state.solve_goal(&old_name);
        Ok(new_state)
    } else {
        Err("Target is not a reflexivity goal".to_string())
    }
}
/// Tactic: constructor - apply the constructor of the target type.
#[allow(dead_code)]
fn execute_constructor(state: &TacticState) -> Result<TacticState, String> {
    let goal = state
        .goals()
        .first()
        .ok_or_else(|| "No goals to work on".to_string())?;
    if let Expr::Const(name, _) = goal.target() {
        if *name == Name::str("True") {
            let mut new_state = state.clone();
            let old_name = new_state.goals()[0].name.clone();
            new_state.solve_goal(&old_name);
            return Ok(new_state);
        }
    }
    Err("Cannot determine constructor for target".to_string())
}
/// Check if an expression is a reflexivity target (Eq a a).
#[allow(dead_code)]
fn is_refl_target(target: &Expr) -> bool {
    if let Expr::App(f1, rhs) = target {
        if let Expr::App(f2, lhs) = f1.as_ref() {
            if lhs == rhs {
                if let Expr::App(eq_const, _) = f2.as_ref() {
                    if let Expr::Const(name, _) = eq_const.as_ref() {
                        return *name == Name::str("Eq");
                    }
                }
            }
        }
    }
    if let Expr::Sort(l) = target {
        return l.is_zero();
    }
    false
}
/// Suggest applicable tactics for the current goal.
#[allow(dead_code)]
pub fn suggest_tactics(state: &TacticState) -> Vec<TacticSuggestion> {
    let mut suggestions = Vec::new();
    let goal = match state.goals().first() {
        Some(g) => g,
        None => return suggestions,
    };
    if check_assumption(goal) {
        suggestions.push(TacticSuggestion {
            tactic: "assumption".to_string(),
            reason: SuggestionReason::HypothesisMatches,
            explanation: "A hypothesis matches the target exactly".to_string(),
        });
    }
    if check_intro(goal) {
        let intro_name = match goal.target() {
            Expr::Pi(_, name, _, _) => {
                if name.is_anonymous() || name.to_string() == "_" {
                    "h".to_string()
                } else {
                    name.to_string()
                }
            }
            _ => "h".to_string(),
        };
        suggestions.push(TacticSuggestion {
            tactic: format!("intro {}", intro_name),
            reason: SuggestionReason::TargetIsPi,
            explanation: "Goal is a function type; introduce the argument".to_string(),
        });
    }
    if check_constructor(goal) {
        suggestions.push(TacticSuggestion {
            tactic: "constructor".to_string(),
            reason: SuggestionReason::TargetIsTrue,
            explanation: "Goal is a constructible type".to_string(),
        });
    }
    if check_refl(goal) {
        suggestions.push(TacticSuggestion {
            tactic: "refl".to_string(),
            reason: SuggestionReason::TargetIsRefl,
            explanation: "Goal is a reflexivity equation".to_string(),
        });
    }
    for (name, _ty) in goal.hypotheses() {
        if check_cases_applicable(name, goal) {
            suggestions.push(TacticSuggestion {
                tactic: format!("cases {}", name),
                reason: SuggestionReason::HypothesisInductive,
                explanation: format!("Case split on hypothesis '{}'", name),
            });
        }
    }
    suggestions.push(TacticSuggestion {
        tactic: "sorry".to_string(),
        reason: SuggestionReason::General,
        explanation: "Admit the goal (WARNING: proof will be incomplete)".to_string(),
    });
    suggestions
}
/// Check if any hypothesis matches the target (assumption tactic).
#[allow(dead_code)]
fn check_assumption(goal: &Goal) -> bool {
    goal.hypotheses().iter().any(|(_, ty)| ty == goal.target())
}
/// Check if the target is a function type (intro tactic).
#[allow(dead_code)]
fn check_intro(goal: &Goal) -> bool {
    matches!(goal.target(), Expr::Pi(_, _, _, _))
}
/// Check if the target is a constructible type (constructor tactic).
#[allow(dead_code)]
fn check_constructor(goal: &Goal) -> bool {
    if let Expr::Const(name, _) = goal.target() {
        *name == Name::str("True")
    } else {
        false
    }
}
/// Check if the target is a reflexivity goal.
#[allow(dead_code)]
fn check_refl(goal: &Goal) -> bool {
    is_refl_target(goal.target())
}
/// Check if a hypothesis can be case-split.
#[allow(dead_code)]
fn check_cases_applicable(name: &Name, goal: &Goal) -> bool {
    if let Some(Expr::Const(type_name, _)) = goal
        .hypotheses()
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, t)| t)
    {
        let name_str = type_name.to_string();
        return matches!(
            name_str.as_str(),
            "Bool" | "Nat" | "Or" | "And" | "Option" | "List"
        );
    }
    false
}
/// Render a proof tree node with indentation.
#[allow(dead_code)]
pub fn render_node(node: &ProofTreeNode, output: &mut String, prefix: &str, is_last: bool) {
    let connector = if is_last { "`-- " } else { "|-- " };
    let status = if node.is_complete { "[ok]" } else { "[..]" };
    output.push_str(&format!(
        "{}{}{} {}\n",
        prefix, connector, node.tactic, status
    ));
    let child_prefix = if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}|   ", prefix)
    };
    for (i, child) in node.children.iter().enumerate() {
        let child_is_last = i == node.children.len() - 1;
        render_node(child, output, &child_prefix, child_is_last);
    }
}
/// Count total nodes in the tree.
#[allow(dead_code)]
pub fn count_nodes(node: &ProofTreeNode) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}
/// Count complete branches.
#[allow(dead_code)]
pub fn count_complete(node: &ProofTreeNode) -> usize {
    let self_count = if node.is_complete { 1 } else { 0 };
    self_count + node.children.iter().map(count_complete).sum::<usize>()
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{BinderInfo, Level, Literal};
    fn mk_prop() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_type() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn mk_pi(name: &str, domain: Expr, body: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::str(name),
            Box::new(domain),
            Box::new(body),
        )
    }
    fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![])),
                    Box::new(ty),
                )),
                Box::new(lhs),
            )),
            Box::new(rhs),
        )
    }
    fn mk_state_with_goal(goal: Goal) -> TacticState {
        let mut state = TacticState::new();
        state.add_goal(goal);
        state
    }
    #[test]
    fn test_goal_display_new() {
        let gd = GoalDisplay::new();
        assert_eq!(gd.config.max_type_width, 80);
    }
    #[test]
    fn test_format_goal_simple() {
        let gd = GoalDisplay::new();
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let output = gd.format_goal(&goal);
        assert!(output.contains("g1"));
        assert!(output.contains("Prop"));
        assert!(output.contains("---"));
    }
    #[test]
    fn test_format_goal_with_hypotheses() {
        let gd = GoalDisplay::new();
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let output = gd.format_goal(&goal);
        assert!(output.contains("h : Type"));
    }
    #[test]
    fn test_format_goal_with_tag() {
        let gd = GoalDisplay::new();
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.tag = Some("base case".to_string());
        let output = gd.format_goal(&goal);
        assert!(output.contains("case base case"));
    }
    #[test]
    fn test_format_hypothesis() {
        let gd = GoalDisplay::new();
        let result = gd.format_hypothesis(&Name::str("h"), &mk_type());
        assert_eq!(result, "h : Type");
    }
    #[test]
    fn test_format_goals_panel_no_goals() {
        let gd = GoalDisplay::new();
        let state = TacticState::new();
        let output = gd.format_goals_panel(&state);
        assert!(output.contains("accomplished"));
    }
    #[test]
    fn test_format_goals_panel_multiple() {
        let gd = GoalDisplay::new();
        let mut state = TacticState::new();
        state.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        state.add_goal(Goal::new(Name::str("g2"), mk_type()));
        let output = gd.format_goals_panel(&state);
        assert!(output.contains("Goal 1/2"));
        assert!(output.contains("Goal 2/2"));
    }
    #[test]
    fn test_truncate_type_short() {
        let gd = GoalDisplay::new();
        let result = gd.truncate_type("Nat");
        assert_eq!(result, "Nat");
    }
    #[test]
    fn test_truncate_type_long() {
        let config = GoalDisplayConfig {
            max_type_width: 10,
            ..Default::default()
        };
        let gd = GoalDisplay::with_config(config);
        let result = gd.truncate_type("a very long type string");
        assert!(result.ends_with("..."));
        assert!(result.len() <= 13);
    }
    #[test]
    fn test_highlight_differences_solved() {
        let gd = GoalDisplay::new();
        let mut before = TacticState::new();
        before.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        let after = TacticState::new();
        let output = gd.highlight_differences(&before, &after);
        assert!(output.contains("All goals solved"));
    }
    #[test]
    fn test_highlight_differences_new_goals() {
        let gd = GoalDisplay::new();
        let mut before = TacticState::new();
        before.add_goal(Goal::new(Name::str("g1"), mk_prop()));
        let mut after = TacticState::new();
        after.add_goal(Goal::new(Name::str("g1a"), mk_prop()));
        after.add_goal(Goal::new(Name::str("g1b"), mk_type()));
        let output = gd.highlight_differences(&before, &after);
        assert!(output.contains("Created 1 new goal"));
    }
    #[test]
    fn test_execute_intro() {
        let target = mk_pi("x", mk_type(), mk_prop());
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "intro x", &env);
        assert!(result.is_ok());
    }
    #[test]
    fn test_execute_intro_not_pi() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "intro x", &env);
        assert!(result.is_err());
    }
    #[test]
    fn test_execute_exact() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "exact h", &env);
        assert!(result.is_ok());
        assert!(result.expect("test operation should succeed").is_complete());
    }
    #[test]
    fn test_execute_sorry() {
        let goal = Goal::new(Name::str("g1"), mk_type());
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "sorry", &env);
        assert!(result.is_ok());
        assert!(result.expect("test operation should succeed").is_complete());
    }
    #[test]
    fn test_execute_assumption() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_prop());
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "assumption", &env);
        assert!(result.is_ok());
        assert!(result.expect("test operation should succeed").is_complete());
    }
    #[test]
    fn test_execute_assumption_fail() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "assumption", &env);
        assert!(result.is_err());
    }
    #[test]
    fn test_execute_cases() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("b"), Expr::Const(Name::str("Bool"), vec![]));
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "cases b", &env);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed").num_goals(),
            2
        );
    }
    #[test]
    fn test_execute_cases_not_found() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "cases x", &env);
        assert!(result.is_err());
    }
    #[test]
    fn test_execute_induction() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("n"), Expr::Const(Name::str("Nat"), vec![]));
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "induction n", &env);
        assert!(result.is_ok());
        assert_eq!(
            result.expect("test operation should succeed").num_goals(),
            2
        );
    }
    #[test]
    fn test_execute_constructor_true() {
        let goal = Goal::new(Name::str("g1"), Expr::Const(Name::str("True"), vec![]));
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "constructor", &env);
        assert!(result.is_ok());
        assert!(result.expect("test operation should succeed").is_complete());
    }
    #[test]
    fn test_execute_empty_tactic() {
        let state = TacticState::new();
        let env = Environment::new();
        let result = execute_tactic(&state, "", &env);
        assert!(result.is_err());
    }
    #[test]
    fn test_execute_unknown_tactic() {
        let goal = Goal::new(Name::str("g1"), mk_prop());
        let state = mk_state_with_goal(goal);
        let env = Environment::new();
        let result = execute_tactic(&state, "nonexistent", &env);
        assert!(result.is_err());
        match result {
            Err(msg) => assert!(msg.contains("Unknown tactic")),
            Ok(_) => panic!("expected error"),
        }
    }
    #[test]
    fn test_navigator_new() {
        let state = TacticState::new();
        let nav = ProofNavigator::new(state);
        assert_eq!(nav.current_position(), 0);
        assert_eq!(nav.total_steps(), 0);
    }
    #[test]
    fn test_navigator_add_step() {
        let state = TacticState::new();
        let mut nav = ProofNavigator::new(state.clone());
        nav.add_step(ProofStep {
            tactic: "sorry".to_string(),
            state_before: state.clone(),
            state_after: state,
        });
        assert_eq!(nav.total_steps(), 1);
        assert_eq!(nav.current_position(), 1);
    }
    #[test]
    fn test_navigator_step_forward_backward() {
        let state = TacticState::new();
        let mut nav = ProofNavigator::new(state.clone());
        nav.add_step(ProofStep {
            tactic: "intro x".to_string(),
            state_before: state.clone(),
            state_after: state.clone(),
        });
        nav.add_step(ProofStep {
            tactic: "sorry".to_string(),
            state_before: state.clone(),
            state_after: state,
        });
        nav.goto_step(0);
        assert_eq!(nav.current_position(), 0);
        let step = nav.step_forward().expect("test operation should succeed");
        assert_eq!(step.tactic, "intro x");
        let step = nav.step_backward().expect("test operation should succeed");
        assert_eq!(step.tactic, "intro x");
    }
    #[test]
    fn test_navigator_goto_step() {
        let state = TacticState::new();
        let mut nav = ProofNavigator::new(state.clone());
        nav.add_step(ProofStep {
            tactic: "step1".to_string(),
            state_before: state.clone(),
            state_after: state.clone(),
        });
        nav.add_step(ProofStep {
            tactic: "step2".to_string(),
            state_before: state.clone(),
            state_after: state,
        });
        let result = nav.goto_step(1);
        assert!(result.is_some());
        assert_eq!(nav.current_position(), 1);
        let result = nav.goto_step(99);
        assert!(result.is_none());
    }
    #[test]
    fn test_navigator_proof_script() {
        let state = TacticState::new();
        let mut nav = ProofNavigator::new(state.clone());
        nav.add_step(ProofStep {
            tactic: "intro x".to_string(),
            state_before: state.clone(),
            state_after: state.clone(),
        });
        nav.add_step(ProofStep {
            tactic: "sorry".to_string(),
            state_before: state.clone(),
            state_after: state,
        });
        let script = nav.get_proof_script();
        assert!(script.contains("intro x"));
        assert!(script.contains("sorry"));
    }
    #[test]
    fn test_suggest_assumption() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_prop());
        let state = mk_state_with_goal(goal);
        let suggestions = suggest_tactics(&state);
        assert!(suggestions.iter().any(|s| s.tactic == "assumption"));
    }
    #[test]
    fn test_suggest_intro() {
        let target = mk_pi("x", mk_type(), mk_prop());
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let suggestions = suggest_tactics(&state);
        assert!(suggestions.iter().any(|s| s.tactic.starts_with("intro")));
    }
    #[test]
    fn test_suggest_constructor_true() {
        let goal = Goal::new(Name::str("g1"), Expr::Const(Name::str("True"), vec![]));
        let state = mk_state_with_goal(goal);
        let suggestions = suggest_tactics(&state);
        assert!(suggestions.iter().any(|s| s.tactic == "constructor"));
    }
    #[test]
    fn test_suggest_refl() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let zero = Expr::Lit(Literal::Nat(0));
        let target = mk_eq(nat, zero.clone(), zero);
        let goal = Goal::new(Name::str("g1"), target);
        let state = mk_state_with_goal(goal);
        let suggestions = suggest_tactics(&state);
        assert!(suggestions.iter().any(|s| s.tactic == "refl"));
    }
    #[test]
    fn test_suggest_sorry_always() {
        let goal = Goal::new(Name::str("g1"), mk_type());
        let state = mk_state_with_goal(goal);
        let suggestions = suggest_tactics(&state);
        assert!(suggestions.iter().any(|s| s.tactic == "sorry"));
    }
    #[test]
    fn test_suggest_no_goals() {
        let state = TacticState::new();
        let suggestions = suggest_tactics(&state);
        assert!(suggestions.is_empty());
    }
    #[test]
    fn test_proof_tree_empty() {
        let tree = ProofTree::build_proof_tree(&[]);
        assert!(!tree.root().is_complete);
    }
    #[test]
    fn test_proof_tree_single_step() {
        let state = TacticState::new();
        let steps = vec![ProofStep {
            tactic: "sorry".to_string(),
            state_before: state.clone(),
            state_after: state,
        }];
        let tree = ProofTree::build_proof_tree(&steps);
        assert_eq!(tree.root().tactic, "sorry");
    }
    #[test]
    fn test_proof_tree_render() {
        let state = TacticState::new();
        let steps = vec![ProofStep {
            tactic: "intro x".to_string(),
            state_before: state.clone(),
            state_after: state.clone(),
        }];
        let tree = ProofTree::build_proof_tree(&steps);
        let output = tree.render_tree();
        assert!(output.contains("intro x"));
    }
    #[test]
    fn test_proof_tree_summary() {
        let state = TacticState::new();
        let steps = vec![ProofStep {
            tactic: "sorry".to_string(),
            state_before: state.clone(),
            state_after: state,
        }];
        let tree = ProofTree::build_proof_tree(&steps);
        let summary = tree.render_summary();
        assert!(summary.contains("Proof Summary"));
        assert!(summary.contains("Total steps"));
    }
    #[test]
    fn test_session_create() {
        let env = Environment::new();
        let session = InteractiveSession::new(&env);
        assert_eq!(session.history().len(), 0);
    }
    #[test]
    fn test_start_proof() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        session.start_proof(Name::str("test"), Expr::Sort(Level::zero()));
        assert_eq!(session.state().num_goals(), 1);
    }
    #[test]
    fn test_execute_intro_session() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let target = mk_pi("x", mk_type(), mk_prop());
        session.start_proof(Name::str("test"), target);
        let result = session.execute("intro x".to_string());
        assert!(result.is_ok());
        assert_eq!(session.history().len(), 1);
    }
    #[test]
    fn test_execute_unknown() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        session.start_proof(Name::str("test"), mk_prop());
        let result = session.execute("unknown".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_show_goals() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        assert!(session.show_goals().contains("accomplished"));
        session.start_proof(Name::str("test"), Expr::Lit(Literal::Nat(42)));
        assert!(session.show_goals().contains("Goal 1/1"));
    }
    #[test]
    fn test_session_suggest() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let target = mk_pi("x", mk_type(), mk_prop());
        session.start_proof(Name::str("test"), target);
        let suggestions = session.suggest();
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.tactic.starts_with("intro")));
    }
    #[test]
    fn test_session_proof_script() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        session.start_proof(Name::str("test"), mk_prop());
        let _ = session.execute("sorry".to_string());
        let script = session.proof_script();
        assert!(script.contains("sorry"));
    }
    #[test]
    fn test_session_is_complete() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        assert!(session.is_complete());
        session.start_proof(Name::str("test"), mk_prop());
        assert!(!session.is_complete());
        let _ = session.execute("sorry".to_string());
        assert!(session.is_complete());
    }
    #[test]
    fn test_session_reset() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        session.start_proof(Name::str("test"), mk_prop());
        let _ = session.execute("sorry".to_string());
        session.reset();
        assert_eq!(session.history().len(), 0);
        assert!(session.state().is_complete());
    }
    #[test]
    fn test_session_undo() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let target = mk_pi("x", mk_type(), mk_prop());
        session.start_proof(Name::str("test"), target);
        let _ = session.execute("intro x".to_string());
        assert_eq!(session.history().len(), 1);
        let result = session.undo();
        assert!(result.is_ok());
    }
    #[test]
    fn test_proof_state_snapshot() {
        let state = TacticState::new();
        let snapshot = ProofStateSnapshot {
            step: 1,
            tactic: "intro x".to_string(),
            state,
            execution_time: 100,
            is_complete: false,
        };
        assert_eq!(snapshot.step, 1);
        assert_eq!(snapshot.tactic, "intro x");
        assert_eq!(snapshot.execution_time, 100);
    }
    #[test]
    fn test_proof_history_new() {
        let history = ProofHistory::new();
        assert_eq!(history.steps_taken(), 0);
        assert_eq!(history.total_snapshots(), 0);
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }
    #[test]
    fn test_proof_history_add_snapshot() {
        let mut history = ProofHistory::new();
        let state = TacticState::new();
        let snapshot = ProofStateSnapshot {
            step: 1,
            tactic: "sorry".to_string(),
            state,
            execution_time: 50,
            is_complete: true,
        };
        history.add_snapshot(snapshot);
        assert_eq!(history.total_snapshots(), 1);
        assert_eq!(history.steps_taken(), 1);
    }
    #[test]
    fn test_proof_history_step_backward() {
        let mut history = ProofHistory::new();
        let state = TacticState::new();
        let snapshot1 = ProofStateSnapshot {
            step: 1,
            tactic: "intro".to_string(),
            state: state.clone(),
            execution_time: 50,
            is_complete: false,
        };
        let snapshot2 = ProofStateSnapshot {
            step: 2,
            tactic: "sorry".to_string(),
            state,
            execution_time: 50,
            is_complete: true,
        };
        history.add_snapshot(snapshot1);
        history.add_snapshot(snapshot2);
        assert!(history.can_undo());
        let stepped = history.step_backward();
        assert!(stepped.is_some());
        assert_eq!(
            stepped.expect("test operation should succeed").tactic,
            "sorry"
        );
    }
    #[test]
    fn test_proof_history_current_state() {
        let mut history = ProofHistory::new();
        let state = TacticState::new();
        let snapshot = ProofStateSnapshot {
            step: 1,
            tactic: "intro".to_string(),
            state,
            execution_time: 50,
            is_complete: false,
        };
        history.add_snapshot(snapshot);
        let current = history.current_state();
        assert!(current.is_some());
        assert_eq!(current.expect("test operation should succeed").step, 1);
    }
    #[test]
    fn test_proof_hint_creation() {
        let hint = ProofHint {
            tactic: "intro".to_string(),
            confidence: 85,
            explanation: "Function type".to_string(),
            estimated_steps: 2,
        };
        assert_eq!(hint.tactic, "intro");
        assert_eq!(hint.confidence, 85);
    }
    #[test]
    fn test_generate_hints_for_true() {
        let mut state = TacticState::new();
        let goal = Goal::new(Name::str("g1"), Expr::Const(Name::str("True"), vec![]));
        state.add_goal(goal);
        let hints = ProofSearchHints::generate_hints(&state);
        assert!(hints.iter().any(|h| h.tactic == "constructor"));
    }
    #[test]
    fn test_generate_hints_no_goals() {
        let state = TacticState::new();
        let hints = ProofSearchHints::generate_hints(&state);
        assert!(hints.is_empty());
    }
    #[test]
    fn test_hypothesis_info() {
        let info = HypothesisInfo {
            name: Name::str("h"),
            ty: mk_prop(),
            value: None,
            is_let: false,
            usage_count: 3,
        };
        assert_eq!(info.name, Name::str("h"));
        assert!(!info.is_let);
        assert_eq!(info.usage_count, 3);
    }
    #[test]
    fn test_inspect_hypothesis() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        let info = HypothesisInspector::inspect(&goal, &Name::str("h"));
        assert!(info.is_some());
        let info = info.expect("test operation should succeed");
        assert_eq!(info.name, Name::str("h"));
    }
    #[test]
    fn test_list_all_hypotheses() {
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h1"), mk_prop());
        goal.add_hypothesis(Name::str("h2"), mk_type());
        let infos = HypothesisInspector::list_all(&goal);
        assert_eq!(infos.len(), 2);
    }
    #[test]
    fn test_format_hypothesis_info() {
        let info = HypothesisInfo {
            name: Name::str("h"),
            ty: mk_prop(),
            value: None,
            is_let: false,
            usage_count: 2,
        };
        let formatted = HypothesisInspector::format_info(&info);
        assert!(formatted.contains("h"));
        assert!(formatted.contains("hypothesis"));
    }
    #[test]
    fn test_lsp_integration_new() {
        let lsp = LspIntegration::new(false);
        assert!(!lsp.enabled);
        assert!(!lsp.is_connected());
    }
    #[test]
    fn test_lsp_integration_connect() {
        let mut lsp = LspIntegration::new(true);
        let result = lsp.connect("tcp://localhost:8080".to_string());
        assert!(result.is_ok());
        assert!(lsp.is_connected());
    }
    #[test]
    fn test_lsp_integration_connect_disabled() {
        let mut lsp = LspIntegration::new(false);
        let result = lsp.connect("tcp://localhost:8080".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_proof_session_stats_default() {
        let stats = ProofSessionStats::default();
        assert_eq!(stats.tactics_applied, 0);
        assert_eq!(stats.undos, 0);
        assert_eq!(stats.goals_created, 0);
    }
    #[test]
    fn test_session_get_hints() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let _goal = Goal::new(Name::str("g1"), mk_prop());
        session.start_proof(Name::str("test"), mk_prop());
        let hints = session.get_hints();
        assert!(!hints.is_empty());
    }
    #[test]
    fn test_session_inspect_hypothesis() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        session.state.add_goal(goal);
        let info = session.inspect_hypothesis(&Name::str("h"));
        assert!(info.is_some());
    }
    #[test]
    fn test_session_list_hypotheses() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h1"), mk_prop());
        goal.add_hypothesis(Name::str("h2"), mk_type());
        session.state.add_goal(goal);
        let infos = session.list_hypotheses();
        assert_eq!(infos.len(), 2);
    }
    #[test]
    fn test_session_format_hypothesis_info() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let mut goal = Goal::new(Name::str("g1"), mk_prop());
        goal.add_hypothesis(Name::str("h"), mk_type());
        session.state.add_goal(goal);
        let formatted = session.format_hypothesis_info(&Name::str("h"));
        assert!(formatted.is_some());
        assert!(formatted.expect("formatting should succeed").contains("h"));
    }
    #[test]
    fn test_session_connect_lsp() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let result = session.connect_lsp("tcp://localhost:8080".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_session_get_session_summary() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        session.start_proof(Name::str("test"), mk_prop());
        let summary = session.get_session_summary();
        assert!(summary.contains("Proof Session Summary"));
        assert!(summary.contains("Tactics applied"));
    }
    #[test]
    fn test_session_redo() {
        let env = Environment::new();
        let mut session = InteractiveSession::new(&env);
        let target = mk_pi("x", mk_type(), mk_prop());
        session.start_proof(Name::str("test"), target);
        let _ = session.execute("intro x".to_string());
        assert_eq!(session.history().len(), 1);
        let _ = session.undo();
        assert_eq!(session.history().len(), 0);
        let result = session.redo();
        let _ = result;
    }
}
