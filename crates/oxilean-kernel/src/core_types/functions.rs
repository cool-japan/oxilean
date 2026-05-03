//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::expr::{BinderInfo, Expr, FVarId, Literal};
pub use crate::level::{Level, LevelMVarId};
pub use crate::name::Name;

use super::types::{
    ConfigNode, FocusStack, LabelSet, SimpleDag, SmallMap, StatSummary, TransformStat,
    VersionedRecord,
};
/// Version string for the OxiLean kernel.
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Return the kernel version as a `(major, minor, patch)` tuple.
#[allow(dead_code)]
pub fn kernel_version() -> (u32, u32, u32) {
    let v = KERNEL_VERSION;
    let parts: Vec<u32> = v.split('.').filter_map(|s| s.parse().ok()).collect();
    match parts.as_slice() {
        [major, minor, patch, ..] => (*major, *minor, *patch),
        [major, minor] => (*major, *minor, 0),
        [major] => (*major, 0, 0),
        [] => (0, 0, 0),
    }
}
/// Convenience: make a `Prop` expression (Sort 0).
#[allow(dead_code)]
pub fn mk_prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Convenience: make `Type 0` (Sort 1).
#[allow(dead_code)]
pub fn mk_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Convenience: make `Type 1` (Sort 2).
#[allow(dead_code)]
pub fn mk_type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// Convenience: make `Sort u` for an arbitrary level.
#[allow(dead_code)]
pub fn mk_sort(level: Level) -> Expr {
    Expr::Sort(level)
}
/// Convenience: make a `Nat` literal expression.
#[allow(dead_code)]
pub fn mk_nat_lit(n: u64) -> Expr {
    Expr::Lit(Literal::Nat(n))
}
/// Convenience: make a `String` literal expression.
#[allow(dead_code)]
pub fn mk_string_lit(s: &str) -> Expr {
    Expr::Lit(Literal::Str(s.to_string()))
}
/// Convenience: make `App(f, a)`.
#[allow(dead_code)]
pub fn mk_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Build `f a1 a2 ... an` from a head `f` and argument list.
#[allow(dead_code)]
pub fn mk_app_spine(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, mk_app)
}
/// Convenience: make a Pi-type `(x : dom) -> cod`.
#[allow(dead_code)]
pub fn mk_pi(name: Name, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(BinderInfo::Default, name, Box::new(dom), Box::new(cod))
}
/// Build a chain of Pi-types from a list of `(name, type)` binders and a result type.
#[allow(dead_code)]
pub fn mk_pi_chain(binders: Vec<(Name, Expr)>, ret: Expr) -> Expr {
    binders
        .into_iter()
        .rev()
        .fold(ret, |acc, (n, ty)| mk_pi(n, ty, acc))
}
/// Convenience: make a lambda `fun x : dom => body`.
#[allow(dead_code)]
pub fn mk_lam(name: Name, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(BinderInfo::Default, name, Box::new(dom), Box::new(body))
}
/// Build a chain of lambdas from a list of `(name, type)` binders and a body.
#[allow(dead_code)]
pub fn mk_lam_chain(binders: Vec<(Name, Expr)>, body: Expr) -> Expr {
    binders
        .into_iter()
        .rev()
        .fold(body, |acc, (n, ty)| mk_lam(n, ty, acc))
}
/// Unfold the head and arguments of an `App` spine.
///
/// `f a b c` → `(f, [a, b, c])`
#[allow(dead_code)]
pub fn unfold_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut cur = expr;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f;
    }
    args.reverse();
    (cur, args)
}
/// Count the number of arguments an expression is applied to.
#[allow(dead_code)]
pub fn app_arity(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, _) => 1 + app_arity(f),
        _ => 0,
    }
}
/// Count Pi binders in a Pi-chain.
#[allow(dead_code)]
pub fn count_pi_binders(expr: &Expr) -> usize {
    match expr {
        Expr::Pi(_, _, _, body) => 1 + count_pi_binders(body),
        _ => 0,
    }
}
/// Count Lam binders in a Lambda-chain.
#[allow(dead_code)]
pub fn count_lam_binders(expr: &Expr) -> usize {
    match expr {
        Expr::Lam(_, _, _, body) => 1 + count_lam_binders(body),
        _ => 0,
    }
}
/// Check whether an expression is closed (contains no `BVar` with index ≥ depth).
#[allow(dead_code)]
pub fn is_closed(expr: &Expr) -> bool {
    is_closed_at(expr, 0)
}
fn is_closed_at(expr: &Expr, depth: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i < depth,
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => true,
        Expr::App(f, a) => is_closed_at(f, depth) && is_closed_at(a, depth),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            is_closed_at(dom, depth) && is_closed_at(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            is_closed_at(ty, depth) && is_closed_at(val, depth) && is_closed_at(body, depth + 1)
        }
        Expr::Proj(_, _, e) => is_closed_at(e, depth),
    }
}
/// Check whether an expression contains no `FVar`, `MVar`, or `BVar` nodes.
#[allow(dead_code)]
pub fn is_ground(expr: &Expr) -> bool {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) => false,
        Expr::Sort(_) | Expr::Lit(_) => true,
        Expr::Const(_, _) => true,
        Expr::App(f, a) => is_ground(f) && is_ground(a),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => is_ground(dom) && is_ground(body),
        Expr::Let(_, ty, val, body) => is_ground(ty) && is_ground(val) && is_ground(body),
        Expr::Proj(_, _, e) => is_ground(e),
    }
}
/// Compute an approximate "size" of an expression (node count).
#[allow(dead_code)]
pub fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) | Expr::Const(_, _) => 1,
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            1 + expr_size(dom) + expr_size(body)
        }
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, e) => 1 + expr_size(e),
    }
}
/// Check whether an expression contains any metavariable (`MVar`).
#[allow(dead_code)]
pub fn has_metavars(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, a) => has_metavars(f) || has_metavars(a),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            has_metavars(dom) || has_metavars(body)
        }
        Expr::Let(_, ty, val, body) => has_metavars(ty) || has_metavars(val) || has_metavars(body),
        Expr::Proj(_, _, e) => has_metavars(e),
        _ => false,
    }
}
/// Collect all `Const` names referenced in an expression.
#[allow(dead_code)]
pub fn collect_const_names(expr: &Expr) -> Vec<Name> {
    let mut names = Vec::new();
    collect_const_names_rec(expr, &mut names);
    names
}
fn collect_const_names_rec(expr: &Expr, acc: &mut Vec<Name>) {
    match expr {
        Expr::Const(n, _) => acc.push(n.clone()),
        Expr::App(f, a) => {
            collect_const_names_rec(f, acc);
            collect_const_names_rec(a, acc);
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            collect_const_names_rec(dom, acc);
            collect_const_names_rec(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_const_names_rec(ty, acc);
            collect_const_names_rec(val, acc);
            collect_const_names_rec(body, acc);
        }
        Expr::Proj(_, _, e) => collect_const_names_rec(e, acc),
        _ => {}
    }
}
/// Collect all `FVar` ids referenced in an expression.
#[allow(dead_code)]
pub fn collect_fvars(expr: &Expr) -> Vec<FVarId> {
    let mut fvars = Vec::new();
    collect_fvars_rec(expr, &mut fvars);
    fvars
}
fn collect_fvars_rec(expr: &Expr, acc: &mut Vec<FVarId>) {
    match expr {
        Expr::FVar(id) => acc.push(*id),
        Expr::App(f, a) => {
            collect_fvars_rec(f, acc);
            collect_fvars_rec(a, acc);
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            collect_fvars_rec(dom, acc);
            collect_fvars_rec(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_rec(ty, acc);
            collect_fvars_rec(val, acc);
            collect_fvars_rec(body, acc);
        }
        Expr::Proj(_, _, e) => collect_fvars_rec(e, acc),
        _ => {}
    }
}
/// Return the "head" of an expression (strip App arguments).
#[allow(dead_code)]
pub fn expr_head(expr: &Expr) -> &Expr {
    match expr {
        Expr::App(f, _) => expr_head(f),
        _ => expr,
    }
}
/// Check whether an expression is an `App` whose head is `Const(name)`.
#[allow(dead_code)]
pub fn is_app_of(expr: &Expr, name: &Name) -> bool {
    matches!(expr_head(expr), Expr::Const(n, _) if n == name)
}
/// Return the maximum de Bruijn index that appears free (not under enough binders).
///
/// Returns `None` if no `BVar` nodes occur.
#[allow(dead_code)]
pub fn max_bvar_index(expr: &Expr) -> Option<u32> {
    max_bvar_index_at(expr, 0)
}
fn max_bvar_index_at(expr: &Expr, depth: u32) -> Option<u32> {
    match expr {
        Expr::BVar(i) => {
            if *i >= depth {
                Some(*i - depth)
            } else {
                None
            }
        }
        Expr::App(f, a) => {
            let l = max_bvar_index_at(f, depth);
            let r = max_bvar_index_at(a, depth);
            match (l, r) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (x, None) | (None, x) => x,
            }
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            let l = max_bvar_index_at(dom, depth);
            let r = max_bvar_index_at(body, depth + 1);
            match (l, r) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (x, None) | (None, x) => x,
            }
        }
        Expr::Let(_, ty, val, body) => {
            let t = max_bvar_index_at(ty, depth);
            let v = max_bvar_index_at(val, depth);
            let b = max_bvar_index_at(body, depth + 1);
            [t, v, b].into_iter().flatten().reduce(|a, b| a.max(b))
        }
        Expr::Proj(_, _, e) => max_bvar_index_at(e, depth),
        _ => None,
    }
}
#[cfg(test)]
mod kernel_util_tests {
    use super::*;
    #[test]
    fn test_kernel_version_parses() {
        let (major, minor, patch) = kernel_version();
        let _ = (major, minor, patch);
    }
    #[test]
    fn test_mk_prop() {
        assert!(matches!(mk_prop(), Expr::Sort(l) if l == Level::zero()));
    }
    #[test]
    fn test_mk_type0() {
        assert!(matches!(mk_type0(), Expr::Sort(l) if l == Level::succ(Level::zero())));
    }
    #[test]
    fn test_mk_nat_lit() {
        assert!(matches!(mk_nat_lit(42), Expr::Lit(Literal::Nat(_))));
    }
    #[test]
    fn test_mk_string_lit() {
        assert!(matches!(mk_string_lit("hello"), Expr::Lit(Literal::Str(_))));
    }
    #[test]
    fn test_mk_app_spine_empty() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let result = mk_app_spine(f.clone(), vec![]);
        assert_eq!(result, f);
    }
    #[test]
    fn test_mk_app_spine() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let result = mk_app_spine(f, vec![a, b]);
        assert!(matches!(result, Expr::App(_, _)));
        assert_eq!(app_arity(&result), 2);
    }
    #[test]
    fn test_unfold_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let e = mk_app_spine(f, vec![a, b]);
        let (head, args) = unfold_app(&e);
        assert!(matches!(head, Expr::Const(n, _) if n == & Name::str("f")));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_is_closed_bvar_0() {
        let e = Expr::BVar(0);
        assert!(!is_closed(&e));
    }
    #[test]
    fn test_is_closed_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(is_closed(&e));
    }
    #[test]
    fn test_is_ground_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(is_ground(&e));
    }
    #[test]
    fn test_is_ground_fvar() {
        let e = Expr::FVar(FVarId(0));
        assert!(!is_ground(&e));
    }
    #[test]
    fn test_expr_size_atom() {
        assert_eq!(expr_size(&Expr::BVar(0)), 1);
        assert_eq!(expr_size(&Expr::Sort(Level::zero())), 1);
    }
    #[test]
    fn test_expr_size_app() {
        let e = mk_app(Expr::BVar(0), Expr::BVar(1));
        assert_eq!(expr_size(&e), 3);
    }
    #[test]
    fn test_has_metavars_false() {
        let e = Expr::Const(Name::str("f"), vec![]);
        assert!(!has_metavars(&e));
    }
    #[test]
    fn test_has_metavars_true() {
        let e = Expr::FVar(FVarId(0));
        assert!(!has_metavars(&e));
    }
    #[test]
    fn test_collect_const_names() {
        let e = mk_app(
            Expr::Const(Name::str("f"), vec![]),
            Expr::Const(Name::str("a"), vec![]),
        );
        let names = collect_const_names(&e);
        assert!(names.contains(&Name::str("f")));
        assert!(names.contains(&Name::str("a")));
    }
    #[test]
    fn test_is_app_of() {
        let e = mk_app(
            Expr::Const(Name::str("List"), vec![]),
            Expr::Const(Name::str("Nat"), vec![]),
        );
        assert!(is_app_of(&e, &Name::str("List")));
        assert!(!is_app_of(&e, &Name::str("Nat")));
    }
    #[test]
    fn test_count_pi_binders() {
        let p = mk_pi(Name::str("x"), mk_prop(), mk_prop());
        assert_eq!(count_pi_binders(&p), 1);
    }
    #[test]
    fn test_count_lam_binders() {
        let l = mk_lam(Name::str("x"), mk_prop(), Expr::BVar(0));
        assert_eq!(count_lam_binders(&l), 1);
    }
}
/// Returns true if an expression contains any `Let` binders.
#[allow(dead_code)]
pub fn has_let_binders(expr: &Expr) -> bool {
    match expr {
        Expr::Let(_, ty, val, body) => {
            let _ = (
                has_let_binders(ty),
                has_let_binders(val),
                has_let_binders(body),
            );
            true
        }
        Expr::App(f, a) => has_let_binders(f) || has_let_binders(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_let_binders(ty) || has_let_binders(body)
        }
        Expr::Proj(_, _, e) => has_let_binders(e),
        _ => false,
    }
}
/// Returns true if an expression contains any projections.
#[allow(dead_code)]
pub fn has_projections(expr: &Expr) -> bool {
    match expr {
        Expr::Proj(_, _, _) => true,
        Expr::App(f, a) => has_projections(f) || has_projections(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_projections(ty) || has_projections(body)
        }
        Expr::Let(_, ty, val, body) => {
            has_projections(ty) || has_projections(val) || has_projections(body)
        }
        _ => false,
    }
}
/// Count the number of `App` nodes in an expression.
#[allow(dead_code)]
pub fn count_apps(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + count_apps(f) + count_apps(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => count_apps(ty) + count_apps(body),
        Expr::Let(_, ty, val, body) => count_apps(ty) + count_apps(val) + count_apps(body),
        Expr::Proj(_, _, e) => count_apps(e),
        _ => 0,
    }
}
/// Count all sort occurrences in an expression.
#[allow(dead_code)]
pub fn count_sorts(expr: &Expr) -> usize {
    match expr {
        Expr::Sort(_) => 1,
        Expr::App(f, a) => count_sorts(f) + count_sorts(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => count_sorts(ty) + count_sorts(body),
        Expr::Let(_, ty, val, body) => count_sorts(ty) + count_sorts(val) + count_sorts(body),
        Expr::Proj(_, _, e) => count_sorts(e),
        _ => 0,
    }
}
/// Check whether an expression is a literal.
#[allow(dead_code)]
pub fn is_literal(expr: &Expr) -> bool {
    matches!(expr, Expr::Lit(_))
}
/// Check whether an expression is a sort.
#[allow(dead_code)]
pub fn is_sort(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(_))
}
/// Check whether an expression is a Pi-type (possibly nested).
#[allow(dead_code)]
pub fn is_pi(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(_, _, _, _))
}
/// Check whether an expression is a lambda.
#[allow(dead_code)]
pub fn is_lam(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(_, _, _, _))
}
/// Check whether an expression is an application.
#[allow(dead_code)]
pub fn is_app(expr: &Expr) -> bool {
    matches!(expr, Expr::App(_, _))
}
/// Check whether an expression is a constant.
#[allow(dead_code)]
pub fn is_const(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(_, _))
}
/// Get the name of a constant, or None if not a constant.
#[allow(dead_code)]
pub fn const_name(expr: &Expr) -> Option<&Name> {
    match expr {
        Expr::Const(n, _) => Some(n),
        _ => None,
    }
}
/// Strip outer Pi binders, collecting binder info.
///
/// Returns `(binders, inner_type)` where `binders` is a list of `(BinderInfo, Name, domain_type)`.
#[allow(dead_code)]
pub fn strip_pi_binders(expr: &Expr) -> (Vec<(BinderInfo, Name, Expr)>, &Expr) {
    let mut binders = Vec::new();
    let mut current = expr;
    while let Expr::Pi(bi, n, ty, body) = current {
        binders.push((*bi, n.clone(), ty.as_ref().clone()));
        current = body;
    }
    (binders, current)
}
/// Strip outer lambda binders, collecting binder info.
///
/// Returns `(binders, body)` where `binders` is a list of `(BinderInfo, Name, domain_type)`.
#[allow(dead_code)]
pub fn strip_lam_binders(expr: &Expr) -> (Vec<(BinderInfo, Name, Expr)>, &Expr) {
    let mut binders = Vec::new();
    let mut current = expr;
    while let Expr::Lam(bi, n, ty, body) = current {
        binders.push((*bi, n.clone(), ty.as_ref().clone()));
        current = body;
    }
    (binders, current)
}
/// Build a Pi type from a list of binders and an inner type.
#[allow(dead_code)]
pub fn build_pi_from_binders(binders: &[(BinderInfo, Name, Expr)], inner: Expr) -> Expr {
    binders.iter().rev().fold(inner, |acc, (bi, n, ty)| {
        Expr::Pi(*bi, n.clone(), Box::new(ty.clone()), Box::new(acc))
    })
}
/// Build a lambda from a list of binders and a body.
#[allow(dead_code)]
pub fn build_lam_from_binders(binders: &[(BinderInfo, Name, Expr)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (bi, n, ty)| {
        Expr::Lam(*bi, n.clone(), Box::new(ty.clone()), Box::new(acc))
    })
}
/// Replace all occurrences of a constant by another expression.
///
/// Traverses the expression and substitutes `replacement` for every
/// `Const(name, _)` node.
#[allow(dead_code)]
pub fn replace_const(expr: &Expr, name: &Name, replacement: &Expr) -> Expr {
    match expr {
        Expr::Const(n, _) if n == name => replacement.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(replace_const(f, name, replacement)),
            Box::new(replace_const(a, name, replacement)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(replace_const(ty, name, replacement)),
            Box::new(replace_const(body, name, replacement)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(replace_const(ty, name, replacement)),
            Box::new(replace_const(body, name, replacement)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(replace_const(ty, name, replacement)),
            Box::new(replace_const(val, name, replacement)),
            Box::new(replace_const(body, name, replacement)),
        ),
        Expr::Proj(n, i, s) => {
            Expr::Proj(n.clone(), *i, Box::new(replace_const(s, name, replacement)))
        }
        e => e.clone(),
    }
}
/// Check if an expression is eta-reducible at the top level.
///
/// An expression `λ x. f x` is eta-reducible to `f` when `x` does not occur free in `f`.
#[allow(dead_code)]
pub fn is_eta_reducible(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(_, _, _, body) => {
            if let Expr::App(f, a) = body.as_ref() {
                if matches!(a.as_ref(), Expr::BVar(0)) {
                    return !contains_bvar(f, 0, 0);
                }
            }
            false
        }
        _ => false,
    }
}
/// Check if `BVar(idx + depth)` occurs in `expr` at the given depth.
#[allow(dead_code)]
pub fn contains_bvar(expr: &Expr, idx: u32, depth: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i == idx + depth,
        Expr::App(f, a) => contains_bvar(f, idx, depth) || contains_bvar(a, idx, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_bvar(ty, idx, depth) || contains_bvar(body, idx, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            contains_bvar(ty, idx, depth)
                || contains_bvar(val, idx, depth)
                || contains_bvar(body, idx, depth + 1)
        }
        Expr::Proj(_, _, s) => contains_bvar(s, idx, depth),
        _ => false,
    }
}
/// Check if two expressions are syntactically equal (no alpha equivalence, just `==`).
#[allow(dead_code)]
pub fn syntactically_equal(e1: &Expr, e2: &Expr) -> bool {
    e1 == e2
}
/// Collect all literals occurring in an expression.
#[allow(dead_code)]
pub fn collect_literals(expr: &Expr) -> Vec<Literal> {
    let mut lits = Vec::new();
    collect_lits_rec(expr, &mut lits);
    lits
}
fn collect_lits_rec(expr: &Expr, acc: &mut Vec<Literal>) {
    match expr {
        Expr::Lit(l) => acc.push(l.clone()),
        Expr::App(f, a) => {
            collect_lits_rec(f, acc);
            collect_lits_rec(a, acc);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_lits_rec(ty, acc);
            collect_lits_rec(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_lits_rec(ty, acc);
            collect_lits_rec(val, acc);
            collect_lits_rec(body, acc);
        }
        Expr::Proj(_, _, e) => collect_lits_rec(e, acc),
        _ => {}
    }
}
/// Return the depth of the deepest nested binder.
#[allow(dead_code)]
pub fn max_binder_depth(expr: &Expr) -> u32 {
    max_binder_depth_impl(expr, 0)
}
fn max_binder_depth_impl(expr: &Expr, depth: u32) -> u32 {
    match expr {
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            let ty_d = max_binder_depth_impl(ty, depth);
            let body_d = max_binder_depth_impl(body, depth + 1);
            ty_d.max(body_d).max(depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            let ty_d = max_binder_depth_impl(ty, depth);
            let val_d = max_binder_depth_impl(val, depth);
            let body_d = max_binder_depth_impl(body, depth + 1);
            ty_d.max(val_d).max(body_d)
        }
        Expr::App(f, a) => max_binder_depth_impl(f, depth).max(max_binder_depth_impl(a, depth)),
        Expr::Proj(_, _, e) => max_binder_depth_impl(e, depth),
        _ => depth,
    }
}
#[cfg(test)]
mod kernel_extra_tests {
    use super::*;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn prop() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_is_literal_true() {
        assert!(is_literal(&Expr::Lit(Literal::Nat(42))));
    }
    #[test]
    fn test_is_literal_false() {
        assert!(!is_literal(&nat()));
    }
    #[test]
    fn test_is_sort() {
        assert!(is_sort(&prop()));
        assert!(!is_sort(&nat()));
    }
    #[test]
    fn test_is_pi() {
        let p = mk_pi(Name::str("x"), prop(), prop());
        assert!(is_pi(&p));
        assert!(!is_pi(&nat()));
    }
    #[test]
    fn test_is_lam() {
        let l = mk_lam(Name::str("x"), prop(), Expr::BVar(0));
        assert!(is_lam(&l));
        assert!(!is_lam(&nat()));
    }
    #[test]
    fn test_is_app() {
        let e = mk_app(nat(), nat());
        assert!(is_app(&e));
        assert!(!is_app(&nat()));
    }
    #[test]
    fn test_is_const() {
        assert!(is_const(&nat()));
        assert!(!is_const(&Expr::BVar(0)));
    }
    #[test]
    fn test_const_name() {
        assert_eq!(const_name(&nat()), Some(&Name::str("Nat")));
        assert!(const_name(&Expr::BVar(0)).is_none());
    }
    #[test]
    fn test_strip_pi_binders_none() {
        let nat_expr = nat();
        let (binders, inner) = strip_pi_binders(&nat_expr);
        assert!(binders.is_empty());
        assert_eq!(*inner, nat());
    }
    #[test]
    fn test_strip_pi_binders_one() {
        let p = mk_pi(Name::str("x"), prop(), prop());
        let (binders, _inner) = strip_pi_binders(&p);
        assert_eq!(binders.len(), 1);
    }
    #[test]
    fn test_strip_lam_binders_one() {
        let l = mk_lam(Name::str("x"), prop(), Expr::BVar(0));
        let (binders, _body) = strip_lam_binders(&l);
        assert_eq!(binders.len(), 1);
    }
    #[test]
    fn test_build_pi_from_binders() {
        let binders = vec![(BinderInfo::Default, Name::str("x"), prop())];
        let ty = build_pi_from_binders(&binders, prop());
        assert!(is_pi(&ty));
    }
    #[test]
    fn test_build_lam_from_binders() {
        let binders = vec![(BinderInfo::Default, Name::str("x"), prop())];
        let l = build_lam_from_binders(&binders, Expr::BVar(0));
        assert!(is_lam(&l));
    }
    #[test]
    fn test_replace_const() {
        let e = nat();
        let result = replace_const(&e, &Name::str("Nat"), &prop());
        assert_eq!(result, prop());
    }
    #[test]
    fn test_replace_const_in_app() {
        let e = mk_app(nat(), nat());
        let result = replace_const(&e, &Name::str("Nat"), &prop());
        if let Expr::App(f, a) = &result {
            assert_eq!(**f, prop());
            assert_eq!(**a, prop());
        }
    }
    #[test]
    fn test_count_apps_zero() {
        assert_eq!(count_apps(&nat()), 0);
    }
    #[test]
    fn test_count_apps_one() {
        let e = mk_app(nat(), nat());
        assert_eq!(count_apps(&e), 1);
    }
    #[test]
    fn test_count_sorts_one() {
        assert_eq!(count_sorts(&prop()), 1);
    }
    #[test]
    fn test_count_sorts_zero() {
        assert_eq!(count_sorts(&nat()), 0);
    }
    #[test]
    fn test_contains_bvar_true() {
        assert!(contains_bvar(&Expr::BVar(0), 0, 0));
    }
    #[test]
    fn test_contains_bvar_false() {
        assert!(!contains_bvar(&Expr::BVar(1), 0, 0));
    }
    #[test]
    fn test_syntactically_equal() {
        assert!(syntactically_equal(&nat(), &nat()));
        assert!(!syntactically_equal(&nat(), &prop()));
    }
    #[test]
    fn test_collect_literals() {
        let e = mk_app(Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2)));
        let lits = collect_literals(&e);
        assert_eq!(lits.len(), 2);
    }
    #[test]
    fn test_max_binder_depth_zero() {
        assert_eq!(max_binder_depth(&nat()), 0);
    }
    #[test]
    fn test_max_binder_depth_one() {
        let l = mk_lam(Name::str("x"), prop(), Expr::BVar(0));
        assert_eq!(max_binder_depth(&l), 1);
    }
    #[test]
    fn test_is_eta_reducible_false() {
        assert!(!is_eta_reducible(&nat()));
        let not_eta = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(prop()),
            Box::new(Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(0)))),
        );
        assert!(!is_eta_reducible(&not_eta));
    }
    #[test]
    fn test_has_let_binders_false() {
        assert!(!has_let_binders(&nat()));
        assert!(!has_let_binders(&mk_pi(Name::str("x"), prop(), prop())));
    }
    #[test]
    fn test_has_projections_false() {
        assert!(!has_projections(&nat()));
        assert!(!has_projections(&Expr::BVar(0)));
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
