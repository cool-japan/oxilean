//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::Expr;
use crate::FVarId;

use super::types::{
    BitSet64, BucketCounter, ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution,
    FocusStack, LabelSet, MinHeap, NonEmptyVec, PathBuf, PrefixCounter, RewriteRule,
    RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch,
    StringPool, Substitution, TokenBucket, TransformStat, TransitiveClosure, VersionedRecord,
    WindowIterator, WriteOnce,
};

/// Substitute bound variable with an expression.
///
/// subst(e, k, s) replaces BVar(k) with s in e
pub fn subst(expr: &Expr, level: u32, sub: &Expr) -> Expr {
    match expr {
        Expr::BVar(idx) => {
            if *idx == level {
                sub.clone()
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(subst(ty, level, sub)),
            Box::new(subst(body, level + 1, sub)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(subst(ty, level, sub)),
            Box::new(subst(body, level + 1, sub)),
        ),
        Expr::App(f, a) => Expr::App(
            Box::new(subst(f, level, sub)),
            Box::new(subst(a, level, sub)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(subst(ty, level, sub)),
            Box::new(subst(val, level, sub)),
            Box::new(subst(body, level + 1, sub)),
        ),
        Expr::Proj(name, idx, e) => Expr::Proj(name.clone(), *idx, Box::new(subst(e, level, sub))),
    }
}
/// Instantiate bound variable with a free variable.
pub fn instantiate(expr: &Expr, level: u32, fvar: FVarId) -> Expr {
    subst(expr, level, &Expr::FVar(fvar))
}
/// Abstract a free variable into a bound variable.
pub fn abstract_fvar(expr: &Expr, fvar: FVarId, level: u32) -> Expr {
    match expr {
        Expr::FVar(v) if *v == fvar => Expr::BVar(level),
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(abstract_fvar(ty, fvar, level)),
            Box::new(abstract_fvar(body, fvar, level + 1)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(abstract_fvar(ty, fvar, level)),
            Box::new(abstract_fvar(body, fvar, level + 1)),
        ),
        Expr::App(f, a) => Expr::App(
            Box::new(abstract_fvar(f, fvar, level)),
            Box::new(abstract_fvar(a, fvar, level)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(abstract_fvar(ty, fvar, level)),
            Box::new(abstract_fvar(val, fvar, level)),
            Box::new(abstract_fvar(body, fvar, level + 1)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Box::new(abstract_fvar(e, fvar, level)))
        }
        Expr::FVar(_) => expr.clone(),
    }
}
/// Check if an expression contains a specific bound variable.
pub fn has_bvar(expr: &Expr, level: u32) -> bool {
    match expr {
        Expr::BVar(idx) => *idx == level,
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_bvar(ty, level) || has_bvar(body, level + 1)
        }
        Expr::App(f, a) => has_bvar(f, level) || has_bvar(a, level),
        Expr::Let(_, ty, val, body) => {
            has_bvar(ty, level) || has_bvar(val, level) || has_bvar(body, level + 1)
        }
        Expr::Proj(_, _, e) => has_bvar(e, level),
    }
}
/// Check if an expression contains a specific free variable.
pub fn has_fvar(expr: &Expr, fvar: FVarId) -> bool {
    match expr {
        Expr::FVar(v) => *v == fvar,
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_fvar(ty, fvar) || has_fvar(body, fvar)
        }
        Expr::App(f, a) => has_fvar(f, fvar) || has_fvar(a, fvar),
        Expr::Let(_, ty, val, body) => {
            has_fvar(ty, fvar) || has_fvar(val, fvar) || has_fvar(body, fvar)
        }
        Expr::Proj(_, _, e) => has_fvar(e, fvar),
    }
}
/// Lift (shift) all bound variable indices in `expr` by `amount` starting
/// from the given `cutoff`.
///
/// This is required when inserting an expression under binders: any free
/// reference (index ≥ cutoff) must be incremented to account for the
/// additional binders above.
pub fn lift(expr: &Expr, cutoff: u32, amount: u32) -> Expr {
    match expr {
        Expr::BVar(idx) => {
            if *idx >= cutoff {
                Expr::BVar(idx + amount)
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(lift(ty, cutoff, amount)),
            Box::new(lift(body, cutoff + 1, amount)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(lift(ty, cutoff, amount)),
            Box::new(lift(body, cutoff + 1, amount)),
        ),
        Expr::App(f, a) => Expr::App(
            Box::new(lift(f, cutoff, amount)),
            Box::new(lift(a, cutoff, amount)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(lift(ty, cutoff, amount)),
            Box::new(lift(val, cutoff, amount)),
            Box::new(lift(body, cutoff + 1, amount)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Box::new(lift(e, cutoff, amount)))
        }
    }
}
/// Lower (un-shift) all bound variable indices ≥ cutoff by `amount`.
///
/// This is the inverse of `lift`. Panics if any index would go below 0.
/// Returns `None` if lowering is not safe (index would underflow).
pub fn lower(expr: &Expr, cutoff: u32, amount: u32) -> Option<Expr> {
    match expr {
        Expr::BVar(idx) => {
            if *idx >= cutoff {
                if *idx < cutoff + amount {
                    None
                } else {
                    Some(Expr::BVar(idx - amount))
                }
            } else {
                Some(expr.clone())
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => Some(expr.clone()),
        Expr::Lam(info, name, ty, body) => {
            let ty2 = lower(ty, cutoff, amount)?;
            let body2 = lower(body, cutoff + 1, amount)?;
            Some(Expr::Lam(
                *info,
                name.clone(),
                Box::new(ty2),
                Box::new(body2),
            ))
        }
        Expr::Pi(info, name, ty, body) => {
            let ty2 = lower(ty, cutoff, amount)?;
            let body2 = lower(body, cutoff + 1, amount)?;
            Some(Expr::Pi(
                *info,
                name.clone(),
                Box::new(ty2),
                Box::new(body2),
            ))
        }
        Expr::App(f, a) => {
            let f2 = lower(f, cutoff, amount)?;
            let a2 = lower(a, cutoff, amount)?;
            Some(Expr::App(Box::new(f2), Box::new(a2)))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = lower(ty, cutoff, amount)?;
            let val2 = lower(val, cutoff, amount)?;
            let body2 = lower(body, cutoff + 1, amount)?;
            Some(Expr::Let(
                name.clone(),
                Box::new(ty2),
                Box::new(val2),
                Box::new(body2),
            ))
        }
        Expr::Proj(name, idx, e) => {
            let e2 = lower(e, cutoff, amount)?;
            Some(Expr::Proj(name.clone(), *idx, Box::new(e2)))
        }
    }
}
/// Apply a bulk substitution to an expression.
pub(super) fn apply_subst(expr: &Expr, subst: &Substitution, offset: u32) -> Expr {
    match expr {
        Expr::BVar(idx) => {
            if *idx >= offset {
                let rel = idx - offset;
                if let Some(replacement) = subst.get(rel) {
                    lift(replacement, 0, offset)
                } else {
                    expr.clone()
                }
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(apply_subst(ty, subst, offset)),
            Box::new(apply_subst(body, subst, offset + 1)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(apply_subst(ty, subst, offset)),
            Box::new(apply_subst(body, subst, offset + 1)),
        ),
        Expr::App(f, a) => Expr::App(
            Box::new(apply_subst(f, subst, offset)),
            Box::new(apply_subst(a, subst, offset)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(apply_subst(ty, subst, offset)),
            Box::new(apply_subst(val, subst, offset)),
            Box::new(apply_subst(body, subst, offset + 1)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Box::new(apply_subst(e, subst, offset)))
        }
    }
}
/// Collect all free variables (FVarId) occurring in `expr`.
pub fn collect_fvars(expr: &Expr) -> Vec<FVarId> {
    let mut result = Vec::new();
    collect_fvars_impl(expr, &mut result);
    result
}
pub(super) fn collect_fvars_impl(expr: &Expr, out: &mut Vec<FVarId>) {
    match expr {
        Expr::FVar(fv) => {
            if !out.contains(fv) {
                out.push(*fv);
            }
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => {}
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars_impl(ty, out);
            collect_fvars_impl(body, out);
        }
        Expr::App(f, a) => {
            collect_fvars_impl(f, out);
            collect_fvars_impl(a, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_impl(ty, out);
            collect_fvars_impl(val, out);
            collect_fvars_impl(body, out);
        }
        Expr::Proj(_, _, e) => collect_fvars_impl(e, out),
    }
}
/// Count the number of occurrences of BVar(level) in expr.
pub fn count_bvar(expr: &Expr, level: u32) -> usize {
    match expr {
        Expr::BVar(idx) => {
            if *idx == level {
                1
            } else {
                0
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_bvar(ty, level) + count_bvar(body, level + 1)
        }
        Expr::App(f, a) => count_bvar(f, level) + count_bvar(a, level),
        Expr::Let(_, ty, val, body) => {
            count_bvar(ty, level) + count_bvar(val, level) + count_bvar(body, level + 1)
        }
        Expr::Proj(_, _, e) => count_bvar(e, level),
    }
}
/// Simultaneously substitute multiple free variables with expressions.
///
/// More efficient than calling `abstract_fvar` + `subst` for each variable
/// individually.
pub fn subst_fvars(expr: &Expr, mapping: &[(FVarId, Expr)]) -> Expr {
    match expr {
        Expr::FVar(fv) => {
            if let Some((_, replacement)) = mapping.iter().find(|(id, _)| id == fv) {
                replacement.clone()
            } else {
                expr.clone()
            }
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(subst_fvars(ty, mapping)),
            Box::new(subst_fvars(body, mapping)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(subst_fvars(ty, mapping)),
            Box::new(subst_fvars(body, mapping)),
        ),
        Expr::App(f, a) => Expr::App(
            Box::new(subst_fvars(f, mapping)),
            Box::new(subst_fvars(a, mapping)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(subst_fvars(ty, mapping)),
            Box::new(subst_fvars(val, mapping)),
            Box::new(subst_fvars(body, mapping)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Box::new(subst_fvars(e, mapping)))
        }
    }
}
/// Compute the syntactic depth of an expression (longest path from root to leaf).
pub fn expr_depth(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_depth(ty).max(expr_depth(body))
        }
        Expr::App(f, a) => 1 + expr_depth(f).max(expr_depth(a)),
        Expr::Let(_, ty, val, body) => {
            1 + expr_depth(ty).max(expr_depth(val)).max(expr_depth(body))
        }
        Expr::Proj(_, _, e) => 1 + expr_depth(e),
    }
}
/// Compute the number of nodes in the expression tree.
pub fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => 1,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => 1 + expr_size(ty) + expr_size(body),
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, e) => 1 + expr_size(e),
    }
}
/// Abstract a list of free variables, turning them into nested lambda binders.
///
/// Given `fvars = [x1, x2, x3]` and their types `tys`, produces:
/// `fun (x1 : T1) (x2 : T2) (x3 : T3) => body`
pub fn close_with_lambdas(
    body: &Expr,
    fvars: &[(FVarId, crate::Name, Expr, crate::BinderInfo)],
) -> Expr {
    let mut result = body.clone();
    for (fvar, name, ty, binfo) in fvars.iter().rev() {
        let abstracted = abstract_fvar(&result, *fvar, 0);
        result = Expr::Lam(
            *binfo,
            name.clone(),
            Box::new(ty.clone()),
            Box::new(abstracted),
        );
    }
    result
}
/// Abstract a list of free variables, turning them into nested pi binders.
///
/// Given `fvars = [x1, x2, x3]` and their types, produces:
/// `(x1 : T1) → (x2 : T2) → (x3 : T3) → body`
pub fn close_with_pis(
    body: &Expr,
    fvars: &[(FVarId, crate::Name, Expr, crate::BinderInfo)],
) -> Expr {
    let mut result = body.clone();
    for (fvar, name, ty, binfo) in fvars.iter().rev() {
        let abstracted = abstract_fvar(&result, *fvar, 0);
        result = Expr::Pi(
            *binfo,
            name.clone(),
            Box::new(ty.clone()),
            Box::new(abstracted),
        );
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal, Name};
    #[test]
    fn test_subst_bvar() {
        let expr = Expr::BVar(0);
        let sub = Expr::Lit(Literal::Nat(42));
        let result = subst(&expr, 0, &sub);
        assert_eq!(result, sub);
    }
    #[test]
    fn test_subst_no_match() {
        let expr = Expr::BVar(1);
        let sub = Expr::Lit(Literal::Nat(42));
        let result = subst(&expr, 0, &sub);
        assert_eq!(result, expr);
    }
    #[test]
    fn test_subst_lambda() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let sub = Expr::Lit(Literal::Nat(42));
        let result = subst(&lam, 0, &sub);
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_instantiate() {
        let expr = Expr::BVar(0);
        let fvar = FVarId(123);
        let result = instantiate(&expr, 0, fvar);
        assert_eq!(result, Expr::FVar(fvar));
    }
    #[test]
    fn test_abstract_fvar() {
        let expr = Expr::FVar(FVarId(123));
        let result = abstract_fvar(&expr, FVarId(123), 0);
        assert_eq!(result, Expr::BVar(0));
    }
    #[test]
    fn test_has_bvar_true() {
        let expr = Expr::BVar(0);
        assert!(has_bvar(&expr, 0));
    }
    #[test]
    fn test_has_bvar_false() {
        let expr = Expr::Lit(Literal::Nat(42));
        assert!(!has_bvar(&expr, 0));
    }
    #[test]
    fn test_has_fvar_true() {
        let expr = Expr::FVar(FVarId(123));
        assert!(has_fvar(&expr, FVarId(123)));
    }
    #[test]
    fn test_has_fvar_false() {
        let expr = Expr::FVar(FVarId(123));
        assert!(!has_fvar(&expr, FVarId(456)));
    }
    #[test]
    fn test_lift_bvar_above_cutoff() {
        let expr = Expr::BVar(2);
        let lifted = lift(&expr, 1, 3);
        assert_eq!(lifted, Expr::BVar(5));
    }
    #[test]
    fn test_lift_bvar_below_cutoff() {
        let expr = Expr::BVar(0);
        let lifted = lift(&expr, 1, 5);
        assert_eq!(lifted, Expr::BVar(0));
    }
    #[test]
    fn test_lift_lambda() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(1)),
        );
        let lifted = lift(&lam, 0, 1);
        if let Expr::Lam(_, _, _, body) = &lifted {
            assert_eq!(**body, Expr::BVar(2));
        } else {
            panic!("Expected Lam");
        }
    }
    #[test]
    fn test_lower_safe() {
        let expr = Expr::BVar(5);
        let result = lower(&expr, 2, 2).expect("result should be present");
        assert_eq!(result, Expr::BVar(3));
    }
    #[test]
    fn test_lower_unsafe() {
        let expr = Expr::BVar(3);
        let result = lower(&expr, 2, 2);
        assert!(result.is_none());
    }
    #[test]
    fn test_lower_below_cutoff() {
        let expr = Expr::BVar(1);
        let result = lower(&expr, 2, 2).expect("result should be present");
        assert_eq!(result, Expr::BVar(1));
    }
    #[test]
    fn test_substitution_single() {
        let replacement = Expr::Lit(Literal::Nat(99));
        let s = Substitution::single(replacement.clone());
        assert_eq!(s.get(0), Some(&replacement));
        assert_eq!(s.get(1), None);
    }
    #[test]
    fn test_substitution_apply() {
        let mut s = Substitution::new();
        let v = Expr::Lit(Literal::Nat(7));
        s.add(0, v.clone());
        let expr = Expr::BVar(0);
        let result = s.apply(&expr);
        assert_eq!(result, v);
    }
    #[test]
    fn test_substitution_no_match() {
        let s = Substitution::new();
        let expr = Expr::BVar(0);
        let result = s.apply(&expr);
        assert_eq!(result, expr);
    }
    #[test]
    fn test_collect_fvars() {
        let fv1 = FVarId(1);
        let fv2 = FVarId(2);
        let expr = Expr::App(Box::new(Expr::FVar(fv1)), Box::new(Expr::FVar(fv2)));
        let fvars = collect_fvars(&expr);
        assert_eq!(fvars.len(), 2);
        assert!(fvars.contains(&fv1));
        assert!(fvars.contains(&fv2));
    }
    #[test]
    fn test_collect_fvars_dedup() {
        let fv = FVarId(42);
        let expr = Expr::App(Box::new(Expr::FVar(fv)), Box::new(Expr::FVar(fv)));
        let fvars = collect_fvars(&expr);
        assert_eq!(fvars.len(), 1);
    }
    #[test]
    fn test_count_bvar() {
        let expr = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(0)));
        assert_eq!(count_bvar(&expr, 0), 2);
        assert_eq!(count_bvar(&expr, 1), 0);
    }
    #[test]
    fn test_subst_fvars() {
        let fv1 = FVarId(10);
        let fv2 = FVarId(20);
        let expr = Expr::App(Box::new(Expr::FVar(fv1)), Box::new(Expr::FVar(fv2)));
        let r1 = Expr::Lit(Literal::Nat(1));
        let r2 = Expr::Lit(Literal::Nat(2));
        let result = subst_fvars(&expr, &[(fv1, r1.clone()), (fv2, r2.clone())]);
        assert_eq!(result, Expr::App(Box::new(r1), Box::new(r2)));
    }
    #[test]
    fn test_expr_depth() {
        let leaf = Expr::Lit(Literal::Nat(0));
        assert_eq!(expr_depth(&leaf), 0);
        let app = Expr::App(Box::new(leaf.clone()), Box::new(leaf.clone()));
        assert_eq!(expr_depth(&app), 1);
        let nested = Expr::App(Box::new(app.clone()), Box::new(app));
        assert_eq!(expr_depth(&nested), 2);
    }
    #[test]
    fn test_expr_size() {
        let leaf = Expr::Lit(Literal::Nat(0));
        assert_eq!(expr_size(&leaf), 1);
        let app = Expr::App(Box::new(leaf.clone()), Box::new(leaf.clone()));
        assert_eq!(expr_size(&app), 3);
    }
    #[test]
    fn test_close_with_lambdas() {
        let fvar = FVarId(1);
        let ty = Expr::Sort(Level::zero());
        let body = Expr::FVar(fvar);
        let closed = close_with_lambdas(
            &body,
            &[(fvar, Name::str("x"), ty.clone(), BinderInfo::Default)],
        );
        if let Expr::Lam(_, name, _, b) = &closed {
            assert_eq!(*name, Name::str("x"));
            assert_eq!(**b, Expr::BVar(0));
        } else {
            panic!("Expected Lam");
        }
    }
    #[test]
    fn test_close_with_pis() {
        let fvar = FVarId(5);
        let ty = Expr::Sort(Level::zero());
        let body = Expr::FVar(fvar);
        let closed = close_with_pis(
            &body,
            &[(fvar, Name::str("x"), ty.clone(), BinderInfo::Default)],
        );
        if let Expr::Pi(_, name, _, b) = &closed {
            assert_eq!(*name, Name::str("x"));
            assert_eq!(**b, Expr::BVar(0));
        } else {
            panic!("Expected Pi");
        }
    }
}
/// Check if an expression is closed (no free BVars at depth 0).
pub fn is_closed(expr: &Expr) -> bool {
    !has_bvar(expr, 0)
}
/// Replace all occurrences of a constant name with another expression.
pub fn subst_const(expr: &Expr, from: &crate::Name, to: &Expr) -> Expr {
    match expr {
        Expr::Const(n, _) if n == from => to.clone(),
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => {
            expr.clone()
        }
        Expr::App(f, a) => Expr::App(
            Box::new(subst_const(f, from, to)),
            Box::new(subst_const(a, from, to)),
        ),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(subst_const(ty, from, to)),
            Box::new(subst_const(body, from, to)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(subst_const(ty, from, to)),
            Box::new(subst_const(body, from, to)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(subst_const(ty, from, to)),
            Box::new(subst_const(val, from, to)),
            Box::new(subst_const(body, from, to)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Box::new(subst_const(e, from, to)))
        }
    }
}
/// Rename a bound variable (change binder name without affecting semantics).
///
/// This is a no-op on the expression structure but useful for pretty-printing.
pub fn rename_binder(expr: &Expr, old_name: &crate::Name, new_name: crate::Name) -> Expr {
    match expr {
        Expr::Lam(info, name, ty, body) if name == old_name => Expr::Lam(
            *info,
            new_name.clone(),
            Box::new(rename_binder(ty, old_name, new_name.clone())),
            Box::new(rename_binder(body, old_name, new_name)),
        ),
        Expr::Pi(info, name, ty, body) if name == old_name => Expr::Pi(
            *info,
            new_name.clone(),
            Box::new(rename_binder(ty, old_name, new_name.clone())),
            Box::new(rename_binder(body, old_name, new_name)),
        ),
        Expr::Lam(info, name, ty, body) => Expr::Lam(
            *info,
            name.clone(),
            Box::new(rename_binder(ty, old_name, new_name.clone())),
            Box::new(rename_binder(body, old_name, new_name)),
        ),
        Expr::Pi(info, name, ty, body) => Expr::Pi(
            *info,
            name.clone(),
            Box::new(rename_binder(ty, old_name, new_name.clone())),
            Box::new(rename_binder(body, old_name, new_name)),
        ),
        Expr::App(f, a) => Expr::App(
            Box::new(rename_binder(f, old_name, new_name.clone())),
            Box::new(rename_binder(a, old_name, new_name)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(rename_binder(ty, old_name, new_name.clone())),
            Box::new(rename_binder(val, old_name, new_name.clone())),
            Box::new(rename_binder(body, old_name, new_name)),
        ),
        Expr::Proj(name, idx, e) => Expr::Proj(
            name.clone(),
            *idx,
            Box::new(rename_binder(e, old_name, new_name)),
        ),
        _ => expr.clone(),
    }
}
/// Collect all constant names that appear in an expression (deduplicated).
pub fn collect_consts(expr: &Expr) -> Vec<crate::Name> {
    let mut result = Vec::new();
    collect_consts_impl(expr, &mut result);
    result
}
pub(super) fn collect_consts_impl(expr: &Expr, out: &mut Vec<crate::Name>) {
    match expr {
        Expr::Const(n, _) => {
            if !out.contains(n) {
                out.push(n.clone());
            }
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::FVar(_) | Expr::Lit(_) => {}
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_consts_impl(ty, out);
            collect_consts_impl(body, out);
        }
        Expr::App(f, a) => {
            collect_consts_impl(f, out);
            collect_consts_impl(a, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_consts_impl(ty, out);
            collect_consts_impl(val, out);
            collect_consts_impl(body, out);
        }
        Expr::Proj(_, _, e) => collect_consts_impl(e, out),
    }
}
/// Count occurrences of a free variable `fv` in an expression.
pub fn count_fvar(expr: &Expr, fv: FVarId) -> usize {
    match expr {
        Expr::FVar(v) => {
            if *v == fv {
                1
            } else {
                0
            }
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_fvar(ty, fv) + count_fvar(body, fv)
        }
        Expr::App(f, a) => count_fvar(f, fv) + count_fvar(a, fv),
        Expr::Let(_, ty, val, body) => {
            count_fvar(ty, fv) + count_fvar(val, fv) + count_fvar(body, fv)
        }
        Expr::Proj(_, _, e) => count_fvar(e, fv),
    }
}
/// Check whether `expr` is a closed term (no BVars and no FVars).
pub fn is_ground(expr: &Expr) -> bool {
    match expr {
        Expr::FVar(_) | Expr::BVar(_) => false,
        Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => true,
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => is_ground(ty) && is_ground(body),
        Expr::App(f, a) => is_ground(f) && is_ground(a),
        Expr::Let(_, ty, val, body) => is_ground(ty) && is_ground(val) && is_ground(body),
        Expr::Proj(_, _, e) => is_ground(e),
    }
}
/// Simultaneously lift all FVars from `mapping` by replacing them with BVars
/// starting at `start_level`.  The first entry in `mapping` maps to BVar(start_level),
/// the second to BVar(start_level+1), etc.
pub fn abstract_fvars_ordered(expr: &Expr, fvars: &[FVarId], start_level: u32) -> Expr {
    let mut result = expr.clone();
    for (i, &fv) in fvars.iter().enumerate() {
        let level = start_level + i as u32;
        result = abstract_fvar(&result, fv, level);
    }
    result
}
/// Check if two expressions are alpha-equivalent (identical up to bound variable
/// renaming).  This is a simple structural check — it does NOT reduce.
pub fn alpha_eq(e1: &Expr, e2: &Expr) -> bool {
    match (e1, e2) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(a), Expr::FVar(b)) => a == b,
        (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => n1 == n2 && ls1 == ls2,
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => alpha_eq(f1, f2) && alpha_eq(a1, a2),
        (Expr::Lam(i1, _, ty1, b1), Expr::Lam(i2, _, ty2, b2)) => {
            i1 == i2 && alpha_eq(ty1, ty2) && alpha_eq(b1, b2)
        }
        (Expr::Pi(i1, _, ty1, b1), Expr::Pi(i2, _, ty2, b2)) => {
            i1 == i2 && alpha_eq(ty1, ty2) && alpha_eq(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            alpha_eq(ty1, ty2) && alpha_eq(v1, v2) && alpha_eq(b1, b2)
        }
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            n1 == n2 && i1 == i2 && alpha_eq(e1, e2)
        }
        _ => false,
    }
}
#[cfg(test)]
mod extra_subst_tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal, Name};
    #[test]
    fn test_collect_consts_app() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("g"), vec![])),
        );
        let consts = collect_consts(&e);
        assert!(consts.contains(&Name::str("f")));
        assert!(consts.contains(&Name::str("g")));
        assert_eq!(consts.len(), 2);
    }
    #[test]
    fn test_collect_consts_deduplicated() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("f"), vec![])),
        );
        assert_eq!(collect_consts(&e).len(), 1);
    }
    #[test]
    fn test_count_fvar_present() {
        let fv = FVarId(7);
        let e = Expr::App(Box::new(Expr::FVar(fv)), Box::new(Expr::FVar(fv)));
        assert_eq!(count_fvar(&e, fv), 2);
    }
    #[test]
    fn test_count_fvar_absent() {
        let fv = FVarId(7);
        let e = Expr::Lit(Literal::Nat(0));
        assert_eq!(count_fvar(&e, fv), 0);
    }
    #[test]
    fn test_is_ground_literal() {
        assert!(is_ground(&Expr::Lit(Literal::Nat(0))));
    }
    #[test]
    fn test_is_ground_bvar() {
        assert!(!is_ground(&Expr::BVar(0)));
    }
    #[test]
    fn test_is_ground_fvar() {
        assert!(!is_ground(&Expr::FVar(FVarId(1))));
    }
    #[test]
    fn test_is_ground_sort() {
        assert!(is_ground(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_alpha_eq_bvar() {
        assert!(alpha_eq(&Expr::BVar(0), &Expr::BVar(0)));
        assert!(!alpha_eq(&Expr::BVar(0), &Expr::BVar(1)));
    }
    #[test]
    fn test_alpha_eq_literal() {
        assert!(alpha_eq(
            &Expr::Lit(Literal::Nat(5)),
            &Expr::Lit(Literal::Nat(5))
        ));
        assert!(!alpha_eq(
            &Expr::Lit(Literal::Nat(5)),
            &Expr::Lit(Literal::Nat(6))
        ));
    }
    #[test]
    fn test_alpha_eq_lambda_ignores_name() {
        let l1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let l2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(alpha_eq(&l1, &l2));
    }
    #[test]
    fn test_abstract_fvars_ordered() {
        let fv0 = FVarId(100);
        let fv1 = FVarId(101);
        let e = Expr::App(Box::new(Expr::FVar(fv0)), Box::new(Expr::FVar(fv1)));
        let result = abstract_fvars_ordered(&e, &[fv0, fv1], 0);
        assert_eq!(
            result,
            Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)))
        );
    }
    #[test]
    fn test_rename_binder() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let renamed = rename_binder(&lam, &Name::str("x"), Name::str("y"));
        if let Expr::Lam(_, name, _, _) = renamed {
            assert_eq!(name, Name::str("y"));
        } else {
            panic!("Expected Lam");
        }
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
#[cfg(test)]
mod tests_tiny_padding {
    use super::*;
    #[test]
    fn test_bitset64() {
        let mut bs = BitSet64::new();
        bs.insert(0);
        bs.insert(63);
        assert!(bs.contains(0));
        assert!(bs.contains(63));
        assert!(!bs.contains(1));
        assert_eq!(bs.len(), 2);
        bs.remove(0);
        assert!(!bs.contains(0));
    }
    #[test]
    fn test_bucket_counter() {
        let mut bc: BucketCounter<4> = BucketCounter::new();
        bc.inc(0);
        bc.inc(0);
        bc.inc(1);
        assert_eq!(bc.get(0), 2);
        assert_eq!(bc.total(), 3);
        assert_eq!(bc.argmax(), 0);
    }
}
