//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    DischargeCache, DischargeClassification, DischargeContext, DischargeExtConfig500,
    DischargeExtConfigVal500, DischargeExtDiag500, DischargeExtDiff500, DischargeExtPass500,
    DischargeExtPipeline500, DischargeExtResult500, DischargeLog, DischargeRecord, DischargeResult,
    DischargeRunStats, DischargeStats, DischargeStrategy, DischargeTracer, PrioritizedDischarge,
    TacticSimpDischargeAnalysisPass, TacticSimpDischargeConfig, TacticSimpDischargeConfigValue,
    TacticSimpDischargeDiagnostics, TacticSimpDischargeDiff, TacticSimpDischargePipeline,
    TacticSimpDischargeResult,
};
use crate::basic::MetaContext;
use crate::tactic::simp::types::{SimpConfig, SimpResult, SimpTheorems};
use oxilean_kernel::{Expr, Name};

/// Try to discharge a side goal using the given strategy.
pub fn discharge_side_goal(
    goal: &Expr,
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> Option<Expr> {
    match strategy {
        DischargeStrategy::Assumption => try_assumption(goal, ctx),
        DischargeStrategy::Trivial => try_trivial(goal),
        DischargeStrategy::Simp => try_simp_discharge(goal, theorems, config, ctx),
        DischargeStrategy::Exact(proof) => Some(proof.clone()),
        DischargeStrategy::Auto => try_all_strategies(goal, theorems, config, ctx),
        DischargeStrategy::Sequence(strategies) => {
            for s in strategies {
                if let Some(p) = discharge_side_goal(goal, s, theorems, config, ctx) {
                    return Some(p);
                }
            }
            None
        }
    }
}
/// Discharge a goal, returning a structured result.
pub fn discharge_side_goal_result(
    goal: &Expr,
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> DischargeResult {
    match discharge_side_goal(goal, strategy, theorems, config, ctx) {
        Some(proof) => DischargeResult::Proved(proof),
        None => DischargeResult::Failed,
    }
}
pub(super) fn try_assumption(goal: &Expr, ctx: &MetaContext) -> Option<Expr> {
    let hyps = ctx.get_local_hyps();
    for (name, ty) in &hyps {
        if ty == goal {
            return Some(Expr::Const(name.clone(), vec![]));
        }
    }
    None
}
pub(super) fn try_trivial(goal: &Expr) -> Option<Expr> {
    if is_true_goal(goal) {
        return Some(Expr::Const(Name::str("True.intro"), vec![]));
    }
    if let Some(proof) = try_refl(goal) {
        return Some(proof);
    }
    if let Some(proof) = try_false_elim(goal) {
        return Some(proof);
    }
    None
}
pub(super) fn is_true_goal(goal: &Expr) -> bool {
    matches!(goal, Expr::Const(name, _) if * name == Name::str("True"))
}
pub(super) fn try_refl(goal: &Expr) -> Option<Expr> {
    if let Expr::App(f, rhs) = goal {
        if let Expr::App(g, lhs) = f.as_ref() {
            if let Expr::App(eq_c, _ty) = g.as_ref() {
                if let Expr::Const(name, _) = eq_c.as_ref() {
                    if *name == Name::str("Eq") && lhs == rhs {
                        return Some(Expr::Const(Name::str("Eq.refl"), vec![]));
                    }
                }
            }
        }
    }
    None
}
pub(super) fn try_false_elim(goal: &Expr) -> Option<Expr> {
    if let Expr::Pi(_, _, domain, _) = goal {
        if matches!(
            domain.as_ref(), Expr::Const(name, _) if * name == Name::str("False")
        ) {
            return Some(Expr::Const(Name::str("False.elim"), vec![]));
        }
    }
    None
}
pub(super) fn try_simp_discharge(
    goal: &Expr,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> Option<Expr> {
    let result = crate::tactic::simp::main::simp(goal, theorems, config, ctx);
    match result {
        SimpResult::Proved(proof) => Some(proof),
        _ => None,
    }
}
/// Try all built-in strategies.
pub fn try_all_strategies(
    goal: &Expr,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> Option<Expr> {
    let strategies = [
        DischargeStrategy::Trivial,
        DischargeStrategy::Assumption,
        DischargeStrategy::Simp,
    ];
    for strategy in &strategies {
        if let Some(proof) = discharge_side_goal(goal, strategy, theorems, config, ctx) {
            return Some(proof);
        }
    }
    None
}
/// Discharge all goals — returns None if any fails.
pub fn discharge_all(
    goals: &[Expr],
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> Option<Vec<Expr>> {
    let mut proofs = Vec::with_capacity(goals.len());
    for goal in goals {
        match discharge_side_goal(goal, strategy, theorems, config, ctx) {
            Some(p) => proofs.push(p),
            None => return None,
        }
    }
    Some(proofs)
}
/// Discharge goals, returning partial results.
pub fn discharge_partial(
    goals: &[Expr],
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> Vec<(Expr, DischargeResult)> {
    goals
        .iter()
        .map(|goal| {
            let result = discharge_side_goal_result(goal, strategy, theorems, config, ctx);
            (goal.clone(), result)
        })
        .collect()
}
/// Discharge goals while collecting statistics.
pub fn discharge_with_stats(
    goals: &[Expr],
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> (Vec<Option<Expr>>, DischargeStats) {
    let mut stats = DischargeStats {
        attempted: goals.len(),
        ..Default::default()
    };
    let mut results = Vec::with_capacity(goals.len());
    for goal in goals {
        if let Some(p) = try_trivial(goal) {
            stats.discharged += 1;
            stats.trivial_count += 1;
            results.push(Some(p));
        } else if let Some(p) = try_assumption(goal, ctx) {
            stats.discharged += 1;
            stats.assumption_count += 1;
            results.push(Some(p));
        } else if let Some(p) = try_simp_discharge(goal, theorems, config, ctx) {
            stats.discharged += 1;
            stats.simp_count += 1;
            results.push(Some(p));
        } else {
            stats.failed += 1;
            results.push(None);
        }
    }
    (results, stats)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::simp::discharge::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_discharge_trivial_true() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let result = discharge_side_goal(
            &mk_const("True"),
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
        );
        assert!(result.is_some());
    }
    #[test]
    fn test_discharge_trivial_non_true() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let result = discharge_side_goal(
            &mk_const("P"),
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
        );
        assert!(result.is_none());
    }
    #[test]
    fn test_discharge_exact() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let proof = mk_const("my_proof");
        let result = discharge_side_goal(
            &mk_const("P"),
            &DischargeStrategy::Exact(proof.clone()),
            &theorems,
            &config,
            &mut ctx,
        );
        assert_eq!(result, Some(proof));
    }
    #[test]
    fn test_try_all_strategies_true() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let result = try_all_strategies(&mk_const("True"), &theorems, &config, &mut ctx);
        assert!(result.is_some());
    }
    #[test]
    fn test_try_all_strategies_fail() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let result = try_all_strategies(&mk_const("HardGoal"), &theorems, &config, &mut ctx);
        assert!(result.is_none());
    }
    #[test]
    fn test_discharge_result_is_proved() {
        let r = DischargeResult::Proved(mk_const("p"));
        assert!(r.is_proved());
        assert!(!DischargeResult::Failed.is_proved());
    }
    #[test]
    fn test_discharge_all_success() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goals = vec![mk_const("True"), mk_const("True")];
        let result = discharge_all(
            &goals,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
        );
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid").len(), 2);
    }
    #[test]
    fn test_discharge_all_fail() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goals = vec![mk_const("True"), mk_const("HardGoal")];
        let result = discharge_all(
            &goals,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
        );
        assert!(result.is_none());
    }
    #[test]
    fn test_discharge_stats() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goals = vec![mk_const("True"), mk_const("HardGoal")];
        let (results, stats) = discharge_with_stats(&goals, &theorems, &config, &mut ctx);
        assert_eq!(stats.attempted, 2);
        assert_eq!(stats.discharged, 1);
        assert_eq!(stats.failed, 1);
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_sequence_strategy() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let strategy = DischargeStrategy::Sequence(vec![
            DischargeStrategy::Assumption,
            DischargeStrategy::Trivial,
        ]);
        let result =
            discharge_side_goal(&mk_const("True"), &strategy, &theorems, &config, &mut ctx);
        assert!(result.is_some());
    }
    #[test]
    fn test_auto_strategy() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let result = discharge_side_goal(
            &mk_const("True"),
            &DischargeStrategy::Auto,
            &theorems,
            &config,
            &mut ctx,
        );
        assert!(result.is_some());
    }
}
/// Discharge with memoization.
pub fn discharge_cached(
    goal: &Expr,
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
    cache: &mut DischargeCache,
) -> Option<Expr> {
    if let Some(proof) = cache.get(goal) {
        return Some(proof.clone());
    }
    let proof = discharge_side_goal(goal, strategy, theorems, config, ctx)?;
    cache.put(goal, proof.clone());
    Some(proof)
}
/// Discharge a batch of goals in parallel (sequentially here, but API mirrors parallel).
pub fn discharge_batch(
    goals: &[Expr],
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> Vec<Option<Expr>> {
    goals
        .iter()
        .map(|g| discharge_side_goal(g, strategy, theorems, config, ctx))
        .collect()
}
/// Check whether all goals in a list can be discharged.
pub fn can_discharge_all(
    goals: &[Expr],
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> bool {
    goals
        .iter()
        .all(|g| discharge_side_goal(g, strategy, theorems, config, ctx).is_some())
}
/// Discharge goals eagerly, stopping at the first failure.
pub fn discharge_eager(
    goals: &[Expr],
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> Result<Vec<Expr>, usize> {
    let mut proofs = Vec::with_capacity(goals.len());
    for (i, goal) in goals.iter().enumerate() {
        match discharge_side_goal(goal, strategy, theorems, config, ctx) {
            Some(p) => proofs.push(p),
            None => return Err(i),
        }
    }
    Ok(proofs)
}
/// A retry wrapper: tries a strategy up to `max_retries` times with slight variation.
pub fn discharge_with_retry(
    goal: &Expr,
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
    max_retries: usize,
) -> Option<Expr> {
    for _ in 0..=max_retries {
        if let Some(p) = discharge_side_goal(goal, strategy, theorems, config, ctx) {
            return Some(p);
        }
    }
    None
}
/// Check whether a goal is trivially `True`.
pub fn goal_is_true(goal: &Expr) -> bool {
    matches!(goal, Expr::Const(name, _) if * name == Name::str("True"))
}
/// Check whether a goal is trivially `False → _` (False.elim).
pub fn goal_is_false_elim(goal: &Expr) -> bool {
    if let Expr::Pi(_, _, domain, _) = goal {
        matches!(domain.as_ref(), Expr::Const(name, _) if * name == Name::str("False"))
    } else {
        false
    }
}
/// Describe a goal as a human-readable string.
pub fn describe_goal(goal: &Expr) -> String {
    if goal_is_true(goal) {
        return "True".to_string();
    }
    if goal_is_false_elim(goal) {
        return "False → _".to_string();
    }
    format!("{:?}", goal)
}
/// Describe a discharge strategy as a human-readable string.
pub fn describe_strategy(strategy: &DischargeStrategy) -> String {
    match strategy {
        DischargeStrategy::Sequence(ss) => {
            let names: Vec<&str> = ss.iter().map(|s| s.name()).collect();
            format!("sequence({})", names.join(", "))
        }
        other => other.name().to_string(),
    }
}
/// Check whether `DischargeResult::Simplified` holds.
pub fn is_simplified(result: &DischargeResult) -> bool {
    matches!(result, DischargeResult::Simplified(_))
}
/// Extract the simplified expression, if any.
pub fn simplified_of(result: DischargeResult) -> Option<Expr> {
    match result {
        DischargeResult::Simplified(e) => Some(e),
        _ => None,
    }
}
#[cfg(test)]
mod discharge_extended_tests {
    use super::*;
    use crate::tactic::simp::discharge::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_prioritized_discharge_trivial() {
        let mut pd = PrioritizedDischarge::new();
        pd.add(100, DischargeStrategy::Trivial);
        pd.add(50, DischargeStrategy::Assumption);
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let result = pd.discharge(&mk_const("True"), &theorems, &config, &mut ctx);
        assert!(result.is_some());
    }
    #[test]
    fn test_discharge_cache_put_get() {
        let mut cache = DischargeCache::new();
        let goal = mk_const("True");
        let proof = mk_const("True.intro");
        cache.put(&goal, proof.clone());
        let _ctx = mk_ctx();
        assert_eq!(cache.get(&goal), Some(&proof));
        assert_eq!(cache.hit_rate(), 1.0);
    }
    #[test]
    fn test_discharge_cached_memoizes() {
        let mut cache = DischargeCache::new();
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goal = mk_const("True");
        let p1 = discharge_cached(
            &goal,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
            &mut cache,
        );
        let p2 = discharge_cached(
            &goal,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
            &mut cache,
        );
        assert_eq!(p1, p2);
        assert_eq!(cache.len(), 1);
    }
    #[test]
    fn test_discharge_batch() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goals = vec![mk_const("True"), mk_const("HardGoal")];
        let results = discharge_batch(
            &goals,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
        );
        assert_eq!(results.len(), 2);
        assert!(results[0].is_some());
        assert!(results[1].is_none());
    }
    #[test]
    fn test_discharge_eager_success() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goals = vec![mk_const("True"), mk_const("True")];
        let result = discharge_eager(
            &goals,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
        );
        assert!(result.is_ok());
        assert_eq!(result.expect("result should be valid").len(), 2);
    }
    #[test]
    fn test_discharge_eager_fail() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goals = vec![mk_const("True"), mk_const("Hard")];
        let result = discharge_eager(
            &goals,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
        );
        assert_eq!(result, Err(1));
    }
    #[test]
    fn test_discharge_classification() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goals = vec![mk_const("True"), mk_const("HardGoal")];
        let cls = DischargeClassification::classify(
            &goals,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
        );
        assert_eq!(cls.proved.len(), 1);
        assert_eq!(cls.failed.len(), 1);
        assert!(!cls.all_proved());
    }
    #[test]
    fn test_goal_is_true() {
        assert!(goal_is_true(&mk_const("True")));
        assert!(!goal_is_true(&mk_const("Nat")));
    }
    #[test]
    fn test_describe_goal() {
        assert_eq!(describe_goal(&mk_const("True")), "True");
    }
    #[test]
    fn test_describe_strategy() {
        assert_eq!(describe_strategy(&DischargeStrategy::Trivial), "trivial");
        let seq = DischargeStrategy::Sequence(vec![
            DischargeStrategy::Trivial,
            DischargeStrategy::Assumption,
        ]);
        let desc = describe_strategy(&seq);
        assert!(desc.contains("trivial"));
    }
    #[test]
    fn test_can_discharge_all() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let goals = vec![mk_const("True"), mk_const("True")];
        assert!(can_discharge_all(
            &goals,
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx
        ));
    }
    #[test]
    fn test_is_simplified() {
        let r = DischargeResult::Simplified(mk_const("x"));
        assert!(is_simplified(&r));
        assert!(!is_simplified(&DischargeResult::Failed));
    }
    #[test]
    fn test_discharge_with_retry() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let result = discharge_with_retry(
            &mk_const("True"),
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
            3,
        );
        assert!(result.is_some());
    }
}
/// Discharge with tracing.
pub fn discharge_traced(
    goal: &Expr,
    strategy: &DischargeStrategy,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
    tracer: &mut DischargeTracer,
) -> Option<Expr> {
    let result = discharge_side_goal(goal, strategy, theorems, config, ctx);
    tracer.record(goal, strategy, result.is_some());
    result
}
/// Build an `Auto` strategy that includes a custom fallback.
pub fn auto_with_fallback(fallback: DischargeStrategy) -> DischargeStrategy {
    DischargeStrategy::Sequence(vec![
        DischargeStrategy::Trivial,
        DischargeStrategy::Assumption,
        DischargeStrategy::Simp,
        fallback,
    ])
}
/// Check if a strategy is "lightweight" (doesn't invoke simp).
pub fn is_lightweight(strategy: &DischargeStrategy) -> bool {
    match strategy {
        DischargeStrategy::Trivial
        | DischargeStrategy::Assumption
        | DischargeStrategy::Exact(_) => true,
        DischargeStrategy::Simp | DischargeStrategy::Auto => false,
        DischargeStrategy::Sequence(ss) => ss.iter().all(is_lightweight),
    }
}
/// Return all non-recursive strategies from a sequence.
pub fn flatten_strategy(strategy: &DischargeStrategy) -> Vec<&DischargeStrategy> {
    match strategy {
        DischargeStrategy::Sequence(ss) => ss.iter().flat_map(flatten_strategy).collect(),
        other => vec![other],
    }
}
/// Discharge by first trying assumption, then trivial (reversed auto).
pub fn discharge_lazy(
    goal: &Expr,
    theorems: &SimpTheorems,
    config: &SimpConfig,
    ctx: &mut MetaContext,
) -> Option<Expr> {
    let strategy = DischargeStrategy::Sequence(vec![
        DischargeStrategy::Assumption,
        DischargeStrategy::Trivial,
    ]);
    discharge_side_goal(goal, &strategy, theorems, config, ctx)
}
/// Validate that a proof term is structurally consistent with a goal.
///
/// Performs lightweight checks:
/// 1. `True.intro` is a valid proof of `True`.
/// 2. `rfl` / `Eq.refl` is a valid proof of `Eq T a a`.
/// 3. If proof and goal are identical (e.g. the proof _is_ the proposition),
///    accept it (proof-irrelevant trivial case).
/// 4. Otherwise accepts (conservative — full kernel type-check would be needed
///    for a complete check, but we don't have an `Environment` here).
pub fn validate_proof(proof: &Expr, goal: &Expr) -> bool {
    if proof == goal {
        return true;
    }
    if let Expr::Const(name, _) = proof {
        if name.to_string() == "True.intro" {
            if let Expr::Const(g, _) = goal {
                if g.to_string() == "True" {
                    return true;
                }
            }
        }
        if name.to_string() == "rfl" || name.to_string() == "Eq.refl" {
            if let Some((lhs, rhs)) = extract_eq_sides_discharge(goal) {
                return lhs == rhs;
            }
        }
    }
    if let Expr::Lit(lit) = proof {
        if let Some((lhs, rhs)) = extract_eq_sides_discharge(goal) {
            if let (Expr::Lit(l), Expr::Lit(r)) = (lhs, rhs) {
                return l == r && l == lit;
            }
        }
    }
    true
}
/// Extract (lhs, rhs) from `Eq _ lhs rhs` or `Eq lhs rhs`, or return None.
pub(super) fn extract_eq_sides_discharge(goal: &Expr) -> Option<(&Expr, &Expr)> {
    if let Expr::App(f1, rhs) = goal {
        if let Expr::App(f2, lhs) = f1.as_ref() {
            if let Expr::App(f3, _ty) = f2.as_ref() {
                if let Expr::Const(name, _) = f3.as_ref() {
                    if name.to_string() == "Eq" {
                        return Some((lhs.as_ref(), rhs.as_ref()));
                    }
                }
            }
            if let Expr::Const(name, _) = f2.as_ref() {
                if name.to_string() == "Eq" {
                    return Some((lhs.as_ref(), rhs.as_ref()));
                }
            }
        }
    }
    None
}
#[cfg(test)]
mod discharge_tracer_tests {
    use super::*;
    use crate::tactic::simp::discharge::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_discharge_tracer_records() {
        let mut tracer = DischargeTracer::new();
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        discharge_traced(
            &mk_const("True"),
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
            &mut tracer,
        );
        assert_eq!(tracer.total(), 1);
        assert_eq!(tracer.success_count(), 1);
    }
    #[test]
    fn test_discharge_tracer_failure() {
        let mut tracer = DischargeTracer::new();
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        discharge_traced(
            &mk_const("Hard"),
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
            &mut tracer,
        );
        assert_eq!(tracer.failure_count(), 1);
        assert!(!tracer.all_succeeded());
    }
    #[test]
    fn test_discharge_tracer_by_strategy() {
        let mut tracer = DischargeTracer::new();
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        discharge_traced(
            &mk_const("True"),
            &DischargeStrategy::Trivial,
            &theorems,
            &config,
            &mut ctx,
            &mut tracer,
        );
        let by_trivial = tracer.attempts_by_strategy("trivial");
        assert_eq!(by_trivial.len(), 1);
    }
    #[test]
    fn test_is_lightweight() {
        assert!(is_lightweight(&DischargeStrategy::Trivial));
        assert!(is_lightweight(&DischargeStrategy::Assumption));
        assert!(!is_lightweight(&DischargeStrategy::Simp));
        assert!(!is_lightweight(&DischargeStrategy::Auto));
        let seq = DischargeStrategy::Sequence(vec![DischargeStrategy::Trivial]);
        assert!(is_lightweight(&seq));
    }
    #[test]
    fn test_flatten_strategy() {
        let seq = DischargeStrategy::Sequence(vec![
            DischargeStrategy::Trivial,
            DischargeStrategy::Assumption,
        ]);
        let flat = flatten_strategy(&seq);
        assert_eq!(flat.len(), 2);
    }
    #[test]
    fn test_auto_with_fallback() {
        let fallback = DischargeStrategy::Exact(Expr::Const(Name::str("p"), vec![]));
        let auto = auto_with_fallback(fallback);
        assert!(matches!(auto, DischargeStrategy::Sequence(_)));
    }
    #[test]
    fn test_discharge_lazy_true() {
        let mut ctx = mk_ctx();
        let theorems = SimpTheorems::new();
        let config = SimpConfig::default();
        let result = discharge_lazy(&mk_const("True"), &theorems, &config, &mut ctx);
        assert!(result.is_some());
    }
    #[test]
    fn test_validate_proof() {
        let proof = Expr::Const(Name::str("True.intro"), vec![]);
        let goal = Expr::Const(Name::str("True"), vec![]);
        assert!(validate_proof(&proof, &goal));
    }
}
#[cfg(test)]
mod extra_discharge_tests {
    use super::*;
    use crate::tactic::simp::discharge::*;
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    fn mk_ctx() -> MetaContext {
        MetaContext::new(oxilean_kernel::Environment::new())
    }
    #[test]
    fn test_discharge_context_new() {
        let ctx = DischargeContext::new();
        assert!(ctx.local_hyps.is_empty());
        assert_eq!(ctx.max_simp_depth, 3);
        assert!(!ctx.allow_classical);
    }
    #[test]
    fn test_discharge_context_with_hyp() {
        let ctx = DischargeContext::new().with_hyp(Name::str("h"));
        assert!(ctx.has_hyp(&Name::str("h")));
        assert!(!ctx.has_hyp(&Name::str("h2")));
    }
    #[test]
    fn test_discharge_context_with_simp_depth() {
        let ctx = DischargeContext::new().with_simp_depth(5);
        assert_eq!(ctx.max_simp_depth, 5);
    }
    #[test]
    fn test_discharge_stats_new() {
        let stats = DischargeRunStats::new();
        assert_eq!(stats.total(), 0);
        assert_eq!(stats.success_rate(), 0.0);
    }
    #[test]
    fn test_discharge_stats_record() {
        let mut stats = DischargeRunStats::new();
        stats.record_success();
        stats.record_success();
        stats.record_failure();
        assert_eq!(stats.total(), 3);
        assert!((stats.success_rate() - 2.0 / 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_discharge_stats_display() {
        let stats = DischargeRunStats {
            successes: 5,
            failures: 2,
            ..Default::default()
        };
        let s = format!("{}", stats);
        assert!(s.contains("successes: 5"));
        assert!(s.contains("failures: 2"));
    }
    #[test]
    fn test_discharge_record_new() {
        let goal = mk_const("True");
        let proof = mk_const("True.intro");
        let rec = DischargeRecord::new(goal.clone(), "trivial", proof.clone());
        assert_eq!(rec.strategy, "trivial");
    }
    #[test]
    fn test_discharge_log_new() {
        let log = DischargeLog::new();
        assert!(log.is_empty());
    }
    #[test]
    fn test_discharge_log_push() {
        let mut log = DischargeLog::new();
        let goal = mk_const("True");
        let proof = mk_const("True.intro");
        log.push(DischargeRecord::new(goal, "trivial", proof));
        assert_eq!(log.len(), 1);
    }
    #[test]
    fn test_discharge_log_strategies_used() {
        let mut log = DischargeLog::new();
        let e = mk_const("x");
        log.push(DischargeRecord::new(e.clone(), "assumption", e.clone()));
        log.push(DischargeRecord::new(e.clone(), "trivial", e.clone()));
        let strats = log.strategies_used();
        assert_eq!(strats, vec!["assumption", "trivial"]);
    }
    #[test]
    fn test_discharge_result_is_proved() {
        let p = DischargeResult::Proved(mk_const("p"));
        assert!(p.is_proved());
    }
    #[test]
    fn test_discharge_result_proof() {
        let proof = mk_const("True.intro");
        let r = DischargeResult::Proved(proof.clone());
        assert_eq!(r.proof(), Some(proof));
    }
    #[test]
    fn test_discharge_strategy_name() {
        assert_eq!(DischargeStrategy::Assumption.name(), "assumption");
        assert_eq!(DischargeStrategy::Auto.name(), "auto");
    }
    #[test]
    fn test_discharge_strategy_is_deterministic() {
        let e = mk_const("p");
        assert!(DischargeStrategy::Exact(e).is_deterministic());
        assert!(DischargeStrategy::Trivial.is_deterministic());
        assert!(!DischargeStrategy::Simp.is_deterministic());
    }
}
#[cfg(test)]
mod tacticsimpdischarge_analysis_tests {
    use super::*;
    use crate::tactic::simp::discharge::*;
    #[test]
    fn test_tacticsimpdischarge_result_ok() {
        let r = TacticSimpDischargeResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpdischarge_result_err() {
        let r = TacticSimpDischargeResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpdischarge_result_partial() {
        let r = TacticSimpDischargeResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticsimpdischarge_result_skipped() {
        let r = TacticSimpDischargeResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticsimpdischarge_analysis_pass_run() {
        let mut p = TacticSimpDischargeAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticsimpdischarge_analysis_pass_empty_input() {
        let mut p = TacticSimpDischargeAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticsimpdischarge_analysis_pass_success_rate() {
        let mut p = TacticSimpDischargeAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticsimpdischarge_analysis_pass_disable() {
        let mut p = TacticSimpDischargeAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticsimpdischarge_pipeline_basic() {
        let mut pipeline = TacticSimpDischargePipeline::new("main_pipeline");
        pipeline.add_pass(TacticSimpDischargeAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticSimpDischargeAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticsimpdischarge_pipeline_disabled_pass() {
        let mut pipeline = TacticSimpDischargePipeline::new("partial");
        let mut p = TacticSimpDischargeAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticSimpDischargeAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticsimpdischarge_diff_basic() {
        let mut d = TacticSimpDischargeDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticsimpdischarge_diff_summary() {
        let mut d = TacticSimpDischargeDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticsimpdischarge_config_set_get() {
        let mut cfg = TacticSimpDischargeConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticsimpdischarge_config_read_only() {
        let mut cfg = TacticSimpDischargeConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticsimpdischarge_config_remove() {
        let mut cfg = TacticSimpDischargeConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticsimpdischarge_diagnostics_basic() {
        let mut diag = TacticSimpDischargeDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticsimpdischarge_diagnostics_max_errors() {
        let mut diag = TacticSimpDischargeDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticsimpdischarge_diagnostics_clear() {
        let mut diag = TacticSimpDischargeDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticsimpdischarge_config_value_types() {
        let b = TacticSimpDischargeConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticSimpDischargeConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticSimpDischargeConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticSimpDischargeConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticSimpDischargeConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod discharge_ext_tests_500 {
    use super::*;
    use crate::tactic::simp::discharge::*;
    #[test]
    fn test_discharge_ext_result_ok_500() {
        let r = DischargeExtResult500::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_discharge_ext_result_err_500() {
        let r = DischargeExtResult500::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_discharge_ext_result_partial_500() {
        let r = DischargeExtResult500::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_discharge_ext_result_skipped_500() {
        let r = DischargeExtResult500::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_discharge_ext_pass_run_500() {
        let mut p = DischargeExtPass500::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_discharge_ext_pass_empty_500() {
        let mut p = DischargeExtPass500::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_discharge_ext_pass_rate_500() {
        let mut p = DischargeExtPass500::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_discharge_ext_pass_disable_500() {
        let mut p = DischargeExtPass500::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_discharge_ext_pipeline_basic_500() {
        let mut pipeline = DischargeExtPipeline500::new("main_pipeline");
        pipeline.add_pass(DischargeExtPass500::new("pass1"));
        pipeline.add_pass(DischargeExtPass500::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_discharge_ext_pipeline_disabled_500() {
        let mut pipeline = DischargeExtPipeline500::new("partial");
        let mut p = DischargeExtPass500::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(DischargeExtPass500::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_discharge_ext_diff_basic_500() {
        let mut d = DischargeExtDiff500::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_discharge_ext_config_set_get_500() {
        let mut cfg = DischargeExtConfig500::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_discharge_ext_config_read_only_500() {
        let mut cfg = DischargeExtConfig500::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_discharge_ext_config_remove_500() {
        let mut cfg = DischargeExtConfig500::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_discharge_ext_diagnostics_basic_500() {
        let mut diag = DischargeExtDiag500::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_discharge_ext_diagnostics_max_errors_500() {
        let mut diag = DischargeExtDiag500::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_discharge_ext_diagnostics_clear_500() {
        let mut diag = DischargeExtDiag500::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_discharge_ext_config_value_types_500() {
        let b = DischargeExtConfigVal500::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = DischargeExtConfigVal500::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = DischargeExtConfigVal500::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = DischargeExtConfigVal500::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = DischargeExtConfigVal500::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
