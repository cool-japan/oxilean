//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::FVarId;
use oxilean_kernel::{Expr, Level, Name};
use std::collections::HashMap;

use super::types::{
    Constraint, ConstraintPriority, ConstraintSet, ConstraintSet2, Rigidity, Substitution,
    TraceEvent, UnificationState, UnificationTracer, Unifier, UnifyConfig, UnifyConstraint,
    UnifyError, UnifyResult, UnifyState,
};

/// Unify two expressions.
///
/// This is a simplified unification that checks structural equality.
/// A full implementation would handle metavariables and constraints.
pub fn unify(lhs: &Expr, rhs: &Expr) -> Result<(), UnifyError> {
    if lhs == rhs {
        return Ok(());
    }
    match (lhs, rhs) {
        (Expr::Sort(l1), Expr::Sort(l2)) => {
            if l1 == l2 {
                Ok(())
            } else {
                Err(UnifyError::LevelMismatch(l1.clone(), l2.clone()))
            }
        }
        (Expr::BVar(i1), Expr::BVar(i2)) if i1 == i2 => Ok(()),
        (Expr::FVar(f1), Expr::FVar(f2)) if f1 == f2 => Ok(()),
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => {
            if n1 == n2 && ls1 == ls2 {
                Ok(())
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            unify(f1, f2)?;
            unify(a1, a2)
        }
        (Expr::Lam(bi1, n1, ty1, b1), Expr::Lam(bi2, n2, ty2, b2)) => {
            if bi1 == bi2 && n1 == n2 {
                unify(ty1, ty2)?;
                unify(b1, b2)
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::Pi(bi1, n1, ty1, b1), Expr::Pi(bi2, n2, ty2, b2)) => {
            if bi1 == bi2 && n1 == n2 {
                unify(ty1, ty2)?;
                unify(b1, b2)
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::Let(n1, ty1, v1, b1), Expr::Let(n2, ty2, v2, b2)) => {
            if n1 == n2 {
                unify(ty1, ty2)?;
                unify(v1, v2)?;
                unify(b1, b2)
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => {
            if l1 == l2 {
                Ok(())
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            if n1 == n2 && i1 == i2 {
                unify(e1, e2)
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        _ => Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone())),
    }
}
/// Check whether metavariable `mvar` (as `BVar(mvar)`) occurs in `expr`.
///
/// Used to detect cyclic substitutions before committing an assignment.
pub fn occurs(mvar: u32, expr: &Expr) -> bool {
    match expr {
        Expr::BVar(i) => *i == mvar,
        Expr::App(f, a) => occurs(mvar, f) || occurs(mvar, a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            occurs(mvar, ty) || occurs(mvar, body)
        }
        Expr::Let(_, ty, val, body) => occurs(mvar, ty) || occurs(mvar, val) || occurs(mvar, body),
        Expr::Proj(_, _, inner) => occurs(mvar, inner),
        _ => false,
    }
}
/// Unify two universe levels.
pub fn unify_levels(l1: &Level, l2: &Level) -> Result<(), UnifyError> {
    if l1 == l2 {
        Ok(())
    } else {
        Err(UnifyError::LevelMismatch(l1.clone(), l2.clone()))
    }
}
/// Check whether level `l1` is provably `≤ l2` syntactically.
pub fn level_le(l1: &Level, l2: &Level) -> bool {
    l1 == l2
}
/// Determine the rigidity of an expression.
pub fn rigidity(expr: &Expr) -> Rigidity {
    match expr {
        Expr::Const(_, _) | Expr::Sort(_) | Expr::Lit(_) => Rigidity::Rigid,
        Expr::BVar(_) => Rigidity::Flex,
        Expr::App(f, _) => rigidity(f),
        _ => Rigidity::Unknown,
    }
}
/// Check structural equality of two names (wrapper for tests).
pub fn names_equal(a: &Name, b: &Name) -> bool {
    a == b
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::unify::*;
    use oxilean_kernel::{BinderInfo, Literal};
    fn nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_const() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    #[test]
    fn test_unify_identical() {
        let expr = Expr::Sort(Level::zero());
        assert!(unify(&expr, &expr).is_ok());
    }
    #[test]
    fn test_unify_sorts() {
        let l1 = Expr::Sort(Level::zero());
        let l2 = Expr::Sort(Level::zero());
        assert!(unify(&l1, &l2).is_ok());
    }
    #[test]
    fn test_unify_different_sorts() {
        let l1 = Expr::Sort(Level::zero());
        let l2 = Expr::Sort(Level::succ(Level::zero()));
        assert!(unify(&l1, &l2).is_err());
    }
    #[test]
    fn test_unify_lits() {
        let lit1 = Expr::Lit(Literal::Nat(42));
        let lit2 = Expr::Lit(Literal::Nat(42));
        assert!(unify(&lit1, &lit2).is_ok());
    }
    #[test]
    fn test_unify_different_lits() {
        let lit1 = Expr::Lit(Literal::Nat(42));
        let lit2 = Expr::Lit(Literal::Nat(100));
        assert!(unify(&lit1, &lit2).is_err());
    }
    #[test]
    fn test_unify_apps() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(Literal::Nat(1));
        let app1 = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let app2 = Expr::App(Box::new(f), Box::new(a));
        assert!(unify(&app1, &app2).is_ok());
    }
    #[test]
    fn test_unify_consts_same() {
        assert!(unify(&nat_const(), &nat_const()).is_ok());
    }
    #[test]
    fn test_unify_consts_different() {
        assert!(unify(&nat_const(), &bool_const()).is_err());
    }
    #[test]
    fn test_substitution_insert_get() {
        let mut s = Substitution::new();
        s.insert(0, nat_const());
        assert_eq!(s.get(0), Some(&nat_const()));
        assert_eq!(s.get(1), None);
    }
    #[test]
    fn test_substitution_len() {
        let mut s = Substitution::new();
        assert_eq!(s.len(), 0);
        s.insert(0, nat_const());
        assert_eq!(s.len(), 1);
    }
    #[test]
    fn test_substitution_remove() {
        let mut s = Substitution::new();
        s.insert(0, nat_const());
        let old = s.remove(0);
        assert_eq!(old, Some(nat_const()));
        assert!(s.is_empty());
    }
    #[test]
    fn test_substitution_merge() {
        let mut a = Substitution::new();
        let mut b = Substitution::new();
        a.insert(0, nat_const());
        b.insert(1, bool_const());
        a.merge(b);
        assert_eq!(a.len(), 2);
    }
    #[test]
    fn test_substitution_apply_shallow() {
        let mut s = Substitution::new();
        s.insert(3, nat_const());
        let result = s.apply_shallow(&Expr::BVar(3));
        assert_eq!(result, nat_const());
    }
    #[test]
    fn test_constraint_set_basic() {
        let mut cs = ConstraintSet::new();
        cs.eq_expr(nat_const(), bool_const());
        assert_eq!(cs.len(), 1);
        assert!(cs.pop().is_some());
        assert!(cs.is_empty());
    }
    #[test]
    fn test_unify_state_assign() {
        let mut state = UnifyState::new();
        state
            .assign(0, nat_const())
            .expect("test operation should succeed");
        assert_eq!(state.subst.get(0), Some(&nat_const()));
    }
    #[test]
    fn test_unify_state_occurs_check() {
        let mut state = UnifyState::new();
        let result = state.assign(0, Expr::BVar(0));
        assert!(matches!(result, Err(UnifyError::OccursCheck)));
    }
    #[test]
    fn test_occurs() {
        let expr = Expr::App(Box::new(nat_const()), Box::new(Expr::BVar(3)));
        assert!(occurs(3, &expr));
        assert!(!occurs(7, &expr));
    }
    #[test]
    fn test_rigidity() {
        assert_eq!(rigidity(&nat_const()), Rigidity::Rigid);
        assert_eq!(rigidity(&Expr::BVar(0)), Rigidity::Flex);
    }
    #[test]
    fn test_unifier_strict() {
        let mut u = Unifier::new();
        let result = u.unify_exprs(&nat_const(), &bool_const());
        assert!(result.is_err());
    }
    #[test]
    fn test_unifier_lenient_deferred() {
        let mut u = Unifier::lenient();
        u.defer(nat_const(), bool_const());
        assert_eq!(u.state.pending.len(), 1);
        let unsolved = u.solve_pending().expect("test operation should succeed");
        assert_eq!(unsolved.len(), 1);
    }
    #[test]
    fn test_unify_levels_equal() {
        let l = Level::zero();
        assert!(unify_levels(&l, &l).is_ok());
    }
    #[test]
    fn test_unify_levels_different() {
        let l1 = Level::zero();
        let l2 = Level::succ(Level::zero());
        assert!(unify_levels(&l1, &l2).is_err());
    }
    #[test]
    fn test_unify_pi() {
        let pi1 = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_const()),
            Box::new(nat_const()),
        );
        let pi2 = pi1.clone();
        assert!(unify(&pi1, &pi2).is_ok());
    }
    #[test]
    fn test_unify_lam() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_const()),
            Box::new(Expr::BVar(0)),
        );
        assert!(unify(&lam, &lam).is_ok());
    }
    #[test]
    fn test_unify_error_display() {
        let e = UnifyError::OccursCheck;
        assert!(!e.to_string().is_empty());
        let e2 = UnifyError::Unsolvable("test".into());
        assert!(e2.to_string().contains("test"));
    }
}
/// Try to unify two expressions up to alpha-equivalence.
///
/// This is a recursive structural check that ignores binder names.
pub fn alpha_unify(lhs: &Expr, rhs: &Expr) -> Result<(), UnifyError> {
    if lhs == rhs {
        return Ok(());
    }
    match (lhs, rhs) {
        (Expr::Sort(l1), Expr::Sort(l2)) => {
            if l1 == l2 {
                Ok(())
            } else {
                Err(UnifyError::LevelMismatch(l1.clone(), l2.clone()))
            }
        }
        (Expr::BVar(i1), Expr::BVar(i2)) if i1 == i2 => Ok(()),
        (Expr::FVar(f1), Expr::FVar(f2)) if f1 == f2 => Ok(()),
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) if n1 == n2 && ls1 == ls2 => Ok(()),
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            alpha_unify(f1, f2)?;
            alpha_unify(a1, a2)
        }
        (Expr::Lam(bi1, _, ty1, b1), Expr::Lam(bi2, _, ty2, b2)) if bi1 == bi2 => {
            alpha_unify(ty1, ty2)?;
            alpha_unify(b1, b2)
        }
        (Expr::Pi(bi1, _, ty1, b1), Expr::Pi(bi2, _, ty2, b2)) if bi1 == bi2 => {
            alpha_unify(ty1, ty2)?;
            alpha_unify(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            alpha_unify(ty1, ty2)?;
            alpha_unify(v1, v2)?;
            alpha_unify(b1, b2)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) if l1 == l2 => Ok(()),
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) if n1 == n2 && i1 == i2 => {
            alpha_unify(e1, e2)
        }
        _ => Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone())),
    }
}
/// Unify two expressions and wrap the result in `UnifyResult`.
pub fn unify_result(lhs: &Expr, rhs: &Expr) -> UnifyResult {
    match unify(lhs, rhs) {
        Ok(()) => UnifyResult::Ok(Substitution::new()),
        Err(e) => UnifyResult::Failed(e),
    }
}
#[cfg(test)]
mod tests_extra {
    use super::*;
    use crate::unify::*;
    use oxilean_kernel::BinderInfo;
    fn nat_const() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_const() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    #[test]
    fn test_alpha_unify_same() {
        let e = nat_const();
        assert!(alpha_unify(&e, &e).is_ok());
    }
    #[test]
    fn test_alpha_unify_lam_different_binder_names() {
        let lam1 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_const()),
            Box::new(Expr::BVar(0)),
        );
        let lam2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(nat_const()),
            Box::new(Expr::BVar(0)),
        );
        assert!(alpha_unify(&lam1, &lam2).is_ok());
    }
    #[test]
    fn test_alpha_unify_pi_different_names() {
        let pi1 = Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(nat_const()),
            Box::new(nat_const()),
        );
        let pi2 = Expr::Pi(
            BinderInfo::Default,
            Name::str("b"),
            Box::new(nat_const()),
            Box::new(nat_const()),
        );
        assert!(alpha_unify(&pi1, &pi2).is_ok());
    }
    #[test]
    fn test_alpha_unify_mismatch() {
        assert!(alpha_unify(&nat_const(), &bool_const()).is_err());
    }
    #[test]
    fn test_unify_result_ok() {
        let r = unify_result(&nat_const(), &nat_const());
        assert!(r.is_ok());
    }
    #[test]
    fn test_unify_result_failed() {
        let r = unify_result(&nat_const(), &bool_const());
        assert!(r.is_failed());
    }
    #[test]
    fn test_unify_result_unwrap_subst() {
        let r = unify_result(&nat_const(), &nat_const());
        let s = r.unwrap_subst();
        assert!(s.is_empty());
    }
    #[test]
    fn test_substitution_chase_no_chain() {
        let s = Substitution::new();
        let e = nat_const();
        let result = s.chase(&e);
        assert_eq!(result, e);
    }
    #[test]
    fn test_substitution_chain() {
        let mut s = Substitution::new();
        s.insert(0, nat_const());
        let root = Expr::BVar(0);
        let result = s.chase(&root);
        assert_eq!(result, nat_const());
    }
    #[test]
    fn test_substitution_domain() {
        let mut s = Substitution::new();
        s.insert(0, nat_const());
        s.insert(1, bool_const());
        let mut domain = s.domain();
        domain.sort();
        assert_eq!(domain, vec![0, 1]);
    }
    #[test]
    fn test_level_le_same() {
        let l = Level::zero();
        assert!(level_le(&l, &l));
    }
    #[test]
    fn test_names_equal() {
        assert!(names_equal(&Name::str("Nat"), &Name::str("Nat")));
        assert!(!names_equal(&Name::str("Nat"), &Name::str("Bool")));
    }
    #[test]
    fn test_unify_state_step_limit() {
        let mut state = UnifyState::with_limit(2);
        state.step().expect("test operation should succeed");
        state.step().expect("test operation should succeed");
        let err = state.step();
        assert!(err.is_err());
    }
    #[test]
    fn test_unify_state_unlimited() {
        let mut state = UnifyState::new();
        for _ in 0..1000 {
            state.step().expect("test operation should succeed");
        }
        assert_eq!(state.steps, 1000);
    }
}
#[cfg(test)]
mod unify_extra_tests {
    use super::*;
    use crate::unify::*;
    fn nat_expr() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_expr() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    #[test]
    fn test_unify_config_default() {
        let cfg = UnifyConfig::default();
        assert!(cfg.occurs_check);
        assert_eq!(cfg.max_depth, 256);
        assert!(!cfg.structural_sorts);
    }
    #[test]
    fn test_unify_config_without_occurs_check() {
        let cfg = UnifyConfig::without_occurs_check();
        assert!(!cfg.occurs_check);
    }
    #[test]
    fn test_unify_config_syntactic() {
        let cfg = UnifyConfig::syntactic();
        assert!(cfg.occurs_check);
        assert!(cfg.structural_sorts);
        assert_eq!(cfg.max_depth, 64);
    }
    #[test]
    fn test_constraint_trivial() {
        let c = UnifyConstraint::new(nat_expr(), nat_expr());
        assert!(c.is_trivial());
        let c2 = UnifyConstraint::new(nat_expr(), bool_expr());
        assert!(!c2.is_trivial());
    }
    #[test]
    fn test_constraint_with_source() {
        let c = UnifyConstraint::with_source(nat_expr(), bool_expr(), "test location");
        assert_eq!(c.source.as_deref(), Some("test location"));
    }
    #[test]
    fn test_constraint_set_add_and_len() {
        let mut cs = ConstraintSet2::new();
        cs.add_eq(nat_expr(), nat_expr());
        cs.add_eq(nat_expr(), bool_expr());
        assert_eq!(cs.len(), 2);
    }
    #[test]
    fn test_constraint_set_remove_trivial() {
        let mut cs = ConstraintSet2::new();
        cs.add_eq(nat_expr(), nat_expr());
        cs.add_eq(nat_expr(), bool_expr());
        cs.remove_trivial();
        assert_eq!(cs.len(), 1);
    }
    #[test]
    fn test_constraint_set_pop() {
        let mut cs = ConstraintSet2::new();
        cs.add_eq(nat_expr(), bool_expr());
        let c = cs.pop().expect("collection should not be empty");
        assert!(!c.is_trivial());
        assert!(cs.is_empty());
    }
    #[test]
    fn test_constraint_set_iter() {
        let mut cs = ConstraintSet2::new();
        cs.add_eq(nat_expr(), nat_expr());
        cs.add_eq(bool_expr(), bool_expr());
        let v: Vec<_> = cs.iter().collect();
        assert_eq!(v.len(), 2);
    }
}
/// The FVar ID offset used to encode metavariables.
pub const MVAR_OFFSET: u64 = 1_000_000;
/// Check whether a metavariable encoded as `FVar(fvar_id)` occurs
/// free in `expr`.  Used as an occurs check before committing an
/// assignment `?m := rhs` to prevent cyclic substitutions.
pub fn occurs_in_fvar(fvar_id: u64, expr: &Expr) -> bool {
    match expr {
        Expr::FVar(fv) => fv.0 == fvar_id,
        Expr::App(f, a) => occurs_in_fvar(fvar_id, f) || occurs_in_fvar(fvar_id, a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            occurs_in_fvar(fvar_id, ty) || occurs_in_fvar(fvar_id, body)
        }
        Expr::Let(_, ty, val, body) => {
            occurs_in_fvar(fvar_id, ty)
                || occurs_in_fvar(fvar_id, val)
                || occurs_in_fvar(fvar_id, body)
        }
        Expr::Proj(_, _, inner) => occurs_in_fvar(fvar_id, inner),
        _ => false,
    }
}
/// Walk a chain of metavariable assignments, returning the deepest
/// unassigned expression.  Prevents infinite loops by following at
/// most `fuel` links.
fn chase_meta(fvar_id: u64, assignments: &HashMap<u64, Expr>) -> Expr {
    let meta_id = fvar_id - MVAR_OFFSET;
    let mut cur = Expr::FVar(FVarId(fvar_id));
    let mut fuel = 256usize;
    loop {
        if fuel == 0 {
            break;
        }
        fuel -= 1;
        match &cur {
            Expr::FVar(fv) if fv.0 >= MVAR_OFFSET => {
                let mid = fv.0 - MVAR_OFFSET;
                if let Some(next) = assignments.get(&mid) {
                    cur = next.clone();
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
    let _ = meta_id;
    cur
}
/// Apply all known metavar assignments shallowly to an expression.
///
/// Replaces every `FVar(id)` where `id >= MVAR_OFFSET` and `id - MVAR_OFFSET`
/// has an assignment with the assigned expression (recursively).
pub fn apply_meta_assignments(expr: &Expr, assignments: &HashMap<u64, Expr>) -> Expr {
    match expr {
        Expr::FVar(fv) if fv.0 >= MVAR_OFFSET => {
            let meta_id = fv.0 - MVAR_OFFSET;
            if let Some(val) = assignments.get(&meta_id) {
                apply_meta_assignments(val, assignments)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(apply_meta_assignments(f, assignments)),
            Box::new(apply_meta_assignments(a, assignments)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(apply_meta_assignments(ty, assignments)),
            Box::new(apply_meta_assignments(body, assignments)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(apply_meta_assignments(ty, assignments)),
            Box::new(apply_meta_assignments(body, assignments)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(apply_meta_assignments(ty, assignments)),
            Box::new(apply_meta_assignments(val, assignments)),
            Box::new(apply_meta_assignments(body, assignments)),
        ),
        Expr::Proj(name, idx, inner) => Expr::Proj(
            name.clone(),
            *idx,
            Box::new(apply_meta_assignments(inner, assignments)),
        ),
        _ => expr.clone(),
    }
}
/// First-order metavar-aware unification.
///
/// Unifies `lhs` with `rhs`, updating `assignments` in-place.
/// `assignments` maps `meta_id` (= `fvar_id - MVAR_OFFSET`) to `Expr`.
///
/// Handles:
/// - Flex-rigid: `?m =? t`  →  assign `?m := t` (with occurs check)
/// - Rigid-flex: `t =? ?m`  →  assign `?m := t`
/// - Flex-flex:  `?m1 =? ?m2`  →  assign `?m1 := ?m2`
/// - Rigid-rigid decomposition (App, Lam, Pi, Let, Const, Sort, Lit, Proj)
pub fn unify_meta_aware(
    lhs: &Expr,
    rhs: &Expr,
    assignments: &mut HashMap<u64, Expr>,
) -> Result<(), UnifyError> {
    let lhs_chased = apply_meta_assignments(lhs, assignments);
    let rhs_chased = apply_meta_assignments(rhs, assignments);
    unify_meta_aware_impl(&lhs_chased, &rhs_chased, assignments)
}
fn unify_meta_aware_impl(
    lhs: &Expr,
    rhs: &Expr,
    assignments: &mut HashMap<u64, Expr>,
) -> Result<(), UnifyError> {
    if lhs == rhs {
        return Ok(());
    }
    let lhs_is_meta = matches!(lhs, Expr::FVar(fv) if fv.0 >= MVAR_OFFSET);
    let rhs_is_meta = matches!(rhs, Expr::FVar(fv) if fv.0 >= MVAR_OFFSET);
    match (lhs_is_meta, rhs_is_meta) {
        (true, true) => {
            if let (Expr::FVar(lf), Expr::FVar(rf)) = (lhs, rhs) {
                if lf.0 == rf.0 {
                    return Ok(());
                }
                let lmid = lf.0 - MVAR_OFFSET;
                assignments.insert(lmid, rhs.clone());
                Ok(())
            } else {
                unreachable!()
            }
        }
        (true, false) => {
            if let Expr::FVar(fv) = lhs {
                let fvar_id = fv.0;
                let chased = chase_meta(fvar_id, assignments);
                if &chased != lhs {
                    return unify_meta_aware_impl(&chased, rhs, assignments);
                }
                if occurs_in_fvar(fvar_id, rhs) {
                    return Err(UnifyError::OccursCheck);
                }
                let meta_id = fvar_id - MVAR_OFFSET;
                assignments.insert(meta_id, rhs.clone());
                Ok(())
            } else {
                unreachable!()
            }
        }
        (false, true) => {
            if let Expr::FVar(fv) = rhs {
                let fvar_id = fv.0;
                let chased = chase_meta(fvar_id, assignments);
                if &chased != rhs {
                    return unify_meta_aware_impl(lhs, &chased, assignments);
                }
                if occurs_in_fvar(fvar_id, lhs) {
                    return Err(UnifyError::OccursCheck);
                }
                let meta_id = fvar_id - MVAR_OFFSET;
                assignments.insert(meta_id, lhs.clone());
                Ok(())
            } else {
                unreachable!()
            }
        }
        (false, false) => unify_rigid_rigid(lhs, rhs, assignments),
    }
}
/// Structural decomposition for two rigid terms.
fn unify_rigid_rigid(
    lhs: &Expr,
    rhs: &Expr,
    assignments: &mut HashMap<u64, Expr>,
) -> Result<(), UnifyError> {
    match (lhs, rhs) {
        (Expr::Sort(l1), Expr::Sort(l2)) => {
            if l1 == l2 {
                Ok(())
            } else {
                Err(UnifyError::LevelMismatch(l1.clone(), l2.clone()))
            }
        }
        (Expr::BVar(i1), Expr::BVar(i2)) => {
            if i1 == i2 {
                Ok(())
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::FVar(f1), Expr::FVar(f2)) => {
            if f1 == f2 {
                Ok(())
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => {
            if n1 == n2 && ls1.len() == ls2.len() {
                for (l1, l2) in ls1.iter().zip(ls2.iter()) {
                    if l1 != l2 {
                        return Err(UnifyError::LevelMismatch(l1.clone(), l2.clone()));
                    }
                }
                Ok(())
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            unify_meta_aware_impl(f1, f2, assignments)?;
            unify_meta_aware_impl(a1, a2, assignments)
        }
        (Expr::Lam(bi1, _, ty1, body1), Expr::Lam(bi2, _, ty2, body2)) => {
            if bi1 != bi2 {
                return Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()));
            }
            unify_meta_aware_impl(ty1, ty2, assignments)?;
            unify_meta_aware_impl(body1, body2, assignments)
        }
        (Expr::Pi(bi1, _, ty1, body1), Expr::Pi(bi2, _, ty2, body2)) => {
            if bi1 != bi2 {
                return Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()));
            }
            unify_meta_aware_impl(ty1, ty2, assignments)?;
            unify_meta_aware_impl(body1, body2, assignments)
        }
        (Expr::Let(_, ty1, val1, body1), Expr::Let(_, ty2, val2, body2)) => {
            unify_meta_aware_impl(ty1, ty2, assignments)?;
            unify_meta_aware_impl(val1, val2, assignments)?;
            unify_meta_aware_impl(body1, body2, assignments)
        }
        (Expr::Lit(l1), Expr::Lit(l2)) => {
            if l1 == l2 {
                Ok(())
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            if n1 == n2 && i1 == i2 {
                unify_meta_aware_impl(e1, e2, assignments)
            } else {
                Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone()))
            }
        }
        _ => Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone())),
    }
}
#[cfg(test)]
mod mvar_unify_tests {
    use super::*;
    use crate::unify::*;
    /// Build a metavar expression: FVar(MVAR_OFFSET + id)
    fn mvar(id: u64) -> Expr {
        Expr::FVar(FVarId(MVAR_OFFSET + id))
    }
    fn nat() -> Expr {
        Expr::Const(oxilean_kernel::Name::str("Nat"), vec![])
    }
    fn bool_e() -> Expr {
        Expr::Const(oxilean_kernel::Name::str("Bool"), vec![])
    }
    #[test]
    fn test_flex_rigid_assigns() {
        let mut asgn = HashMap::new();
        unify_meta_aware(&mvar(0), &nat(), &mut asgn).expect("unification should succeed");
        assert_eq!(asgn.get(&0), Some(&nat()));
    }
    #[test]
    fn test_rigid_flex_assigns() {
        let mut asgn = HashMap::new();
        unify_meta_aware(&nat(), &mvar(1), &mut asgn).expect("unification should succeed");
        assert_eq!(asgn.get(&1), Some(&nat()));
    }
    #[test]
    fn test_flex_flex_assigns() {
        let mut asgn = HashMap::new();
        unify_meta_aware(&mvar(0), &mvar(1), &mut asgn).expect("unification should succeed");
        assert_eq!(asgn.get(&0), Some(&mvar(1)));
    }
    #[test]
    fn test_rigid_rigid_same_const() {
        let mut asgn = HashMap::new();
        assert!(unify_meta_aware(&nat(), &nat(), &mut asgn).is_ok());
        assert!(asgn.is_empty());
    }
    #[test]
    fn test_rigid_rigid_different_const_fails() {
        let mut asgn = HashMap::new();
        assert!(unify_meta_aware(&nat(), &bool_e(), &mut asgn).is_err());
    }
    #[test]
    fn test_occurs_check_prevents_cyclic() {
        let mut asgn = HashMap::new();
        let f = nat();
        let rhs = Expr::App(Box::new(f), Box::new(mvar(0)));
        let err = unify_meta_aware(&mvar(0), &rhs, &mut asgn);
        assert!(matches!(err, Err(UnifyError::OccursCheck)));
    }
    #[test]
    fn test_unify_app_with_meta() {
        let f = Expr::Const(oxilean_kernel::Name::str("f"), vec![]);
        let lhs = Expr::App(Box::new(f.clone()), Box::new(mvar(0)));
        let rhs = Expr::App(Box::new(f), Box::new(nat()));
        let mut asgn = HashMap::new();
        unify_meta_aware(&lhs, &rhs, &mut asgn).expect("unification should succeed");
        assert_eq!(asgn.get(&0), Some(&nat()));
    }
    #[test]
    fn test_already_assigned_meta_chase() {
        let mut asgn = HashMap::new();
        asgn.insert(0u64, nat());
        assert!(unify_meta_aware(&mvar(0), &nat(), &mut asgn).is_ok());
    }
    #[test]
    fn test_already_assigned_meta_conflict() {
        let mut asgn = HashMap::new();
        asgn.insert(0u64, nat());
        assert!(unify_meta_aware(&mvar(0), &bool_e(), &mut asgn).is_err());
    }
    #[test]
    fn test_occurs_in_fvar_basic() {
        let id = MVAR_OFFSET + 3;
        let expr = Expr::App(Box::new(nat()), Box::new(Expr::FVar(FVarId(id))));
        assert!(occurs_in_fvar(id, &expr));
        assert!(!occurs_in_fvar(id + 1, &expr));
    }
    #[test]
    fn test_apply_meta_assignments_simple() {
        let mut asgn = HashMap::new();
        asgn.insert(0u64, nat());
        let expr = mvar(0);
        let result = apply_meta_assignments(&expr, &asgn);
        assert_eq!(result, nat());
    }
    #[test]
    fn test_apply_meta_assignments_chain() {
        let mut asgn = HashMap::new();
        asgn.insert(0u64, mvar(1));
        asgn.insert(1u64, nat());
        let result = apply_meta_assignments(&mvar(0), &asgn);
        assert_eq!(result, nat());
    }
}
/// Check whether two levels are syntactically equal.
#[allow(dead_code)]
pub fn levels_equal(l1: &Level, l2: &Level) -> bool {
    match (l1, l2) {
        (Level::Zero, Level::Zero) => true,
        (Level::Succ(a), Level::Succ(b)) => levels_equal(a, b),
        (Level::Max(a1, b1), Level::Max(a2, b2)) => levels_equal(a1, a2) && levels_equal(b1, b2),
        (Level::IMax(a1, b1), Level::IMax(a2, b2)) => levels_equal(a1, a2) && levels_equal(b1, b2),
        (Level::Param(n1), Level::Param(n2)) => n1 == n2,
        (Level::MVar(i), Level::MVar(j)) => i == j,
        _ => false,
    }
}
/// Normalize a level by reducing `Max(l, l)` to `l`, `Succ(Zero)` to `1`, etc.
#[allow(dead_code)]
pub fn normalize_level(l: &Level) -> Level {
    match l {
        Level::Max(a, b) => {
            let na = normalize_level(a);
            let nb = normalize_level(b);
            if levels_equal(&na, &nb) {
                na
            } else {
                Level::Max(Box::new(na), Box::new(nb))
            }
        }
        Level::IMax(a, b) => {
            let na = normalize_level(a);
            let nb = normalize_level(b);
            if matches!(nb, Level::Zero) {
                Level::Zero
            } else {
                Level::IMax(Box::new(na), Box::new(nb))
            }
        }
        Level::Succ(inner) => Level::Succ(Box::new(normalize_level(inner))),
        other => other.clone(),
    }
}
/// Compute the "depth" (number of `Succ` wrappers) of a level.
#[allow(dead_code)]
pub fn level_depth(l: &Level) -> Option<u32> {
    match l {
        Level::Zero => Some(0),
        Level::Succ(inner) => level_depth(inner).map(|d| d + 1),
        _ => None,
    }
}
/// Add a constant offset to a level: `level + n`.
#[allow(dead_code)]
pub fn level_add(l: Level, n: u32) -> Level {
    (0..n).fold(l, |acc, _| Level::Succ(Box::new(acc)))
}
/// Compute `max(l1, l2)` as a level expression (simplified: not normalised).
#[allow(dead_code)]
pub fn level_max(l1: Level, l2: Level) -> Level {
    match (level_depth(&l1), level_depth(&l2)) {
        (Some(d1), Some(d2)) => level_add(Level::Zero, d1.max(d2)),
        _ => Level::Max(Box::new(l1), Box::new(l2)),
    }
}
/// Collect all universe parameters that appear (free) in a level expression.
#[allow(dead_code)]
pub fn level_free_params(l: &Level) -> Vec<Name> {
    let mut params = Vec::new();
    collect_level_params(l, &mut params);
    params
}
fn collect_level_params(l: &Level, acc: &mut Vec<Name>) {
    match l {
        Level::Param(n) if !acc.contains(n) => {
            acc.push(n.clone());
        }
        Level::Succ(inner) => collect_level_params(inner, acc),
        Level::Max(a, b) | Level::IMax(a, b) => {
            collect_level_params(a, acc);
            collect_level_params(b, acc);
        }
        _ => {}
    }
}
/// Check whether two expressions are definitionally equal under a substitution,
/// using structural (syntactic) equality only (no reduction).
#[allow(dead_code)]
pub fn structurally_equal(e1: &Expr, e2: &Expr, subst: &Substitution) -> bool {
    let e1 = subst.apply_recursive(e1);
    let e2 = subst.apply_recursive(e2);
    structurally_equal_nf(&e1, &e2)
}
pub fn structurally_equal_nf(e1: &Expr, e2: &Expr) -> bool {
    match (e1, e2) {
        (Expr::BVar(i), Expr::BVar(j)) => i == j,
        (Expr::FVar(a), Expr::FVar(b)) => a == b,
        (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => {
            n1 == n2
                && ls1.len() == ls2.len()
                && ls1.iter().zip(ls2).all(|(a, b)| levels_equal(a, b))
        }
        (Expr::Sort(l1), Expr::Sort(l2)) => levels_equal(l1, l2),
        (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
        (Expr::App(f1, a1), Expr::App(f2, a2)) => {
            structurally_equal_nf(f1, f2) && structurally_equal_nf(a1, a2)
        }
        (Expr::Lam(bi1, _, ty1, b1), Expr::Lam(bi2, _, ty2, b2)) => {
            bi1 == bi2 && structurally_equal_nf(ty1, ty2) && structurally_equal_nf(b1, b2)
        }
        (Expr::Pi(bi1, _, ty1, b1), Expr::Pi(bi2, _, ty2, b2)) => {
            bi1 == bi2 && structurally_equal_nf(ty1, ty2) && structurally_equal_nf(b1, b2)
        }
        (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
            structurally_equal_nf(ty1, ty2)
                && structurally_equal_nf(v1, v2)
                && structurally_equal_nf(b1, b2)
        }
        (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
            n1 == n2 && i1 == i2 && structurally_equal_nf(e1, e2)
        }
        _ => false,
    }
}
/// Classify a constraint by its priority.
#[allow(dead_code)]
pub fn classify_constraint(c: &Constraint, subst: &Substitution) -> ConstraintPriority {
    match c {
        Constraint::LevelEq(..) | Constraint::LevelLe(..) => ConstraintPriority::RigidRigid,
        Constraint::ExprEq(lhs, rhs) => {
            let l = subst.apply_recursive(lhs);
            let r = subst.apply_recursive(rhs);
            let l_flex = matches!(
                & l, Expr::BVar(_) if subst.get(if let Expr::BVar(i) = & l { * i } else {
                0 }).is_none()
            );
            let r_flex = matches!(
                & r, Expr::BVar(_) if subst.get(if let Expr::BVar(i) = & r { * i } else {
                0 }).is_none()
            );
            match (l_flex, r_flex) {
                (true, true) => ConstraintPriority::FlexFlex,
                (true, false) | (false, true) => ConstraintPriority::FlexRigid,
                (false, false) => ConstraintPriority::RigidRigid,
            }
        }
    }
}
/// Check whether an expression is a "pattern" in the Miller pattern sense:
/// a metavariable applied to a list of distinct bound variables.
#[allow(dead_code)]
pub fn is_miller_pattern(expr: &Expr) -> bool {
    let (head, args) = collect_spine(expr);
    if !matches!(head, Expr::BVar(_)) {
        return false;
    }
    let mut seen = std::collections::HashSet::new();
    for a in &args {
        match a {
            Expr::BVar(i) => {
                if !seen.insert(*i) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}
/// Collect the head and argument spine of a left-nested application.
#[allow(dead_code)]
pub fn collect_spine(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut cur = expr;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f;
    }
    args.reverse();
    (cur, args)
}
/// Count the number of arguments in the application spine.
#[allow(dead_code)]
pub fn app_spine_len(expr: &Expr) -> usize {
    collect_spine(expr).1.len()
}
/// Rebuild an expression from a head and a list of arguments.
#[allow(dead_code)]
pub fn rebuild_app(head: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter()
        .fold(head, |acc, arg| Expr::App(Box::new(acc), Box::new(arg)))
}
/// Reduce an expression to its weak-head normal form under a substitution.
///
/// This simplified implementation handles:
/// - Metavariable dereferencing (via the substitution).
/// - Beta-reduction of `(λx.b) a → b[a/x]`.
///
/// It does NOT perform delta-reduction (unfolding definitions).
#[allow(dead_code)]
pub fn whnf(expr: &Expr, subst: &Substitution) -> Expr {
    let expr = subst.apply_shallow(expr);
    match &expr {
        Expr::App(f, a) => {
            let f_whnf = whnf(f, subst);
            if let Expr::Lam(_, _, _ty, body) = &f_whnf {
                let reduced = beta_subst(body, a, 0);
                whnf(&reduced, subst)
            } else {
                Expr::App(Box::new(f_whnf), a.clone())
            }
        }
        _ => expr,
    }
}
/// Substitute BVar(depth) with `replacement` in `expr`, adjusting indices.
#[allow(dead_code)]
pub fn beta_subst(expr: &Expr, replacement: &Expr, depth: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i == depth {
                lift_bvars(replacement, 0, depth)
            } else if *i > depth {
                Expr::BVar(i - 1)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(beta_subst(f, replacement, depth)),
            Box::new(beta_subst(a, replacement, depth)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(beta_subst(ty, replacement, depth)),
            Box::new(beta_subst(body, replacement, depth + 1)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(beta_subst(ty, replacement, depth)),
            Box::new(beta_subst(body, replacement, depth + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(beta_subst(ty, replacement, depth)),
            Box::new(beta_subst(val, replacement, depth)),
            Box::new(beta_subst(body, replacement, depth + 1)),
        ),
        Expr::Proj(n, i, inner) => Expr::Proj(
            n.clone(),
            *i,
            Box::new(beta_subst(inner, replacement, depth)),
        ),
        other => other.clone(),
    }
}
/// Lift all BVar indices ≥ `cutoff` in `expr` by `amount`.
#[allow(dead_code)]
pub fn lift_bvars(expr: &Expr, cutoff: u32, amount: u32) -> Expr {
    if amount == 0 {
        return expr.clone();
    }
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                Expr::BVar(i + amount)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(lift_bvars(f, cutoff, amount)),
            Box::new(lift_bvars(a, cutoff, amount)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(lift_bvars(ty, cutoff, amount)),
            Box::new(lift_bvars(body, cutoff + 1, amount)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(lift_bvars(ty, cutoff, amount)),
            Box::new(lift_bvars(body, cutoff + 1, amount)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(lift_bvars(ty, cutoff, amount)),
            Box::new(lift_bvars(val, cutoff, amount)),
            Box::new(lift_bvars(body, cutoff + 1, amount)),
        ),
        Expr::Proj(n, i, inner) => {
            Expr::Proj(n.clone(), *i, Box::new(lift_bvars(inner, cutoff, amount)))
        }
        other => other.clone(),
    }
}
/// Eta-expand an expression relative to a Pi type.
///
/// If `expr` has type `Π(x : A). B` and is not already a lambda,
/// return `λx. expr x` (with a fresh BVar).
#[allow(dead_code)]
pub fn eta_expand(expr: &Expr, ty: &Expr) -> Option<Expr> {
    if let Expr::Pi(bi, name, domain, _body) = ty {
        if matches!(expr, Expr::Lam(..)) {
            return None;
        }
        let lifted = lift_bvars(expr, 0, 1);
        let app = Expr::App(Box::new(lifted), Box::new(Expr::BVar(0)));
        Some(Expr::Lam(*bi, name.clone(), domain.clone(), Box::new(app)))
    } else {
        None
    }
}
/// Check whether `Lam(_, _, _, App(body, BVar(0)))` is an eta-redex.
///
/// Returns `Some(body)` if it is, `None` otherwise.
#[allow(dead_code)]
pub fn eta_reduce(expr: &Expr) -> Option<&Expr> {
    if let Expr::Lam(_, _, _, body) = expr {
        if let Expr::App(f, arg) = body.as_ref() {
            if matches!(arg.as_ref(), Expr::BVar(0)) {
                if !contains_bvar(f, 0) {
                    return Some(f);
                }
            }
        }
    }
    None
}
/// Check whether `expr` contains a bound variable with index `target`.
#[allow(dead_code)]
pub fn contains_bvar(expr: &Expr, target: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i == target,
        Expr::App(f, a) => contains_bvar(f, target) || contains_bvar(a, target),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_bvar(ty, target) || contains_bvar(body, target + 1)
        }
        Expr::Let(_, ty, val, body) => {
            contains_bvar(ty, target)
                || contains_bvar(val, target)
                || contains_bvar(body, target + 1)
        }
        Expr::Proj(_, _, inner) => contains_bvar(inner, target),
        _ => false,
    }
}
/// Unify two expressions, returning the resulting substitution.
///
/// This is a high-level wrapper around `UnificationState`.
#[allow(dead_code)]
pub fn unify_exprs(lhs: Expr, rhs: Expr) -> Result<Substitution, UnifyError> {
    let mut state = UnificationState::new();
    state.add_eq(lhs, rhs);
    state.run()?;
    Ok(state.subst)
}
/// Unify a list of expression pairs.
#[allow(dead_code)]
pub fn unify_many(pairs: Vec<(Expr, Expr)>) -> Result<Substitution, UnifyError> {
    let mut state = UnificationState::new();
    for (lhs, rhs) in pairs {
        state.add_eq(lhs, rhs);
    }
    state.run()?;
    Ok(state.subst)
}
/// Check whether two expressions are unifiable (ignoring the resulting substitution).
#[allow(dead_code)]
pub fn unifiable(lhs: &Expr, rhs: &Expr) -> bool {
    unify_exprs(lhs.clone(), rhs.clone()).is_ok()
}
#[cfg(test)]
mod unify_extended_tests {
    use super::*;
    use crate::unify::*;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_e() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    fn zero() -> Expr {
        Expr::Const(Name::str("Nat.zero"), vec![])
    }
    fn succ(e: Expr) -> Expr {
        Expr::App(
            Box::new(Expr::Const(Name::str("Nat.succ"), vec![])),
            Box::new(e),
        )
    }
    fn bvar(i: u32) -> Expr {
        Expr::BVar(i)
    }
    #[test]
    fn test_substitution_apply_recursive() {
        let mut subst = Substitution::new();
        subst.insert(0, nat());
        let expr = Expr::App(Box::new(bvar(0)), Box::new(bvar(0)));
        let applied = subst.apply_recursive(&expr);
        assert!(matches!(applied, Expr::App(..)));
        if let Expr::App(f, a) = applied {
            assert!(matches!(*f, Expr::Const(..)));
            assert!(matches!(*a, Expr::Const(..)));
        }
    }
    #[test]
    fn test_substitution_restrict() {
        let mut subst = Substitution::new();
        subst.insert(0, nat());
        subst.insert(1, bool_e());
        let restricted = subst.restrict(&[0]);
        assert!(restricted.contains(0));
        assert!(!restricted.contains(1));
    }
    #[test]
    fn test_substitution_sorted_pairs() {
        let mut subst = Substitution::new();
        subst.insert(2, nat());
        subst.insert(0, bool_e());
        let pairs = subst.sorted_pairs();
        assert_eq!(pairs[0].0, 0);
        assert_eq!(pairs[1].0, 2);
    }
    #[test]
    fn test_levels_equal_zero() {
        assert!(levels_equal(&Level::zero(), &Level::zero()));
    }
    #[test]
    fn test_levels_equal_succ() {
        let l1 = Level::Succ(Box::new(Level::zero()));
        let l2 = Level::Succ(Box::new(Level::zero()));
        assert!(levels_equal(&l1, &l2));
    }
    #[test]
    fn test_levels_equal_different() {
        let l1 = Level::zero();
        let l2 = Level::Succ(Box::new(Level::zero()));
        assert!(!levels_equal(&l1, &l2));
    }
    #[test]
    fn test_normalize_level_max_same() {
        let l = Level::Succ(Box::new(Level::zero()));
        let max = Level::Max(Box::new(l.clone()), Box::new(l.clone()));
        let n = normalize_level(&max);
        assert!(levels_equal(&n, &l));
    }
    #[test]
    fn test_normalize_level_imax_zero() {
        let l = Level::Succ(Box::new(Level::zero()));
        let imax = Level::IMax(Box::new(l), Box::new(Level::zero()));
        let n = normalize_level(&imax);
        assert!(matches!(n, Level::Zero));
    }
    #[test]
    fn test_level_depth() {
        let l = Level::Succ(Box::new(Level::Succ(Box::new(Level::zero()))));
        assert_eq!(level_depth(&l), Some(2));
        let param = Level::Param(Name::str("u"));
        assert_eq!(level_depth(&param), None);
    }
    #[test]
    fn test_level_add() {
        let l = level_add(Level::zero(), 3);
        assert_eq!(level_depth(&l), Some(3));
    }
    #[test]
    fn test_level_max_numeric() {
        let l1 = level_add(Level::zero(), 2);
        let l2 = level_add(Level::zero(), 5);
        let m = level_max(l1, l2);
        assert_eq!(level_depth(&m), Some(5));
    }
    #[test]
    fn test_level_free_params() {
        let l = Level::Max(
            Box::new(Level::Param(Name::str("u"))),
            Box::new(Level::Param(Name::str("v"))),
        );
        let params = level_free_params(&l);
        assert_eq!(params.len(), 2);
    }
    #[test]
    fn test_structurally_equal_same() {
        let subst = Substitution::new();
        assert!(structurally_equal(&nat(), &nat(), &subst));
    }
    #[test]
    fn test_structurally_equal_different() {
        let subst = Substitution::new();
        assert!(!structurally_equal(&nat(), &bool_e(), &subst));
    }
    #[test]
    fn test_structurally_equal_with_subst() {
        let mut subst = Substitution::new();
        subst.insert(0, nat());
        assert!(structurally_equal(&bvar(0), &nat(), &subst));
    }
    #[test]
    fn test_collect_spine_no_app() {
        let n = nat();
        let (head, args) = collect_spine(&n);
        assert!(matches!(head, Expr::Const(..)));
        assert!(args.is_empty());
    }
    #[test]
    fn test_collect_spine_single_app() {
        let expr = succ(zero());
        let (head, args) = collect_spine(&expr);
        assert!(matches!(head, Expr::Const(..)));
        assert_eq!(args.len(), 1);
    }
    #[test]
    fn test_collect_spine_two_args() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(Box::new(f), Box::new(nat()))),
            Box::new(bool_e()),
        );
        let (head, args) = collect_spine(&expr);
        assert!(matches!(head, Expr::Const(..)));
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_rebuild_app_empty() {
        let e = rebuild_app(nat(), vec![]);
        assert!(matches!(e, Expr::Const(..)));
    }
    #[test]
    fn test_rebuild_app_one_arg() {
        let e = rebuild_app(nat(), vec![bool_e()]);
        assert!(matches!(e, Expr::App(..)));
    }
    #[test]
    fn test_beta_subst_simple() {
        let body = bvar(0);
        let result = beta_subst(&body, &nat(), 0);
        assert_eq!(result, nat());
    }
    #[test]
    fn test_beta_subst_no_match() {
        let body = bvar(1);
        let result = beta_subst(&body, &nat(), 0);
        assert!(matches!(result, Expr::BVar(0)));
    }
    #[test]
    fn test_lift_bvars_simple() {
        let e = bvar(0);
        let lifted = lift_bvars(&e, 0, 2);
        assert!(matches!(lifted, Expr::BVar(2)));
    }
    #[test]
    fn test_lift_bvars_below_cutoff() {
        let e = bvar(1);
        let lifted = lift_bvars(&e, 2, 3);
        assert!(matches!(lifted, Expr::BVar(1)));
    }
    #[test]
    fn test_contains_bvar_true() {
        let e = Expr::App(Box::new(nat()), Box::new(bvar(0)));
        assert!(contains_bvar(&e, 0));
    }
    #[test]
    fn test_contains_bvar_false() {
        let e = nat();
        assert!(!contains_bvar(&e, 0));
    }
    #[test]
    fn test_whnf_no_beta() {
        let subst = Substitution::new();
        let e = nat();
        assert_eq!(whnf(&e, &subst), nat());
    }
    #[test]
    fn test_whnf_beta_reduce() {
        use oxilean_kernel::{BinderInfo, Name};
        let subst = Substitution::new();
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(bvar(0)),
        );
        let app = Expr::App(Box::new(lam), Box::new(bool_e()));
        let result = whnf(&app, &subst);
        assert_eq!(result, bool_e());
    }
    #[test]
    fn test_unification_state_add_and_run_trivial() {
        let mut state = UnificationState::new();
        state.add_eq(nat(), nat());
        assert!(state.run().is_ok());
    }
    #[test]
    fn test_unification_state_fail() {
        let mut state = UnificationState::new();
        state.add_eq(nat(), bool_e());
        assert!(state.run().is_err());
    }
    #[test]
    fn test_unification_state_decompose_app() {
        let mut state = UnificationState::new();
        let lhs = Expr::App(Box::new(nat()), Box::new(nat()));
        let rhs = Expr::App(Box::new(nat()), Box::new(nat()));
        state.add_eq(lhs, rhs);
        assert!(state.run().is_ok());
    }
    #[test]
    fn test_unify_exprs_same() {
        let result = unify_exprs(nat(), nat());
        assert!(result.is_ok());
    }
    #[test]
    fn test_unify_exprs_fail() {
        let result = unify_exprs(nat(), bool_e());
        assert!(result.is_err());
    }
    #[test]
    fn test_unify_many_all_same() {
        let pairs = vec![(nat(), nat()), (bool_e(), bool_e())];
        assert!(unify_many(pairs).is_ok());
    }
    #[test]
    fn test_unifiable_true() {
        assert!(unifiable(&nat(), &nat()));
    }
    #[test]
    fn test_unifiable_false() {
        assert!(!unifiable(&nat(), &bool_e()));
    }
    #[test]
    fn test_tracer_record() {
        let mut tracer = UnificationTracer::new();
        tracer.enable();
        tracer.record(TraceEvent::AssignMeta {
            mvar: 0,
            expr: nat(),
        });
        assert_eq!(tracer.count_assignments(), 1);
    }
    #[test]
    fn test_tracer_disabled() {
        let mut tracer = UnificationTracer::new();
        tracer.record(TraceEvent::AssignMeta {
            mvar: 0,
            expr: nat(),
        });
        assert_eq!(tracer.count_assignments(), 0);
    }
    #[test]
    fn test_tracer_clear() {
        let mut tracer = UnificationTracer::new();
        tracer.enable();
        tracer.record(TraceEvent::AssignMeta {
            mvar: 0,
            expr: nat(),
        });
        tracer.clear();
        assert_eq!(tracer.events().len(), 0);
    }
    #[test]
    fn test_constraint_set_drain() {
        let mut cs = ConstraintSet::new();
        cs.eq_expr(nat(), nat());
        cs.eq_expr(bool_e(), bool_e());
        let drained = cs.drain();
        assert_eq!(drained.len(), 2);
        assert!(cs.is_empty());
    }
    #[test]
    fn test_constraint_set_extend() {
        let mut cs1 = ConstraintSet::new();
        let mut cs2 = ConstraintSet::new();
        cs1.eq_expr(nat(), nat());
        cs2.eq_expr(bool_e(), bool_e());
        cs1.extend(cs2);
        assert_eq!(cs1.len(), 2);
    }
    #[test]
    fn test_constraint_set_count_expr_eq() {
        let mut cs = ConstraintSet::new();
        cs.eq_expr(nat(), nat());
        cs.eq_level(Level::zero(), Level::zero());
        assert_eq!(cs.count_expr_eq(), 1);
        assert_eq!(cs.count_level_constraints(), 1);
    }
    #[test]
    fn test_substitution_compose() {
        let mut s1 = Substitution::new();
        s1.insert(0, nat());
        let mut s2 = Substitution::new();
        s2.insert(1, bvar(0));
        let composed = s1.compose(&s2);
        assert_eq!(composed.get(1), Some(&nat()));
    }
}
