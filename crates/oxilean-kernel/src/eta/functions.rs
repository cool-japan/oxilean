//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{BinderInfo, Expr, Name};

use super::types::{
    ConfigNode, EtaChecker, EtaJob, EtaJobQueue, EtaLog, EtaNormInfo, EtaNormalCache, EtaOpCounter,
    EtaOutcome, EtaReductionStats, EtaStatCounter, EtaStructure, FocusStack, LabelSet, LambdaStats,
    NonEmptyVec, RewriteRule, SimpleDag, SmallMap, StatSummary, TransformStat, VersionedRecord,
    WindowIterator,
};

/// Perform eta expansion: given f, produce λx. f x.
pub fn eta_expand(expr: &Expr, arg_name: Name, arg_ty: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Default,
        arg_name,
        Box::new(arg_ty),
        Box::new(Expr::App(Box::new(expr.clone()), Box::new(Expr::BVar(0)))),
    )
}
/// Perform eta expansion with an implicit binder.
pub fn eta_expand_implicit(expr: &Expr, arg_name: Name, arg_ty: Expr) -> Expr {
    Expr::Lam(
        BinderInfo::Implicit,
        arg_name,
        Box::new(arg_ty),
        Box::new(Expr::App(Box::new(expr.clone()), Box::new(Expr::BVar(0)))),
    )
}
/// Check whether an expression can be eta-expanded (not already a lambda).
pub fn is_eta_expandable(expr: &Expr) -> bool {
    !matches!(expr, Expr::Lam(..))
}
/// Perform one step of eta contraction.
///
/// Contracts (λx. f x) to f when x is not free in f.
pub fn eta_contract(expr: &Expr) -> Option<Expr> {
    if let Expr::Lam(_, _, _, body) = expr {
        if let Expr::App(f, a) = &**body {
            if let Expr::BVar(0) = **a {
                if !contains_bvar(f, 0) {
                    return Some(shift_down(f, 1));
                }
            }
        }
    }
    None
}
/// Fully eta-contract an expression.
pub fn eta_contract_full(expr: &Expr) -> Expr {
    if let Some(c) = eta_contract(expr) {
        return eta_contract_full(&c);
    }
    match expr {
        Expr::App(func, arg) => Expr::App(
            Box::new(eta_contract_full(func)),
            Box::new(eta_contract_full(arg)),
        ),
        Expr::Lam(i, n, ty, body) => Expr::Lam(
            *i,
            n.clone(),
            Box::new(eta_contract_full(ty)),
            Box::new(eta_contract_full(body)),
        ),
        Expr::Pi(i, n, ty, body) => Expr::Pi(
            *i,
            n.clone(),
            Box::new(eta_contract_full(ty)),
            Box::new(eta_contract_full(body)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(eta_contract_full(ty)),
            Box::new(eta_contract_full(val)),
            Box::new(eta_contract_full(body)),
        ),
        _ => expr.clone(),
    }
}
/// Check eta equivalence (normalize then compare).
pub fn eta_equiv(e1: &Expr, e2: &Expr) -> bool {
    eta_normalize(e1) == eta_normalize(e2)
}
/// Eta-normalize an expression (recursively eta-contracting).
pub fn eta_normalize(expr: &Expr) -> Expr {
    let inner = eta_normalize_inner(expr);
    match eta_contract(&inner) {
        Some(c) => eta_normalize(&c),
        None => inner,
    }
}
fn eta_normalize_inner(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, a) => Expr::App(Box::new(eta_normalize(f)), Box::new(eta_normalize(a))),
        Expr::Lam(i, n, ty, body) => Expr::Lam(
            *i,
            n.clone(),
            Box::new(eta_normalize(ty)),
            Box::new(eta_normalize(body)),
        ),
        Expr::Pi(i, n, ty, body) => Expr::Pi(
            *i,
            n.clone(),
            Box::new(eta_normalize(ty)),
            Box::new(eta_normalize(body)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(eta_normalize(ty)),
            Box::new(eta_normalize(val)),
            Box::new(eta_normalize(body)),
        ),
        _ => expr.clone(),
    }
}
/// Check if an expression contains a specific bound variable at the given De Bruijn index.
pub fn contains_bvar(expr: &Expr, idx: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i == idx,
        Expr::App(f, a) => contains_bvar(f, idx) || contains_bvar(a, idx),
        Expr::Lam(_, _, ty, body) => contains_bvar(ty, idx) || contains_bvar(body, idx + 1),
        Expr::Pi(_, _, ty, body) => contains_bvar(ty, idx) || contains_bvar(body, idx + 1),
        Expr::Let(_, ty, val, body) => {
            contains_bvar(ty, idx) || contains_bvar(val, idx) || contains_bvar(body, idx + 1)
        }
        _ => false,
    }
}
/// Count occurrences of a bound variable in an expression.
pub fn count_bvar(expr: &Expr, idx: u32) -> usize {
    match expr {
        Expr::BVar(i) => {
            if *i == idx {
                1
            } else {
                0
            }
        }
        Expr::App(f, a) => count_bvar(f, idx) + count_bvar(a, idx),
        Expr::Lam(_, _, ty, body) => count_bvar(ty, idx) + count_bvar(body, idx + 1),
        Expr::Pi(_, _, ty, body) => count_bvar(ty, idx) + count_bvar(body, idx + 1),
        Expr::Let(_, ty, val, body) => {
            count_bvar(ty, idx) + count_bvar(val, idx) + count_bvar(body, idx + 1)
        }
        _ => 0,
    }
}
/// Shift down bound variables (for removing a binder).
pub fn shift_down(expr: &Expr, amount: u32) -> Expr {
    shift_helper(expr, amount, 0)
}
/// Shift up bound variables (for adding a binder).
pub fn shift_up(expr: &Expr, amount: u32) -> Expr {
    shift_up_helper(expr, amount, 0)
}
fn shift_helper(expr: &Expr, amount: u32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                Expr::BVar(i.saturating_sub(amount))
            } else {
                Expr::BVar(*i)
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(shift_helper(f, amount, cutoff)),
            Box::new(shift_helper(a, amount, cutoff)),
        ),
        Expr::Lam(i, n, ty, body) => Expr::Lam(
            *i,
            n.clone(),
            Box::new(shift_helper(ty, amount, cutoff)),
            Box::new(shift_helper(body, amount, cutoff + 1)),
        ),
        Expr::Pi(i, n, ty, body) => Expr::Pi(
            *i,
            n.clone(),
            Box::new(shift_helper(ty, amount, cutoff)),
            Box::new(shift_helper(body, amount, cutoff + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(shift_helper(ty, amount, cutoff)),
            Box::new(shift_helper(val, amount, cutoff)),
            Box::new(shift_helper(body, amount, cutoff + 1)),
        ),
        _ => expr.clone(),
    }
}
fn shift_up_helper(expr: &Expr, amount: u32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                Expr::BVar(*i + amount)
            } else {
                Expr::BVar(*i)
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(shift_up_helper(f, amount, cutoff)),
            Box::new(shift_up_helper(a, amount, cutoff)),
        ),
        Expr::Lam(i, n, ty, body) => Expr::Lam(
            *i,
            n.clone(),
            Box::new(shift_up_helper(ty, amount, cutoff)),
            Box::new(shift_up_helper(body, amount, cutoff + 1)),
        ),
        Expr::Pi(i, n, ty, body) => Expr::Pi(
            *i,
            n.clone(),
            Box::new(shift_up_helper(ty, amount, cutoff)),
            Box::new(shift_up_helper(body, amount, cutoff + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(shift_up_helper(ty, amount, cutoff)),
            Box::new(shift_up_helper(val, amount, cutoff)),
            Box::new(shift_up_helper(body, amount, cutoff + 1)),
        ),
        _ => expr.clone(),
    }
}
/// Eta-expand a term with an explicit Pi binder.
pub fn eta_expand_pi(expr: &Expr, binder_info: BinderInfo, arg_name: Name, arg_ty: Expr) -> Expr {
    Expr::Lam(
        binder_info,
        arg_name,
        Box::new(arg_ty),
        Box::new(Expr::App(
            Box::new(shift_up(expr, 1)),
            Box::new(Expr::BVar(0)),
        )),
    )
}
/// Check whether an expression is in eta-long normal form.
pub fn is_eta_long(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(_, _, _, body) => is_eta_long(body),
        Expr::App(f, a) => is_eta_long(f) && is_eta_long(a),
        Expr::Pi(_, _, ty, body) => is_eta_long(ty) && is_eta_long(body),
        Expr::Let(_, ty, val, body) => is_eta_long(ty) && is_eta_long(val) && is_eta_long(body),
        _ => true,
    }
}
/// Count the number of leading lambda binders.
pub fn binder_depth(expr: &Expr) -> usize {
    match expr {
        Expr::Lam(_, _, _, body) => 1 + binder_depth(body),
        _ => 0,
    }
}
/// Peel off all leading lambdas, returning binders and body.
pub fn peel_lambdas(expr: &Expr) -> (Vec<(BinderInfo, Name, Expr)>, &Expr) {
    let mut binders = Vec::new();
    let mut current = expr;
    while let Expr::Lam(info, name, ty, body) = current {
        binders.push((*info, name.clone(), *ty.clone()));
        current = body;
    }
    (binders, current)
}
/// Substitute replacement for BVar(depth) in expr.
pub fn subst_bvar(expr: &Expr, depth: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i == depth {
                replacement.clone()
            } else if *i > depth {
                Expr::BVar(*i - 1)
            } else {
                Expr::BVar(*i)
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(subst_bvar(f, depth, replacement)),
            Box::new(subst_bvar(a, depth, replacement)),
        ),
        Expr::Lam(i, n, ty, body) => Expr::Lam(
            *i,
            n.clone(),
            Box::new(subst_bvar(ty, depth, replacement)),
            Box::new(subst_bvar(body, depth + 1, replacement)),
        ),
        Expr::Pi(i, n, ty, body) => Expr::Pi(
            *i,
            n.clone(),
            Box::new(subst_bvar(ty, depth, replacement)),
            Box::new(subst_bvar(body, depth + 1, replacement)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(subst_bvar(ty, depth, replacement)),
            Box::new(subst_bvar(val, depth, replacement)),
            Box::new(subst_bvar(body, depth + 1, replacement)),
        ),
        _ => expr.clone(),
    }
}
/// Check if a lambda body is the identity (lambda x. x).
pub fn is_identity_lambda(expr: &Expr) -> bool {
    if let Expr::Lam(_, _, _, body) = expr {
        matches!(body.as_ref(), Expr::BVar(0))
    } else {
        false
    }
}
/// Count the nesting depth of applications.
pub fn app_depth(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, _) => 1 + app_depth(f),
        _ => 0,
    }
}
/// Collect the spine of an application: (head, [arg1, arg2, ...]).
pub fn collect_app_spine(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut current = expr;
    while let Expr::App(f, a) = current {
        args.push(a.as_ref());
        current = f;
    }
    args.reverse();
    (current, args)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Level;
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    fn mk_lam(name: &str, ty: Expr, body: Expr) -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            Name::str(name),
            Box::new(ty),
            Box::new(body),
        )
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    #[test]
    fn test_eta_expand() {
        let f = mk_const("f");
        let expanded = eta_expand(&f, Name::str("x"), sort());
        if let Expr::Lam(_, name, _, body) = expanded {
            assert_eq!(name, Name::str("x"));
            if let Expr::App(fn_expr, arg_expr) = *body {
                assert_eq!(*fn_expr, f);
                assert_eq!(*arg_expr, Expr::BVar(0));
            } else {
                panic!("Expected App in body");
            }
        } else {
            panic!("Expected Lam");
        }
    }
    #[test]
    fn test_is_eta_expandable() {
        assert!(is_eta_expandable(&mk_const("f")));
        assert!(!is_eta_expandable(&mk_lam("x", sort(), Expr::BVar(0))));
    }
    #[test]
    fn test_eta_contract() {
        let f = mk_const("f");
        let lam = mk_lam("x", sort(), mk_app(f.clone(), Expr::BVar(0)));
        let contracted = eta_contract(&lam);
        assert!(contracted.is_some());
        assert_eq!(contracted.expect("contracted should be valid"), f);
    }
    #[test]
    fn test_eta_contract_none_identity() {
        let lam = mk_lam("x", sort(), Expr::BVar(0));
        assert!(eta_contract(&lam).is_none());
    }
    #[test]
    fn test_eta_contract_full_recurses_into_app() {
        let f = mk_const("f");
        let g = mk_const("g");
        let inner_lam = mk_lam("x", sort(), mk_app(f.clone(), Expr::BVar(0)));
        let expr = mk_app(inner_lam, g.clone());
        let contracted = eta_contract_full(&expr);
        assert_eq!(contracted, mk_app(f, g));
    }
    #[test]
    fn test_eta_contract_full_recurses_into_lam_body() {
        let f = mk_const("f");
        let inner = mk_lam("x", sort(), mk_app(f.clone(), Expr::BVar(0)));
        let outer = mk_lam("y", sort(), inner);
        let contracted = eta_contract_full(&outer);
        assert_eq!(contracted, mk_lam("y", sort(), f));
    }
    #[test]
    fn test_contains_bvar() {
        let expr = mk_app(Expr::BVar(2), Expr::BVar(0));
        assert!(contains_bvar(&expr, 2));
        assert!(contains_bvar(&expr, 0));
        assert!(!contains_bvar(&expr, 1));
    }
    #[test]
    fn test_count_bvar() {
        let expr = mk_app(Expr::BVar(0), Expr::BVar(0));
        assert_eq!(count_bvar(&expr, 0), 2);
        assert_eq!(count_bvar(&expr, 1), 0);
    }
    #[test]
    fn test_shift_down() {
        assert_eq!(shift_down(&Expr::BVar(3), 1), Expr::BVar(2));
    }
    #[test]
    fn test_shift_up() {
        assert_eq!(shift_up(&Expr::BVar(1), 2), Expr::BVar(3));
    }
    #[test]
    fn test_binder_depth() {
        let lam = mk_lam("x", sort(), mk_lam("y", sort(), Expr::BVar(0)));
        assert_eq!(binder_depth(&lam), 2);
        assert_eq!(binder_depth(&mk_const("f")), 0);
    }
    #[test]
    fn test_peel_lambdas() {
        let inner = mk_const("body");
        let lam = mk_lam("x", sort(), mk_lam("y", sort(), inner.clone()));
        let (binders, body) = peel_lambdas(&lam);
        assert_eq!(binders.len(), 2);
        assert_eq!(*body, inner);
    }
    #[test]
    fn test_eta_normalize_idempotent() {
        let f = mk_const("f");
        let lam = mk_lam("x", sort(), mk_app(f.clone(), Expr::BVar(0)));
        let normalized = eta_normalize(&lam);
        assert_eq!(normalized, f);
        assert_eq!(eta_normalize(&normalized), normalized);
    }
    #[test]
    fn test_eta_expand_pi() {
        let f = mk_const("f");
        let expanded = eta_expand_pi(&f, BinderInfo::Default, Name::str("x"), sort());
        assert!(matches!(expanded, Expr::Lam(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_is_identity_lambda() {
        assert!(is_identity_lambda(&mk_lam("x", sort(), Expr::BVar(0))));
        assert!(!is_identity_lambda(&mk_lam("x", sort(), mk_const("y"))));
    }
    #[test]
    fn test_collect_app_spine() {
        let f = mk_const("f");
        let a = mk_const("a");
        let b = mk_const("b");
        let expr = mk_app(mk_app(f.clone(), a.clone()), b.clone());
        let (head, args) = collect_app_spine(&expr);
        assert_eq!(*head, f);
        assert_eq!(args.len(), 2);
    }
}
/// Check whether two expressions are eta-equivalent up to the given depth.
pub fn eta_equiv_depth(e1: &Expr, e2: &Expr, max_depth: u32) -> bool {
    if max_depth == 0 {
        return e1 == e2;
    }
    let n1 = eta_normalize(e1);
    let n2 = eta_normalize(e2);
    n1 == n2
}
/// Eta-expand an expression to match a target number of lambda binders.
///
/// If `expr` already has `n` or more binders, it is returned unchanged.
pub fn eta_expand_to(expr: &Expr, n: usize) -> Expr {
    let current = binder_depth(expr);
    if current >= n {
        return expr.clone();
    }
    let extra = n - current;
    let mut result = expr.clone();
    for i in 0..extra {
        let arg_name = Name::str(format!("_x{}", i));
        result = eta_expand_pi(&result, BinderInfo::Default, arg_name, Expr::BVar(0));
    }
    result
}
/// Apply a sequence of arguments to a head expression.
pub fn mk_app_spine(head: &Expr, args: &[Expr]) -> Expr {
    args.iter().fold(head.clone(), |acc, arg| {
        Expr::App(Box::new(acc), Box::new(arg.clone()))
    })
}
/// Rebuild a lambda from peeled binders and a body.
pub fn rebuild_lam(binders: &[(BinderInfo, Name, Expr)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (info, name, ty)| {
        Expr::Lam(*info, name.clone(), Box::new(ty.clone()), Box::new(acc))
    })
}
/// Rebuild a Pi from peeled binders and a body.
pub fn rebuild_pi(binders: &[(BinderInfo, Name, Expr)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (info, name, ty)| {
        Expr::Pi(*info, name.clone(), Box::new(ty.clone()), Box::new(acc))
    })
}
/// Collect the Pi binders of a type, returning (binders, return_type).
pub fn peel_pis(expr: &Expr) -> (Vec<(BinderInfo, Name, Expr)>, &Expr) {
    let mut binders = Vec::new();
    let mut current = expr;
    while let Expr::Pi(info, name, ty, body) = current {
        binders.push((*info, name.clone(), *ty.clone()));
        current = body;
    }
    (binders, current)
}
/// Count the number of leading Pi binders.
pub fn pi_depth(expr: &Expr) -> usize {
    match expr {
        Expr::Pi(_, _, _, body) => 1 + pi_depth(body),
        _ => 0,
    }
}
/// Attempt to compute the arity of an expression by counting Pi binders.
pub fn arity(expr: &Expr) -> usize {
    pi_depth(expr)
}
/// Check whether an expression is a "simple" application (head is a constant).
pub fn is_simple_app(expr: &Expr) -> bool {
    let (head, _) = collect_app_spine(expr);
    matches!(head, Expr::Const(..))
}
/// Check whether an expression is closed (no free BVars at depth >= `cutoff`).
pub fn is_closed_at(expr: &Expr, cutoff: u32) -> bool {
    !contains_bvar(expr, cutoff)
}
/// Check whether an expression is completely closed (no BVars and no FVars).
pub fn is_closed(expr: &Expr) -> bool {
    match expr {
        Expr::BVar(_) => false,
        Expr::FVar(_) => false,
        Expr::App(f, a) => is_closed(f) && is_closed(a),
        Expr::Lam(_, _, ty, body) => is_closed(ty) && is_closed(body),
        Expr::Pi(_, _, ty, body) => is_closed(ty) && is_closed(body),
        Expr::Let(_, ty, val, body) => is_closed(ty) && is_closed(val) && is_closed(body),
        Expr::Proj(_, _, inner) => is_closed(inner),
        _ => true,
    }
}
/// Replace all lambda binders in `expr` with pi binders (unsafe structural change).
pub fn lam_to_pi(expr: &Expr) -> Expr {
    match expr {
        Expr::Lam(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(lam_to_pi(ty)),
            Box::new(lam_to_pi(body)),
        ),
        Expr::App(f, a) => Expr::App(Box::new(lam_to_pi(f)), Box::new(lam_to_pi(a))),
        _ => expr.clone(),
    }
}
/// Substitute `replacement` for the outermost bound variable (BVar(0)).
pub fn beta_reduce_one(lam: &Expr, arg: &Expr) -> Option<Expr> {
    if let Expr::Lam(_, _, _, body) = lam {
        Some(subst_bvar(body, 0, arg))
    } else {
        None
    }
}
/// Apply beta reduction repeatedly until no redex exists at the top level.
pub fn beta_reduce_full(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, a) => {
            let f2 = beta_reduce_full(f);
            let a2 = beta_reduce_full(a);
            match beta_reduce_one(&f2, &a2) {
                Some(reduced) => beta_reduce_full(&reduced),
                None => Expr::App(Box::new(f2), Box::new(a2)),
            }
        }
        Expr::Lam(i, n, ty, body) => Expr::Lam(
            *i,
            n.clone(),
            Box::new(beta_reduce_full(ty)),
            Box::new(beta_reduce_full(body)),
        ),
        Expr::Pi(i, n, ty, body) => Expr::Pi(
            *i,
            n.clone(),
            Box::new(beta_reduce_full(ty)),
            Box::new(beta_reduce_full(body)),
        ),
        Expr::Let(_n, _ty, val, body) => {
            let val2 = beta_reduce_full(val);
            let body2 = subst_bvar(body, 0, &val2);
            beta_reduce_full(&body2)
        }
        _ => expr.clone(),
    }
}
/// Normalize by both eta-contracting and beta-reducing.
pub fn normalize_full(expr: &Expr) -> Expr {
    let beta = beta_reduce_full(expr);
    eta_normalize(&beta)
}
/// Lift a substitution under `n` binders (shift the replacement up by `n`).
pub fn subst_bvar_shifted(expr: &Expr, depth: u32, replacement: &Expr, shift: u32) -> Expr {
    let shifted = shift_up(replacement, shift);
    subst_bvar(expr, depth, &shifted)
}
/// Check structural equality after full normalization.
pub fn normal_eq(e1: &Expr, e2: &Expr) -> bool {
    normalize_full(e1) == normalize_full(e2)
}
#[cfg(test)]
mod eta_extended_tests {
    use super::*;
    use crate::Level;
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    fn mk_lam(name: &str, ty: Expr, body: Expr) -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            Name::str(name),
            Box::new(ty),
            Box::new(body),
        )
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    fn mk_pi(name: &str, ty: Expr, body: Expr) -> Expr {
        Expr::Pi(
            BinderInfo::Default,
            Name::str(name),
            Box::new(ty),
            Box::new(body),
        )
    }
    #[test]
    fn test_lambda_stats() {
        let expr = mk_lam(
            "x",
            sort(),
            mk_lam("y", sort(), mk_app(Expr::BVar(1), Expr::BVar(0))),
        );
        let stats = LambdaStats::compute(&expr);
        assert_eq!(stats.lambda_count, 2);
        assert_eq!(stats.app_count, 1);
    }
    #[test]
    fn test_peel_pis() {
        let body = mk_const("Nat");
        let pi_expr = mk_pi("x", sort(), mk_pi("y", sort(), body.clone()));
        let (binders, ret) = peel_pis(&pi_expr);
        assert_eq!(binders.len(), 2);
        assert_eq!(*ret, body);
    }
    #[test]
    fn test_pi_depth() {
        let expr = mk_pi("x", sort(), mk_pi("y", sort(), mk_const("Nat")));
        assert_eq!(pi_depth(&expr), 2);
        assert_eq!(pi_depth(&mk_const("Nat")), 0);
    }
    #[test]
    fn test_rebuild_lam() {
        let binders = vec![(BinderInfo::Default, Name::str("x"), sort())];
        let body = Expr::BVar(0);
        let result = rebuild_lam(&binders, body);
        assert!(matches!(result, Expr::Lam(..)));
    }
    #[test]
    fn test_rebuild_pi() {
        let binders = vec![(BinderInfo::Default, Name::str("x"), sort())];
        let body = Expr::BVar(0);
        let result = rebuild_pi(&binders, body);
        assert!(matches!(result, Expr::Pi(..)));
    }
    #[test]
    fn test_mk_app_spine() {
        let f = mk_const("f");
        let args = vec![mk_const("a"), mk_const("b")];
        let result = mk_app_spine(&f, &args);
        let (head, spine) = collect_app_spine(&result);
        assert_eq!(*head, f);
        assert_eq!(spine.len(), 2);
    }
    #[test]
    fn test_is_closed() {
        assert!(is_closed(&mk_const("f")));
        assert!(!is_closed(&Expr::BVar(0)));
        assert!(!is_closed(&mk_lam("x", sort(), Expr::BVar(0))));
        assert!(is_closed(&mk_lam("x", sort(), mk_const("a"))));
    }
    #[test]
    fn test_beta_reduce_one() {
        let lam = mk_lam("x", sort(), Expr::BVar(0));
        let arg = mk_const("a");
        let result = beta_reduce_one(&lam, &arg);
        assert_eq!(result, Some(arg));
    }
    #[test]
    fn test_beta_reduce_full() {
        let lam = mk_lam("x", sort(), Expr::BVar(0));
        let arg = mk_const("a");
        let app = mk_app(lam, arg.clone());
        assert_eq!(beta_reduce_full(&app), arg);
    }
    #[test]
    fn test_normal_eq() {
        let f = mk_const("f");
        let lam = mk_lam("x", sort(), mk_app(f.clone(), Expr::BVar(0)));
        assert!(normal_eq(&lam, &f));
    }
    #[test]
    fn test_is_simple_app() {
        let expr = mk_app(mk_const("f"), mk_const("a"));
        assert!(is_simple_app(&expr));
        let lam_app = mk_app(mk_lam("x", sort(), Expr::BVar(0)), mk_const("a"));
        assert!(!is_simple_app(&lam_app));
    }
    #[test]
    fn test_lam_to_pi() {
        let lam = mk_lam("x", sort(), Expr::BVar(0));
        let pi = lam_to_pi(&lam);
        assert!(matches!(pi, Expr::Pi(..)));
    }
    #[test]
    fn test_arity() {
        let ty = mk_pi("a", sort(), mk_pi("b", sort(), mk_const("Nat")));
        assert_eq!(arity(&ty), 2);
    }
}
/// Apply a list of rewrite rules to an expression, innermost-first.
pub fn rewrite_with_rules(expr: &Expr, rules: &[RewriteRule]) -> Expr {
    let inner = match expr {
        Expr::App(f, a) => Expr::App(
            Box::new(rewrite_with_rules(f, rules)),
            Box::new(rewrite_with_rules(a, rules)),
        ),
        Expr::Lam(i, n, ty, body) => Expr::Lam(
            *i,
            n.clone(),
            Box::new(rewrite_with_rules(ty, rules)),
            Box::new(rewrite_with_rules(body, rules)),
        ),
        Expr::Pi(i, n, ty, body) => Expr::Pi(
            *i,
            n.clone(),
            Box::new(rewrite_with_rules(ty, rules)),
            Box::new(rewrite_with_rules(body, rules)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(rewrite_with_rules(ty, rules)),
            Box::new(rewrite_with_rules(val, rules)),
            Box::new(rewrite_with_rules(body, rules)),
        ),
        _ => expr.clone(),
    };
    for rule in rules {
        if let Some(rewritten) = rule.apply_top(&inner) {
            return rewritten;
        }
    }
    inner
}
/// Check whether an expression is a beta-redex (lambda applied to an argument).
pub fn is_beta_redex(expr: &Expr) -> bool {
    matches!(expr, Expr::App(f, _) if matches!(f.as_ref(), Expr::Lam(..)))
}
/// Check whether an expression is an eta-redex (λx. f x with x not free in f).
pub fn is_eta_redex(expr: &Expr) -> bool {
    eta_contract(expr).is_some()
}
/// Count the total number of nodes in an expression tree.
pub fn node_count(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + node_count(f) + node_count(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + node_count(ty) + node_count(body)
        }
        Expr::Let(_, ty, val, body) => 1 + node_count(ty) + node_count(val) + node_count(body),
        _ => 1,
    }
}
/// Check whether an expression has no lambdas anywhere in it.
pub fn is_lambda_free(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(..) => false,
        Expr::App(f, a) => is_lambda_free(f) && is_lambda_free(a),
        Expr::Pi(_, _, ty, body) => is_lambda_free(ty) && is_lambda_free(body),
        Expr::Let(_, ty, val, body) => {
            is_lambda_free(ty) && is_lambda_free(val) && is_lambda_free(body)
        }
        _ => true,
    }
}
/// Compute a simple structural hash of an expression.
pub fn structural_hash(expr: &Expr) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    format!("{:?}", expr).hash(&mut hasher);
    hasher.finish()
}
/// Wrap an expression in `n` identity lambdas: `λ_0. λ_1. ... expr`.
pub fn wrap_in_lambdas(expr: &Expr, n: usize) -> Expr {
    (0..n).fold(expr.clone(), |acc, i| {
        Expr::Lam(
            BinderInfo::Default,
            Name::str(format!("_w{}", i)),
            Box::new(Expr::Sort(crate::Level::zero())),
            Box::new(shift_up(&acc, 1)),
        )
    })
}
/// Apply `n` BVar arguments to an expression: `expr BVar(0) BVar(1) ... BVar(n-1)`.
pub fn apply_bvars(expr: &Expr, n: u32) -> Expr {
    (0..n).fold(expr.clone(), |acc, i| {
        Expr::App(Box::new(acc), Box::new(Expr::BVar(i)))
    })
}
/// Reduce `let x := v in body` to `body[x := v]`.
pub fn reduce_let(expr: &Expr) -> Option<Expr> {
    if let Expr::Let(_, _, val, body) = expr {
        Some(subst_bvar(body, 0, val))
    } else {
        None
    }
}
/// Fully reduce all let bindings in an expression.
pub fn reduce_lets(expr: &Expr) -> Expr {
    match expr {
        Expr::Let(_, _, val, body) => {
            let val2 = reduce_lets(val);
            let body2 = reduce_lets(body);
            reduce_lets(&subst_bvar(&body2, 0, &val2))
        }
        Expr::App(f, a) => Expr::App(Box::new(reduce_lets(f)), Box::new(reduce_lets(a))),
        Expr::Lam(i, n, ty, body) => Expr::Lam(
            *i,
            n.clone(),
            Box::new(reduce_lets(ty)),
            Box::new(reduce_lets(body)),
        ),
        Expr::Pi(i, n, ty, body) => Expr::Pi(
            *i,
            n.clone(),
            Box::new(reduce_lets(ty)),
            Box::new(reduce_lets(body)),
        ),
        _ => expr.clone(),
    }
}
#[cfg(test)]
mod eta_further_tests {
    use super::*;
    use crate::Level;
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn mk_const(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    fn mk_lam(name: &str, ty: Expr, body: Expr) -> Expr {
        Expr::Lam(
            BinderInfo::Default,
            Name::str(name),
            Box::new(ty),
            Box::new(body),
        )
    }
    fn mk_app(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    #[test]
    fn test_is_beta_redex() {
        let lam = mk_lam("x", sort(), Expr::BVar(0));
        let app = mk_app(lam, mk_const("a"));
        assert!(is_beta_redex(&app));
        assert!(!is_beta_redex(&mk_const("f")));
    }
    #[test]
    fn test_is_eta_redex() {
        let f = mk_const("f");
        let lam = mk_lam("x", sort(), mk_app(f.clone(), Expr::BVar(0)));
        assert!(is_eta_redex(&lam));
        assert!(!is_eta_redex(&f));
    }
    #[test]
    fn test_node_count() {
        let expr = mk_app(mk_const("f"), mk_const("a"));
        assert_eq!(node_count(&expr), 3);
        assert_eq!(node_count(&mk_const("f")), 1);
    }
    #[test]
    fn test_is_lambda_free() {
        assert!(is_lambda_free(&mk_const("f")));
        assert!(!is_lambda_free(&mk_lam("x", sort(), Expr::BVar(0))));
    }
    #[test]
    fn test_rewrite_rule_apply() {
        let rule = RewriteRule::new("id", mk_const("x"), mk_const("y"));
        assert_eq!(rule.apply_top(&mk_const("x")), Some(mk_const("y")));
        assert_eq!(rule.apply_top(&mk_const("z")), None);
    }
    #[test]
    fn test_rewrite_with_rules() {
        let rules = vec![RewriteRule::new("r", mk_const("a"), mk_const("b"))];
        let expr = mk_app(mk_const("f"), mk_const("a"));
        let result = rewrite_with_rules(&expr, &rules);
        assert_eq!(result, mk_app(mk_const("f"), mk_const("b")));
    }
    #[test]
    fn test_reduce_let() {
        let body = Expr::BVar(0);
        let val = mk_const("v");
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(sort()),
            Box::new(val.clone()),
            Box::new(body),
        );
        assert_eq!(reduce_let(&let_expr), Some(val));
    }
    #[test]
    fn test_wrap_in_lambdas() {
        let body = mk_const("f");
        let wrapped = wrap_in_lambdas(&body, 2);
        assert_eq!(binder_depth(&wrapped), 2);
    }
    #[test]
    fn test_apply_bvars() {
        let f = mk_const("f");
        let result = apply_bvars(&f, 2);
        let (_, args) = collect_app_spine(&result);
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_structural_hash_stable() {
        let e = mk_const("foo");
        assert_eq!(structural_hash(&e), structural_hash(&e));
    }
}
/// Eta-normalize and track how many contractions were performed.
#[allow(dead_code)]
pub fn eta_normalize_tracked(expr: &Expr) -> EtaNormInfo {
    let mut count = 0;
    let mut current = expr.clone();
    while let Some(c) = eta_contract(&current) {
        count += 1;
        current = c;
    }
    if count == 0 {
        EtaNormInfo::already_normal(current)
    } else {
        EtaNormInfo::contracted(current, count)
    }
}
/// Determine if two expressions are definitionally equal under eta.
///
/// Eta-normalizes both then checks structural equality.
#[allow(dead_code)]
pub fn eta_def_eq(e1: &Expr, e2: &Expr) -> bool {
    eta_normalize(e1) == eta_normalize(e2)
}
/// Return the "eta-head" of an expression.
///
/// The eta-head is the result of stripping all outer lambda binders that
/// form eta-redexes. For `λx. f x` this returns `f` (if x not free in f).
#[allow(dead_code)]
pub fn eta_head(expr: &Expr) -> Expr {
    let contracted = eta_contract_full(expr);
    if contracted == *expr {
        expr.clone()
    } else {
        eta_head(&contracted)
    }
}
/// Count the maximum lambda depth (outermost consecutive lambdas).
#[allow(dead_code)]
pub fn outer_lambda_depth(expr: &Expr) -> usize {
    match expr {
        Expr::Lam(_, _, _, body) => 1 + outer_lambda_depth(body),
        _ => 0,
    }
}
/// Expand an expression to exactly `n` lambda binders using eta expansion.
///
/// If the expression already has >= n outer lambdas, it is returned unchanged.
#[allow(dead_code)]
pub fn eta_expand_with_ty(expr: &Expr, n: usize, arg_ty: &Expr) -> Expr {
    let current_depth = outer_lambda_depth(expr);
    let mut result = expr.clone();
    for _ in current_depth..n {
        result = eta_expand(&result, Name::str("_x"), arg_ty.clone());
    }
    result
}
/// Check if an expression has no free variables using a recursive walk.
///
/// This is an alias for [`is_closed`] provided for symmetry with `is_open`.
#[allow(dead_code)]
pub fn expr_is_closed(expr: &Expr) -> bool {
    is_closed(expr)
}
/// Produce a one-line human-readable description of an expression's structure.
#[allow(dead_code)]
pub fn expr_structure_desc(expr: &Expr) -> &'static str {
    match expr {
        Expr::BVar(_) => "BVar",
        Expr::FVar(_) => "FVar",
        Expr::Const(_, _) => "Const",
        Expr::Sort(_) => "Sort",
        Expr::Lam(_, _, _, _) => "Lam",
        Expr::Pi(_, _, _, _) => "Pi",
        Expr::App(_, _) => "App",
        Expr::Let(_, _, _, _) => "Let",
        Expr::Lit(_) => "Lit",
        Expr::Proj(_, _, _) => "Proj",
    }
}
#[cfg(test)]
mod extra_eta_tests {
    use super::*;
    fn mk_sort_expr() -> Expr {
        Expr::Sort(crate::Level::zero())
    }
    fn mk_const_e(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    fn mk_app_e(f: Expr, a: Expr) -> Expr {
        Expr::App(Box::new(f), Box::new(a))
    }
    fn mk_lam_e(body: Expr) -> Expr {
        Expr::Lam(
            crate::BinderInfo::Default,
            Name::str("x"),
            Box::new(mk_sort_expr()),
            Box::new(body),
        )
    }
    #[test]
    fn test_eta_normalize_tracked_already_normal() {
        let e = mk_const_e("f");
        let info = eta_normalize_tracked(&e);
        assert!(info.already_normal);
        assert_eq!(info.contractions, 0);
    }
    #[test]
    fn test_eta_normalize_tracked_contracts_one() {
        let f = mk_const_e("f");
        let e = mk_lam_e(mk_app_e(f.clone(), Expr::BVar(0)));
        let info = eta_normalize_tracked(&e);
        assert!(!info.already_normal);
        assert_eq!(info.contractions, 1);
        assert_eq!(info.normalized, f);
    }
    #[test]
    fn test_eta_def_eq_same() {
        let e = mk_const_e("foo");
        assert!(eta_def_eq(&e, &e));
    }
    #[test]
    fn test_eta_def_eq_eta_variants() {
        let f = mk_const_e("f");
        let e = mk_lam_e(mk_app_e(f.clone(), Expr::BVar(0)));
        assert!(eta_def_eq(&e, &f));
    }
    #[test]
    fn test_outer_lambda_depth_zero() {
        assert_eq!(outer_lambda_depth(&mk_const_e("x")), 0);
    }
    #[test]
    fn test_outer_lambda_depth_one() {
        let e = mk_lam_e(Expr::BVar(0));
        assert_eq!(outer_lambda_depth(&e), 1);
    }
    #[test]
    fn test_outer_lambda_depth_two() {
        let e = mk_lam_e(mk_lam_e(Expr::BVar(0)));
        assert_eq!(outer_lambda_depth(&e), 2);
    }
    #[test]
    fn test_eta_expand_to_already_deep_enough() {
        let e = mk_lam_e(Expr::BVar(0));
        let expanded = eta_expand_with_ty(&e, 1, &mk_sort_expr());
        assert_eq!(outer_lambda_depth(&expanded), 1);
    }
    #[test]
    fn test_is_closed_const() {
        assert!(expr_is_closed(&mk_const_e("Nat")));
    }
    #[test]
    fn test_is_closed_fvar() {
        let e = Expr::FVar(crate::FVarId::new(1));
        assert!(!expr_is_closed(&e));
    }
    #[test]
    fn test_is_closed_app_with_fvar() {
        let f = mk_const_e("f");
        let a = Expr::FVar(crate::FVarId::new(99));
        let e = mk_app_e(f, a);
        assert!(!expr_is_closed(&e));
    }
    #[test]
    fn test_is_closed_nested() {
        let e = mk_app_e(mk_const_e("f"), mk_const_e("a"));
        assert!(expr_is_closed(&e));
    }
    #[test]
    fn test_expr_structure_desc() {
        assert_eq!(expr_structure_desc(&mk_const_e("x")), "Const");
        assert_eq!(expr_structure_desc(&mk_lam_e(Expr::BVar(0))), "Lam");
        assert_eq!(expr_structure_desc(&Expr::BVar(0)), "BVar");
    }
    #[test]
    fn test_eta_head_already_contracted() {
        let f = mk_const_e("f");
        assert_eq!(eta_head(&f), f);
    }
    #[test]
    fn test_eta_info_contracted_fields() {
        let info = EtaNormInfo::contracted(mk_const_e("x"), 3);
        assert!(!info.already_normal);
        assert_eq!(info.contractions, 3);
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
mod tests_eta_padding {
    use super::*;
    #[test]
    fn test_eta_reduction_stats() {
        let mut s = EtaReductionStats::new();
        s.examined = 10;
        s.reductions = 7;
        assert!((s.ratio() - 0.7).abs() < 1e-9);
    }
    #[test]
    fn test_eta_op_counter() {
        let mut c = EtaOpCounter::new();
        c.inc("check");
        c.inc("check");
        c.inc("reduce");
        assert_eq!(c.get("check"), 2);
        assert_eq!(c.total(), 3);
    }
    #[test]
    fn test_eta_normal_cache() {
        let mut cache = EtaNormalCache::new();
        cache.insert(42, true);
        assert_eq!(cache.query(42), Some(true));
        assert_eq!(cache.query(99), None);
        cache.clear();
        assert_eq!(cache.len(), 0);
    }
    #[test]
    fn test_eta_job_queue() {
        let mut q = EtaJobQueue::new();
        q.enqueue(EtaJob::new(1, 100, 5));
        q.enqueue(EtaJob::new(2, 200, 10));
        q.enqueue(EtaJob::new(3, 300, 1));
        assert_eq!(q.len(), 3);
        let job = q.dequeue().expect("job should be present");
        assert_eq!(job.prio, 10);
    }
    #[test]
    fn test_eta_outcome() {
        assert!(EtaOutcome::Reduced.is_success());
        assert!(EtaOutcome::AlreadyNormal.is_success());
        assert!(!EtaOutcome::NotApplicable.is_success());
        assert_eq!(EtaOutcome::Reduced.label(), "reduced");
    }
    #[test]
    fn test_eta_log() {
        let mut log = EtaLog::new();
        log.record(EtaOutcome::Reduced);
        log.record(EtaOutcome::AlreadyNormal);
        log.record(EtaOutcome::NotApplicable);
        assert_eq!(log.count(EtaOutcome::Reduced), 1);
        assert!((log.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_eta_structure() {
        let s = EtaStructure::Lambda(3);
        assert_eq!(s.arity(), 3);
        let a = EtaStructure::Atomic;
        assert_eq!(a.arity(), 0);
    }
    #[test]
    fn test_eta_checker() {
        let mut ec = EtaChecker::new();
        let outcome = ec.is_eta_normal(42);
        assert!(!outcome.is_success());
    }
}
#[cfg(test)]
mod tests_eta_stat_counter {
    use super::*;
    #[test]
    fn test_eta_stat_counter() {
        let mut c = EtaStatCounter::new();
        c.record(10);
        c.record(20);
        c.record(30);
        assert!((c.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(c.count(), 3);
    }
}
