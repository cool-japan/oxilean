//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Environment, Expr, Reducer};
use std::collections::HashMap;

use super::types::{
    ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet, LazyNormal,
    MemoizedNormalizer, MinHeap, NonEmptyVec, NormStats, NormStrategy, PathBuf, PrefixCounter,
    RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc,
    StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Normalize an expression to full normal form (no environment).
///
/// Reduces all beta-redexes, let-bindings, and under all binders.
pub fn normalize(expr: &Expr) -> Expr {
    let mut reducer = Reducer::new();
    normalize_impl(&mut reducer, expr, None)
}
/// Normalize an expression with an environment.
///
/// Also unfolds definitions according to the environment.
pub fn normalize_env(expr: &Expr, env: &Environment) -> Expr {
    let mut reducer = Reducer::new();
    normalize_impl(&mut reducer, expr, Some(env))
}
/// Internal normalization with optional environment.
pub(super) fn normalize_impl(
    reducer: &mut Reducer,
    expr: &Expr,
    env: Option<&Environment>,
) -> Expr {
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => expr.clone(),
        Expr::Const(_, _) => {
            let whnf = match env {
                Some(e) => reducer.whnf_env(expr, e),
                None => reducer.whnf(expr),
            };
            if whnf == *expr {
                expr.clone()
            } else {
                normalize_impl(reducer, &whnf, env)
            }
        }
        Expr::Lam(info, name, ty, body) => {
            let ty_norm = normalize_impl(reducer, ty, env);
            let body_norm = normalize_impl(reducer, body, env);
            Expr::Lam(*info, name.clone(), Box::new(ty_norm), Box::new(body_norm))
        }
        Expr::Pi(info, name, ty, body) => {
            let ty_norm = normalize_impl(reducer, ty, env);
            let body_norm = normalize_impl(reducer, body, env);
            Expr::Pi(*info, name.clone(), Box::new(ty_norm), Box::new(body_norm))
        }
        Expr::App(_, _) => {
            let whnf = match env {
                Some(e) => reducer.whnf_env(expr, e),
                None => reducer.whnf(expr),
            };
            if let Expr::App(f_whnf, a_whnf) = whnf {
                let f_norm = normalize_impl(reducer, &f_whnf, env);
                let a_norm = normalize_impl(reducer, &a_whnf, env);
                Expr::App(Box::new(f_norm), Box::new(a_norm))
            } else {
                normalize_impl(reducer, &whnf, env)
            }
        }
        Expr::Let(_, _, _, _) => {
            let whnf = match env {
                Some(e) => reducer.whnf_env(expr, e),
                None => reducer.whnf(expr),
            };
            normalize_impl(reducer, &whnf, env)
        }
        Expr::Proj(name, idx, e) => {
            let whnf = match env {
                Some(env) => reducer.whnf_env(expr, env),
                None => reducer.whnf(expr),
            };
            if let Expr::Proj(_, _, _) = &whnf {
                let e_norm = normalize_impl(reducer, e, env);
                Expr::Proj(name.clone(), *idx, Box::new(e_norm))
            } else {
                normalize_impl(reducer, &whnf, env)
            }
        }
    }
}
/// Check if two expressions are equal after full normalization.
pub fn alpha_eq(e1: &Expr, e2: &Expr) -> bool {
    let n1 = normalize(e1);
    let n2 = normalize(e2);
    n1 == n2
}
/// Check if two expressions are equal after full normalization with environment.
pub fn alpha_eq_env(e1: &Expr, e2: &Expr, env: &Environment) -> bool {
    let n1 = normalize_env(e1, env);
    let n2 = normalize_env(e2, env);
    n1 == n2
}
/// Compute weak head normal form and then fully normalize.
pub fn normalize_whnf(expr: &Expr) -> Expr {
    let mut reducer = Reducer::new();
    let whnf = reducer.whnf(expr);
    normalize_impl(&mut reducer, &whnf, None)
}
/// Check if an expression is already in normal form.
///
/// An expression is in normal form if normalizing it yields
/// the same expression.
pub fn is_normal_form(expr: &Expr) -> bool {
    let normalized = normalize(expr);
    normalized == *expr
}
/// Evaluate an expression with an environment to a value.
///
/// This is like normalize_env but stops at constructors and
/// lambda abstractions.
pub fn evaluate(expr: &Expr, env: &Environment) -> Expr {
    let mut reducer = Reducer::new();
    let whnf = reducer.whnf_env(expr, env);
    match &whnf {
        Expr::Lit(_) | Expr::Lam(_, _, _, _) | Expr::Sort(_) => whnf,
        Expr::App(_, _) => {
            let (head, args) = collect_app_parts(&whnf);
            if is_constructor_head(head, env) {
                let head_norm = head.clone();
                let mut result = head_norm;
                for arg in args {
                    let arg_eval = evaluate(arg, env);
                    result = Expr::App(Box::new(result), Box::new(arg_eval));
                }
                result
            } else {
                normalize_impl(&mut reducer, &whnf, Some(env))
            }
        }
        _ => normalize_impl(&mut reducer, &whnf, Some(env)),
    }
}
/// Collect head and args from nested application.
pub(super) fn collect_app_parts(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}
/// Check if an expression head is a constructor.
pub(super) fn is_constructor_head(head: &Expr, env: &Environment) -> bool {
    if let Expr::Const(name, _) = head {
        env.is_constructor(name)
    } else {
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal, Name};
    #[test]
    fn test_normalize_sort() {
        let expr = Expr::Sort(Level::zero());
        let norm = normalize(&expr);
        assert_eq!(norm, expr);
    }
    #[test]
    fn test_normalize_lit() {
        let expr = Expr::Lit(Literal::Nat(42));
        let norm = normalize(&expr);
        assert_eq!(norm, expr);
    }
    #[test]
    fn test_normalize_bvar() {
        let expr = Expr::BVar(0);
        let norm = normalize(&expr);
        assert_eq!(norm, expr);
    }
    #[test]
    fn test_normalize_lambda() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let norm = normalize(&lam);
        assert!(matches!(norm, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_normalize_beta() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let arg = Expr::Lit(Literal::Nat(42));
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let norm = normalize(&app);
        assert_eq!(norm, arg);
    }
    #[test]
    fn test_alpha_eq_same() {
        let e1 = Expr::Lit(Literal::Nat(42));
        let e2 = Expr::Lit(Literal::Nat(42));
        assert!(alpha_eq(&e1, &e2));
    }
    #[test]
    fn test_alpha_eq_different() {
        let e1 = Expr::Lit(Literal::Nat(42));
        let e2 = Expr::Lit(Literal::Nat(43));
        assert!(!alpha_eq(&e1, &e2));
    }
    #[test]
    fn test_normalize_nested() {
        let inner_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(1)),
        );
        let outer_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(inner_lam),
        );
        let arg = Expr::Lit(Literal::Nat(42));
        let app = Expr::App(Box::new(outer_lam), Box::new(arg));
        let norm = normalize(&app);
        assert!(matches!(norm, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_is_normal_form() {
        assert!(is_normal_form(&Expr::Lit(Literal::Nat(42))));
        assert!(is_normal_form(&Expr::BVar(0)));
        assert!(is_normal_form(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_normalize_with_env() {
        let mut env = Environment::new();
        env.add(crate::Declaration::Definition {
            name: Name::str("answer"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: Expr::Lit(Literal::Nat(42)),
            hint: crate::ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        let expr = Expr::Const(Name::str("answer"), vec![]);
        let norm = normalize_env(&expr, &env);
        assert_eq!(norm, Expr::Lit(Literal::Nat(42)));
    }
    #[test]
    fn test_alpha_eq_env() {
        let mut env = Environment::new();
        env.add(crate::Declaration::Definition {
            name: Name::str("a"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: Expr::Lit(Literal::Nat(42)),
            hint: crate::ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        env.add(crate::Declaration::Definition {
            name: Name::str("b"),
            univ_params: vec![],
            ty: Expr::Const(Name::str("Nat"), vec![]),
            val: Expr::Lit(Literal::Nat(42)),
            hint: crate::ReducibilityHint::Regular(1),
        })
        .expect("value should be present");
        let e1 = Expr::Const(Name::str("a"), vec![]);
        let e2 = Expr::Const(Name::str("b"), vec![]);
        assert!(alpha_eq_env(&e1, &e2, &env));
    }
}
/// Normalize only the type-level parts of an expression (sorts and Pi domains).
///
/// This is useful when you want to normalize types without touching proof terms.
#[allow(dead_code)]
pub fn normalize_types(expr: &Expr) -> Expr {
    normalize_types_impl(expr)
}
pub(super) fn normalize_types_impl(expr: &Expr) -> Expr {
    match expr {
        Expr::Pi(bk, n, ty, body) => {
            let ty2 = normalize_types_impl(ty);
            let body2 = normalize_types_impl(body);
            Expr::Pi(*bk, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Sort(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(normalize_types_impl(f)),
            Box::new(normalize_types_impl(a)),
        ),
        Expr::Lam(bk, n, ty, body) => {
            let ty2 = normalize_types_impl(ty);
            Expr::Lam(
                *bk,
                n.clone(),
                Box::new(ty2),
                Box::new(body.as_ref().clone()),
            )
        }
        other => other.clone(),
    }
}
/// Reduce an expression to head-normal form without environment (no delta-unfolding).
///
/// Head-normal form means the outermost expression is not a redex:
/// - Not a beta-redex (no `(lam ...) arg`)
/// - Not a let-binding at the top level
#[allow(dead_code)]
pub fn head_normal_form(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, arg) => {
            let f_hnf = head_normal_form(f);
            match f_hnf {
                Expr::Lam(_, _, _, body) => {
                    let substituted = subst_bvar_norm(*body, 0, arg);
                    head_normal_form(&substituted)
                }
                other_f => Expr::App(Box::new(other_f), arg.clone()),
            }
        }
        Expr::Let(_, _, val, body) => {
            let substituted = subst_bvar_norm(*body.clone(), 0, val);
            head_normal_form(&substituted)
        }
        other => other.clone(),
    }
}
pub(super) fn subst_bvar_norm(term: Expr, depth: u32, replacement: &Expr) -> Expr {
    match term {
        Expr::BVar(i) => {
            if i == depth {
                replacement.clone()
            } else if i > depth {
                Expr::BVar(i - 1)
            } else {
                Expr::BVar(i)
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(subst_bvar_norm(*f, depth, replacement)),
            Box::new(subst_bvar_norm(*a, depth, replacement)),
        ),
        Expr::Lam(bk, n, ty, body) => Expr::Lam(
            bk,
            n,
            Box::new(subst_bvar_norm(*ty, depth, replacement)),
            Box::new(subst_bvar_norm(*body, depth + 1, replacement)),
        ),
        Expr::Pi(bk, n, ty, body) => Expr::Pi(
            bk,
            n,
            Box::new(subst_bvar_norm(*ty, depth, replacement)),
            Box::new(subst_bvar_norm(*body, depth + 1, replacement)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n,
            Box::new(subst_bvar_norm(*ty, depth, replacement)),
            Box::new(subst_bvar_norm(*val, depth, replacement)),
            Box::new(subst_bvar_norm(*body, depth + 1, replacement)),
        ),
        Expr::Proj(idx, n, e) => {
            Expr::Proj(idx, n, Box::new(subst_bvar_norm(*e, depth, replacement)))
        }
        other => other,
    }
}
/// Count the number of reduction steps to reach normal form.
///
/// Returns the step count and the normalized expression.
#[allow(dead_code)]
pub fn count_reduction_steps(expr: &Expr) -> (usize, Expr) {
    let mut steps = 0usize;
    let result = count_steps_impl(expr, &mut steps);
    (steps, result)
}
pub(super) fn count_steps_impl(expr: &Expr, steps: &mut usize) -> Expr {
    match expr {
        Expr::App(f, arg) => {
            let f_norm = count_steps_impl(f, steps);
            let arg_norm = count_steps_impl(arg, steps);
            if let Expr::Lam(_, _, _, body) = f_norm {
                *steps += 1;
                let substituted = subst_bvar_norm(*body, 0, &arg_norm);
                count_steps_impl(&substituted, steps)
            } else {
                Expr::App(Box::new(f_norm), Box::new(arg_norm))
            }
        }
        Expr::Let(_, _, val, body) => {
            *steps += 1;
            let substituted = subst_bvar_norm(*body.clone(), 0, val);
            count_steps_impl(&substituted, steps)
        }
        Expr::Lam(bk, n, ty, body) => {
            let ty2 = count_steps_impl(ty, steps);
            let body2 = count_steps_impl(body, steps);
            Expr::Lam(*bk, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bk, n, ty, body) => {
            let ty2 = count_steps_impl(ty, steps);
            let body2 = count_steps_impl(body, steps);
            Expr::Pi(*bk, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Proj(idx, n, e) => Expr::Proj(idx.clone(), *n, Box::new(count_steps_impl(e, steps))),
        other => other.clone(),
    }
}
/// Normalize a slice of expressions in parallel (sequential implementation).
///
/// Returns a `Vec` of normalized expressions in the same order.
#[allow(dead_code)]
pub fn normalize_many(exprs: &[Expr]) -> Vec<Expr> {
    exprs.iter().map(normalize).collect()
}
/// Normalize a slice of expressions using an environment.
#[allow(dead_code)]
pub fn normalize_many_env(exprs: &[Expr], env: &Environment) -> Vec<Expr> {
    exprs.iter().map(|e| normalize_env(e, env)).collect()
}
/// Check if an expression is already in weak head normal form.
///
/// An expression is in WHNF if:
/// - It is a sort, literal, free variable, or constant
/// - It is a lambda (already a value)
/// - It is a Pi type
/// - It is an application whose head is not a lambda
#[allow(dead_code)]
pub fn is_in_whnf(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(_) | Expr::Lit(_) | Expr::FVar(_) | Expr::Const(_, _) => true,
        Expr::Lam(_, _, _, _) | Expr::Pi(_, _, _, _) => true,
        Expr::App(f, _) => !matches!(f.as_ref(), Expr::Lam(_, _, _, _)),
        Expr::BVar(_) => true,
        Expr::Let(_, _, _, _) => false,
        Expr::Proj(_, _, _) => false,
    }
}
/// Normalize an expression fully, but only up to a given depth limit.
///
/// Stops descending past `max_depth` levels of binders/applications.
/// Returns the partially normalized expression.
#[allow(dead_code)]
pub fn normalize_fully(expr: &Expr, max_depth: usize) -> Expr {
    normalize_fully_impl(expr, max_depth, 0)
}
pub(super) fn normalize_fully_impl(expr: &Expr, max_depth: usize, current: usize) -> Expr {
    if current >= max_depth {
        return expr.clone();
    }
    match expr {
        Expr::App(f, arg) => {
            let f2 = normalize_fully_impl(f, max_depth, current + 1);
            let a2 = normalize_fully_impl(arg, max_depth, current + 1);
            if let Expr::Lam(_, _, _, body) = f2 {
                let sub = subst_bvar_norm(*body, 0, &a2);
                normalize_fully_impl(&sub, max_depth, current + 1)
            } else {
                Expr::App(Box::new(f2), Box::new(a2))
            }
        }
        Expr::Lam(bk, n, ty, body) => {
            let ty2 = normalize_fully_impl(ty, max_depth, current + 1);
            let b2 = normalize_fully_impl(body, max_depth, current + 1);
            Expr::Lam(*bk, n.clone(), Box::new(ty2), Box::new(b2))
        }
        Expr::Pi(bk, n, ty, body) => {
            let ty2 = normalize_fully_impl(ty, max_depth, current + 1);
            let b2 = normalize_fully_impl(body, max_depth, current + 1);
            Expr::Pi(*bk, n.clone(), Box::new(ty2), Box::new(b2))
        }
        Expr::Let(_, _, val, body) => {
            let sub = subst_bvar_norm(*body.clone(), 0, val);
            normalize_fully_impl(&sub, max_depth, current + 1)
        }
        other => other.clone(),
    }
}
/// Apply exactly `n` reduction steps to the expression.
///
/// Returns the result after at most `n` steps. If the expression
/// is already in normal form before `n` steps, returns early.
#[allow(dead_code)]
pub fn reduce_n_steps(expr: &Expr, n: usize) -> Expr {
    let mut current = expr.clone();
    for _ in 0..n {
        let next = reduce_one_step(&current);
        if next == current {
            break;
        }
        current = next;
    }
    current
}
pub(super) fn reduce_one_step(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, arg) => {
            if let Expr::Lam(_, _, _, body) = f.as_ref() {
                subst_bvar_norm(*body.clone(), 0, arg)
            } else {
                let f2 = reduce_one_step(f);
                Expr::App(Box::new(f2), arg.clone())
            }
        }
        Expr::Let(_, _, val, body) => subst_bvar_norm(*body.clone(), 0, val),
        other => other.clone(),
    }
}
/// Normalize only constants in the expression that appear in the given whitelist.
///
/// Useful when you want selective unfolding of specific definitions.
#[allow(dead_code)]
pub fn normalize_selective(expr: &Expr, env: &Environment, whitelist: &[crate::Name]) -> Expr {
    match expr {
        Expr::Const(name, _) => {
            if whitelist.contains(name) {
                if let Some(ci) = env.find(name) {
                    if let Some(val) = ci.value() {
                        return normalize_selective(val, env, whitelist);
                    }
                }
            }
            expr.clone()
        }
        Expr::App(f, a) => {
            let f2 = normalize_selective(f, env, whitelist);
            let a2 = normalize_selective(a, env, whitelist);
            if let Expr::Lam(_, _, _, body) = f2 {
                let sub = subst_bvar_norm(*body, 0, &a2);
                normalize_selective(&sub, env, whitelist)
            } else {
                Expr::App(Box::new(f2), Box::new(a2))
            }
        }
        Expr::Lam(bk, n, ty, body) => Expr::Lam(
            *bk,
            n.clone(),
            Box::new(normalize_selective(ty, env, whitelist)),
            Box::new(normalize_selective(body, env, whitelist)),
        ),
        Expr::Pi(bk, n, ty, body) => Expr::Pi(
            *bk,
            n.clone(),
            Box::new(normalize_selective(ty, env, whitelist)),
            Box::new(normalize_selective(body, env, whitelist)),
        ),
        Expr::Let(_n, _ty, val, body) => {
            let sub = subst_bvar_norm(*body.clone(), 0, val);
            normalize_selective(&sub, env, whitelist)
        }
        Expr::Proj(idx, n, e) => Expr::Proj(
            idx.clone(),
            *n,
            Box::new(normalize_selective(e, env, whitelist)),
        ),
        other => other.clone(),
    }
}
#[cfg(test)]
mod extended_normalize_tests {
    use super::*;
    use crate::{BinderInfo as BinderKind, Expr, Level, Literal, Name};
    fn mk_nat(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    fn mk_sort(n: u32) -> Expr {
        let mut l = Level::zero();
        for _ in 0..n {
            l = Level::succ(l);
        }
        Expr::Sort(l)
    }
    #[test]
    fn test_normalize_types_pi() {
        let pi = Expr::Pi(
            BinderKind::Default,
            Name::str("x"),
            Box::new(mk_sort(0)),
            Box::new(mk_sort(0)),
        );
        let result = normalize_types(&pi);
        assert!(matches!(result, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_head_normal_form_const() {
        let c = mk_const("Nat");
        let hnf = head_normal_form(&c);
        assert_eq!(hnf, c);
    }
    #[test]
    fn test_head_normal_form_app_lam() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderKind::Default,
            Name::str("x"),
            Box::new(mk_sort(0)),
            Box::new(body),
        );
        let arg = mk_nat(5);
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let hnf = head_normal_form(&app);
        assert_eq!(hnf, arg);
    }
    #[test]
    fn test_count_reduction_steps_zero() {
        let c = mk_const("pure");
        let (steps, result) = count_reduction_steps(&c);
        assert_eq!(steps, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn test_count_reduction_steps_one() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderKind::Default,
            Name::str("x"),
            Box::new(mk_sort(0)),
            Box::new(body),
        );
        let arg = mk_nat(7);
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let (steps, result) = count_reduction_steps(&app);
        assert!(steps >= 1);
        assert_eq!(result, arg);
    }
    #[test]
    fn test_memoized_normalizer_cache() {
        let mut memo = MemoizedNormalizer::new();
        let c = mk_const("f");
        assert_eq!(memo.cache_size(), 0);
        memo.normalize(&c);
        assert_eq!(memo.cache_size(), 1);
        memo.normalize(&c);
        assert_eq!(memo.cache_size(), 1);
        memo.clear_cache();
        assert_eq!(memo.cache_size(), 0);
    }
    #[test]
    fn test_normalize_many() {
        let exprs = vec![mk_nat(1), mk_const("True"), mk_nat(2)];
        let results = normalize_many(&exprs);
        assert_eq!(results.len(), 3);
    }
    #[test]
    fn test_is_in_whnf_const() {
        assert!(is_in_whnf(&mk_const("Nat")));
    }
    #[test]
    fn test_is_in_whnf_let() {
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(mk_sort(0)),
            Box::new(mk_nat(0)),
            Box::new(Expr::BVar(0)),
        );
        assert!(!is_in_whnf(&let_expr));
    }
    #[test]
    fn test_normalize_fully_depth_zero() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderKind::Default,
            Name::str("x"),
            Box::new(mk_sort(0)),
            Box::new(body.clone()),
        );
        let arg = mk_nat(3);
        let app = Expr::App(Box::new(lam), Box::new(arg));
        let result = normalize_fully(&app, 0);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_reduce_n_steps_zero() {
        let c = mk_const("x");
        let result = reduce_n_steps(&c, 0);
        assert_eq!(result, c);
    }
    #[test]
    fn test_reduce_n_steps_one() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderKind::Default,
            Name::str("x"),
            Box::new(mk_sort(0)),
            Box::new(body),
        );
        let arg = mk_nat(99);
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let result = reduce_n_steps(&app, 1);
        assert_eq!(result, arg);
    }
    #[test]
    fn test_normalize_selective_no_env() {
        let env = Environment::new();
        let c = mk_const("myDef");
        let whitelist = vec![Name::str("myDef")];
        let result = normalize_selective(&c, &env, &whitelist);
        assert_eq!(result, c);
    }
}
/// Apply a normalization strategy to an expression.
pub fn normalize_with_strategy(expr: &Expr, strategy: NormStrategy) -> Expr {
    match strategy {
        NormStrategy::None => expr.clone(),
        NormStrategy::Whnf => normalize_whnf(expr),
        NormStrategy::Full => normalize(expr),
        NormStrategy::Bounded(depth) => normalize_fully(expr, depth),
    }
}
/// Normalize only up to `n` binders deep.
///
/// This is useful for inspecting the head of a term without fully normalizing
/// the bodies of lambdas/pis.
#[allow(dead_code)]
pub fn normalize_binders(expr: &Expr, n: usize) -> Expr {
    if n == 0 {
        return expr.clone();
    }
    match expr {
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(normalize(ty)),
            Box::new(normalize_binders(body, n - 1)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(normalize(ty)),
            Box::new(normalize_binders(body, n - 1)),
        ),
        other => normalize(other),
    }
}
/// Normalize while collecting statistics.
#[allow(dead_code)]
pub fn normalize_with_stats(expr: &Expr) -> (Expr, NormStats) {
    let mut stats = NormStats::new();
    let result = norm_stats_impl(expr, &mut stats);
    (result, stats)
}
pub(super) fn norm_stats_impl(expr: &Expr, stats: &mut NormStats) -> Expr {
    stats.visited += 1;
    match expr {
        Expr::App(f, arg) => {
            let f2 = norm_stats_impl(f, stats);
            let a2 = norm_stats_impl(arg, stats);
            if let Expr::Lam(_, _, _, body) = &f2 {
                stats.beta_steps += 1;
                let sub = subst_bvar_norm(*body.clone(), 0, &a2);
                norm_stats_impl(&sub, stats)
            } else {
                Expr::App(Box::new(f2), Box::new(a2))
            }
        }
        Expr::Let(_, _, val, body) => {
            stats.let_steps += 1;
            let v2 = norm_stats_impl(val, stats);
            let sub = subst_bvar_norm(*body.clone(), 0, &v2);
            norm_stats_impl(&sub, stats)
        }
        Expr::Lam(bk, n, ty, body) => {
            let ty2 = norm_stats_impl(ty, stats);
            let b2 = norm_stats_impl(body, stats);
            Expr::Lam(*bk, n.clone(), Box::new(ty2), Box::new(b2))
        }
        Expr::Pi(bk, n, ty, body) => {
            let ty2 = norm_stats_impl(ty, stats);
            let b2 = norm_stats_impl(body, stats);
            Expr::Pi(*bk, n.clone(), Box::new(ty2), Box::new(b2))
        }
        other => other.clone(),
    }
}
#[cfg(test)]
mod normalize_new_tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal, Name};
    fn mk_nat(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn mk_sort0() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_norm_strategy_none() {
        let e = Expr::App(
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(mk_sort0()),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(mk_nat(5)),
        );
        let result = normalize_with_strategy(&e, NormStrategy::None);
        assert_eq!(result, e);
    }
    #[test]
    fn test_norm_strategy_full() {
        let e = Expr::App(
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(mk_sort0()),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(mk_nat(7)),
        );
        let result = normalize_with_strategy(&e, NormStrategy::Full);
        assert_eq!(result, mk_nat(7));
    }
    #[test]
    fn test_norm_strategy_bounded() {
        let e = mk_nat(3);
        let result = normalize_with_strategy(&e, NormStrategy::Bounded(10));
        assert_eq!(result, e);
    }
    #[test]
    fn test_lazy_normal_basic() {
        let e = mk_nat(42);
        let lazy = LazyNormal::new(e.clone());
        assert!(!lazy.is_evaluated());
        assert_eq!(lazy.normalized(), &e);
        assert!(lazy.is_evaluated());
    }
    #[test]
    fn test_lazy_normal_normalizes() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(mk_sort0()),
            Box::new(body),
        );
        let arg = mk_nat(10);
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let lazy = LazyNormal::new(app);
        assert_eq!(lazy.normalized(), &arg);
    }
    #[test]
    fn test_normalize_with_stats_no_reductions() {
        let e = mk_nat(1);
        let (result, stats) = normalize_with_stats(&e);
        assert_eq!(result, e);
        assert_eq!(stats.beta_steps, 0);
        assert_eq!(stats.let_steps, 0);
        assert_eq!(stats.visited, 1);
    }
    #[test]
    fn test_normalize_with_stats_beta() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(mk_sort0()),
            Box::new(body),
        );
        let arg = mk_nat(3);
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let (result, stats) = normalize_with_stats(&app);
        assert_eq!(result, arg);
        assert!(stats.beta_steps >= 1);
    }
    #[test]
    fn test_normalize_with_stats_let() {
        let val = mk_nat(5);
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(mk_sort0()),
            Box::new(val.clone()),
            Box::new(Expr::BVar(0)),
        );
        let (result, stats) = normalize_with_stats(&let_expr);
        assert_eq!(result, val);
        assert!(stats.let_steps >= 1);
    }
    #[test]
    fn test_norm_stats_total_steps() {
        let stats = NormStats {
            beta_steps: 3,
            let_steps: 2,
            ..Default::default()
        };
        assert_eq!(stats.total_steps(), 5);
    }
    #[test]
    fn test_normalize_binders_depth1() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat.clone()),
            Box::new(nat.clone()),
        );
        let result = normalize_binders(&lam, 1);
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_norm_strategy_whnf() {
        let e = mk_nat(1);
        let result = normalize_with_strategy(&e, NormStrategy::Whnf);
        assert_eq!(result, e);
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
