//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Reducer};

use super::types::{
    ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, HeadForm, LabelSet,
    MinHeap, NonEmptyVec, PathBuf, PrefixCounter, RedexInfo, RedexKind, ReductionBound,
    ReductionMemo, ReductionResult, ReductionStats, ReductionStep, ReductionStrategy,
    ReductionTrace, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec,
    StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Reduce an expression using a specific strategy.
///
/// This is the primary entry point for expression reduction. It dispatches
/// to the appropriate reduction algorithm based on the strategy.
pub fn reduce_with_strategy(expr: &Expr, strategy: ReductionStrategy) -> Expr {
    let mut reducer = Reducer::new();
    match strategy {
        ReductionStrategy::WHNF => reducer.whnf(expr),
        ReductionStrategy::NF => reduce_to_nf(expr, &mut reducer, 500),
        ReductionStrategy::OneStep => one_step_reduce(expr).unwrap_or_else(|| expr.clone()),
        ReductionStrategy::CBV => reduce_cbv(expr, &mut reducer),
        ReductionStrategy::CBN => reduce_cbn(expr, &mut reducer),
        ReductionStrategy::HeadOnly => reduce_head(expr),
    }
}
/// Reduce an expression to full normal form.
///
/// After reducing to WHNF, recursively reduce all sub-expressions.
/// The fuel parameter limits total recursion to avoid infinite loops.
pub(super) fn reduce_to_nf(expr: &Expr, reducer: &mut Reducer, fuel: u32) -> Expr {
    if fuel == 0 {
        return expr.clone();
    }
    let whnf = reducer.whnf(expr);
    match whnf {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) | Expr::Const(_, _) => whnf,
        Expr::Lam(bi, name, ty, body) => {
            let ty_nf = reduce_to_nf(&ty, reducer, fuel - 1);
            let body_nf = reduce_to_nf(&body, reducer, fuel - 1);
            Expr::Lam(bi, name, Box::new(ty_nf), Box::new(body_nf))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_nf = reduce_to_nf(&ty, reducer, fuel - 1);
            let body_nf = reduce_to_nf(&body, reducer, fuel - 1);
            Expr::Pi(bi, name, Box::new(ty_nf), Box::new(body_nf))
        }
        Expr::App(f, a) => {
            let f_nf = reduce_to_nf(&f, reducer, fuel - 1);
            let a_nf = reduce_to_nf(&a, reducer, fuel - 1);
            let rebuilt = Expr::App(Box::new(f_nf), Box::new(a_nf));
            let whnf2 = reducer.whnf(&rebuilt);
            if whnf2 == rebuilt {
                rebuilt
            } else {
                reduce_to_nf(&whnf2, reducer, fuel.saturating_sub(1))
            }
        }
        Expr::Let(_name, _ty, val, body) => {
            let reduced = crate::subst::instantiate(&body, &val);
            reduce_to_nf(&reduced, reducer, fuel - 1)
        }
        Expr::Proj(struct_name, idx, e) => {
            let e_nf = reduce_to_nf(&e, reducer, fuel - 1);
            Expr::Proj(struct_name, idx, Box::new(e_nf))
        }
    }
}
/// Apply exactly one reduction step (outermost-leftmost).
///
/// Returns `Some(reduced)` if a redex was found, `None` if already in NF.
#[allow(clippy::only_used_in_recursion)]
pub(super) fn one_step_reduce(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_, _, _, body) = f.as_ref() {
                let reduced = crate::subst::instantiate(body, a);
                return Some(reduced);
            }
            if let Some(f_reduced) = one_step_reduce(f) {
                return Some(Expr::App(Box::new(f_reduced), a.clone()));
            }
            if let Some(a_reduced) = one_step_reduce(a) {
                return Some(Expr::App(f.clone(), Box::new(a_reduced)));
            }
            None
        }
        Expr::Let(_, _, val, body) => {
            let reduced = crate::subst::instantiate(body, val);
            Some(reduced)
        }
        Expr::Lam(bi, name, ty, body) => {
            if let Some(ty_r) = one_step_reduce(ty) {
                return Some(Expr::Lam(*bi, name.clone(), Box::new(ty_r), body.clone()));
            }
            if let Some(body_r) = one_step_reduce(body) {
                return Some(Expr::Lam(*bi, name.clone(), ty.clone(), Box::new(body_r)));
            }
            None
        }
        Expr::Pi(bi, name, ty, body) => {
            if let Some(ty_r) = one_step_reduce(ty) {
                return Some(Expr::Pi(*bi, name.clone(), Box::new(ty_r), body.clone()));
            }
            if let Some(body_r) = one_step_reduce(body) {
                return Some(Expr::Pi(*bi, name.clone(), ty.clone(), Box::new(body_r)));
            }
            None
        }
        Expr::Proj(struct_name, idx, e) => {
            if let Some(e_r) = one_step_reduce(e) {
                return Some(Expr::Proj(struct_name.clone(), *idx, Box::new(e_r)));
            }
            None
        }
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) | Expr::Const(_, _) => None,
    }
}
/// Call-by-value reduction: evaluate arguments to WHNF before applying.
///
/// In CBV, all arguments are reduced to WHNF (values) before the function
/// body is entered. This is strict/eager evaluation.
pub(super) fn reduce_cbv(expr: &Expr, reducer: &mut Reducer) -> Expr {
    reduce_cbv_impl(expr, reducer, 500)
}
#[allow(clippy::only_used_in_recursion)]
pub(super) fn reduce_cbv_impl(expr: &Expr, reducer: &mut Reducer, fuel: u32) -> Expr {
    if fuel == 0 {
        return expr.clone();
    }
    match expr {
        Expr::App(f, a) => {
            let a_val = reducer.whnf(a);
            let f_val = reduce_cbv_impl(f, reducer, fuel - 1);
            match f_val {
                Expr::Lam(_, _, _, body) => {
                    let reduced = crate::subst::instantiate(&body, &a_val);
                    reduce_cbv_impl(&reduced, reducer, fuel - 1)
                }
                _ => Expr::App(Box::new(f_val), Box::new(a_val)),
            }
        }
        Expr::Let(_, _, val, body) => {
            let val_whnf = reducer.whnf(val);
            let reduced = crate::subst::instantiate(body, &val_whnf);
            reduce_cbv_impl(&reduced, reducer, fuel - 1)
        }
        Expr::Lam(_, _, _, _) | Expr::Pi(_, _, _, _) => reducer.whnf(expr),
        _ => reducer.whnf(expr),
    }
}
/// Call-by-name reduction: reduce function before arguments, pass args unevaluated.
///
/// In CBN, arguments are substituted without first being evaluated.
/// The function is reduced to a lambda, then the body is evaluated with
/// the argument passed as-is.
pub(super) fn reduce_cbn(expr: &Expr, reducer: &mut Reducer) -> Expr {
    reduce_cbn_impl(expr, reducer, 500)
}
#[allow(clippy::only_used_in_recursion)]
pub(super) fn reduce_cbn_impl(expr: &Expr, reducer: &mut Reducer, fuel: u32) -> Expr {
    if fuel == 0 {
        return expr.clone();
    }
    match expr {
        Expr::App(f, a) => {
            let f_val = reduce_cbn_impl(f, reducer, fuel - 1);
            match f_val {
                Expr::Lam(_, _, _, body) => {
                    let reduced = crate::subst::instantiate(&body, a);
                    reduce_cbn_impl(&reduced, reducer, fuel - 1)
                }
                _ => Expr::App(Box::new(f_val), a.clone()),
            }
        }
        Expr::Let(_, _, val, body) => {
            let reduced = crate::subst::instantiate(body, val);
            reduce_cbn_impl(&reduced, reducer, fuel - 1)
        }
        _ => reducer.whnf(expr),
    }
}
/// Reduce only the head of an expression (not arguments).
///
/// For an application `f a1 ... an`, reduces `f` to WHNF without
/// reducing the arguments.
pub fn reduce_head(expr: &Expr) -> Expr {
    let mut reducer = Reducer::new();
    reducer.whnf(expr)
}
/// Check if an expression is in normal form.
///
/// An expression is in normal form if no reduction rules apply. This
/// conservatively checks syntactic normal form without performing reductions.
pub fn is_normal_form(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) => true,
        Expr::Const(_, _) => true,
        Expr::Lit(_) => true,
        Expr::Lam(_, _, ty, body) => is_normal_form(ty) && is_normal_form(body),
        Expr::Pi(_, _, ty, body) => is_normal_form(ty) && is_normal_form(body),
        Expr::App(f, a) => {
            if matches!(f.as_ref(), Expr::Lam(_, _, _, _)) {
                return false;
            }
            is_normal_form(f) && is_normal_form(a)
        }
        Expr::Let(_, _, _, _) => false,
        Expr::Proj(_, _, e) => is_normal_form(e),
    }
}
/// Count the number of reduction steps needed.
///
/// Iteratively reduces the expression until it reaches a fixed point,
/// counting the steps taken. Capped at 10,000 steps to prevent infinite loops.
pub fn count_reduction_steps(expr: &Expr, _strategy: ReductionStrategy) -> usize {
    let mut steps = 0;
    let mut current = expr.clone();
    loop {
        let next = reduce_with_strategy(&current, ReductionStrategy::OneStep);
        if next == current {
            break;
        }
        steps += 1;
        current = next;
        if steps > 10000 {
            break;
        }
    }
    steps
}
/// Check if two expressions are convertible (definitionally equal).
///
/// Reduces both expressions to normal form and checks for syntactic equality.
/// This is a coarse approximation; the kernel's definitional equality checker
/// is more precise.
pub fn are_convertible(e1: &Expr, e2: &Expr) -> bool {
    let nf1 = reduce_with_strategy(e1, ReductionStrategy::NF);
    let nf2 = reduce_with_strategy(e2, ReductionStrategy::NF);
    nf1 == nf2
}
/// Attempt a single reduction step, returning a `ReductionResult`.
///
/// Unlike `count_reduction_steps`, this function performs at most one step
/// and clearly indicates whether the expression changed.
pub fn try_reduce_step(expr: &Expr) -> ReductionResult {
    let reduced = reduce_with_strategy(expr, ReductionStrategy::OneStep);
    if reduced == *expr {
        ReductionResult::Normal(reduced)
    } else {
        ReductionResult::Reduced(reduced)
    }
}
/// Build a reduction trace for an expression.
///
/// Reduces the expression step-by-step, recording each intermediate form.
/// Stops after `max_steps` steps (default 100).
pub fn build_reduction_trace(expr: &Expr, max_steps: usize) -> ReductionTrace {
    let mut trace = ReductionTrace::new();
    let mut current = expr.clone();
    for _ in 0..max_steps {
        let next = reduce_with_strategy(&current, ReductionStrategy::OneStep);
        if next == current {
            trace.reached_normal = true;
            break;
        }
        trace.steps.push(ReductionStep {
            before: current.clone(),
            after: next.clone(),
            rule: "beta/delta".to_string(),
        });
        current = next;
    }
    if !trace.reached_normal && trace.len() >= max_steps {
        trace.truncated = true;
    }
    trace
}
/// Compute the size (number of nodes) of an expression.
///
/// Used as a heuristic for reduction complexity.
pub fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + expr_size(ty) + expr_size(body),
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, e) => 1 + expr_size(e),
    }
}
/// Compute the depth of an expression tree.
pub fn expr_depth(expr: &Expr) -> usize {
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        Expr::Proj(_, _, e) => 1 + expr_depth(e),
    }
}
/// Count the number of beta-redexes in an expression.
///
/// A beta-redex is an application of the form `(λx. e) a`.
pub fn count_beta_redexes(expr: &Expr) -> usize {
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::App(f, a) => {
            let is_redex = matches!(f.as_ref(), Expr::Lam(_, _, _, _));
            let base = if is_redex { 1 } else { 0 };
            base + count_beta_redexes(f) + count_beta_redexes(a)
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_beta_redexes(ty) + count_beta_redexes(body)
        }
        Expr::Let(_, ty, val, body) => {
            1 + count_beta_redexes(ty) + count_beta_redexes(val) + count_beta_redexes(body)
        }
        Expr::Proj(_, _, e) => count_beta_redexes(e),
    }
}
/// Classify the head form of an expression.
pub fn classify_head(expr: &Expr) -> HeadForm {
    match expr {
        Expr::BVar(i) => HeadForm::BVar(*i),
        Expr::FVar(_) => HeadForm::FVar,
        Expr::Const(name, _) => HeadForm::Const(name.clone()),
        Expr::Sort(_) => HeadForm::Sort,
        Expr::Lam(_, _, _, _) => HeadForm::Lambda,
        Expr::Pi(_, _, _, _) => HeadForm::Pi,
        Expr::Lit(_) => HeadForm::Lit,
        Expr::Let(_, _, _, _) => HeadForm::Let,
        Expr::Proj(_, _, _) => HeadForm::Proj,
        Expr::App(f, _) => {
            if matches!(f.as_ref(), Expr::Lam(_, _, _, _)) {
                HeadForm::BetaRedex
            } else {
                classify_head(f)
            }
        }
    }
}
/// Find all redexes in an expression.
///
/// Returns information about each reducible sub-expression.
pub fn find_redexes(expr: &Expr) -> Vec<RedexInfo> {
    let mut redexes = Vec::new();
    find_redexes_rec(expr, 0, &mut redexes);
    redexes
}
pub(super) fn find_redexes_rec(expr: &Expr, depth: usize, redexes: &mut Vec<RedexInfo>) {
    match expr {
        Expr::App(f, a) => {
            if matches!(f.as_ref(), Expr::Lam(_, _, _, _)) {
                redexes.push(RedexInfo {
                    kind: RedexKind::Beta,
                    depth,
                    size: expr_size(expr),
                });
            }
            find_redexes_rec(f, depth + 1, redexes);
            find_redexes_rec(a, depth + 1, redexes);
        }
        Expr::Let(_, ty, val, body) => {
            redexes.push(RedexInfo {
                kind: RedexKind::Let,
                depth,
                size: expr_size(expr),
            });
            find_redexes_rec(ty, depth + 1, redexes);
            find_redexes_rec(val, depth + 1, redexes);
            find_redexes_rec(body, depth + 1, redexes);
        }
        Expr::Proj(_, _, e) => {
            find_redexes_rec(e, depth + 1, redexes);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            find_redexes_rec(ty, depth + 1, redexes);
            find_redexes_rec(body, depth + 1, redexes);
        }
        _ => {}
    }
}
/// Reduce an expression with a bound on the number of steps.
///
/// Returns the reduced expression and statistics.
pub fn reduce_bounded(
    expr: &Expr,
    strategy: ReductionStrategy,
    bound: ReductionBound,
) -> (Expr, ReductionStats) {
    let mut stats = ReductionStats::new();
    let mut current = expr.clone();
    loop {
        if bound.exceeded(stats.total_steps, expr_size(&current)) {
            stats.aborted = true;
            break;
        }
        let next = reduce_with_strategy(&current, strategy);
        if next == current {
            break;
        }
        stats.total_steps += 1;
        let head = classify_head(&current);
        match head {
            HeadForm::BetaRedex => stats.beta_steps += 1,
            HeadForm::Let => stats.let_steps += 1,
            _ => stats.delta_steps += 1,
        }
        current = next;
    }
    (current, stats)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal, Name};
    fn mk_nat_lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn mk_lam(body: Expr) -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(body),
        )
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    fn mk_let(val: Expr, body: Expr) -> Expr {
        Expr::Let(
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(val),
            Box::new(body),
        )
    }
    #[test]
    fn test_reduce_whnf() {
        let expr = Expr::Sort(Level::zero());
        let result = reduce_with_strategy(&expr, ReductionStrategy::WHNF);
        assert_eq!(result, expr);
    }
    #[test]
    fn test_is_normal_form_sort() {
        let expr = Expr::Sort(Level::zero());
        assert!(is_normal_form(&expr));
    }
    #[test]
    fn test_is_normal_form_lit() {
        let expr = Expr::Lit(Literal::Nat(42));
        assert!(is_normal_form(&expr));
    }
    #[test]
    fn test_not_normal_form_beta() {
        let lam = mk_lam(Expr::BVar(0));
        let app = mk_app(lam, mk_nat_lit(42));
        assert!(!is_normal_form(&app));
    }
    #[test]
    fn test_not_normal_form_let() {
        let let_expr = mk_let(mk_nat_lit(42), Expr::BVar(0));
        assert!(!is_normal_form(&let_expr));
    }
    #[test]
    fn test_are_convertible_same() {
        let e1 = mk_nat_lit(42);
        let e2 = mk_nat_lit(42);
        assert!(are_convertible(&e1, &e2));
    }
    #[test]
    fn test_count_reduction_steps_zero() {
        let expr = mk_nat_lit(42);
        let steps = count_reduction_steps(&expr, ReductionStrategy::WHNF);
        assert_eq!(steps, 0);
    }
    #[test]
    fn test_reduction_strategy_name() {
        assert_eq!(ReductionStrategy::WHNF.name(), "whnf");
        assert_eq!(ReductionStrategy::NF.name(), "nf");
        assert_eq!(ReductionStrategy::CBV.name(), "cbv");
        assert_eq!(ReductionStrategy::HeadOnly.name(), "head-only");
    }
    #[test]
    fn test_reduction_strategy_lazy_eager() {
        assert!(ReductionStrategy::WHNF.is_lazy());
        assert!(ReductionStrategy::CBV.is_eager());
        assert!(!ReductionStrategy::WHNF.is_eager());
    }
    #[test]
    fn test_reduction_strategy_complete() {
        assert!(ReductionStrategy::NF.is_complete());
        assert!(!ReductionStrategy::WHNF.is_complete());
    }
    #[test]
    fn test_try_reduce_step_normal() {
        let expr = mk_nat_lit(5);
        let result = try_reduce_step(&expr);
        assert!(result.is_normal());
    }
    #[test]
    fn test_try_reduce_step_reduces() {
        let lam = mk_lam(Expr::BVar(0));
        let app = mk_app(lam, mk_nat_lit(0));
        let result = try_reduce_step(&app);
        assert!(result.was_reduced());
    }
    #[test]
    fn test_classify_head_sort() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(classify_head(&e), HeadForm::Sort);
    }
    #[test]
    fn test_classify_head_lambda() {
        let e = mk_lam(Expr::BVar(0));
        assert_eq!(classify_head(&e), HeadForm::Lambda);
    }
    #[test]
    fn test_classify_head_beta_redex() {
        let lam = mk_lam(Expr::BVar(0));
        let app = mk_app(lam, mk_nat_lit(42));
        assert_eq!(classify_head(&app), HeadForm::BetaRedex);
    }
    #[test]
    fn test_classify_head_let() {
        let e = mk_let(mk_nat_lit(1), Expr::BVar(0));
        assert_eq!(classify_head(&e), HeadForm::Let);
    }
    #[test]
    fn test_expr_size_lit() {
        assert_eq!(expr_size(&mk_nat_lit(0)), 1);
    }
    #[test]
    fn test_expr_size_app() {
        let app = mk_app(mk_nat_lit(1), mk_nat_lit(2));
        assert_eq!(expr_size(&app), 3);
    }
    #[test]
    fn test_expr_depth() {
        let lit = mk_nat_lit(0);
        assert_eq!(expr_depth(&lit), 1);
        let app = mk_app(lit.clone(), lit.clone());
        assert_eq!(expr_depth(&app), 2);
    }
    #[test]
    fn test_count_beta_redexes_none() {
        let expr = mk_nat_lit(42);
        assert_eq!(count_beta_redexes(&expr), 0);
    }
    #[test]
    fn test_count_beta_redexes_one() {
        let lam = mk_lam(Expr::BVar(0));
        let app = mk_app(lam, mk_nat_lit(5));
        assert_eq!(count_beta_redexes(&app), 1);
    }
    #[test]
    fn test_count_beta_redexes_let() {
        let e = mk_let(mk_nat_lit(1), Expr::BVar(0));
        assert_eq!(count_beta_redexes(&e), 1);
    }
    #[test]
    fn test_find_redexes_empty() {
        let expr = mk_nat_lit(42);
        let redexes = find_redexes(&expr);
        assert!(redexes.is_empty());
    }
    #[test]
    fn test_find_redexes_beta() {
        let lam = mk_lam(Expr::BVar(0));
        let app = mk_app(lam, mk_nat_lit(5));
        let redexes = find_redexes(&app);
        assert!(!redexes.is_empty());
        assert!(redexes.iter().any(|r| r.kind == RedexKind::Beta));
    }
    #[test]
    fn test_find_redexes_let() {
        let e = mk_let(mk_nat_lit(1), Expr::BVar(0));
        let redexes = find_redexes(&e);
        assert!(redexes.iter().any(|r| r.kind == RedexKind::Let));
    }
    #[test]
    fn test_reduction_stats_summary() {
        let stats = ReductionStats {
            beta_steps: 3,
            delta_steps: 1,
            let_steps: 0,
            proj_steps: 0,
            total_steps: 4,
            aborted: false,
        };
        let s = stats.summary();
        assert!(s.contains("β:3"));
        assert!(s.contains("total:4"));
    }
    #[test]
    fn test_reduction_bound_exceeded() {
        let bound = ReductionBound::Steps(5);
        assert!(!bound.exceeded(4, 100));
        assert!(bound.exceeded(5, 100));
    }
    #[test]
    fn test_reduce_bounded_normal() {
        let expr = mk_nat_lit(42);
        let (result, stats) =
            reduce_bounded(&expr, ReductionStrategy::WHNF, ReductionBound::Steps(100));
        assert_eq!(result, expr);
        assert!(!stats.any_reductions());
    }
    #[test]
    fn test_build_reduction_trace_normal() {
        let expr = mk_nat_lit(1);
        let trace = build_reduction_trace(&expr, 10);
        assert!(trace.reached_normal);
        assert!(trace.is_empty());
    }
    #[test]
    fn test_head_form_is_neutral() {
        assert!(HeadForm::BVar(0).is_neutral());
        assert!(HeadForm::FVar.is_neutral());
        assert!(HeadForm::Const(Name::str("foo")).is_neutral());
        assert!(!HeadForm::Lambda.is_neutral());
    }
    #[test]
    fn test_redex_kind_description() {
        assert!(RedexKind::Beta.description().contains('β'));
        assert!(RedexKind::Let.description().contains("let"));
        let delta = RedexKind::Delta(Name::str("foo"));
        assert!(delta.description().contains("foo"));
    }
    #[test]
    fn test_reduction_result_accessors() {
        let e = mk_nat_lit(0);
        let r = ReductionResult::Normal(e.clone());
        assert!(r.is_normal());
        assert!(!r.was_reduced());
        assert_eq!(r.into_expr(), e);
    }
}
/// Check whether two expressions are alpha-equivalent (same up to variable renaming).
///
/// Uses a structural comparison, treating `BVar` indices as-is.
/// This is a lightweight check — it does not perform reduction.
pub fn alpha_equiv(e1: &Expr, e2: &Expr) -> bool {
    match (e1, e2) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(a), Expr::FVar(b)) => a == b,
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => alpha_equiv(f1, f2) && alpha_equiv(a1, a2),
        (Expr::Lam(_, _, t1, b1), Expr::Lam(_, _, t2, b2)) => {
            alpha_equiv(t1, t2) && alpha_equiv(b1, b2)
        }
        (Expr::Pi(_, _, t1, b1), Expr::Pi(_, _, t2, b2)) => {
            alpha_equiv(t1, t2) && alpha_equiv(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            alpha_equiv(ty1, ty2) && alpha_equiv(v1, v2) && alpha_equiv(b1, b2)
        }
        _ => false,
    }
}
/// Compute a fingerprint (hash) for an expression.
///
/// Used as a quick non-equality check before full comparison.
pub fn expr_fingerprint(expr: &Expr) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    format!("{:?}", expr).hash(&mut h);
    h.finish()
}
#[cfg(test)]
mod extra_reduction_tests {
    use super::*;
    use crate::{Level, Literal, Name};
    fn mk_nat_lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn mk_sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_reduction_memo_insert_get() {
        let mut memo = ReductionMemo::new();
        let expr = mk_nat_lit(5);
        let result = mk_nat_lit(5);
        memo.insert(ReductionStrategy::WHNF, &expr, result.clone());
        let got = memo.get(ReductionStrategy::WHNF, &expr);
        assert_eq!(got, Some(&result));
    }
    #[test]
    fn test_reduction_memo_miss() {
        let mut memo = ReductionMemo::new();
        let expr = mk_nat_lit(42);
        assert_eq!(memo.get(ReductionStrategy::NF, &expr), None);
    }
    #[test]
    fn test_reduction_memo_hit_rate() {
        let mut memo = ReductionMemo::new();
        let expr = mk_nat_lit(1);
        memo.insert(ReductionStrategy::WHNF, &expr, expr.clone());
        let _ = memo.get(ReductionStrategy::WHNF, &expr);
        let _ = memo.get(ReductionStrategy::NF, &expr);
        let rate = memo.hit_rate();
        assert!(rate > 0.0 && rate <= 1.0);
    }
    #[test]
    fn test_reduction_memo_clear() {
        let mut memo = ReductionMemo::new();
        let expr = mk_nat_lit(1);
        memo.insert(ReductionStrategy::WHNF, &expr, expr.clone());
        memo.clear();
        assert!(memo.is_empty());
    }
    #[test]
    fn test_alpha_equiv_bvar() {
        assert!(alpha_equiv(&Expr::BVar(0), &Expr::BVar(0)));
        assert!(!alpha_equiv(&Expr::BVar(0), &Expr::BVar(1)));
    }
    #[test]
    fn test_alpha_equiv_sort() {
        assert!(alpha_equiv(&mk_sort(), &mk_sort()));
    }
    #[test]
    fn test_alpha_equiv_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = mk_nat_lit(1);
        let e1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let e2 = Expr::App(Box::new(f), Box::new(a));
        assert!(alpha_equiv(&e1, &e2));
    }
    #[test]
    fn test_alpha_equiv_different_constructors() {
        let e1 = mk_nat_lit(1);
        let e2 = mk_sort();
        assert!(!alpha_equiv(&e1, &e2));
    }
    #[test]
    fn test_expr_fingerprint_same() {
        let e = mk_nat_lit(7);
        assert_eq!(expr_fingerprint(&e), expr_fingerprint(&e));
    }
    #[test]
    fn test_expr_fingerprint_different() {
        let e1 = mk_nat_lit(1);
        let e2 = mk_nat_lit(2);
        assert_ne!(expr_fingerprint(&e1), expr_fingerprint(&e2));
    }
    #[test]
    fn test_reduction_memo_strategy_separation() {
        let mut memo = ReductionMemo::new();
        let expr = mk_nat_lit(3);
        memo.insert(ReductionStrategy::WHNF, &expr, mk_nat_lit(10));
        memo.insert(ReductionStrategy::NF, &expr, mk_nat_lit(20));
        assert_eq!(
            memo.get(ReductionStrategy::WHNF, &expr),
            Some(&mk_nat_lit(10))
        );
        assert_eq!(
            memo.get(ReductionStrategy::NF, &expr),
            Some(&mk_nat_lit(20))
        );
    }
    #[test]
    fn test_reduction_memo_len() {
        let mut memo = ReductionMemo::new();
        let e = mk_nat_lit(0);
        memo.insert(ReductionStrategy::WHNF, &e, e.clone());
        assert_eq!(memo.len(), 1);
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
