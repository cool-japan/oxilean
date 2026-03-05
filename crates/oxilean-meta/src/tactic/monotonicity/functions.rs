//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    MonoChain, MonoConclusion, MonoConfig, MonoPremise, MonoRelation, MonoResult, MonoRule,
    MonoRuleSet, MonoStats, MonoSubGoal, TacticMonotonicityAnalysisPass, TacticMonotonicityConfig,
    TacticMonotonicityConfigValue, TacticMonotonicityDiagnostics, TacticMonotonicityDiff,
    TacticMonotonicityPipeline, TacticMonotonicityResult,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};
use std::collections::{HashMap, HashSet};

/// Default maximum depth for recursive monotonicity application.
pub(super) const DEFAULT_MONO_MAX_DEPTH: usize = 8;
/// Maximum number of monotonicity rules in a rule set.
pub(super) const MAX_MONO_RULES: usize = 1024;
/// Default priority for built-in monotonicity rules.
const DEFAULT_MONO_PRIORITY: u32 = 1000;
/// High priority for exact-match rules.
const HIGH_MONO_PRIORITY: u32 = 100;
/// Low priority for fallback rules.
const LOW_MONO_PRIORITY: u32 = 5000;
/// Decompose a relational expression into `(lhs, rhs, relation)`.
///
/// Handles patterns like:
/// - `@LE.le α inst a b` -> (a, b, Le)
/// - `@LT.lt α inst a b` -> (a, b, Lt)
/// - `@Eq α a b` -> (a, b, custom "Eq")
pub fn decompose_relation(expr: &Expr) -> Option<(Expr, Expr, MonoRelation)> {
    let (head, args) = collect_app_args(expr);
    if let Expr::Const(name, _) = &head {
        let name_str = name.to_string();
        if name_str.contains("LE.le") || name_str.contains("le") {
            if args.len() >= 4 {
                return Some((args[2].clone(), args[3].clone(), MonoRelation::Le));
            }
            if args.len() >= 2 {
                return Some((
                    args[args.len() - 2].clone(),
                    args[args.len() - 1].clone(),
                    MonoRelation::Le,
                ));
            }
        }
        if name_str.contains("LT.lt") || name_str.contains("lt") {
            if args.len() >= 4 {
                return Some((args[2].clone(), args[3].clone(), MonoRelation::Lt));
            }
            if args.len() >= 2 {
                return Some((
                    args[args.len() - 2].clone(),
                    args[args.len() - 1].clone(),
                    MonoRelation::Lt,
                ));
            }
        }
        if name_str.contains("GE.ge") || name_str.contains("ge") {
            if args.len() >= 4 {
                return Some((args[2].clone(), args[3].clone(), MonoRelation::Ge));
            }
            if args.len() >= 2 {
                return Some((
                    args[args.len() - 2].clone(),
                    args[args.len() - 1].clone(),
                    MonoRelation::Ge,
                ));
            }
        }
        if name_str.contains("GT.gt") || name_str.contains("gt") {
            if args.len() >= 4 {
                return Some((args[2].clone(), args[3].clone(), MonoRelation::Gt));
            }
            if args.len() >= 2 {
                return Some((
                    args[args.len() - 2].clone(),
                    args[args.len() - 1].clone(),
                    MonoRelation::Gt,
                ));
            }
        }
        if (name_str.contains("Dvd.dvd") || name_str.contains("dvd")) && args.len() >= 2 {
            return Some((
                args[args.len() - 2].clone(),
                args[args.len() - 1].clone(),
                MonoRelation::Dvd,
            ));
        }
        if (name_str.contains("HasSubset.subset") || name_str.contains("subset")) && args.len() >= 2
        {
            return Some((
                args[args.len() - 2].clone(),
                args[args.len() - 1].clone(),
                MonoRelation::Subset,
            ));
        }
    }
    None
}
/// Collect application arguments (head, args).
pub(super) fn collect_app_args(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut head = expr.clone();
    while let Expr::App(f, a) = head {
        args.push(*a);
        head = *f;
    }
    args.reverse();
    (head, args)
}
/// Rebuild an application from head and arguments.
pub(super) fn mk_app(head: Expr, args: Vec<Expr>) -> Expr {
    let mut result = head;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg));
    }
    result
}
/// Get the head constant name from an expression.
pub(super) fn get_head_const(expr: &Expr) -> Option<Name> {
    match expr {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => get_head_const(f),
        _ => None,
    }
}
/// Decompose the lhs/rhs into function + arguments.
pub(super) fn decompose_application(expr: &Expr) -> Option<(Name, Vec<Expr>)> {
    let (head, args) = collect_app_args(expr);
    if let Expr::Const(name, _) = &head {
        Some((name.clone(), args))
    } else {
        None
    }
}
/// Generate sub-goals from a monotonicity rule applied to specific arguments.
///
/// Given a rule and the lhs/rhs arguments, creates the sub-goal expressions
/// that need to be proved for the rule to apply.
pub fn generate_mono_goals(
    rule: &MonoRule,
    lhs_args: &[Expr],
    rhs_args: &[Expr],
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MonoSubGoal>> {
    let mut sub_goals = Vec::new();
    for (i, premise) in rule.premises.iter().enumerate() {
        if premise.lhs_arg_index >= lhs_args.len() || premise.rhs_arg_index >= rhs_args.len() {
            if premise.optional {
                continue;
            }
            return Err(TacticError::Failed(format!(
                "mono: argument index out of range for rule {}",
                rule.name
            )));
        }
        let lhs_arg = &lhs_args[premise.lhs_arg_index];
        let rhs_arg = &rhs_args[premise.rhs_arg_index];
        let sub_goal_type = build_relation_expr(&premise.relation, lhs_arg, rhs_arg);
        if premise.optional && exprs_equal(lhs_arg, rhs_arg) {
            continue;
        }
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(sub_goal_type.clone(), MetavarKind::Natural);
        sub_goals.push(MonoSubGoal {
            mvar_id,
            target: sub_goal_type,
            premise_index: i,
            relation: premise.relation.clone(),
        });
    }
    Ok(sub_goals)
}
/// Build a relational expression `a R b`.
pub(super) fn build_relation_expr(relation: &MonoRelation, lhs: &Expr, rhs: &Expr) -> Expr {
    let rel_name = relation.lean_name();
    let rel_const = Expr::Const(rel_name, vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(rel_const), Box::new(lhs.clone()))),
        Box::new(rhs.clone()),
    )
}
/// Shallow syntactic equality check.
pub(super) fn exprs_equal(a: &Expr, b: &Expr) -> bool {
    match (a, b) {
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
        (Expr::FVar(id1), Expr::FVar(id2)) => id1 == id2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => exprs_equal(f1, f2) && exprs_equal(a1, a2),
        (Expr::Lam(bi1, n1, t1, b1), Expr::Lam(bi2, n2, t2, b2)) => {
            bi1 == bi2 && n1 == n2 && exprs_equal(t1, t2) && exprs_equal(b1, b2)
        }
        (Expr::Pi(bi1, n1, t1, b1), Expr::Pi(bi2, n2, t2, b2)) => {
            bi1 == bi2 && n1 == n2 && exprs_equal(t1, t2) && exprs_equal(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        _ => false,
    }
}
/// Register default monotonicity rules for common operations.
pub(super) fn register_default_rules(set: &mut MonoRuleSet) {
    set.add_rule(MonoRule::new(
        Name::str("add_le_add"),
        MonoConclusion {
            function: Name::str("HAdd.hAdd"),
            relation: MonoRelation::Le,
            num_args: 2,
        },
        vec![
            MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 0,
                rhs_arg_index: 0,
                optional: false,
            },
            MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 1,
                rhs_arg_index: 1,
                optional: false,
            },
        ],
        Expr::Const(Name::str("add_le_add"), vec![]),
        DEFAULT_MONO_PRIORITY,
    ));
    set.add_rule(MonoRule::new(
        Name::str("add_le_add_left"),
        MonoConclusion {
            function: Name::str("HAdd.hAdd"),
            relation: MonoRelation::Le,
            num_args: 2,
        },
        vec![MonoPremise {
            relation: MonoRelation::Le,
            lhs_arg_index: 1,
            rhs_arg_index: 1,
            optional: false,
        }],
        Expr::Const(Name::str("add_le_add_left"), vec![]),
        DEFAULT_MONO_PRIORITY + 100,
    ));
    set.add_rule(MonoRule::new(
        Name::str("add_le_add_right"),
        MonoConclusion {
            function: Name::str("HAdd.hAdd"),
            relation: MonoRelation::Le,
            num_args: 2,
        },
        vec![MonoPremise {
            relation: MonoRelation::Le,
            lhs_arg_index: 0,
            rhs_arg_index: 0,
            optional: false,
        }],
        Expr::Const(Name::str("add_le_add_right"), vec![]),
        DEFAULT_MONO_PRIORITY + 100,
    ));
    set.add_rule(MonoRule::new(
        Name::str("mul_le_mul"),
        MonoConclusion {
            function: Name::str("HMul.hMul"),
            relation: MonoRelation::Le,
            num_args: 2,
        },
        vec![
            MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 0,
                rhs_arg_index: 0,
                optional: false,
            },
            MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 1,
                rhs_arg_index: 1,
                optional: false,
            },
        ],
        Expr::Const(Name::str("mul_le_mul"), vec![]),
        DEFAULT_MONO_PRIORITY,
    ));
    set.add_rule(MonoRule::new(
        Name::str("sub_le_sub"),
        MonoConclusion {
            function: Name::str("HSub.hSub"),
            relation: MonoRelation::Le,
            num_args: 2,
        },
        vec![
            MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 0,
                rhs_arg_index: 0,
                optional: false,
            },
            MonoPremise {
                relation: MonoRelation::Ge,
                lhs_arg_index: 1,
                rhs_arg_index: 1,
                optional: false,
            },
        ],
        Expr::Const(Name::str("sub_le_sub"), vec![]),
        DEFAULT_MONO_PRIORITY,
    ));
    set.add_rule(MonoRule::new(
        Name::str("pow_le_pow_left"),
        MonoConclusion {
            function: Name::str("HPow.hPow"),
            relation: MonoRelation::Le,
            num_args: 2,
        },
        vec![MonoPremise {
            relation: MonoRelation::Le,
            lhs_arg_index: 0,
            rhs_arg_index: 0,
            optional: false,
        }],
        Expr::Const(Name::str("pow_le_pow_left"), vec![]),
        DEFAULT_MONO_PRIORITY,
    ));
    set.add_rule(MonoRule::new(
        Name::str("add_lt_add_of_lt_of_le"),
        MonoConclusion {
            function: Name::str("HAdd.hAdd"),
            relation: MonoRelation::Lt,
            num_args: 2,
        },
        vec![
            MonoPremise {
                relation: MonoRelation::Lt,
                lhs_arg_index: 0,
                rhs_arg_index: 0,
                optional: false,
            },
            MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 1,
                rhs_arg_index: 1,
                optional: false,
            },
        ],
        Expr::Const(Name::str("add_lt_add_of_lt_of_le"), vec![]),
        DEFAULT_MONO_PRIORITY,
    ));
    set.add_rule(MonoRule::new(
        Name::str("Nat.succ_le_succ"),
        MonoConclusion {
            function: Name::str("Nat.succ"),
            relation: MonoRelation::Le,
            num_args: 1,
        },
        vec![MonoPremise {
            relation: MonoRelation::Le,
            lhs_arg_index: 0,
            rhs_arg_index: 0,
            optional: false,
        }],
        Expr::Const(Name::str("Nat.succ_le_succ"), vec![]),
        DEFAULT_MONO_PRIORITY,
    ));
}
/// Main entry point: decompose a relational goal and apply monotonicity rules.
pub fn tac_mono(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<MonoResult> {
    let config = MonoConfig::default();
    tac_mono_with_config(&config, state, ctx)
}
/// Apply mono with a specific configuration.
pub fn tac_mono_with_config(
    config: &MonoConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<MonoResult> {
    let mut rule_set = if config.use_defaults {
        MonoRuleSet::with_defaults()
    } else {
        MonoRuleSet::new()
    };
    for rule in &config.custom_rules {
        rule_set.add_rule(rule.clone());
    }
    tac_mono_with_rules(&rule_set, config, state, ctx)
}
/// Apply mono with explicit rules and configuration.
#[allow(clippy::too_many_arguments)]
pub fn tac_mono_with_rules(
    rules: &MonoRuleSet,
    config: &MonoConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<MonoResult> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let mut stats = MonoStats::default();
    let (lhs, rhs, relation) = decompose_relation(&target).ok_or_else(|| {
        TacticError::GoalMismatch("mono: goal is not a relational expression".to_string())
    })?;
    let (lhs_func, lhs_args) = decompose_application(&lhs).ok_or_else(|| {
        TacticError::GoalMismatch("mono: lhs is not a function application".to_string())
    })?;
    let (rhs_func, rhs_args) = decompose_application(&rhs).ok_or_else(|| {
        TacticError::GoalMismatch("mono: rhs is not a function application".to_string())
    })?;
    if lhs_func != rhs_func {
        return Err(TacticError::GoalMismatch(format!(
            "mono: lhs function '{}' differs from rhs function '{}'",
            lhs_func, rhs_func
        )));
    }
    let candidates = rules.query(&relation, &lhs_func);
    stats.rules_considered = candidates.len();
    if candidates.is_empty() {
        return Err(TacticError::Failed(format!(
            "mono: no rules found for {} with relation {}",
            lhs_func, relation
        )));
    }
    for rule in &candidates {
        stats.rules_tried += 1;
        match generate_mono_goals(rule, &lhs_args, &rhs_args, ctx) {
            Ok(sub_goals) => {
                stats.rules_applied += 1;
                stats.subgoals_generated = sub_goals.len();
                let _new_goal_ids: Vec<MVarId> = sub_goals.iter().map(|sg| sg.mvar_id).collect();
                let mut remaining = Vec::new();
                let mut closed = 0;
                for sg in &sub_goals {
                    let trivially_closed = try_close_trivially(sg, config, ctx);
                    if trivially_closed {
                        closed += 1;
                        if config.try_refl {
                            stats.closed_by_refl += 1;
                        }
                    } else {
                        remaining.push(sg.mvar_id);
                    }
                }
                let proof = build_mono_proof(rule, &lhs_args, &rhs_args, &sub_goals, ctx);
                state.close_goal(proof, ctx)?;
                if !remaining.is_empty() {
                    state.push_goals(remaining.clone());
                }
                return Ok(MonoResult {
                    success: true,
                    remaining_goals: remaining,
                    closed_goals: closed,
                    applied_rule: Some(rule.name.clone()),
                    stats,
                });
            }
            Err(_) => {
                continue;
            }
        }
    }
    Err(TacticError::Failed(
        "mono: no applicable rule succeeded".to_string(),
    ))
}
/// Try to close a sub-goal trivially (by reflexivity or assumption).
pub(super) fn try_close_trivially(
    sub_goal: &MonoSubGoal,
    config: &MonoConfig,
    ctx: &mut MetaContext,
) -> bool {
    if config.try_refl {
        if let Some((lhs, rhs, _)) = decompose_relation(&sub_goal.target) {
            if exprs_equal(&lhs, &rhs) {
                let proof = Expr::Const(Name::str("le_refl"), vec![]);
                ctx.assign_mvar(sub_goal.mvar_id, proof);
                return true;
            }
        }
    }
    false
}
/// Build the proof term for a monotonicity rule application.
///
/// Constructs `rule.proof arg₀ arg₁ … sub_proof₀ sub_proof₁ …` where
/// the leading arguments come from `lhs_args` (the LHS function arguments,
/// used to instantiate implicit type/instance parameters) and the trailing
/// arguments are the proofs of the sub-goals retrieved from `ctx`.
pub(super) fn build_mono_proof(
    rule: &MonoRule,
    lhs_args: &[Expr],
    _rhs_args: &[Expr],
    sub_goals: &[MonoSubGoal],
    ctx: &MetaContext,
) -> Expr {
    let mut term = rule.proof.clone();
    for arg in lhs_args.iter().take(rule.num_implicit_args) {
        term = Expr::App(Box::new(term), Box::new(arg.clone()));
    }
    for sub_goal in sub_goals {
        let sub_proof = ctx
            .get_mvar_assignment(sub_goal.mvar_id)
            .cloned()
            .unwrap_or_else(|| Expr::Const(Name::str("sorry"), vec![]));
        term = Expr::App(Box::new(term), Box::new(sub_proof));
    }
    term
}
/// Check if a function name has any monotonicity rules in the given rule set.
pub fn is_monotone_in_ruleset(func: &Name, rule_set: &MonoRuleSet) -> bool {
    !rule_set.rules_for_function(func).is_empty()
}
/// Get all functions that have monotonicity rules for a given relation.
pub fn monotone_functions_for_relation(
    rule_set: &MonoRuleSet,
    relation: &MonoRelation,
) -> Vec<Name> {
    rule_set
        .all_rules()
        .iter()
        .filter(|r| {
            std::mem::discriminant(&r.conclusion.relation) == std::mem::discriminant(relation)
        })
        .map(|r| r.conclusion.function.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
}
/// Combine two monotonicity relations (e.g., Le ∘ Le = Le, Lt ∘ Le = Lt).
pub fn combine_relations(r1: &MonoRelation, r2: &MonoRelation) -> Option<MonoRelation> {
    match (r1, r2) {
        (MonoRelation::Le, MonoRelation::Le) => Some(MonoRelation::Le),
        (MonoRelation::Le, MonoRelation::Lt) => Some(MonoRelation::Lt),
        (MonoRelation::Lt, MonoRelation::Le) => Some(MonoRelation::Lt),
        (MonoRelation::Lt, MonoRelation::Lt) => Some(MonoRelation::Lt),
        (MonoRelation::Ge, MonoRelation::Ge) => Some(MonoRelation::Ge),
        (MonoRelation::Ge, MonoRelation::Gt) => Some(MonoRelation::Gt),
        (MonoRelation::Gt, MonoRelation::Ge) => Some(MonoRelation::Gt),
        (MonoRelation::Gt, MonoRelation::Gt) => Some(MonoRelation::Gt),
        (MonoRelation::Dvd, MonoRelation::Dvd) => Some(MonoRelation::Dvd),
        (MonoRelation::Subset, MonoRelation::Subset) => Some(MonoRelation::Subset),
        _ => None,
    }
}
/// Count the total number of monotonicity rules in a rule set.
pub fn count_rules(rule_set: &MonoRuleSet) -> usize {
    rule_set.all_rules().len()
}
/// Check if two expressions have the same head function and argument count.
pub fn structurally_compatible(lhs: &Expr, rhs: &Expr) -> bool {
    let (lhs_head, lhs_args) = collect_app_args(lhs);
    let (rhs_head, rhs_args) = collect_app_args(rhs);
    exprs_equal(&lhs_head, &rhs_head) && lhs_args.len() == rhs_args.len()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::monotonicity::*;
    use oxilean_kernel::Environment;
    fn mk_test_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_const(name: &str) -> Expr {
        Expr::Const(Name::str(name), vec![])
    }
    fn mk_app2(f: Expr, a: Expr, b: Expr) -> Expr {
        Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b))
    }
    #[test]
    fn test_mono_relation_display() {
        assert_eq!(format!("{}", MonoRelation::Le), "<=");
        assert_eq!(format!("{}", MonoRelation::Lt), "<");
        assert_eq!(format!("{}", MonoRelation::Ge), ">=");
        assert_eq!(format!("{}", MonoRelation::Gt), ">");
        assert_eq!(format!("{}", MonoRelation::Dvd), "|");
        assert_eq!(format!("{}", MonoRelation::Subset), "subset");
    }
    #[test]
    fn test_mono_relation_flip() {
        assert_eq!(MonoRelation::Le.flip(), MonoRelation::Ge);
        assert_eq!(MonoRelation::Lt.flip(), MonoRelation::Gt);
        assert_eq!(MonoRelation::Ge.flip(), MonoRelation::Le);
        assert_eq!(MonoRelation::Gt.flip(), MonoRelation::Lt);
    }
    #[test]
    fn test_mono_relation_is_inequality() {
        assert!(MonoRelation::Le.is_inequality());
        assert!(MonoRelation::Lt.is_inequality());
        assert!(MonoRelation::Ge.is_inequality());
        assert!(MonoRelation::Gt.is_inequality());
        assert!(!MonoRelation::Dvd.is_inequality());
        assert!(!MonoRelation::Subset.is_inequality());
    }
    #[test]
    fn test_mono_relation_compatible() {
        assert!(MonoRelation::Le.is_compatible(&MonoRelation::Le));
        assert!(MonoRelation::Le.is_compatible(&MonoRelation::Lt));
        assert!(MonoRelation::Lt.is_compatible(&MonoRelation::Le));
        assert!(!MonoRelation::Le.is_compatible(&MonoRelation::Ge));
        assert!(!MonoRelation::Le.is_compatible(&MonoRelation::Dvd));
    }
    #[test]
    fn test_mono_relation_lean_name() {
        assert_eq!(MonoRelation::Le.lean_name(), Name::str("LE.le"));
        assert_eq!(MonoRelation::Lt.lean_name(), Name::str("LT.lt"));
    }
    #[test]
    fn test_mono_relation_from_name() {
        let le_name = Name::str("LE.le");
        let rel = MonoRelation::from_name(&le_name);
        assert!(rel.is_some());
        assert_eq!(rel.expect("rel should be valid"), MonoRelation::Le);
    }
    #[test]
    fn test_mono_rule_creation() {
        let rule = MonoRule::new(
            Name::str("test_rule"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 2,
            },
            vec![MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 0,
                rhs_arg_index: 0,
                optional: false,
            }],
            mk_const("test_proof"),
            DEFAULT_MONO_PRIORITY,
        );
        assert_eq!(rule.name, Name::str("test_rule"));
        assert_eq!(rule.priority, DEFAULT_MONO_PRIORITY);
        assert_eq!(rule.num_subgoals(), 1);
    }
    #[test]
    fn test_mono_rule_matches() {
        let rule = MonoRule::new(
            Name::str("add_le_add"),
            MonoConclusion {
                function: Name::str("HAdd.hAdd"),
                relation: MonoRelation::Le,
                num_args: 2,
            },
            vec![],
            mk_const("proof"),
            DEFAULT_MONO_PRIORITY,
        );
        assert!(rule.matches(&Name::str("HAdd.hAdd"), &MonoRelation::Le));
        assert!(rule.matches(&Name::str("HAdd.hAdd"), &MonoRelation::Lt));
        assert!(!rule.matches(&Name::str("HMul.hMul"), &MonoRelation::Le));
    }
    #[test]
    fn test_mono_rule_with_tag() {
        let rule = MonoRule::new(
            Name::str("test"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("proof"),
            DEFAULT_MONO_PRIORITY,
        )
        .with_tag("arithmetic");
        assert!(rule.tags.contains("arithmetic"));
    }
    #[test]
    fn test_mono_rule_optional_premises() {
        let rule = MonoRule::new(
            Name::str("test"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 2,
            },
            vec![
                MonoPremise {
                    relation: MonoRelation::Le,
                    lhs_arg_index: 0,
                    rhs_arg_index: 0,
                    optional: false,
                },
                MonoPremise {
                    relation: MonoRelation::Le,
                    lhs_arg_index: 1,
                    rhs_arg_index: 1,
                    optional: true,
                },
            ],
            mk_const("proof"),
            DEFAULT_MONO_PRIORITY,
        );
        assert_eq!(rule.num_subgoals(), 1);
    }
    #[test]
    fn test_mono_rule_set_empty() {
        let set = MonoRuleSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }
    #[test]
    fn test_mono_rule_set_add_and_query() {
        let mut set = MonoRuleSet::new();
        let rule = MonoRule::new(
            Name::str("add_le_add"),
            MonoConclusion {
                function: Name::str("HAdd.hAdd"),
                relation: MonoRelation::Le,
                num_args: 2,
            },
            vec![],
            mk_const("proof"),
            DEFAULT_MONO_PRIORITY,
        );
        set.add_rule(rule);
        assert_eq!(set.len(), 1);
        let results = set.query(&MonoRelation::Le, &Name::str("HAdd.hAdd"));
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_mono_rule_set_query_no_match() {
        let mut set = MonoRuleSet::new();
        set.add_rule(MonoRule::new(
            Name::str("add_le_add"),
            MonoConclusion {
                function: Name::str("HAdd.hAdd"),
                relation: MonoRelation::Le,
                num_args: 2,
            },
            vec![],
            mk_const("proof"),
            DEFAULT_MONO_PRIORITY,
        ));
        let results = set.query(&MonoRelation::Dvd, &Name::str("HAdd.hAdd"));
        assert!(results.is_empty());
    }
    #[test]
    fn test_mono_rule_set_with_defaults() {
        let set = MonoRuleSet::with_defaults();
        assert!(!set.is_empty());
        assert!(set.len() >= 5);
    }
    #[test]
    fn test_mono_rule_set_priority_ordering() {
        let mut set = MonoRuleSet::new();
        set.add_rule(MonoRule::new(
            Name::str("low_priority"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("proof1"),
            5000,
        ));
        set.add_rule(MonoRule::new(
            Name::str("high_priority"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("proof2"),
            100,
        ));
        let results = set.query(&MonoRelation::Le, &Name::str("f"));
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, Name::str("high_priority"));
        assert_eq!(results[1].name, Name::str("low_priority"));
    }
    #[test]
    fn test_mono_rule_set_merge() {
        let mut set1 = MonoRuleSet::new();
        set1.add_rule(MonoRule::new(
            Name::str("rule1"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("proof1"),
            DEFAULT_MONO_PRIORITY,
        ));
        let mut set2 = MonoRuleSet::new();
        set2.add_rule(MonoRule::new(
            Name::str("rule2"),
            MonoConclusion {
                function: Name::str("g"),
                relation: MonoRelation::Lt,
                num_args: 1,
            },
            vec![],
            mk_const("proof2"),
            DEFAULT_MONO_PRIORITY,
        ));
        set1.merge(&set2);
        assert_eq!(set1.len(), 2);
    }
    #[test]
    fn test_mono_rule_set_remove_function() {
        let mut set = MonoRuleSet::new();
        set.add_rule(MonoRule::new(
            Name::str("rule1"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("proof1"),
            DEFAULT_MONO_PRIORITY,
        ));
        set.add_rule(MonoRule::new(
            Name::str("rule2"),
            MonoConclusion {
                function: Name::str("g"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("proof2"),
            DEFAULT_MONO_PRIORITY,
        ));
        set.remove_function(&Name::str("f"));
        let results = set.query(&MonoRelation::Le, &Name::str("f"));
        assert!(results.is_empty());
    }
    #[test]
    fn test_decompose_relation_le() {
        let le = mk_const("LE.le");
        let a = mk_const("a");
        let b = mk_const("b");
        let expr = mk_app2(le, a.clone(), b.clone());
        let result = decompose_relation(&expr);
        assert!(result.is_some());
        let (lhs, rhs, relation) = result.expect("result should be valid");
        assert!(exprs_equal(&lhs, &a));
        assert!(exprs_equal(&rhs, &b));
        assert_eq!(relation, MonoRelation::Le);
    }
    #[test]
    fn test_decompose_relation_lt() {
        let lt = mk_const("LT.lt");
        let a = mk_const("x");
        let b = mk_const("y");
        let expr = mk_app2(lt, a.clone(), b.clone());
        let result = decompose_relation(&expr);
        assert!(result.is_some());
        let (_, _, relation) = result.expect("result should be valid");
        assert_eq!(relation, MonoRelation::Lt);
    }
    #[test]
    fn test_decompose_relation_non_relation() {
        let f = mk_const("not_a_relation_xyz");
        let a = mk_const("a");
        let result = decompose_relation(&Expr::App(Box::new(f), Box::new(a)));
        assert!(result.is_none());
    }
    #[test]
    fn test_decompose_application() {
        let f = mk_const("HAdd.hAdd");
        let a = mk_const("a");
        let b = mk_const("b");
        let expr = mk_app2(f, a, b);
        let result = decompose_application(&expr);
        assert!(result.is_some());
        let (name, args) = result.expect("result should be valid");
        assert_eq!(name, Name::str("HAdd.hAdd"));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_generate_mono_goals_basic() {
        let mut ctx = mk_test_ctx();
        let rule = MonoRule::new(
            Name::str("add_le_add"),
            MonoConclusion {
                function: Name::str("HAdd.hAdd"),
                relation: MonoRelation::Le,
                num_args: 2,
            },
            vec![
                MonoPremise {
                    relation: MonoRelation::Le,
                    lhs_arg_index: 0,
                    rhs_arg_index: 0,
                    optional: false,
                },
                MonoPremise {
                    relation: MonoRelation::Le,
                    lhs_arg_index: 1,
                    rhs_arg_index: 1,
                    optional: false,
                },
            ],
            mk_const("proof"),
            DEFAULT_MONO_PRIORITY,
        );
        let lhs_args = vec![mk_const("a"), mk_const("b")];
        let rhs_args = vec![mk_const("c"), mk_const("d")];
        let sub_goals = generate_mono_goals(&rule, &lhs_args, &rhs_args, &mut ctx)
            .expect("sub_goals should be present");
        assert_eq!(sub_goals.len(), 2);
        assert_eq!(sub_goals[0].relation, MonoRelation::Le);
        assert_eq!(sub_goals[1].relation, MonoRelation::Le);
    }
    #[test]
    fn test_generate_mono_goals_out_of_range() {
        let mut ctx = mk_test_ctx();
        let rule = MonoRule::new(
            Name::str("test"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 2,
            },
            vec![MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 5,
                rhs_arg_index: 0,
                optional: false,
            }],
            mk_const("proof"),
            DEFAULT_MONO_PRIORITY,
        );
        let lhs_args = vec![mk_const("a")];
        let rhs_args = vec![mk_const("b")];
        let result = generate_mono_goals(&rule, &lhs_args, &rhs_args, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_generate_mono_goals_optional_same_arg() {
        let mut ctx = mk_test_ctx();
        let rule = MonoRule::new(
            Name::str("test"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![MonoPremise {
                relation: MonoRelation::Le,
                lhs_arg_index: 0,
                rhs_arg_index: 0,
                optional: true,
            }],
            mk_const("proof"),
            DEFAULT_MONO_PRIORITY,
        );
        let a = mk_const("same");
        let lhs_args = vec![a.clone()];
        let rhs_args = vec![a];
        let sub_goals = generate_mono_goals(&rule, &lhs_args, &rhs_args, &mut ctx)
            .expect("sub_goals should be present");
        assert!(sub_goals.is_empty());
    }
    #[test]
    fn test_mono_config_default() {
        let config = MonoConfig::default();
        assert_eq!(config.max_depth, DEFAULT_MONO_MAX_DEPTH);
        assert!(config.use_defaults);
        assert!(config.try_refl);
        assert!(config.try_assumption);
    }
    #[test]
    fn test_mono_config_custom_only() {
        let config = MonoConfig::custom_only(vec![]);
        assert!(!config.use_defaults);
        assert!(config.custom_rules.is_empty());
    }
    #[test]
    fn test_mono_config_with_max_depth() {
        let config = MonoConfig::default().with_max_depth(3);
        assert_eq!(config.max_depth, 3);
    }
    #[test]
    fn test_mono_stats_default() {
        let stats = MonoStats::default();
        assert_eq!(stats.rules_considered, 0);
        assert_eq!(stats.rules_tried, 0);
        assert_eq!(stats.rules_applied, 0);
        assert_eq!(stats.subgoals_generated, 0);
    }
    #[test]
    fn test_exprs_equal_same() {
        let a = mk_const("x");
        assert!(exprs_equal(&a, &a));
    }
    #[test]
    fn test_exprs_equal_different() {
        let a = mk_const("x");
        let b = mk_const("y");
        assert!(!exprs_equal(&a, &b));
    }
    #[test]
    fn test_exprs_equal_app() {
        let f = mk_const("f");
        let a = mk_const("a");
        let e1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let e2 = Expr::App(Box::new(f), Box::new(a));
        assert!(exprs_equal(&e1, &e2));
    }
    #[test]
    fn test_exprs_equal_bvar() {
        assert!(exprs_equal(&Expr::BVar(0), &Expr::BVar(0)));
        assert!(!exprs_equal(&Expr::BVar(0), &Expr::BVar(1)));
    }
    #[test]
    fn test_build_relation_expr() {
        let a = mk_const("a");
        let b = mk_const("b");
        let rel_expr = build_relation_expr(&MonoRelation::Le, &a, &b);
        assert!(matches!(rel_expr, Expr::App(_, _)));
    }
    #[test]
    fn test_mono_rule_set_rules_for_function() {
        let set = MonoRuleSet::with_defaults();
        let add_rules = set.rules_for_function(&Name::str("HAdd.hAdd"));
        assert!(!add_rules.is_empty());
    }
    #[test]
    fn test_mono_conclusion_fields() {
        let conclusion = MonoConclusion {
            function: Name::str("HAdd.hAdd"),
            relation: MonoRelation::Le,
            num_args: 2,
        };
        assert_eq!(conclusion.num_args, 2);
        assert_eq!(conclusion.function, Name::str("HAdd.hAdd"));
    }
    #[test]
    fn test_mono_premise_fields() {
        let premise = MonoPremise {
            relation: MonoRelation::Le,
            lhs_arg_index: 0,
            rhs_arg_index: 0,
            optional: false,
        };
        assert!(!premise.optional);
        assert_eq!(premise.lhs_arg_index, 0);
    }
    #[test]
    fn test_mono_sub_goal_fields() {
        let sg = MonoSubGoal {
            mvar_id: MVarId(42),
            target: mk_const("target"),
            premise_index: 0,
            relation: MonoRelation::Le,
        };
        assert_eq!(sg.mvar_id, MVarId(42));
        assert_eq!(sg.premise_index, 0);
    }
    #[test]
    fn test_mono_result_fields() {
        let result = MonoResult {
            success: true,
            remaining_goals: vec![MVarId(1), MVarId(2)],
            closed_goals: 0,
            applied_rule: Some(Name::str("add_le_add")),
            stats: MonoStats::default(),
        };
        assert!(result.success);
        assert_eq!(result.remaining_goals.len(), 2);
    }
    #[test]
    fn test_mono_relation_custom() {
        let custom = MonoRelation::Custom(Name::str("my_rel"));
        assert!(!custom.is_inequality());
        assert_eq!(format!("{}", custom), "my_rel");
        assert_eq!(custom.lean_name(), Name::str("my_rel"));
    }
    #[test]
    fn test_collect_app_args_empty() {
        let c = mk_const("f");
        let (head, args) = collect_app_args(&c);
        assert!(args.is_empty());
        assert!(exprs_equal(&head, &c));
    }
    #[test]
    fn test_collect_app_args_nested() {
        let f = mk_const("f");
        let a = mk_const("a");
        let b = mk_const("b");
        let c = mk_const("c");
        let expr = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
                Box::new(b.clone()),
            )),
            Box::new(c.clone()),
        );
        let (head, args) = collect_app_args(&expr);
        assert!(exprs_equal(&head, &f));
        assert_eq!(args.len(), 3);
    }
    #[test]
    fn test_get_head_const_simple() {
        let c = mk_const("f");
        assert_eq!(get_head_const(&c), Some(Name::str("f")));
    }
    #[test]
    fn test_get_head_const_app() {
        let f = mk_const("f");
        let a = mk_const("a");
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(get_head_const(&app), Some(Name::str("f")));
    }
    #[test]
    fn test_get_head_const_bvar() {
        let bv = Expr::BVar(0);
        assert!(get_head_const(&bv).is_none());
    }
    #[test]
    fn test_is_monotone_in_ruleset_hadd() {
        let set = MonoRuleSet::with_defaults();
        assert!(is_monotone_in_ruleset(&Name::str("HAdd.hAdd"), &set));
    }
    #[test]
    fn test_is_monotone_in_ruleset_unknown() {
        let set = MonoRuleSet::with_defaults();
        assert!(!is_monotone_in_ruleset(
            &Name::str("nonexistent_func"),
            &set
        ));
    }
    #[test]
    fn test_is_monotone_in_empty_ruleset() {
        let set = MonoRuleSet::new();
        assert!(!is_monotone_in_ruleset(&Name::str("HAdd.hAdd"), &set));
    }
    #[test]
    fn test_combine_relations_le_le() {
        let result = combine_relations(&MonoRelation::Le, &MonoRelation::Le);
        assert!(matches!(result, Some(MonoRelation::Le)));
    }
    #[test]
    fn test_combine_relations_le_lt() {
        let result = combine_relations(&MonoRelation::Le, &MonoRelation::Lt);
        assert!(matches!(result, Some(MonoRelation::Lt)));
    }
    #[test]
    fn test_combine_relations_lt_le() {
        let result = combine_relations(&MonoRelation::Lt, &MonoRelation::Le);
        assert!(matches!(result, Some(MonoRelation::Lt)));
    }
    #[test]
    fn test_combine_relations_incompatible() {
        let result = combine_relations(&MonoRelation::Le, &MonoRelation::Ge);
        assert!(result.is_none());
    }
    #[test]
    fn test_combine_relations_ge_ge() {
        let result = combine_relations(&MonoRelation::Ge, &MonoRelation::Ge);
        assert!(matches!(result, Some(MonoRelation::Ge)));
    }
    #[test]
    fn test_combine_relations_gt_ge() {
        let result = combine_relations(&MonoRelation::Gt, &MonoRelation::Ge);
        assert!(matches!(result, Some(MonoRelation::Gt)));
    }
    #[test]
    fn test_count_rules_default() {
        let set = MonoRuleSet::with_defaults();
        assert!(count_rules(&set) > 0);
    }
    #[test]
    fn test_count_rules_empty() {
        let set = MonoRuleSet::new();
        assert_eq!(count_rules(&set), 0);
    }
    #[test]
    fn test_mono_chain_singleton() {
        let rule = MonoRule::new(
            Name::str("test_rule"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("test_rule"),
            100,
        );
        let chain = MonoChain::singleton(rule);
        assert_eq!(chain.len(), 1);
        assert!(!chain.is_empty());
        assert!(matches!(chain.relation, MonoRelation::Le));
    }
    #[test]
    fn test_mono_chain_extend() {
        let rule1 = MonoRule::new(
            Name::str("rule1"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("rule1"),
            100,
        );
        let rule2 = MonoRule::new(
            Name::str("rule2"),
            MonoConclusion {
                function: Name::str("g"),
                relation: MonoRelation::Lt,
                num_args: 1,
            },
            vec![],
            mk_const("rule2"),
            100,
        );
        let chain = MonoChain::singleton(rule1);
        let extended = chain.extend(rule2);
        assert!(extended.is_some());
        let ext = extended.expect("ext should be present");
        assert_eq!(ext.len(), 2);
        assert!(matches!(ext.relation, MonoRelation::Lt));
    }
    #[test]
    fn test_mono_chain_extend_incompatible() {
        let rule1 = MonoRule::new(
            Name::str("rule1"),
            MonoConclusion {
                function: Name::str("f"),
                relation: MonoRelation::Le,
                num_args: 1,
            },
            vec![],
            mk_const("rule1"),
            100,
        );
        let rule2 = MonoRule::new(
            Name::str("rule2"),
            MonoConclusion {
                function: Name::str("g"),
                relation: MonoRelation::Ge,
                num_args: 1,
            },
            vec![],
            mk_const("rule2"),
            100,
        );
        let chain = MonoChain::singleton(rule1);
        let extended = chain.extend(rule2);
        assert!(extended.is_none());
    }
    #[test]
    fn test_structurally_compatible_same() {
        let f = mk_const("f");
        let a = mk_const("a");
        let b = mk_const("b");
        let e1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let e2 = Expr::App(Box::new(f), Box::new(b));
        assert!(structurally_compatible(&e1, &e2));
    }
    #[test]
    fn test_structurally_compatible_different_head() {
        let f = mk_const("f");
        let g = mk_const("g");
        let a = mk_const("a");
        let e1 = Expr::App(Box::new(f), Box::new(a.clone()));
        let e2 = Expr::App(Box::new(g), Box::new(a));
        assert!(!structurally_compatible(&e1, &e2));
    }
    #[test]
    fn test_structurally_compatible_different_arity() {
        let f = mk_const("f");
        let a = mk_const("a");
        let b = mk_const("b");
        let e1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let e2 = mk_app2(f, a, b);
        assert!(!structurally_compatible(&e1, &e2));
    }
    #[test]
    fn test_monotone_functions_for_le_relation() {
        let set = MonoRuleSet::with_defaults();
        let funcs = monotone_functions_for_relation(&set, &MonoRelation::Le);
        assert!(!funcs.is_empty());
    }
}
#[cfg(test)]
mod tacticmonotonicity_analysis_tests {
    use super::*;
    use crate::tactic::monotonicity::*;
    #[test]
    fn test_tacticmonotonicity_result_ok() {
        let r = TacticMonotonicityResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticmonotonicity_result_err() {
        let r = TacticMonotonicityResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticmonotonicity_result_partial() {
        let r = TacticMonotonicityResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticmonotonicity_result_skipped() {
        let r = TacticMonotonicityResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticmonotonicity_analysis_pass_run() {
        let mut p = TacticMonotonicityAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticmonotonicity_analysis_pass_empty_input() {
        let mut p = TacticMonotonicityAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticmonotonicity_analysis_pass_success_rate() {
        let mut p = TacticMonotonicityAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticmonotonicity_analysis_pass_disable() {
        let mut p = TacticMonotonicityAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticmonotonicity_pipeline_basic() {
        let mut pipeline = TacticMonotonicityPipeline::new("main_pipeline");
        pipeline.add_pass(TacticMonotonicityAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticMonotonicityAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticmonotonicity_pipeline_disabled_pass() {
        let mut pipeline = TacticMonotonicityPipeline::new("partial");
        let mut p = TacticMonotonicityAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticMonotonicityAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticmonotonicity_diff_basic() {
        let mut d = TacticMonotonicityDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticmonotonicity_diff_summary() {
        let mut d = TacticMonotonicityDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticmonotonicity_config_set_get() {
        let mut cfg = TacticMonotonicityConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticmonotonicity_config_read_only() {
        let mut cfg = TacticMonotonicityConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticmonotonicity_config_remove() {
        let mut cfg = TacticMonotonicityConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticmonotonicity_diagnostics_basic() {
        let mut diag = TacticMonotonicityDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticmonotonicity_diagnostics_max_errors() {
        let mut diag = TacticMonotonicityDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticmonotonicity_diagnostics_clear() {
        let mut diag = TacticMonotonicityDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticmonotonicity_config_value_types() {
        let b = TacticMonotonicityConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticMonotonicityConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticMonotonicityConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticMonotonicityConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticMonotonicityConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
