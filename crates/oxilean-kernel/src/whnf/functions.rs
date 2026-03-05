//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Environment, Expr, FVarId, Level, Literal, Name, Reducer};

use super::types::{
    ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet, MinHeap,
    NonEmptyVec, PathBuf, PrefixCounter, ReductionBudget, RewriteRule, RewriteRuleSet, SimpleDag,
    SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, StuckReason,
    TokenBucket, TransformStat, TransitiveClosure, VersionedRecord, WhnfCache, WhnfCacheKey,
    WhnfConfig, WhnfDepthBudget, WhnfHead, WhnfReductionOrder, WhnfStats, WindowIterator,
    WriteOnce,
};

/// Reduce an expression to Weak Head Normal Form using the default reducer.
pub fn whnf(expr: &Expr) -> Expr {
    let mut reducer = Reducer::new();
    reducer.whnf(expr)
}
/// Reduce an expression to WHNF in the context of an Environment.
pub fn whnf_env(expr: &Expr, env: &Environment) -> Expr {
    let mut reducer = Reducer::new();
    reducer.whnf_env(expr, env)
}
/// Check whether an expression is already in Weak Head Normal Form.
pub fn is_whnf(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => true,
        Expr::Lam(_, _, _, _) | Expr::Pi(_, _, _, _) => true,
        Expr::App(f, _) => !matches!(f.as_ref(), Expr::Lam(_, _, _, _)),
        Expr::Let(_, _, _, _) => false,
        _ => true,
    }
}
/// Check whether the WHNF of an expression is a Sort.
pub fn whnf_is_sort(expr: &Expr) -> bool {
    matches!(whnf(expr), Expr::Sort(_))
}
/// Check whether the WHNF of an expression is a Pi type.
pub fn whnf_is_pi(expr: &Expr) -> bool {
    matches!(whnf(expr), Expr::Pi(_, _, _, _))
}
/// Check whether the WHNF of an expression is a lambda abstraction.
pub fn whnf_is_lambda(expr: &Expr) -> bool {
    matches!(whnf(expr), Expr::Lam(_, _, _, _))
}
/// Check whether the WHNF of an expression is a named constant.
pub fn whnf_is_const(expr: &Expr) -> Option<Name> {
    match whnf(expr) {
        Expr::Const(n, _) => Some(n),
        _ => None,
    }
}
/// Check whether the WHNF of an expression is a literal.
pub fn whnf_is_lit(expr: &Expr) -> Option<Literal> {
    match whnf(expr) {
        Expr::Lit(l) => Some(l),
        _ => None,
    }
}
/// Decompose an expression (after WHNF) into its head and spine arguments.
pub fn whnf_spine(expr: &Expr) -> (WhnfHead, Vec<Expr>) {
    let reduced = whnf(expr);
    spine_of(&reduced)
}
/// Decompose an already-reduced expression into head and arguments.
pub fn spine_of(expr: &Expr) -> (WhnfHead, Vec<Expr>) {
    let mut args = Vec::new();
    let mut current = expr.clone();
    loop {
        match current {
            Expr::App(f, a) => {
                args.push(*a);
                current = *f;
            }
            other => {
                args.reverse();
                let head = match other {
                    Expr::Sort(l) => WhnfHead::Sort(l),
                    Expr::BVar(i) => WhnfHead::BVar(i),
                    Expr::FVar(id) => WhnfHead::FVar(id),
                    Expr::Const(n, ls) => WhnfHead::Const(n, ls),
                    Expr::Lam(bi, n, ty, body) => WhnfHead::Lam(bi, n, ty, body),
                    Expr::Pi(bi, n, ty, body) => WhnfHead::Pi(bi, n, ty, body),
                    Expr::Lit(l) => WhnfHead::Lit(l),
                    Expr::App(_, _) | Expr::Let(_, _, _, _) => unreachable!(),
                    _ => WhnfHead::BVar(0),
                };
                return (head, args);
            }
        }
    }
}
/// Decompose with WHNF in environment.
pub fn whnf_spine_env(expr: &Expr, env: &Environment) -> (WhnfHead, Vec<Expr>) {
    let reduced = whnf_env(expr, env);
    spine_of(&reduced)
}
/// Reduce with a step budget. Returns None if the budget is exhausted.
pub fn whnf_budgeted(expr: &Expr, budget: &mut ReductionBudget) -> Option<Expr> {
    if !budget.consume() {
        return None;
    }
    Some(whnf(expr))
}
/// Compare the WHNF heads of two expressions for structural equality.
pub fn whnf_heads_match(a: &Expr, b: &Expr) -> bool {
    let (ha, _) = whnf_spine(a);
    let (hb, _) = whnf_spine(b);
    match (&ha, &hb) {
        (WhnfHead::Sort(la), WhnfHead::Sort(lb)) => la == lb,
        (WhnfHead::BVar(i), WhnfHead::BVar(j)) => i == j,
        (WhnfHead::FVar(i), WhnfHead::FVar(j)) => i == j,
        (WhnfHead::Const(na, _), WhnfHead::Const(nb, _)) => na == nb,
        (WhnfHead::Lit(la), WhnfHead::Lit(lb)) => la == lb,
        _ => false,
    }
}
/// Count the number of arguments in an application spine.
pub fn app_arity(expr: &Expr) -> usize {
    let (_, args) = spine_of(expr);
    args.len()
}
/// Destructure a Pi type after WHNF reduction.
pub fn whnf_as_pi(expr: &Expr) -> Option<(BinderInfo, Name, Expr, Expr)> {
    match whnf(expr) {
        Expr::Pi(bi, name, dom, cod) => Some((bi, name, *dom, *cod)),
        _ => None,
    }
}
/// Destructure a Pi type after WHNF reduction with environment.
pub fn whnf_as_pi_env(expr: &Expr, env: &Environment) -> Option<(BinderInfo, Name, Expr, Expr)> {
    match whnf_env(expr, env) {
        Expr::Pi(bi, name, dom, cod) => Some((bi, name, *dom, *cod)),
        _ => None,
    }
}
/// Extract the universe level from a WHNF Sort expression.
pub fn whnf_as_sort(expr: &Expr) -> Option<Level> {
    match whnf(expr) {
        Expr::Sort(l) => Some(l),
        _ => None,
    }
}
/// Collect the full Pi telescope (sequence of binders) from a type.
pub fn collect_pi_telescope(ty: &Expr) -> (Vec<(BinderInfo, Name, Expr)>, Expr) {
    let mut binders = Vec::new();
    let mut current = ty.clone();
    loop {
        match whnf(&current) {
            Expr::Pi(bi, name, dom, body) => {
                binders.push((bi, name, *dom));
                current = *body;
            }
            other => return (binders, other),
        }
    }
}
/// Collect the full Pi telescope using an environment for delta unfolding.
pub fn collect_pi_telescope_env(
    ty: &Expr,
    env: &Environment,
) -> (Vec<(BinderInfo, Name, Expr)>, Expr) {
    let mut binders = Vec::new();
    let mut current = ty.clone();
    loop {
        match whnf_env(&current, env) {
            Expr::Pi(bi, name, dom, body) => {
                binders.push((bi, name, *dom));
                current = *body;
            }
            other => return (binders, other),
        }
    }
}
/// Determine why a WHNF expression is stuck.
pub fn stuck_reason(expr: &Expr) -> StuckReason {
    match whnf(expr) {
        Expr::FVar(id) => StuckReason::FreeVariable(id),
        Expr::Const(n, _) => StuckReason::OpaqueConst(n),
        _ => StuckReason::NormalForm,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Level;
    fn sort0() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn sort1() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn identity_lam() -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(sort0()),
            Box::new(Expr::BVar(0)),
        )
    }
    #[test]
    fn test_is_whnf_sort() {
        assert!(is_whnf(&sort0()));
        assert!(is_whnf(&sort1()));
    }
    #[test]
    fn test_is_whnf_bvar() {
        assert!(is_whnf(&Expr::BVar(0)));
        assert!(is_whnf(&Expr::BVar(5)));
    }
    #[test]
    fn test_is_whnf_lit() {
        assert!(is_whnf(&Expr::Lit(Literal::Nat(42))));
        assert!(is_whnf(&Expr::Lit(Literal::Str("hello".into()))));
    }
    #[test]
    fn test_is_whnf_lam() {
        assert!(is_whnf(&identity_lam()));
    }
    #[test]
    fn test_is_whnf_let_false() {
        let e = Expr::Let(
            Name::str("x"),
            Box::new(sort0()),
            Box::new(Expr::Lit(Literal::Nat(1))),
            Box::new(Expr::BVar(0)),
        );
        assert!(!is_whnf(&e));
    }
    #[test]
    fn test_is_whnf_app_lambda_false() {
        let app = Expr::App(
            Box::new(identity_lam()),
            Box::new(Expr::Lit(Literal::Nat(1))),
        );
        assert!(!is_whnf(&app));
    }
    #[test]
    fn test_whnf_beta_reduce() {
        let arg = Expr::Lit(Literal::Nat(99));
        let app = Expr::App(Box::new(identity_lam()), Box::new(arg.clone()));
        let result = whnf(&app);
        assert_eq!(result, arg);
    }
    #[test]
    fn test_whnf_is_sort_true() {
        assert!(whnf_is_sort(&sort0()));
    }
    #[test]
    fn test_whnf_is_sort_false() {
        assert!(!whnf_is_sort(&identity_lam()));
    }
    #[test]
    fn test_whnf_is_pi() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(sort0()),
            Box::new(sort0()),
        );
        assert!(whnf_is_pi(&pi));
    }
    #[test]
    fn test_whnf_is_lambda() {
        assert!(whnf_is_lambda(&identity_lam()));
    }
    #[test]
    fn test_whnf_is_const() {
        let c = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(whnf_is_const(&c), Some(Name::str("Nat")));
    }
    #[test]
    fn test_whnf_is_lit() {
        let l = Expr::Lit(Literal::Nat(7));
        assert_eq!(whnf_is_lit(&l), Some(Literal::Nat(7)));
    }
    #[test]
    fn test_spine_of_no_args() {
        let c = Expr::Const(Name::str("Nat"), vec![]);
        let (head, args) = spine_of(&c);
        assert_eq!(head.as_const_name(), Some(&Name::str("Nat")));
        assert!(args.is_empty());
    }
    #[test]
    fn test_spine_of_two_args() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (_, args) = spine_of(&app);
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], a);
        assert_eq!(args[1], b);
    }
    #[test]
    fn test_collect_pi_telescope_empty() {
        let (binders, body) = collect_pi_telescope(&sort0());
        assert!(binders.is_empty());
        assert_eq!(body, sort0());
    }
    #[test]
    fn test_collect_pi_telescope_one() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(sort0()),
            Box::new(sort1()),
        );
        let (binders, body) = collect_pi_telescope(&pi);
        assert_eq!(binders.len(), 1);
        assert_eq!(binders[0].1, Name::str("x"));
        assert_eq!(body, sort1());
    }
    #[test]
    fn test_collect_pi_telescope_nested() {
        let pi_inner = Expr::Pi(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(sort0()),
            Box::new(sort1()),
        );
        let pi_outer = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(sort0()),
            Box::new(pi_inner),
        );
        let (binders, _body) = collect_pi_telescope(&pi_outer);
        assert_eq!(binders.len(), 2);
        assert_eq!(binders[0].1, Name::str("x"));
        assert_eq!(binders[1].1, Name::str("y"));
    }
    #[test]
    fn test_reduction_budget_basic() {
        let mut budget = ReductionBudget::new(3);
        assert!(budget.consume());
        assert!(budget.consume());
        assert!(budget.consume());
        assert!(!budget.consume());
    }
    #[test]
    fn test_reduction_budget_unlimited() {
        let mut budget = ReductionBudget::unlimited();
        for _ in 0..1000 {
            assert!(budget.consume());
        }
    }
    #[test]
    fn test_whnf_stats_default() {
        let stats = WhnfStats::new();
        assert_eq!(stats.total_steps(), 0);
        assert!(!stats.any_progress());
    }
    #[test]
    fn test_whnf_stats_total() {
        let stats = WhnfStats {
            beta_steps: 3,
            delta_steps: 2,
            zeta_steps: 1,
            iota_steps: 4,
            total_exprs: 10,
        };
        assert_eq!(stats.total_steps(), 10);
        assert!(stats.any_progress());
    }
    #[test]
    fn test_whnf_cache_key() {
        let e1 = sort0();
        let e2 = sort0();
        let k1 = WhnfCacheKey::from_expr(&e1);
        let k2 = WhnfCacheKey::from_expr(&e2);
        assert_eq!(k1, k2);
    }
    #[test]
    fn test_stuck_reason_normal_form() {
        assert_eq!(stuck_reason(&sort0()), StuckReason::NormalForm);
    }
    #[test]
    fn test_stuck_reason_free_var() {
        let fv = Expr::FVar(FVarId::new(42));
        assert_eq!(
            stuck_reason(&fv),
            StuckReason::FreeVariable(FVarId::new(42))
        );
    }
    #[test]
    fn test_app_arity() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let b = Expr::Lit(Literal::Nat(2));
        let app = Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b));
        assert_eq!(app_arity(&app), 2);
    }
    #[test]
    fn test_whnf_as_pi() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(sort0()),
            Box::new(sort1()),
        );
        let result = whnf_as_pi(&pi);
        assert!(result.is_some());
        let (bi, name, dom, cod) = result.expect("result should be valid");
        assert_eq!(bi, BinderInfo::Default);
        assert_eq!(name, Name::str("x"));
        assert_eq!(dom, sort0());
        assert_eq!(cod, sort1());
    }
    #[test]
    fn test_whnf_as_sort() {
        assert_eq!(whnf_as_sort(&sort0()), Some(Level::zero()));
        assert_eq!(whnf_as_sort(&identity_lam()), None);
    }
    #[test]
    fn test_whnf_heads_match_sorts() {
        assert!(whnf_heads_match(&sort0(), &sort0()));
        assert!(!whnf_heads_match(&sort0(), &sort1()));
    }
    #[test]
    fn test_whnf_heads_match_const() {
        let c1 = Expr::Const(Name::str("Nat"), vec![]);
        let c2 = Expr::Const(Name::str("Nat"), vec![]);
        let c3 = Expr::Const(Name::str("Int"), vec![]);
        assert!(whnf_heads_match(&c1, &c2));
        assert!(!whnf_heads_match(&c1, &c3));
    }
    #[test]
    fn test_whnf_head_to_expr() {
        let head = WhnfHead::Sort(Level::zero());
        assert_eq!(head.to_expr(), sort0());
    }
    #[test]
    fn test_whnf_budgeted_success() {
        let mut budget = ReductionBudget::new(10);
        let result = whnf_budgeted(&sort0(), &mut budget);
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid"), sort0());
    }
    #[test]
    fn test_whnf_budgeted_exhausted() {
        let mut budget = ReductionBudget::new(0);
        let result = whnf_budgeted(&sort0(), &mut budget);
        assert!(result.is_none());
    }
}
/// Fully normalize an expression by repeatedly applying WHNF
/// and then recursively normalizing all subexpressions.
///
/// Warning: may not terminate for non-normalizing terms.
#[allow(dead_code)]
pub fn normalize_full(expr: &Expr) -> Expr {
    let w = whnf(expr);
    match w {
        Expr::App(f, a) => {
            let nf = normalize_full(&f);
            let na = normalize_full(&a);
            Expr::App(Box::new(nf), Box::new(na))
        }
        Expr::Lam(bi, name, ty, body) => {
            let nty = normalize_full(&ty);
            let nbody = normalize_full(&body);
            Expr::Lam(bi, name, Box::new(nty), Box::new(nbody))
        }
        Expr::Pi(bi, name, ty, body) => {
            let nty = normalize_full(&ty);
            let nbody = normalize_full(&body);
            Expr::Pi(bi, name, Box::new(nty), Box::new(nbody))
        }
        Expr::Let(name, ty, val, body) => {
            let nty = normalize_full(&ty);
            let nval = normalize_full(&val);
            let nbody = normalize_full(&body);
            Expr::Let(name, Box::new(nty), Box::new(nval), Box::new(nbody))
        }
        other => other,
    }
}
/// Normalize just the head of an expression, without recursing into subexpressions.
#[allow(dead_code)]
pub fn normalize_head(expr: &Expr) -> Expr {
    whnf(expr)
}
/// Check whether a WHNF expression's head is a given constant.
#[allow(dead_code)]
pub fn whnf_head_is_const(expr: &Expr, name: &Name) -> bool {
    match whnf(expr) {
        Expr::Const(n, _) => n == *name,
        Expr::App(f, _) => whnf_head_is_const(&f, name),
        _ => false,
    }
}
/// Check whether a WHNF expression is a free variable.
#[allow(dead_code)]
pub fn whnf_is_fvar(expr: &Expr) -> bool {
    matches!(whnf(expr), Expr::FVar(_))
}
/// Extract the FVarId from a WHNF expression if it is a free variable.
#[allow(dead_code)]
pub fn whnf_as_fvar(expr: &Expr) -> Option<FVarId> {
    match whnf(expr) {
        Expr::FVar(id) => Some(id),
        _ => None,
    }
}
#[cfg(test)]
mod extra_whnf_tests {
    use super::*;
    #[test]
    fn test_normalize_full_sort() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(normalize_full(&e), e);
    }
    #[test]
    fn test_normalize_full_beta() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let arg = Expr::Lit(Literal::Nat(7));
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let result = normalize_full(&app);
        assert_eq!(result, arg);
    }
    #[test]
    fn test_normalize_full_lam() {
        let e = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let result = normalize_full(&e);
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_normalize_head_sort() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(normalize_head(&e), e);
    }
    #[test]
    fn test_whnf_head_is_const_direct() {
        let c = Expr::Const(Name::str("Nat"), vec![]);
        assert!(whnf_head_is_const(&c, &Name::str("Nat")));
        assert!(!whnf_head_is_const(&c, &Name::str("Bool")));
    }
    #[test]
    fn test_whnf_is_fvar_true() {
        let fv = Expr::FVar(FVarId::new(0));
        assert!(whnf_is_fvar(&fv));
    }
    #[test]
    fn test_whnf_is_fvar_false() {
        let c = Expr::Const(Name::str("Nat"), vec![]);
        assert!(!whnf_is_fvar(&c));
    }
    #[test]
    fn test_whnf_as_fvar_some() {
        let id = FVarId::new(42);
        let fv = Expr::FVar(id);
        assert_eq!(whnf_as_fvar(&fv), Some(id));
    }
    #[test]
    fn test_whnf_as_fvar_none() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(whnf_as_fvar(&e), None);
    }
    #[test]
    fn test_whnf_depth_budget_consumed() {
        let mut b = WhnfDepthBudget::new(5);
        for _ in 0..5 {
            assert!(b.consume());
        }
        assert!(b.is_exhausted());
        assert!(!b.consume());
    }
    #[test]
    fn test_whnf_depth_budget_consumed_count() {
        let mut b = WhnfDepthBudget::new(10);
        b.consume();
        b.consume();
        b.consume();
        assert_eq!(b.consumed(), 3);
    }
    #[test]
    fn test_whnf_depth_budget_reset() {
        let mut b = WhnfDepthBudget::new(3);
        b.consume();
        b.consume();
        b.reset();
        assert_eq!(b.consumed(), 0);
    }
    #[test]
    fn test_reduction_order_display() {
        assert_eq!(format!("{}", WhnfReductionOrder::BetaFirst), "beta-first");
        assert_eq!(format!("{}", WhnfReductionOrder::DeltaFirst), "delta-first");
        assert_eq!(
            format!("{}", WhnfReductionOrder::StructuralOnly),
            "structural-only"
        );
    }
}
/// Count the total number of nodes (subexpressions) in an expression.
///
/// Useful for estimating the size of a proof term or type.
#[allow(dead_code)]
pub fn count_nodes(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) => 1,
        Expr::Const(_, levels) => 1 + levels.len(),
        Expr::App(f, a) => 1 + count_nodes(f) + count_nodes(a),
        Expr::Lam(_, _, ty, body) => 1 + count_nodes(ty) + count_nodes(body),
        Expr::Pi(_, _, ty, body) => 1 + count_nodes(ty) + count_nodes(body),
        Expr::Let(_, ty, val, body) => 1 + count_nodes(ty) + count_nodes(val) + count_nodes(body),
        _ => 1,
    }
}
/// Compute the depth (maximum nesting) of an expression tree.
#[allow(dead_code)]
pub fn expr_depth(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) | Expr::Const(_, _) => 1,
        Expr::App(f, a) => 1 + count_nodes(f).max(count_nodes(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        _ => 1,
    }
}
#[cfg(test)]
mod whnf_cache_tests {
    use super::*;
    #[test]
    fn test_whnf_cache_empty() {
        let cache = WhnfCache::new(10);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }
    #[test]
    fn test_whnf_cache_insert_get() {
        let mut cache = WhnfCache::new(10);
        let key = WhnfCacheKey { expr_hash: 42 };
        let expr = Expr::Sort(Level::zero());
        cache.insert(key.clone(), expr.clone());
        assert_eq!(cache.len(), 1);
        let result = cache.get(&key);
        assert!(result.is_some());
        assert_eq!(*result.expect("result should be valid"), expr);
    }
    #[test]
    fn test_whnf_cache_miss() {
        let mut cache = WhnfCache::new(10);
        let key = WhnfCacheKey { expr_hash: 99 };
        let result = cache.get(&key);
        assert!(result.is_none());
        assert_eq!(cache.misses(), 1);
    }
    #[test]
    fn test_whnf_cache_hit_rate() {
        let mut cache = WhnfCache::new(10);
        let key = WhnfCacheKey { expr_hash: 1 };
        cache.insert(key.clone(), Expr::Sort(Level::zero()));
        cache.get(&key);
        cache.get(&WhnfCacheKey { expr_hash: 999 });
        assert!((cache.hit_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_whnf_cache_capacity_eviction() {
        let mut cache = WhnfCache::new(2);
        let e = Expr::Sort(Level::zero());
        cache.insert(WhnfCacheKey { expr_hash: 1 }, e.clone());
        cache.insert(WhnfCacheKey { expr_hash: 2 }, e.clone());
        cache.insert(WhnfCacheKey { expr_hash: 3 }, e.clone());
        assert_eq!(cache.len(), 2);
    }
    #[test]
    fn test_whnf_cache_clear() {
        let mut cache = WhnfCache::new(10);
        cache.insert(WhnfCacheKey { expr_hash: 1 }, Expr::Sort(Level::zero()));
        cache.clear();
        assert!(cache.is_empty());
    }
    #[test]
    fn test_whnf_config_default() {
        let cfg = WhnfConfig::default();
        assert!(cfg.beta);
        assert!(cfg.delta);
        assert!(cfg.any_enabled());
    }
    #[test]
    fn test_whnf_config_structural() {
        let cfg = WhnfConfig::structural();
        assert!(!cfg.delta);
        assert!(cfg.beta);
    }
    #[test]
    fn test_whnf_config_no_delta() {
        let cfg = WhnfConfig::default().no_delta();
        assert!(!cfg.delta);
    }
    #[test]
    fn test_whnf_config_no_beta() {
        let cfg = WhnfConfig::default().no_beta();
        assert!(!cfg.beta);
    }
    #[test]
    fn test_count_nodes_sort() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(count_nodes(&e), 1);
    }
    #[test]
    fn test_count_nodes_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app = Expr::App(Box::new(f), Box::new(a));
        assert_eq!(count_nodes(&app), 3);
    }
    #[test]
    fn test_expr_depth_sort() {
        let e = Expr::Sort(Level::zero());
        assert_eq!(expr_depth(&e), 1);
    }
    #[test]
    fn test_expr_depth_nested() {
        let inner = Expr::Sort(Level::zero());
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(inner.clone()),
            Box::new(inner),
        );
        assert!(expr_depth(&lam) >= 2);
    }
    #[test]
    fn test_whnf_config_with_limit() {
        let cfg = WhnfConfig::with_limit(100);
        assert_eq!(cfg.max_steps, 100);
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
