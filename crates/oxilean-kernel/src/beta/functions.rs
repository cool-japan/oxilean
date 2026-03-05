//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{instantiate, BinderInfo, Expr, Name};

use super::types::{
    BetaStats, ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack, LabelSet,
    NonEmptyVec, PathBuf, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec,
    StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Perform one step of beta reduction.
///
/// If the expression is an application of a lambda to an argument,
/// substitute the argument for the bound variable in the lambda body.
pub fn beta_step(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_bi, _n, _ty, body) = f.as_ref() {
                Some(instantiate(body, a))
            } else {
                None
            }
        }
        _ => None,
    }
}
/// Perform beta reduction to normal form.
///
/// Repeatedly apply beta reduction until no more reductions are possible.
pub fn beta_normalize(expr: &Expr) -> Expr {
    beta_normalize_impl(expr, 1000)
}
fn beta_normalize_impl(expr: &Expr, fuel: u32) -> Expr {
    if fuel == 0 {
        return expr.clone();
    }
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_bi, _n, _ty, body) = f.as_ref() {
                let reduced = instantiate(body, a);
                beta_normalize_impl(&reduced, fuel - 1)
            } else {
                let f_norm = beta_normalize_impl(f, fuel - 1);
                let a_norm = beta_normalize_impl(a, fuel - 1);
                if let Expr::Lam(_bi, _n, _ty, body) = &f_norm {
                    let reduced = instantiate(body, &a_norm);
                    beta_normalize_impl(&reduced, fuel - 1)
                } else {
                    Expr::App(Box::new(f_norm), Box::new(a_norm))
                }
            }
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty_norm = beta_normalize_impl(ty, fuel - 1);
            let body_norm = beta_normalize_impl(body, fuel - 1);
            Expr::Lam(*bi, n.clone(), Box::new(ty_norm), Box::new(body_norm))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty_norm = beta_normalize_impl(ty, fuel - 1);
            let body_norm = beta_normalize_impl(body, fuel - 1);
            Expr::Pi(*bi, n.clone(), Box::new(ty_norm), Box::new(body_norm))
        }
        Expr::Let(n, ty, val, body) => {
            let ty_norm = beta_normalize_impl(ty, fuel - 1);
            let val_norm = beta_normalize_impl(val, fuel - 1);
            let body_norm = beta_normalize_impl(body, fuel - 1);
            Expr::Let(
                n.clone(),
                Box::new(ty_norm),
                Box::new(val_norm),
                Box::new(body_norm),
            )
        }
        Expr::Proj(n, i, s) => {
            let s_norm = beta_normalize_impl(s, fuel - 1);
            Expr::Proj(n.clone(), *i, Box::new(s_norm))
        }
        e => e.clone(),
    }
}
/// Check if an expression is in beta normal form.
pub fn is_beta_normal(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, _) => {
            if matches!(f.as_ref(), Expr::Lam(..)) {
                return false;
            }
            is_beta_normal(f) && is_beta_normal(f)
        }
        Expr::Lam(_, _, ty, body) => is_beta_normal(ty) && is_beta_normal(body),
        Expr::Pi(_, _, ty, body) => is_beta_normal(ty) && is_beta_normal(body),
        Expr::Let(_, ty, val, body) => {
            is_beta_normal(ty) && is_beta_normal(val) && is_beta_normal(body)
        }
        Expr::Proj(_, _, s) => is_beta_normal(s),
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(..) | Expr::Lit(_) => true,
    }
}
/// Beta reduce under a specific variable binding.
pub fn beta_under_binder(body: &Expr, arg: &Expr) -> Expr {
    instantiate(body, arg)
}
/// Create a beta redex (lambda application that can be reduced).
pub fn mk_beta_redex(ty: Expr, body: Expr, arg: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        )),
        Box::new(arg),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Literal};
    #[test]
    fn test_beta_step_simple() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let arg = Expr::Lit(Literal::Nat(42));
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat),
            Box::new(body),
        );
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let result = beta_step(&app);
        assert!(result.is_some());
        assert_eq!(result.expect("result should be valid"), arg);
    }
    #[test]
    fn test_beta_step_no_redex() {
        let e = Expr::BVar(0);
        assert!(beta_step(&e).is_none());
    }
    #[test]
    fn test_beta_normalize() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let arg = Expr::Lit(Literal::Nat(42));
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat),
            Box::new(body),
        );
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let result = beta_normalize(&app);
        assert_eq!(result, arg);
    }
    #[test]
    fn test_is_beta_normal() {
        let e = Expr::Lit(Literal::Nat(42));
        assert!(is_beta_normal(&e));
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat.clone()),
            Box::new(Expr::BVar(0)),
        );
        assert!(is_beta_normal(&lam));
        let app = Expr::App(Box::new(lam), Box::new(Expr::Lit(Literal::Nat(42))));
        assert!(!is_beta_normal(&app));
    }
    #[test]
    fn test_mk_beta_redex() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let arg = Expr::Lit(Literal::Nat(42));
        let redex = mk_beta_redex(nat, body, arg.clone());
        let result = beta_normalize(&redex);
        assert_eq!(result, arg);
    }
}
/// Perform one step of beta reduction at the outermost position.
///
/// Returns (reduced_expr, true) if a reduction was performed,
/// or (expr.clone(), false) if the expression is already in WHNF.
pub fn beta_step_with_flag(expr: &Expr) -> (Expr, bool) {
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_bi, _n, _ty, body) = f.as_ref() {
                (instantiate(body, a), true)
            } else {
                (expr.clone(), false)
            }
        }
        _ => (expr.clone(), false),
    }
}
/// Reduce an application spine.
///
/// Given head and a list of arguments, applies them sequentially,
/// reducing whenever head is a lambda.
pub fn reduce_app_spine(head: &Expr, args: &[Expr]) -> Expr {
    let mut result = head.clone();
    for arg in args {
        result = match result {
            Expr::Lam(_bi, _n, _ty, body) => instantiate(&body, arg),
            other => Expr::App(Box::new(other), Box::new(arg.clone())),
        };
    }
    result
}
/// Collect the application spine of an expression.
///
/// Returns (head, args) such that e = head args\[0\] ... args\[n-1\].
pub fn collect_app_spine(e: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut current = e;
    while let Expr::App(f, a) = current {
        args.push(a.as_ref());
        current = f;
    }
    args.reverse();
    (current, args)
}
/// Eta-expand an expression of a known function type.
///
/// Given f : A -> B, produces lambda x : A. f x.
pub fn eta_expand(f: Expr, domain_ty: Expr) -> Expr {
    let var = Expr::BVar(0);
    let body = Expr::App(Box::new(f), Box::new(var));
    Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(domain_ty),
        Box::new(body),
    )
}
/// Check if an expression is eta-reducible.
///
/// An expression lambda x. f x is eta-reducible to f if x does not appear in f.
pub fn is_eta_reducible(expr: &Expr) -> bool {
    if let Expr::Lam(_, _, _, body) = expr {
        if let Expr::App(f, arg) = body.as_ref() {
            if let Expr::BVar(0) = arg.as_ref() {
                return !has_loose_bvar_aux(f, 0);
            }
        }
    }
    false
}
fn has_loose_bvar_aux(expr: &Expr, idx: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i == idx,
        Expr::App(f, a) => has_loose_bvar_aux(f, idx) || has_loose_bvar_aux(a, idx),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_loose_bvar_aux(ty, idx) || has_loose_bvar_aux(body, idx + 1)
        }
        Expr::Let(_, ty, val, body) => {
            has_loose_bvar_aux(ty, idx)
                || has_loose_bvar_aux(val, idx)
                || has_loose_bvar_aux(body, idx + 1)
        }
        Expr::Proj(_, _, s) => has_loose_bvar_aux(s, idx),
        _ => false,
    }
}
/// Count the number of beta reduction steps to normalize an expression.
pub fn count_reduction_steps(expr: &Expr) -> usize {
    count_steps_impl(expr, 100)
}
fn count_steps_impl(expr: &Expr, fuel: usize) -> usize {
    if fuel == 0 {
        return 0;
    }
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_bi, _n, _ty, body) = f.as_ref() {
                let reduced = instantiate(body, a);
                1 + count_steps_impl(&reduced, fuel - 1)
            } else {
                count_steps_impl(f, fuel - 1) + count_steps_impl(a, fuel - 1)
            }
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_steps_impl(ty, fuel - 1) + count_steps_impl(body, fuel - 1)
        }
        Expr::Let(_, ty, val, body) => {
            count_steps_impl(ty, fuel - 1)
                + count_steps_impl(val, fuel - 1)
                + count_steps_impl(body, fuel - 1)
        }
        Expr::Proj(_, _, s) => count_steps_impl(s, fuel - 1),
        _ => 0,
    }
}
/// Create a K combinator: lambda x. lambda y. x.
pub fn mk_k_combinator(ty_x: Expr, ty_y: Expr) -> Expr {
    let inner = Expr::Lam(
        BinderInfo::Default,
        Name::str("y"),
        Box::new(ty_y),
        Box::new(Expr::BVar(1)),
    );
    Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(ty_x),
        Box::new(inner),
    )
}
/// Apply the K combinator: K x y = x.
pub fn apply_k(x: Expr, y: Expr, ty_x: Expr, ty_y: Expr) -> Expr {
    let k = mk_k_combinator(ty_x, ty_y);
    let kx = Expr::App(Box::new(k), Box::new(x));
    Expr::App(Box::new(kx), Box::new(y))
}
/// Create an I combinator: lambda x. x.
pub fn mk_i_combinator(ty: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(ty),
        Box::new(Expr::BVar(0)),
    )
}
/// Beta reduce with statistics collection.
pub fn beta_normalize_with_stats(expr: &Expr, stats: &mut BetaStats) -> Expr {
    beta_stats_impl(expr, 1000, stats, 0)
}
fn beta_stats_impl(expr: &Expr, fuel: u32, stats: &mut BetaStats, depth: u32) -> Expr {
    stats.update_depth(depth);
    if fuel == 0 {
        stats.record_fuel_exhaustion();
        return expr.clone();
    }
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_bi, _n, _ty, body) = f.as_ref() {
                let reduced = instantiate(body, a);
                stats.record_reduction();
                beta_stats_impl(&reduced, fuel - 1, stats, depth)
            } else {
                let f_norm = beta_stats_impl(f, fuel - 1, stats, depth + 1);
                let a_norm = beta_stats_impl(a, fuel - 1, stats, depth + 1);
                if let Expr::Lam(_bi, _n, _ty, body) = &f_norm {
                    let reduced = instantiate(body, &a_norm);
                    stats.record_reduction();
                    beta_stats_impl(&reduced, fuel - 1, stats, depth)
                } else {
                    Expr::App(Box::new(f_norm), Box::new(a_norm))
                }
            }
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty_n = beta_stats_impl(ty, fuel - 1, stats, depth + 1);
            let body_n = beta_stats_impl(body, fuel - 1, stats, depth + 1);
            Expr::Lam(*bi, n.clone(), Box::new(ty_n), Box::new(body_n))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty_n = beta_stats_impl(ty, fuel - 1, stats, depth + 1);
            let body_n = beta_stats_impl(body, fuel - 1, stats, depth + 1);
            Expr::Pi(*bi, n.clone(), Box::new(ty_n), Box::new(body_n))
        }
        Expr::Let(n, ty, val, body) => {
            let ty_n = beta_stats_impl(ty, fuel - 1, stats, depth + 1);
            let val_n = beta_stats_impl(val, fuel - 1, stats, depth + 1);
            let body_n = beta_stats_impl(body, fuel - 1, stats, depth + 1);
            Expr::Let(n.clone(), Box::new(ty_n), Box::new(val_n), Box::new(body_n))
        }
        Expr::Proj(n, i, s) => {
            let s_n = beta_stats_impl(s, fuel - 1, stats, depth + 1);
            Expr::Proj(n.clone(), *i, Box::new(s_n))
        }
        e => e.clone(),
    }
}
/// Check if an expression is in weak head normal form.
///
/// An expression is in WHNF if its outermost form is not a beta redex.
pub fn is_whnf(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, _) => !matches!(f.as_ref(), Expr::Lam(_, _, _, _)),
        Expr::Let(_, _, _, _) => false,
        _ => true,
    }
}
/// Reduce an expression to weak head normal form.
///
/// Only reduces the outermost redex; does not reduce under binders.
pub fn beta_whnf(expr: &Expr) -> Expr {
    beta_whnf_impl(expr, 1000)
}
fn beta_whnf_impl(expr: &Expr, fuel: u32) -> Expr {
    if fuel == 0 {
        return expr.clone();
    }
    match expr {
        Expr::App(f, a) => {
            let f_whnf = beta_whnf_impl(f, fuel - 1);
            if let Expr::Lam(_bi, _n, _ty, body) = &f_whnf {
                let reduced = instantiate(body, a);
                beta_whnf_impl(&reduced, fuel - 1)
            } else {
                Expr::App(Box::new(f_whnf), a.clone())
            }
        }
        Expr::Let(_n, _ty, val, body) => {
            let reduced = instantiate(body, val);
            beta_whnf_impl(&reduced, fuel - 1)
        }
        e => e.clone(),
    }
}
/// Perform fueled beta normalization.
///
/// Returns `(normalized_expr, remaining_fuel)`.
pub fn beta_normalize_fueled(expr: &Expr, fuel: u32) -> (Expr, u32) {
    beta_fueled_impl(expr, fuel)
}
fn beta_fueled_impl(expr: &Expr, fuel: u32) -> (Expr, u32) {
    if fuel == 0 {
        return (expr.clone(), 0);
    }
    match expr {
        Expr::App(f, a) => {
            let (f_norm, f_fuel) = beta_fueled_impl(f, fuel - 1);
            if let Expr::Lam(_bi, _n, _ty, body) = &f_norm {
                let reduced = instantiate(body, a);
                beta_fueled_impl(&reduced, f_fuel)
            } else {
                let (a_norm, a_fuel) = beta_fueled_impl(a, f_fuel);
                if let Expr::Lam(_bi, _n, _ty, body) = &f_norm {
                    let reduced = instantiate(body, &a_norm);
                    beta_fueled_impl(&reduced, a_fuel)
                } else {
                    (Expr::App(Box::new(f_norm), Box::new(a_norm)), a_fuel)
                }
            }
        }
        Expr::Lam(bi, n, ty, body) => {
            let (ty_n, t_fuel) = beta_fueled_impl(ty, fuel - 1);
            let (body_n, b_fuel) = beta_fueled_impl(body, t_fuel);
            (
                Expr::Lam(*bi, n.clone(), Box::new(ty_n), Box::new(body_n)),
                b_fuel,
            )
        }
        Expr::Pi(bi, n, ty, body) => {
            let (ty_n, t_fuel) = beta_fueled_impl(ty, fuel - 1);
            let (body_n, b_fuel) = beta_fueled_impl(body, t_fuel);
            (
                Expr::Pi(*bi, n.clone(), Box::new(ty_n), Box::new(body_n)),
                b_fuel,
            )
        }
        Expr::Let(_n, _ty, val, body) => {
            let reduced = instantiate(body, val);
            beta_fueled_impl(&reduced, fuel - 1)
        }
        Expr::Proj(n, i, s) => {
            let (s_n, s_fuel) = beta_fueled_impl(s, fuel - 1);
            (Expr::Proj(n.clone(), *i, Box::new(s_n)), s_fuel)
        }
        e => (e.clone(), fuel),
    }
}
/// Eta-reduce `lambda x. f x` to `f` when `x` is not free in `f`.
pub fn eta_reduce(expr: &Expr) -> Option<Expr> {
    if let Expr::Lam(_, _, _, body) = expr {
        if let Expr::App(f, arg) = body.as_ref() {
            if let Expr::BVar(0) = arg.as_ref() {
                if !has_loose_bvar_aux(f, 0) {
                    return Some(*f.clone());
                }
            }
        }
    }
    None
}
/// Head beta normalization: reduce only the head of an application spine.
pub fn beta_head_normalize(expr: &Expr) -> Expr {
    beta_head_impl(expr, 100)
}
fn beta_head_impl(expr: &Expr, fuel: u32) -> Expr {
    if fuel == 0 {
        return expr.clone();
    }
    match expr {
        Expr::App(f, a) => {
            let f_head = beta_head_impl(f, fuel - 1);
            if let Expr::Lam(_bi, _n, _ty, body) = &f_head {
                let reduced = instantiate(body, a);
                beta_head_impl(&reduced, fuel - 1)
            } else {
                Expr::App(Box::new(f_head), a.clone())
            }
        }
        e => e.clone(),
    }
}
/// Count the number of redexes at the outermost position.
pub fn count_redexes(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, _) => {
            if matches!(f.as_ref(), Expr::Lam(_, _, _, _)) {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}
/// Estimate how many steps are needed to normalize an expression.
pub fn estimate_reduction_depth(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => {
            let base = if matches!(f.as_ref(), Expr::Lam(_, _, _, _)) {
                1
            } else {
                0
            };
            base + estimate_reduction_depth(f) + estimate_reduction_depth(a)
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            estimate_reduction_depth(ty) + estimate_reduction_depth(body)
        }
        Expr::Let(_, ty, val, body) => {
            1 + estimate_reduction_depth(ty)
                + estimate_reduction_depth(val)
                + estimate_reduction_depth(body)
        }
        Expr::Proj(_, _, s) => estimate_reduction_depth(s),
        _ => 0,
    }
}
/// Create an identity function for a given type: `lambda x : ty. x`.
pub fn mk_identity(ty: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(ty),
        Box::new(Expr::BVar(0)),
    )
}
/// Create a constant function: `lambda _ : dom. val`.
pub fn mk_const_fn(dom: Expr, val: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(dom),
        Box::new(val),
    )
}
/// Compose two functions: `lambda x : dom. f (g x)`.
pub fn mk_compose(f: Expr, g: Expr, dom: Expr) -> Expr {
    let var = Expr::BVar(0);
    let g_app = Expr::App(Box::new(g), Box::new(var));
    let f_app = Expr::App(Box::new(f), Box::new(g_app));
    Expr::Lam(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(dom),
        Box::new(f_app),
    )
}
/// Create a multi-argument beta redex.
pub fn mk_multi_beta_redex(tys: &[Expr], body: Expr, args: &[Expr]) -> Expr {
    assert_eq!(tys.len(), args.len());
    let mut lam = body;
    for (i, ty) in tys.iter().enumerate().rev() {
        lam = Expr::Lam(
            BinderInfo::Default,
            Name::str(format!("x{}", i)),
            Box::new(ty.clone()),
            Box::new(lam),
        );
    }
    let mut result = lam;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg.clone()));
    }
    result
}
#[cfg(test)]
mod extra_beta_tests {
    use super::*;
    use crate::Literal;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    #[test]
    fn test_beta_step_with_flag_redex() {
        let id = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(42)));
        let (result, reduced) = beta_step_with_flag(&app);
        assert!(reduced);
        assert_eq!(result, lit(42));
    }
    #[test]
    fn test_beta_step_with_flag_no_redex() {
        let e = Expr::Const(Name::str("f"), vec![]);
        let (_, reduced) = beta_step_with_flag(&e);
        assert!(!reduced);
    }
    #[test]
    fn test_collect_app_spine() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = lit(1);
        let b = lit(2);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a))),
            Box::new(b),
        );
        let (head, args) = collect_app_spine(&app);
        assert_eq!(head, &f);
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_reduce_app_spine() {
        let id = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let result = reduce_app_spine(&id, &[lit(99)]);
        assert_eq!(result, lit(99));
    }
    #[test]
    fn test_is_eta_reducible_true() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let body = Expr::App(Box::new(f), Box::new(Expr::BVar(0)));
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(body),
        );
        assert!(is_eta_reducible(&lam));
    }
    #[test]
    fn test_is_eta_reducible_false() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        assert!(!is_eta_reducible(&lam));
    }
    #[test]
    fn test_eta_expand_then_apply() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let expanded = eta_expand(f.clone(), nat());
        let result = beta_normalize(&Expr::App(Box::new(expanded), Box::new(lit(5))));
        let expected = Expr::App(Box::new(f), Box::new(lit(5)));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_mk_k_combinator() {
        let k = mk_k_combinator(nat(), nat());
        let kxy = Expr::App(
            Box::new(Expr::App(Box::new(k), Box::new(lit(10)))),
            Box::new(lit(20)),
        );
        assert_eq!(beta_normalize(&kxy), lit(10));
    }
    #[test]
    fn test_apply_k() {
        let kxy = apply_k(lit(10), lit(20), nat(), nat());
        assert_eq!(beta_normalize(&kxy), lit(10));
    }
    #[test]
    fn test_mk_i_combinator() {
        let i = mk_i_combinator(nat());
        let result = beta_normalize(&Expr::App(Box::new(i), Box::new(lit(7))));
        assert_eq!(result, lit(7));
    }
    #[test]
    fn test_count_reduction_steps() {
        let id = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(1)));
        let steps = count_reduction_steps(&app);
        assert!(steps >= 1);
    }
    #[test]
    fn test_beta_stats_tracking() {
        let mut stats = BetaStats::new();
        let id = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(3)));
        let result = beta_normalize_with_stats(&app, &mut stats);
        assert_eq!(result, lit(3));
        assert!(stats.total_reductions >= 1);
    }
    #[test]
    fn test_beta_stats_max_depth() {
        let mut stats = BetaStats::new();
        let e = Expr::Sort(crate::Level::zero());
        beta_normalize_with_stats(&e, &mut stats);
        assert_eq!(stats.max_depth, 0);
    }
}
/// Normal-order beta reduction.
///
/// Reduces the leftmost, outermost redex first.
/// This strategy is guaranteed to terminate if any strategy terminates.
pub fn beta_normal_order(expr: &Expr) -> Expr {
    beta_normal_order_impl(expr, 200)
}
fn beta_normal_order_impl(expr: &Expr, fuel: u32) -> Expr {
    if fuel == 0 {
        return expr.clone();
    }
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_bi, _n, _ty, body) = f.as_ref() {
                let reduced = instantiate(body, a);
                beta_normal_order_impl(&reduced, fuel - 1)
            } else {
                let f_norm = beta_normal_order_impl(f, fuel - 1);
                if let Expr::Lam(_bi, _n, _ty, body) = &f_norm {
                    let reduced = instantiate(body, a);
                    beta_normal_order_impl(&reduced, fuel - 1)
                } else {
                    let a_norm = beta_normal_order_impl(a, fuel - 1);
                    Expr::App(Box::new(f_norm), Box::new(a_norm))
                }
            }
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty_norm = beta_normal_order_impl(ty, fuel - 1);
            let body_norm = beta_normal_order_impl(body, fuel - 1);
            Expr::Lam(*bi, n.clone(), Box::new(ty_norm), Box::new(body_norm))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty_norm = beta_normal_order_impl(ty, fuel - 1);
            let body_norm = beta_normal_order_impl(body, fuel - 1);
            Expr::Pi(*bi, n.clone(), Box::new(ty_norm), Box::new(body_norm))
        }
        Expr::Let(n, ty, val, body) => {
            let ty_n = beta_normal_order_impl(ty, fuel - 1);
            let val_n = beta_normal_order_impl(val, fuel - 1);
            let body_n = beta_normal_order_impl(body, fuel - 1);
            Expr::Let(n.clone(), Box::new(ty_n), Box::new(val_n), Box::new(body_n))
        }
        Expr::Proj(n, i, s) => {
            Expr::Proj(n.clone(), *i, Box::new(beta_normal_order_impl(s, fuel - 1)))
        }
        e => e.clone(),
    }
}
/// Applicative-order beta reduction.
///
/// Reduces arguments before the function body. Evaluates eagerly.
pub fn beta_applicative_order(expr: &Expr) -> Expr {
    beta_applicative_impl(expr, 200)
}
fn beta_applicative_impl(expr: &Expr, fuel: u32) -> Expr {
    if fuel == 0 {
        return expr.clone();
    }
    match expr {
        Expr::App(f, a) => {
            let f_norm = beta_applicative_impl(f, fuel - 1);
            let a_norm = beta_applicative_impl(a, fuel - 1);
            if let Expr::Lam(_bi, _n, _ty, body) = &f_norm {
                let reduced = instantiate(body, &a_norm);
                beta_applicative_impl(&reduced, fuel - 1)
            } else {
                Expr::App(Box::new(f_norm), Box::new(a_norm))
            }
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty_norm = beta_applicative_impl(ty, fuel - 1);
            let body_norm = beta_applicative_impl(body, fuel - 1);
            Expr::Lam(*bi, n.clone(), Box::new(ty_norm), Box::new(body_norm))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty_norm = beta_applicative_impl(ty, fuel - 1);
            let body_norm = beta_applicative_impl(body, fuel - 1);
            Expr::Pi(*bi, n.clone(), Box::new(ty_norm), Box::new(body_norm))
        }
        Expr::Let(n, ty, val, body) => {
            let ty_n = beta_applicative_impl(ty, fuel - 1);
            let val_n = beta_applicative_impl(val, fuel - 1);
            let body_n = beta_applicative_impl(body, fuel - 1);
            Expr::Let(n.clone(), Box::new(ty_n), Box::new(val_n), Box::new(body_n))
        }
        Expr::Proj(n, i, s) => {
            Expr::Proj(n.clone(), *i, Box::new(beta_applicative_impl(s, fuel - 1)))
        }
        e => e.clone(),
    }
}
/// Check if two expressions are beta-equivalent.
///
/// Two expressions are beta-equivalent if they have the same normal form.
pub fn beta_equivalent(e1: &Expr, e2: &Expr) -> bool {
    let n1 = beta_normalize(e1);
    let n2 = beta_normalize(e2);
    n1 == n2
}
/// Reduce all let-bindings by substitution.
///
/// Converts  to .
pub fn reduce_lets(expr: &Expr) -> Expr {
    match expr {
        Expr::Let(_, _, val, body) => {
            let val_reduced = reduce_lets(val);
            let body_with_val = instantiate(body, &val_reduced);
            reduce_lets(&body_with_val)
        }
        Expr::App(f, a) => Expr::App(Box::new(reduce_lets(f)), Box::new(reduce_lets(a))),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(reduce_lets(ty)),
            Box::new(reduce_lets(body)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(reduce_lets(ty)),
            Box::new(reduce_lets(body)),
        ),
        Expr::Proj(n, i, s) => Expr::Proj(n.clone(), *i, Box::new(reduce_lets(s))),
        e => e.clone(),
    }
}
/// Check if an expression contains any let-bindings.
pub fn has_let(expr: &Expr) -> bool {
    match expr {
        Expr::Let(_, _, _, _) => true,
        Expr::App(f, a) => has_let(f) || has_let(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => has_let(ty) || has_let(body),
        Expr::Proj(_, _, s) => has_let(s),
        _ => false,
    }
}
#[cfg(test)]
mod strategy_tests {
    use super::*;
    use crate::Literal;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn identity() -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        )
    }
    #[test]
    fn test_normal_order_simple() {
        let app = Expr::App(Box::new(identity()), Box::new(lit(5)));
        assert_eq!(beta_normal_order(&app), lit(5));
    }
    #[test]
    fn test_applicative_order_simple() {
        let app = Expr::App(Box::new(identity()), Box::new(lit(5)));
        assert_eq!(beta_applicative_order(&app), lit(5));
    }
    #[test]
    fn test_beta_equivalent() {
        let id = identity();
        let e1 = Expr::App(Box::new(id.clone()), Box::new(lit(7)));
        let e2 = lit(7);
        assert!(beta_equivalent(&e1, &e2));
    }
    #[test]
    fn test_beta_not_equivalent() {
        assert!(!beta_equivalent(&lit(1), &lit(2)));
    }
    #[test]
    fn test_reduce_lets_basic() {
        let body = Expr::BVar(0);
        let val = lit(42);
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(nat()),
            Box::new(val.clone()),
            Box::new(body),
        );
        let result = reduce_lets(&let_expr);
        assert_eq!(result, val);
    }
    #[test]
    fn test_has_let_true() {
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(nat()),
            Box::new(lit(1)),
            Box::new(Expr::BVar(0)),
        );
        assert!(has_let(&let_expr));
    }
    #[test]
    fn test_has_let_false() {
        assert!(!has_let(&lit(42)));
        assert!(!has_let(&nat()));
    }
    #[test]
    fn test_reduce_lets_nested() {
        let inner_let = Expr::Let(
            Name::str("y"),
            Box::new(nat()),
            Box::new(lit(2)),
            Box::new(Expr::BVar(1)),
        );
        let outer_let = Expr::Let(
            Name::str("x"),
            Box::new(nat()),
            Box::new(lit(1)),
            Box::new(inner_let),
        );
        let result = reduce_lets(&outer_let);
        assert_eq!(result, lit(1));
    }
}
#[cfg(test)]
mod extra_beta_tests2 {
    use super::*;
    use crate::Literal;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    #[test]
    fn test_normal_order_reduces_id() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(5)));
        assert_eq!(beta_normal_order(&app), lit(5));
    }
    #[test]
    fn test_applicative_order_reduces_id() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id.clone()), Box::new(lit(7)));
        assert_eq!(beta_applicative_order(&app), lit(7));
    }
    #[test]
    fn test_both_strategies_agree_on_id() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(3)));
        assert_eq!(beta_normal_order(&app), beta_applicative_order(&app));
    }
    #[test]
    fn test_beta_equivalent_same_normal_form() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(3)));
        assert!(beta_equivalent(&app, &lit(3)));
    }
    #[test]
    fn test_beta_equivalent_different() {
        assert!(!beta_equivalent(&lit(1), &lit(2)));
    }
    #[test]
    fn test_reduce_lets_identity() {
        let val = lit(42);
        let body = Expr::BVar(0);
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(nat()),
            Box::new(val.clone()),
            Box::new(body),
        );
        assert_eq!(reduce_lets(&let_expr), val);
    }
    #[test]
    fn test_has_let_none() {
        assert!(!has_let(&lit(42)));
        assert!(!has_let(&nat()));
    }
    #[test]
    fn test_has_let_some() {
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(nat()),
            Box::new(lit(1)),
            Box::new(Expr::BVar(0)),
        );
        assert!(has_let(&let_expr));
    }
    #[test]
    fn test_beta_normal_order_no_redex() {
        let e = lit(42);
        assert_eq!(beta_normal_order(&e), e);
    }
    #[test]
    fn test_beta_applicative_order_no_redex() {
        let e = lit(42);
        assert_eq!(beta_applicative_order(&e), e);
    }
    #[test]
    fn test_is_whnf_sort() {
        let s = Expr::Sort(crate::Level::zero());
        assert!(is_whnf(&s));
    }
    #[test]
    fn test_is_whnf_let_is_not() {
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(nat()),
            Box::new(lit(1)),
            Box::new(Expr::BVar(0)),
        );
        assert!(!is_whnf(&let_expr));
    }
    #[test]
    fn test_beta_whnf_simple() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(9)));
        assert_eq!(beta_whnf(&app), lit(9));
    }
    #[test]
    fn test_beta_whnf_const() {
        let c = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(beta_whnf(&c), c);
    }
    #[test]
    fn test_beta_normalize_fueled_uses_fuel() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(5)));
        let (result, remaining) = beta_normalize_fueled(&app, 10);
        assert_eq!(result, lit(5));
        assert!(remaining < 10);
    }
    #[test]
    fn test_eta_reduce_simple() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let body = Expr::App(Box::new(f.clone()), Box::new(Expr::BVar(0)));
        let lam = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(body),
        );
        assert_eq!(eta_reduce(&lam), Some(f));
    }
    #[test]
    fn test_eta_reduce_non_eta() {
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(body),
        );
        assert!(eta_reduce(&lam).is_none());
    }
    #[test]
    fn test_mk_identity_reduces_correctly() {
        let id = mk_identity(nat());
        let app = Expr::App(Box::new(id), Box::new(lit(42)));
        assert_eq!(beta_normalize(&app), lit(42));
    }
    #[test]
    fn test_mk_const_fn_ignores_arg() {
        let c = mk_const_fn(nat(), lit(99));
        let app = Expr::App(Box::new(c), Box::new(lit(0)));
        assert_eq!(beta_normalize(&app), lit(99));
    }
    #[test]
    fn test_beta_head_normalize_basic() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(77)));
        assert_eq!(beta_head_normalize(&app), lit(77));
    }
    #[test]
    fn test_count_redexes_redex() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(1)));
        assert_eq!(count_redexes(&app), 1);
    }
    #[test]
    fn test_estimate_reduction_depth_zero() {
        assert_eq!(estimate_reduction_depth(&lit(42)), 0);
    }
    #[test]
    fn test_estimate_reduction_depth_redex() {
        let id = Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let app = Expr::App(Box::new(id), Box::new(lit(1)));
        assert!(estimate_reduction_depth(&app) >= 1);
    }
    #[test]
    fn test_mk_multi_beta_redex_single() {
        let body = Expr::BVar(0);
        let redex = mk_multi_beta_redex(&[nat()], body, &[lit(55)]);
        assert_eq!(beta_normalize(&redex), lit(55));
    }
    #[test]
    fn test_mk_compose_applies() {
        let id1 = mk_identity(nat());
        let id2 = mk_identity(nat());
        let composed = mk_compose(id1, id2, nat());
        let app = Expr::App(Box::new(composed), Box::new(lit(5)));
        assert_eq!(beta_normalize(&app), lit(5));
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
