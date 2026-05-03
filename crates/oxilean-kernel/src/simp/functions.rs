//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, Literal, Name};

use super::types::{
    ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet, MinHeap,
    NonEmptyVec, PathBuf, PrefixCounter, RewriteRule, RewriteRuleSet, SimpDirection, SimpLemma,
    SimpLemmaSet, SimpResult, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary,
    Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure, VersionedRecord,
    WindowIterator, WriteOnce,
};

/// Simplify an expression using basic rewrite rules.
pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, a) => {
            let f_simp = simplify(f);
            let a_simp = simplify(a);
            if let Expr::Const(name, _) = &f_simp {
                if let Some(result) = try_simplify_nat_op(name, &a_simp) {
                    return result;
                }
            }
            Expr::App(Box::new(f_simp), Box::new(a_simp))
        }
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(simplify(ty)),
            Box::new(simplify(body)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(simplify(ty)),
            Box::new(simplify(body)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(simplify(ty)),
            Box::new(simplify(val)),
            Box::new(simplify(body)),
        ),
        _ => expr.clone(),
    }
}
fn try_simplify_nat_op(name: &Name, arg: &Expr) -> Option<Expr> {
    match name.to_string().as_str() {
        "Nat.succ" => {
            if let Expr::Lit(Literal::Nat(n)) = arg {
                Some(Expr::Lit(Literal::Nat(n + 1)))
            } else {
                None
            }
        }
        _ => None,
    }
}
/// Normalize an expression (reduce to normal form).
pub fn normalize(expr: &Expr) -> Expr {
    use crate::Reducer;
    let mut reducer = Reducer::new();
    let whnf = reducer.whnf(expr);
    match &whnf {
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(normalize(ty)),
            Box::new(normalize(body)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(normalize(ty)),
            Box::new(normalize(body)),
        ),
        Expr::App(f, a) => Expr::App(Box::new(normalize(f)), Box::new(normalize(a))),
        _ => whnf,
    }
}
/// Check if two expressions are alpha-equivalent (structurally equal).
pub fn alpha_eq(e1: &Expr, e2: &Expr) -> bool {
    match (e1, e2) {
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
        (Expr::FVar(f1), Expr::FVar(f2)) => f1 == f2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => alpha_eq(f1, f2) && alpha_eq(a1, a2),
        (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2)) => {
            alpha_eq(ty1, ty2) && alpha_eq(b1, b2)
        }
        (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
            alpha_eq(ty1, ty2) && alpha_eq(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            alpha_eq(ty1, ty2) && alpha_eq(v1, v2) && alpha_eq(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            n1 == n2 && i1 == i2 && alpha_eq(e1, e2)
        }
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal, Name};
    #[test]
    fn test_simplify_nat_succ() {
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(5))),
        );
        let result = simplify(&expr);
        assert_eq!(result, Expr::Lit(Literal::Nat(6)));
    }
    #[test]
    fn test_alpha_eq_bvar() {
        let e1 = Expr::BVar(0);
        let e2 = Expr::BVar(0);
        assert!(alpha_eq(&e1, &e2));
    }
    #[test]
    fn test_alpha_eq_lambda() {
        let lam1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let lam2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(alpha_eq(&lam1, &lam2));
    }
    #[test]
    fn test_alpha_eq_different() {
        let e1 = Expr::Lit(Literal::Nat(1));
        let e2 = Expr::Lit(Literal::Nat(2));
        assert!(!alpha_eq(&e1, &e2));
    }
}
/// Check if `expr` is headed by constant `head_name`.
pub fn is_app_of(expr: &Expr, head_name: &str) -> bool {
    match expr {
        Expr::App(f, _) => is_app_of(f, head_name),
        Expr::Const(n, _) => n.to_string() == head_name,
        _ => false,
    }
}
/// Collect the arguments of a spine application `f a1 a2 … an`.
///
/// Returns `(head, [a1, …, an])`.
pub fn decompose_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut current = expr;
    while let Expr::App(f, a) = current {
        args.push(a.as_ref());
        current = f;
    }
    args.reverse();
    (current, args)
}
/// Build an application spine from a head and argument list.
pub fn mk_app(head: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter()
        .fold(head, |f, a| Expr::App(Box::new(f), Box::new(a)))
}
/// Evaluate a closed Nat arithmetic expression to a literal, if possible.
pub fn eval_nat(expr: &Expr) -> Option<u64> {
    match expr {
        Expr::Lit(Literal::Nat(n)) => Some(*n),
        Expr::App(f, a) => {
            if let Expr::Const(name, _) = f.as_ref() {
                let name_str = name.to_string();
                let arg_val = eval_nat(a);
                match name_str.as_str() {
                    "Nat.succ" => return arg_val.map(|n| n + 1),
                    "Nat.pred" => return arg_val.map(|n| n.saturating_sub(1)),
                    "Nat.zero" => return Some(0),
                    _ => {}
                }
            }
            if let Expr::App(f2, a2) = f.as_ref() {
                if let Expr::Const(name, _) = f2.as_ref() {
                    let name_str = name.to_string();
                    let lhs = eval_nat(a2)?;
                    let rhs = eval_nat(a)?;
                    return match name_str.as_str() {
                        "Nat.add" => Some(lhs + rhs),
                        "Nat.mul" => Some(lhs * rhs),
                        "Nat.sub" => Some(lhs.saturating_sub(rhs)),
                        "Nat.div" => Some(lhs.checked_div(rhs).unwrap_or(0)),
                        "Nat.mod" => Some(lhs.checked_rem(rhs).unwrap_or(0)),
                        "Nat.pow" => Some(lhs.saturating_pow(rhs as u32)),
                        "Nat.min" => Some(lhs.min(rhs)),
                        "Nat.max" => Some(lhs.max(rhs)),
                        _ => None,
                    };
                }
            }
            None
        }
        _ => None,
    }
}
/// Apply `simplify` to all sub-expressions one level deep.
pub fn simp_congruence(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, a) => Expr::App(Box::new(simplify(f)), Box::new(simplify(a))),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(simplify(ty)),
            Box::new(simplify(body)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(simplify(ty)),
            Box::new(simplify(body)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(simplify(ty)),
            Box::new(simplify(val)),
            Box::new(simplify(body)),
        ),
        other => other.clone(),
    }
}
/// Simplify to a maximum depth (prevents looping on large terms).
pub fn simplify_bounded(expr: &Expr, max_depth: usize) -> Expr {
    if max_depth == 0 {
        return expr.clone();
    }
    let simp_sub = |e: &Expr| simplify_bounded(e, max_depth - 1);
    match expr {
        Expr::App(f, a) => {
            let f_s = simp_sub(f);
            let a_s = simp_sub(a);
            if let Expr::Const(name, _) = &f_s {
                if let Some(r) = try_simplify_nat_op(name, &a_s) {
                    return r;
                }
            }
            Expr::App(Box::new(f_s), Box::new(a_s))
        }
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(simp_sub(ty)),
            Box::new(simp_sub(body)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(simp_sub(ty)),
            Box::new(simp_sub(body)),
        ),
        other => other.clone(),
    }
}
/// Remove eta-redexes: `(fun x => f x)` → `f` (when `x` not free in `f`).
///
/// This is a best-effort, single-pass eta-reduction.
pub fn eta_reduce(expr: &Expr) -> Expr {
    match expr {
        Expr::Lam(_, _, _, body) => {
            if let Expr::App(f, a) = body.as_ref() {
                if matches!(a.as_ref(), Expr::BVar(0)) && !contains_bvar(f, 0) {
                    return shift_bvars(f, -1, 0);
                }
            }
            expr.clone()
        }
        _ => expr.clone(),
    }
}
/// Check if an expression contains the given de Bruijn variable.
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
/// Shift all de Bruijn variables `≥ cutoff` by `shift` (signed).
fn shift_bvars(expr: &Expr, shift: i32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                let new_i = (*i as i32 + shift).max(0) as u32;
                Expr::BVar(new_i)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(shift_bvars(f, shift, cutoff)),
            Box::new(shift_bvars(a, shift, cutoff)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(shift_bvars(ty, shift, cutoff)),
            Box::new(shift_bvars(body, shift, cutoff + 1)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(shift_bvars(ty, shift, cutoff)),
            Box::new(shift_bvars(body, shift, cutoff + 1)),
        ),
        other => other.clone(),
    }
}
/// Return the AST size (number of nodes) of an expression.
pub fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + expr_size(ty) + expr_size(body),
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, inner) => 1 + expr_size(inner),
        _ => 1,
    }
}
/// Return the depth (longest path to leaf) of an expression.
pub fn expr_depth(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        Expr::Proj(_, _, inner) => 1 + expr_depth(inner),
        _ => 0,
    }
}
#[cfg(test)]
mod simp_extra_tests {
    use super::*;
    use crate::Name;
    #[test]
    fn test_is_app_of() {
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nat.add"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(1))),
            )),
            Box::new(Expr::Lit(Literal::Nat(2))),
        );
        assert!(is_app_of(&e, "Nat.add"));
        assert!(!is_app_of(&e, "Nat.mul"));
    }
    #[test]
    fn test_decompose_app() {
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("f"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(1))),
            )),
            Box::new(Expr::Lit(Literal::Nat(2))),
        );
        let (head, args) = decompose_app(&e);
        assert!(matches!(head, Expr::Const(_, _)));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_eval_nat_literal() {
        let e = Expr::Lit(Literal::Nat(42));
        assert_eq!(eval_nat(&e), Some(42));
    }
    #[test]
    fn test_eval_nat_succ() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(4))),
        );
        assert_eq!(eval_nat(&e), Some(5));
    }
    #[test]
    fn test_eval_nat_add() {
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nat.add"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(3))),
            )),
            Box::new(Expr::Lit(Literal::Nat(7))),
        );
        assert_eq!(eval_nat(&e), Some(10));
    }
    #[test]
    fn test_eval_nat_pred() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.pred"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(5))),
        );
        assert_eq!(eval_nat(&e), Some(4));
        let e_zero = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.pred"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(0))),
        );
        assert_eq!(eval_nat(&e_zero), Some(0));
    }
    #[test]
    fn test_eval_nat_pow() {
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nat.pow"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(2))),
            )),
            Box::new(Expr::Lit(Literal::Nat(3))),
        );
        assert_eq!(eval_nat(&e), Some(8));
    }
    #[test]
    fn test_simplify_nat_succ_chain() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
                Box::new(Expr::Lit(Literal::Nat(0))),
            )),
        );
        let result = simplify(&e);
        assert_eq!(result, Expr::Lit(Literal::Nat(2)));
    }
    #[test]
    fn test_contains_bvar() {
        assert!(contains_bvar(&Expr::BVar(0), 0));
        assert!(!contains_bvar(&Expr::BVar(1), 0));
        let app = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        assert!(contains_bvar(&app, 0));
        assert!(contains_bvar(&app, 1));
        assert!(!contains_bvar(&app, 2));
    }
    #[test]
    fn test_expr_size() {
        let lit = Expr::Lit(Literal::Nat(1));
        assert_eq!(expr_size(&lit), 1);
        let app = Expr::App(Box::new(lit.clone()), Box::new(lit.clone()));
        assert_eq!(expr_size(&app), 3);
    }
    #[test]
    fn test_expr_depth() {
        let lit = Expr::Lit(Literal::Nat(1));
        assert_eq!(expr_depth(&lit), 0);
        let app = Expr::App(Box::new(lit.clone()), Box::new(lit.clone()));
        assert_eq!(expr_depth(&app), 1);
    }
    #[test]
    fn test_simp_lemma_reversed() {
        let lhs = Expr::Lit(Literal::Nat(1));
        let rhs = Expr::Lit(Literal::Nat(2));
        let lemma = SimpLemma::forward(Name::str("test"), lhs.clone(), rhs.clone());
        assert_eq!(lemma.direction, SimpDirection::Forward);
        let rev = lemma.reversed();
        assert_eq!(rev.direction, SimpDirection::Backward);
    }
    #[test]
    fn test_mk_app() {
        let head = Expr::Const(Name::str("f"), vec![]);
        let args = vec![Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2))];
        let e = mk_app(head, args);
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_simplify_bounded_depth_zero() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(5))),
        );
        let result = simplify_bounded(&e, 0);
        assert_eq!(result, e);
    }
    #[test]
    fn test_alpha_eq_lit() {
        let e1 = Expr::Lit(Literal::Nat(42));
        let e2 = Expr::Lit(Literal::Nat(42));
        let e3 = Expr::Lit(Literal::Nat(43));
        assert!(alpha_eq(&e1, &e2));
        assert!(!alpha_eq(&e1, &e3));
    }
    #[test]
    fn test_eta_reduce_no_change() {
        let e = Expr::Lit(Literal::Nat(5));
        assert_eq!(eta_reduce(&e), e);
    }
}
/// Check whether `expr` is a Nat literal with value `n`.
pub fn is_nat_lit(expr: &Expr, n: u64) -> bool {
    matches!(expr, Expr::Lit(Literal::Nat(m)) if * m == n)
}
/// Check whether `expr` is `Nat.zero` or the literal `0`.
pub fn is_zero(expr: &Expr) -> bool {
    is_nat_lit(expr, 0) || matches!(expr, Expr::Const(name, _) if name.to_string() == "Nat.zero")
}
/// Check whether `expr` is `Nat.succ _`.
pub fn is_succ(expr: &Expr) -> bool {
    matches!(
        expr, Expr::App(f, _) if matches!(f.as_ref(), Expr::Const(name, _) if name
        .to_string() == "Nat.succ")
    )
}
/// Extract the predecessor from a `Nat.succ n` expression.
pub fn succ_of(expr: &Expr) -> Option<&Expr> {
    if let Expr::App(f, a) = expr {
        if matches!(f.as_ref(), Expr::Const(name, _) if name.to_string() == "Nat.succ") {
            return Some(a);
        }
    }
    None
}
/// Apply a single rewrite rule `lhs = rhs` throughout `expr`.
///
/// Returns `(new_expr, changed)`.
pub fn rewrite_once(expr: &Expr, lhs: &Expr, rhs: &Expr) -> (Expr, bool) {
    if alpha_eq(expr, lhs) {
        return (rhs.clone(), true);
    }
    let mut changed = false;
    let new_expr = match expr {
        Expr::App(f, a) => {
            let (new_f, c1) = rewrite_once(f, lhs, rhs);
            let (new_a, c2) = rewrite_once(a, lhs, rhs);
            changed = c1 || c2;
            Expr::App(Box::new(new_f), Box::new(new_a))
        }
        Expr::Lam(bi, name, ty, body) => {
            let (new_ty, c1) = rewrite_once(ty, lhs, rhs);
            let (new_body, c2) = rewrite_once(body, lhs, rhs);
            changed = c1 || c2;
            Expr::Lam(*bi, name.clone(), Box::new(new_ty), Box::new(new_body))
        }
        Expr::Pi(bi, name, ty, body) => {
            let (new_ty, c1) = rewrite_once(ty, lhs, rhs);
            let (new_body, c2) = rewrite_once(body, lhs, rhs);
            changed = c1 || c2;
            Expr::Pi(*bi, name.clone(), Box::new(new_ty), Box::new(new_body))
        }
        other => other.clone(),
    };
    (new_expr, changed)
}
/// Apply a rewrite rule exhaustively until no more rewrites are possible.
pub fn rewrite_all(expr: &Expr, lhs: &Expr, rhs: &Expr) -> Expr {
    let mut current = expr.clone();
    loop {
        let (new_expr, changed) = rewrite_once(&current, lhs, rhs);
        if !changed {
            return current;
        }
        current = new_expr;
    }
}
/// Collect all `FVarId`s appearing in `expr`.
pub fn free_vars(expr: &Expr) -> Vec<crate::FVarId> {
    let mut result = Vec::new();
    free_vars_rec(expr, &mut result);
    result.sort_by_key(|fv| fv.0);
    result.dedup_by_key(|fv| fv.0);
    result
}
fn free_vars_rec(expr: &Expr, out: &mut Vec<crate::FVarId>) {
    match expr {
        Expr::FVar(fv) => out.push(*fv),
        Expr::App(f, a) => {
            free_vars_rec(f, out);
            free_vars_rec(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            free_vars_rec(ty, out);
            free_vars_rec(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            free_vars_rec(ty, out);
            free_vars_rec(val, out);
            free_vars_rec(body, out);
        }
        Expr::Proj(_, _, inner) => free_vars_rec(inner, out),
        _ => {}
    }
}
/// Return `true` if `expr` contains no free variables.
pub fn is_closed(expr: &Expr) -> bool {
    free_vars(expr).is_empty()
}
#[cfg(test)]
mod simp_extra_tests2 {
    use super::*;
    use crate::{FVarId, Name};
    #[test]
    fn test_is_nat_lit() {
        assert!(is_nat_lit(&Expr::Lit(Literal::Nat(5)), 5));
        assert!(!is_nat_lit(&Expr::Lit(Literal::Nat(5)), 6));
    }
    #[test]
    fn test_is_zero() {
        assert!(is_zero(&Expr::Lit(Literal::Nat(0))));
        assert!(is_zero(&Expr::Const(Name::str("Nat.zero"), vec![])));
        assert!(!is_zero(&Expr::Lit(Literal::Nat(1))));
    }
    #[test]
    fn test_is_succ_and_succ_of() {
        let succ_n = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(Expr::Lit(Literal::Nat(3))),
        );
        assert!(is_succ(&succ_n));
        assert!(succ_of(&succ_n).is_some());
        assert!(!is_succ(&Expr::Lit(Literal::Nat(5))));
        assert!(succ_of(&Expr::Lit(Literal::Nat(5))).is_none());
    }
    #[test]
    fn test_rewrite_once() {
        let lhs = Expr::Lit(Literal::Nat(1));
        let rhs = Expr::Lit(Literal::Nat(99));
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(lhs.clone()),
        );
        let (new_expr, changed) = rewrite_once(&expr, &lhs, &rhs);
        assert!(changed);
        if let Expr::App(_, a) = &new_expr {
            assert_eq!(a.as_ref(), &rhs);
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_rewrite_all() {
        let lhs = Expr::Lit(Literal::Nat(0));
        let rhs = Expr::Lit(Literal::Nat(100));
        let expr = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("f"), vec![])),
                Box::new(lhs.clone()),
            )),
            Box::new(lhs.clone()),
        );
        let result = rewrite_all(&expr, &lhs, &rhs);
        let size = expr_size(&result);
        assert!(size >= 3);
    }
    #[test]
    fn test_free_vars() {
        let expr = Expr::FVar(FVarId(42));
        let fvs = free_vars(&expr);
        assert_eq!(fvs.len(), 1);
        assert_eq!(fvs[0].0, 42);
    }
    #[test]
    fn test_is_closed() {
        let lit = Expr::Lit(Literal::Nat(5));
        assert!(is_closed(&lit));
        let fvar = Expr::FVar(FVarId(1));
        assert!(!is_closed(&fvar));
    }
    #[test]
    fn test_decompose_app_single() {
        let e = Expr::Const(Name::str("x"), vec![]);
        let (head, args) = decompose_app(&e);
        assert!(matches!(head, Expr::Const(_, _)));
        assert!(args.is_empty());
    }
}
/// Apply a `SimpLemmaSet` and record which lemmas were used.
pub fn simp_with_trace(expr: &Expr, set: &SimpLemmaSet) -> SimpResult {
    let mut current = expr.clone();
    let mut applied = Vec::new();
    let mut any_changed = false;
    loop {
        let mut pass_changed = false;
        for lemma in set.iter() {
            let (new_expr, c) =
                rewrite_once(&current, lemma.effective_lhs(), lemma.effective_rhs());
            if c {
                current = new_expr;
                applied.push(lemma.name.clone());
                pass_changed = true;
                any_changed = true;
            }
        }
        if !pass_changed {
            break;
        }
    }
    if any_changed {
        SimpResult::simplified(current, applied)
    } else {
        SimpResult::unchanged(current)
    }
}
/// Check if `expr` is the constant `True`.
pub fn is_true(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if name.to_string() == "True")
}
/// Check if `expr` is the constant `False`.
pub fn is_false_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(name, _) if name.to_string() == "False")
}
/// Check if `expr` is `Prop` (Sort 0).
pub fn is_prop(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(l) if l.is_zero())
}
/// Fold constant Nat arithmetic in an expression (recursive).
///
/// Eagerly evaluates `Nat.add`, `Nat.mul`, `Nat.sub`, `Nat.succ`
/// wherever both operands are literals.
pub fn fold_nat_constants(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, a) => {
            let f2 = fold_nat_constants(f);
            let a2 = fold_nat_constants(a);
            if let Some(n) = eval_nat(&Expr::App(Box::new(f2.clone()), Box::new(a2.clone()))) {
                Expr::Lit(Literal::Nat(n))
            } else {
                Expr::App(Box::new(f2), Box::new(a2))
            }
        }
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(fold_nat_constants(ty)),
            Box::new(fold_nat_constants(body)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(fold_nat_constants(ty)),
            Box::new(fold_nat_constants(body)),
        ),
        other => other.clone(),
    }
}
#[cfg(test)]
mod simp_set_tests {
    use super::*;
    use crate::Name;
    fn mk_lit(n: u64) -> Expr {
        Expr::Lit(Literal::Nat(n))
    }
    fn mk_c(s: &str) -> Expr {
        Expr::Const(Name::str(s), vec![])
    }
    #[test]
    fn test_simp_lemma_set_empty() {
        let set = SimpLemmaSet::new();
        assert!(set.is_empty());
        let (result, changed) = set.apply_once(&mk_lit(1));
        assert!(!changed);
        assert_eq!(result, mk_lit(1));
    }
    #[test]
    fn test_simp_lemma_set_apply_once() {
        let mut set = SimpLemmaSet::new();
        let lhs = mk_lit(0);
        let rhs = mk_lit(100);
        set.add(SimpLemma::forward(
            Name::str("zero_to_hundred"),
            lhs.clone(),
            rhs.clone(),
        ));
        let (result, changed) = set.apply_once(&lhs);
        assert!(changed);
        assert_eq!(result, rhs);
    }
    #[test]
    fn test_simp_lemma_set_apply_all() {
        let mut set = SimpLemmaSet::new();
        let a = mk_lit(1);
        let b = mk_lit(2);
        let c = mk_lit(3);
        set.add(SimpLemma::forward(Name::str("r1"), a.clone(), b.clone()));
        set.add(SimpLemma::forward(Name::str("r2"), b.clone(), c.clone()));
        let result = set.apply_all(&a);
        assert_eq!(result, c);
    }
    #[test]
    fn test_simp_lemma_set_find() {
        let mut set = SimpLemmaSet::new();
        let lemma = SimpLemma::forward(Name::str("myRule"), mk_lit(0), mk_lit(1));
        set.add(lemma);
        assert!(set.find(&Name::str("myRule")).is_some());
        assert!(set.find(&Name::str("unknownRule")).is_none());
    }
    #[test]
    fn test_simp_with_trace_no_change() {
        let set = SimpLemmaSet::new();
        let e = mk_lit(5);
        let result = simp_with_trace(&e, &set);
        assert!(!result.changed);
        assert_eq!(result.expr, e);
    }
    #[test]
    fn test_simp_with_trace_change() {
        let mut set = SimpLemmaSet::new();
        let lhs = mk_lit(0);
        let rhs = mk_lit(99);
        set.add(SimpLemma::forward(Name::str("r"), lhs.clone(), rhs.clone()));
        let result = simp_with_trace(&lhs, &set);
        assert!(result.changed);
        assert_eq!(result.expr, rhs);
        assert_eq!(result.applied.len(), 1);
    }
    #[test]
    fn test_is_true() {
        assert!(is_true(&mk_c("True")));
        assert!(!is_true(&mk_c("False")));
        assert!(!is_true(&mk_lit(1)));
    }
    #[test]
    fn test_is_false_expr() {
        assert!(is_false_expr(&mk_c("False")));
        assert!(!is_false_expr(&mk_c("True")));
    }
    #[test]
    fn test_is_prop() {
        use crate::Level;
        assert!(is_prop(&Expr::Sort(Level::zero())));
        assert!(!is_prop(&Expr::Sort(Level::succ(Level::zero()))));
    }
    #[test]
    fn test_fold_nat_constants_succ() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(mk_lit(3)),
        );
        let result = fold_nat_constants(&e);
        assert_eq!(result, mk_lit(4));
    }
    #[test]
    fn test_fold_nat_constants_no_change() {
        let e = mk_lit(7);
        let result = fold_nat_constants(&e);
        assert_eq!(result, e);
    }
    #[test]
    fn test_simp_result_unchanged() {
        let e = mk_lit(1);
        let r = SimpResult::unchanged(e.clone());
        assert!(!r.changed);
        assert_eq!(r.expr, e);
    }
    #[test]
    fn test_simp_result_simplified() {
        let e = mk_lit(2);
        let r = SimpResult::simplified(e.clone(), vec![Name::str("ruleX")]);
        assert!(r.changed);
        assert_eq!(r.applied.len(), 1);
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
