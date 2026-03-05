//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    EqualityInfo, MatchResult, RewriteDirection, RewriteHintDb, RewriteHypAnnotation,
    RewriteHypInfo, RewriteLoopConfig, RewritePosition, RewriteRule, RewriteSeq, RewriteSequence,
    RewriteStep, RewriteSystem, RewriteTacticStats, RewriteTrace, SetoidRewrite,
};
use crate::basic::{MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};

/// `rewrite [name]` — rewrite using a named hypothesis or constant.
///
/// Given a hypothesis `name : a = b` in the local context, rewrites
/// occurrences of `a` to `b` in the goal (forward direction) or
/// `b` to `a` (backward direction).
pub fn tac_rewrite_named(
    name: &Name,
    direction: RewriteDirection,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let hyps = ctx.get_local_hyps();
    let hyp_type = hyps
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, ty)| ty.clone())
        .ok_or_else(|| TacticError::Failed(format!("rewrite: hypothesis '{}' not found", name)))?;
    let eq_info = EqualityInfo::from_expr(&hyp_type)
        .ok_or_else(|| TacticError::Failed(format!("rewrite: '{}' is not an equality", name)))?;
    let proof = Expr::Const(name.clone(), vec![]);
    tac_rewrite(proof, &eq_info, direction, state, ctx)
}
/// `rewrite [h]` — rewrite the goal using an equality proof.
///
/// Given `h : a = b` and goal `... a ...`, produces goal `... b ...`
/// (for forward direction).
pub fn tac_rewrite(
    eq_proof: Expr,
    eq_type: &EqualityInfo,
    direction: RewriteDirection,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let (from, to) = match direction {
        RewriteDirection::Forward => (&eq_type.lhs, &eq_type.rhs),
        RewriteDirection::Backward => (&eq_type.rhs, &eq_type.lhs),
    };
    let new_target = replace_subexpr(&target, from, to);
    if new_target == target {
        return Err(TacticError::Failed(
            "rewrite: pattern not found in goal".into(),
        ));
    }
    let (new_goal_id, new_goal_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    let proof = build_rewrite_proof(&eq_proof, &new_goal_expr, direction);
    ctx.assign_mvar(goal, proof);
    state.replace_goal(vec![new_goal_id]);
    Ok(())
}
/// Replace occurrences of `from` with `to` in an expression.
pub(super) fn replace_subexpr(expr: &Expr, from: &Expr, to: &Expr) -> Expr {
    if expr == from {
        return to.clone();
    }
    match expr {
        Expr::App(f, a) => {
            let f2 = replace_subexpr(f, from, to);
            let a2 = replace_subexpr(a, from, to);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = replace_subexpr(ty, from, to);
            let body2 = replace_subexpr(body, from, to);
            Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = replace_subexpr(ty, from, to);
            let body2 = replace_subexpr(body, from, to);
            Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = replace_subexpr(ty, from, to);
            let val2 = replace_subexpr(val, from, to);
            let body2 = replace_subexpr(body, from, to);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = replace_subexpr(e, from, to);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Build the proof term for a rewrite step.
pub(super) fn build_rewrite_proof(
    eq_proof: &Expr,
    new_goal: &Expr,
    direction: RewriteDirection,
) -> Expr {
    let transport_fn = match direction {
        RewriteDirection::Forward => Name::str("Eq.mpr"),
        RewriteDirection::Backward => Name::str("Eq.mp"),
    };
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(transport_fn, vec![Level::zero()])),
            Box::new(eq_proof.clone()),
        )),
        Box::new(new_goal.clone()),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::rewrite::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_eq_type(lhs: Expr, rhs: Expr) -> Expr {
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![Level::zero()])),
                    Box::new(nat_ty),
                )),
                Box::new(lhs),
            )),
            Box::new(rhs),
        )
    }
    #[test]
    fn test_equality_info_from_expr() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq_ty = mk_eq_type(a.clone(), b.clone());
        let info = EqualityInfo::from_expr(&eq_ty).expect("info should be present");
        assert_eq!(info.lhs, a);
        assert_eq!(info.rhs, b);
    }
    #[test]
    fn test_equality_info_non_eq() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert!(EqualityInfo::from_expr(&expr).is_none());
    }
    #[test]
    fn test_replace_subexpr() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let f = Expr::Const(Name::str("f"), vec![]);
        let fa = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let fb = Expr::App(Box::new(f), Box::new(b.clone()));
        let result = replace_subexpr(&fa, &a, &b);
        assert_eq!(result, fb);
    }
    #[test]
    fn test_replace_subexpr_no_match() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let result = replace_subexpr(&a, &b, &c);
        assert_eq!(result, a);
    }
    #[test]
    fn test_rewrite_direction() {
        assert_eq!(RewriteDirection::Forward, RewriteDirection::Forward);
        assert_ne!(RewriteDirection::Forward, RewriteDirection::Backward);
    }
    #[test]
    fn test_rewrite_forward() {
        let mut ctx = mk_ctx();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let p = Expr::Const(Name::str("P"), vec![]);
        let goal_ty = Expr::App(Box::new(p), Box::new(a.clone()));
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let eq_info = EqualityInfo {
            ty: nat_ty,
            lhs: a,
            rhs: b,
        };
        let h = Expr::Const(Name::str("h"), vec![]);
        let result = tac_rewrite(h, &eq_info, RewriteDirection::Forward, &mut state, &mut ctx);
        assert!(result.is_ok());
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_rewrite_no_match() {
        let mut ctx = mk_ctx();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let p = Expr::Const(Name::str("P"), vec![]);
        let goal_ty = Expr::App(Box::new(p), Box::new(c));
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let eq_info = EqualityInfo {
            ty: nat_ty,
            lhs: a,
            rhs: b,
        };
        let h = Expr::Const(Name::str("h"), vec![]);
        let result = tac_rewrite(h, &eq_info, RewriteDirection::Forward, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_rewrite_forward_lit() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let one = Expr::Lit(oxilean_kernel::Literal::Nat(1));
        let x = Expr::Const(Name::str("x"), vec![]);
        let goal_ty = mk_eq_type(x.clone(), one.clone());
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let eq_info = EqualityInfo {
            ty: nat_ty,
            lhs: x,
            rhs: zero,
        };
        let eq_proof = Expr::Const(Name::str("h"), vec![]);
        let result = tac_rewrite(
            eq_proof,
            &eq_info,
            RewriteDirection::Forward,
            &mut state,
            &mut ctx,
        );
        assert!(
            result.is_ok(),
            "forward rewrite should succeed: {:?}",
            result
        );
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_rewrite_not_found_in_goal() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let zero = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        let one = Expr::Lit(oxilean_kernel::Literal::Nat(1));
        let two = Expr::Lit(oxilean_kernel::Literal::Nat(2));
        let three = Expr::Lit(oxilean_kernel::Literal::Nat(3));
        let goal_ty = mk_eq_type(zero, one);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let eq_info = EqualityInfo {
            ty: nat_ty,
            lhs: two,
            rhs: three,
        };
        let eq_proof = Expr::Const(Name::str("h"), vec![]);
        let result = tac_rewrite(
            eq_proof,
            &eq_info,
            RewriteDirection::Forward,
            &mut state,
            &mut ctx,
        );
        assert!(
            result.is_err(),
            "rewrite with non-matching pattern should fail"
        );
    }
    #[test]
    fn test_rewrite_backward() {
        let mut ctx = mk_ctx();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let p = Expr::Const(Name::str("P"), vec![]);
        let goal_ty = Expr::App(Box::new(p), Box::new(b.clone()));
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let eq_info = EqualityInfo {
            ty: nat_ty,
            lhs: a,
            rhs: b,
        };
        let h = Expr::Const(Name::str("h"), vec![]);
        let result = tac_rewrite(
            h,
            &eq_info,
            RewriteDirection::Backward,
            &mut state,
            &mut ctx,
        );
        assert!(result.is_ok(), "backward rewrite should succeed");
        assert_eq!(state.num_goals(), 1);
    }
}
/// Count occurrences of `pattern` in `expr`.
pub fn count_occurrences(expr: &Expr, pattern: &Expr) -> usize {
    if expr == pattern {
        return 1;
    }
    match expr {
        Expr::App(f, a) => count_occurrences(f, pattern) + count_occurrences(a, pattern),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_occurrences(ty, pattern) + count_occurrences(body, pattern)
        }
        Expr::Let(_, ty, val, body) => {
            count_occurrences(ty, pattern)
                + count_occurrences(val, pattern)
                + count_occurrences(body, pattern)
        }
        Expr::Proj(_, _, e) => count_occurrences(e, pattern),
        _ => 0,
    }
}
/// Replace only the first occurrence of `from` with `to`.
pub fn replace_first_occurrence(expr: &Expr, from: &Expr, to: &Expr) -> (Expr, bool) {
    if expr == from {
        return (to.clone(), true);
    }
    match expr {
        Expr::App(f, a) => {
            let (f2, found) = replace_first_occurrence(f, from, to);
            if found {
                return (Expr::App(Box::new(f2), a.clone()), true);
            }
            let (a2, found) = replace_first_occurrence(a, from, to);
            (Expr::App(Box::new(f2), Box::new(a2)), found)
        }
        Expr::Lam(bi, n, ty, body) => {
            let (ty2, found) = replace_first_occurrence(ty, from, to);
            if found {
                return (Expr::Lam(*bi, n.clone(), Box::new(ty2), body.clone()), true);
            }
            let (body2, found) = replace_first_occurrence(body, from, to);
            (
                Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2)),
                found,
            )
        }
        Expr::Pi(bi, n, ty, body) => {
            let (ty2, found) = replace_first_occurrence(ty, from, to);
            if found {
                return (Expr::Pi(*bi, n.clone(), Box::new(ty2), body.clone()), true);
            }
            let (body2, found) = replace_first_occurrence(body, from, to);
            (
                Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2)),
                found,
            )
        }
        _ => (expr.clone(), false),
    }
}
/// `rw_hyp [h] at hyp` — rewrite inside a hypothesis.
pub fn tac_rewrite_hyp(
    eq_name: &Name,
    direction: RewriteDirection,
    hyp_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let hyps = ctx.get_local_hyps();
    let eq_ty = hyps
        .iter()
        .find(|(n, _)| n == eq_name)
        .map(|(_, ty)| ty.clone())
        .ok_or_else(|| TacticError::Failed(format!("rewrite: '{}' not found", eq_name)))?;
    let eq_info = EqualityInfo::from_expr(&eq_ty)
        .ok_or_else(|| TacticError::Failed(format!("{} is not an equality", eq_name)))?;
    let hyp_ty = hyps
        .iter()
        .find(|(n, _)| n == hyp_name)
        .map(|(_, ty)| ty.clone())
        .ok_or_else(|| TacticError::UnknownHyp(hyp_name.clone()))?;
    let (from, to) = match direction {
        RewriteDirection::Forward => (&eq_info.lhs, &eq_info.rhs),
        RewriteDirection::Backward => (&eq_info.rhs, &eq_info.lhs),
    };
    let new_hyp_ty = replace_subexpr(&hyp_ty, from, to);
    if new_hyp_ty == hyp_ty {
        return Err(TacticError::Failed("rewrite_hyp: pattern not found".into()));
    }
    ctx.clear_local(hyp_name);
    ctx.mk_local_decl(
        hyp_name.clone(),
        new_hyp_ty,
        oxilean_kernel::BinderInfo::Default,
    );
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(target, crate::basic::MetavarKind::Natural);
    ctx.assign_mvar(goal, new_expr);
    state.replace_goal(vec![new_id]);
    Ok(())
}
/// Check if an expression is an equality.
pub fn is_equality_expr(expr: &Expr) -> bool {
    EqualityInfo::from_expr(expr).is_some()
}
/// Collect equalities from hypotheses.
pub fn collect_equalities(hyps: &[(Name, Expr)]) -> Vec<(Name, EqualityInfo)> {
    hyps.iter()
        .filter_map(|(n, ty)| EqualityInfo::from_expr(ty).map(|info| (n.clone(), info)))
        .collect()
}
/// Build an equality expression `@Eq α a b`.
pub fn mk_eq_expr(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Eq"), vec![Level::zero()])),
                Box::new(ty),
            )),
            Box::new(lhs),
        )),
        Box::new(rhs),
    )
}
/// Build `@Eq.refl α a`.
pub fn mk_eq_refl_expr(ty: Expr, a: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Eq.refl"), vec![Level::zero()])),
            Box::new(ty),
        )),
        Box::new(a),
    )
}
#[cfg(test)]
mod extended_rewrite_tests {
    use super::*;
    use crate::tactic::rewrite::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    fn mk_eq(lhs: Expr, rhs: Expr) -> Expr {
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        mk_eq_expr(nat_ty, lhs, rhs)
    }
    #[test]
    fn test_count_occurrences() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let expr = Expr::App(Box::new(a.clone()), Box::new(a.clone()));
        assert_eq!(count_occurrences(&expr, &a), 2);
    }
    #[test]
    fn test_count_zero() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        assert_eq!(count_occurrences(&a, &b), 0);
    }
    #[test]
    fn test_replace_first() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let expr = Expr::App(Box::new(a.clone()), Box::new(a.clone()));
        let (result, found) = replace_first_occurrence(&expr, &a, &b);
        assert!(found);
        if let Expr::App(f, arg) = result {
            assert_eq!(*f, b);
            assert_eq!(*arg, a);
        }
    }
    #[test]
    fn test_replace_first_not_found() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let (result, found) = replace_first_occurrence(&a, &b, &c);
        assert!(!found);
        assert_eq!(result, a);
    }
    #[test]
    fn test_is_equality_expr() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq = mk_eq(a, b);
        assert!(is_equality_expr(&eq));
        assert!(!is_equality_expr(&Expr::BVar(0)));
    }
    #[test]
    fn test_collect_equalities() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq = mk_eq(a, b);
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let hyps = vec![(Name::str("h"), eq), (Name::str("n"), nat)];
        let eqs = collect_equalities(&hyps);
        assert_eq!(eqs.len(), 1);
    }
    #[test]
    fn test_mk_eq_expr() {
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq = mk_eq_expr(nat_ty, a.clone(), b.clone());
        let info = EqualityInfo::from_expr(&eq).expect("info should be present");
        assert_eq!(info.lhs, a);
        assert_eq!(info.rhs, b);
    }
    #[test]
    fn test_rewrite_sequence_empty() {
        let mut ctx = mk_ctx();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(
            Expr::Const(Name::str("P"), vec![]),
            crate::basic::MetavarKind::Natural,
        );
        let mut state = TacticState::single(mvar_id);
        let seq = RewriteSequence::new();
        assert_eq!(
            seq.execute(&mut state, &mut ctx)
                .expect("value should be present"),
            0
        );
    }
    #[test]
    fn test_rewrite_sequence_builder() {
        let seq = RewriteSequence::new()
            .then_forward(Name::str("h1"))
            .then_backward(Name::str("h2"));
        assert_eq!(seq.rewrites.len(), 2);
        assert_eq!(seq.rewrites[0].1, RewriteDirection::Forward);
        assert_eq!(seq.rewrites[1].1, RewriteDirection::Backward);
    }
    #[test]
    fn test_mk_eq_refl_expr() {
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let refl = mk_eq_refl_expr(nat_ty, a);
        assert!(matches!(refl, Expr::App(_, _)));
    }
    #[test]
    fn test_tac_rewrite_hyp_fail() {
        let mut ctx = mk_ctx();
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(
            Expr::Const(Name::str("P"), vec![]),
            crate::basic::MetavarKind::Natural,
        );
        let mut state = TacticState::single(mvar_id);
        assert!(tac_rewrite_hyp(
            &Name::str("h"),
            RewriteDirection::Forward,
            &Name::str("hyp"),
            &mut state,
            &mut ctx
        )
        .is_err());
    }
    #[test]
    fn test_equality_info_from_expr() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq = mk_eq(a.clone(), b.clone());
        let info = EqualityInfo::from_expr(&eq).expect("info should be present");
        assert_eq!(info.lhs, a);
        assert_eq!(info.rhs, b);
    }
    #[test]
    fn test_equality_info_non_eq() {
        assert!(EqualityInfo::from_expr(&Expr::BVar(0)).is_none());
    }
    #[test]
    fn test_replace_subexpr_nested() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let f = Expr::Const(Name::str("f"), vec![]);
        let inner = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let outer = Expr::App(Box::new(f.clone()), Box::new(inner));
        let result = replace_subexpr(&outer, &a, &b);
        let expected = Expr::App(
            Box::new(f.clone()),
            Box::new(Expr::App(Box::new(f), Box::new(b))),
        );
        assert_eq!(result, expected);
    }
}
/// Collect all equality hypotheses from context.
///
/// Returns a list of `RewriteHypInfo` for each hypothesis `h : lhs = rhs`.
#[allow(dead_code)]
pub fn collect_eq_hyps(ctx: &MetaContext) -> Vec<RewriteHypInfo> {
    ctx.get_local_hyps()
        .into_iter()
        .filter_map(|(name, ty)| {
            if let Some((lhs, rhs)) = extract_eq_sides(&ty) {
                Some(RewriteHypInfo {
                    name: name.to_string(),
                    lhs,
                    rhs,
                })
            } else {
                None
            }
        })
        .collect()
}
/// Extract the two sides of an equality `lhs = rhs` from an expression.
///
/// Returns `None` if the expression is not an equality.
#[allow(dead_code)]
pub fn extract_eq_sides(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(f, rhs) = expr {
        if let Expr::App(f2, lhs) = f.as_ref() {
            if let Expr::App(eq_const, _ty) = f2.as_ref() {
                if matches!(
                    eq_const.as_ref(), Expr::Const(n, _) if n == & Name::str("Eq")
                ) {
                    return Some((*lhs.clone(), *rhs.clone()));
                }
            }
        }
    }
    None
}
/// Build an `Eq T lhs rhs` expression.
#[allow(dead_code)]
pub fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![]);
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(Box::new(eq_const), Box::new(ty))),
            Box::new(lhs),
        )),
        Box::new(rhs),
    )
}
/// Build an `Eq.refl T a` expression (proof that `a = a`).
#[allow(dead_code)]
pub fn mk_eq_refl(ty: Expr, a: Expr) -> Expr {
    let refl_const = Expr::Const(Name::str("Eq.refl"), vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(refl_const), Box::new(ty))),
        Box::new(a),
    )
}
/// Build an `Eq.symm` expression (flip `lhs = rhs` to `rhs = lhs`).
#[allow(dead_code)]
pub fn mk_eq_symm(ty: Expr, lhs: Expr, rhs: Expr, proof: Expr) -> Expr {
    let symm_const = Expr::Const(Name::str("Eq.symm"), vec![]);
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(Box::new(symm_const), Box::new(ty))),
                Box::new(lhs),
            )),
            Box::new(rhs),
        )),
        Box::new(proof),
    )
}
/// Count the number of times `pattern` occurs in `expr` (syntactic equality).
#[allow(dead_code)]
pub fn count_occurrences_expr(expr: &Expr, pattern: &Expr) -> usize {
    if expr == pattern {
        return 1;
    }
    match expr {
        Expr::App(f, a) => count_occurrences_expr(f, pattern) + count_occurrences_expr(a, pattern),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            count_occurrences_expr(dom, pattern) + count_occurrences_expr(body, pattern)
        }
        Expr::Let(_, ty, val, body) => {
            count_occurrences_expr(ty, pattern)
                + count_occurrences_expr(val, pattern)
                + count_occurrences_expr(body, pattern)
        }
        _ => 0,
    }
}
/// Replace the first occurrence of `old` with `new` in `expr`.
#[allow(dead_code)]
pub fn replace_first_expr(expr: &Expr, old: &Expr, new: &Expr) -> (Expr, bool) {
    if expr == old {
        return (new.clone(), true);
    }
    match expr {
        Expr::App(f, a) => {
            let (new_f, replaced) = replace_first_expr(f, old, new);
            if replaced {
                return (Expr::App(Box::new(new_f), a.clone()), true);
            }
            let (new_a, replaced) = replace_first_expr(a, old, new);
            (Expr::App(Box::new(new_f), Box::new(new_a)), replaced)
        }
        Expr::Lam(bi, n, dom, body) => {
            let (new_dom, replaced) = replace_first_expr(dom, old, new);
            if replaced {
                return (
                    Expr::Lam(*bi, n.clone(), Box::new(new_dom), body.clone()),
                    true,
                );
            }
            let (new_body, replaced) = replace_first_expr(body, old, new);
            (
                Expr::Lam(*bi, n.clone(), Box::new(new_dom), Box::new(new_body)),
                replaced,
            )
        }
        Expr::Pi(bi, n, dom, body) => {
            let (new_dom, replaced) = replace_first_expr(dom, old, new);
            if replaced {
                return (
                    Expr::Pi(*bi, n.clone(), Box::new(new_dom), body.clone()),
                    true,
                );
            }
            let (new_body, replaced) = replace_first_expr(body, old, new);
            (
                Expr::Pi(*bi, n.clone(), Box::new(new_dom), Box::new(new_body)),
                replaced,
            )
        }
        other => (other.clone(), false),
    }
}
/// Check whether an expression contains a given subexpression.
#[allow(dead_code)]
pub fn contains_subexpr(expr: &Expr, pattern: &Expr) -> bool {
    count_occurrences_expr(expr, pattern) > 0
}
#[cfg(test)]
mod rewrite_extra_tests {
    use super::*;
    use crate::tactic::rewrite::*;
    #[test]
    fn test_rewrite_seq_empty() {
        let seq = RewriteSeq::empty();
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }
    #[test]
    fn test_rewrite_seq_then() {
        let seq = RewriteSeq::empty()
            .then(Name::str("h1"))
            .then_rev(Name::str("h2"));
        assert_eq!(seq.len(), 2);
        assert!(!seq.steps[0].1);
        assert!(seq.steps[1].1);
    }
    #[test]
    fn test_extract_eq_sides_non_eq() {
        let e = Expr::Sort(Level::zero());
        assert!(extract_eq_sides(&e).is_none());
    }
    #[test]
    fn test_extract_eq_sides_eq() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let ty = Expr::Sort(Level::succ(Level::zero()));
        let eq = mk_eq(ty, a.clone(), b.clone());
        let sides = extract_eq_sides(&eq);
        assert!(sides.is_some());
        let (lhs, rhs) = sides.expect("sides should be valid");
        assert_eq!(lhs, a);
        assert_eq!(rhs, b);
    }
    #[test]
    fn test_mk_eq_refl_structure() {
        let ty = Expr::Sort(Level::zero());
        let a = Expr::Const(Name::str("a"), vec![]);
        let refl = mk_eq_refl(ty, a);
        assert!(matches!(refl, Expr::App(_, _)));
    }
    #[test]
    fn test_count_occurrences_expr_zero() {
        let e = Expr::Const(Name::str("a"), vec![]);
        let pattern = Expr::Const(Name::str("b"), vec![]);
        assert_eq!(count_occurrences_expr(&e, &pattern), 0);
    }
    #[test]
    fn test_count_occurrences_expr_one() {
        let a = Expr::Const(Name::str("a"), vec![]);
        assert_eq!(count_occurrences_expr(&a, &a), 1);
    }
    #[test]
    fn test_count_occurrences_expr_app() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let e = Expr::App(Box::new(a.clone()), Box::new(a.clone()));
        assert_eq!(count_occurrences_expr(&e, &a), 2);
    }
    #[test]
    fn test_contains_subexpr_true() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let e = Expr::App(Box::new(a.clone()), Box::new(b));
        assert!(contains_subexpr(&e, &a));
    }
    #[test]
    fn test_contains_subexpr_false() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let e = Expr::App(Box::new(a), Box::new(b));
        assert!(!contains_subexpr(&e, &c));
    }
    #[test]
    fn test_replace_first_expr_identity() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let (result, replaced) = replace_first_expr(&a, &b, &c);
        assert!(!replaced);
        assert_eq!(result, a);
    }
    #[test]
    fn test_replace_first_expr_hit() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let (result, replaced) = replace_first_expr(&a, &a, &b);
        assert!(replaced);
        assert_eq!(result, b);
    }
    #[test]
    fn test_collect_eq_hyps_empty() {
        let env = oxilean_kernel::Environment::new();
        let ctx_ref = crate::basic::MetaContext::new(env);
        let hyps = collect_eq_hyps(&ctx_ref);
        assert!(hyps.is_empty());
    }
}
/// Collect all positions in `expr` where `pattern` occurs.
#[allow(dead_code)]
pub fn find_positions(expr: &Expr, pattern: &Expr) -> Vec<RewritePosition> {
    let mut positions = Vec::new();
    find_positions_rec(expr, pattern, &RewritePosition::root(), &mut positions);
    positions
}
pub(super) fn find_positions_rec(
    expr: &Expr,
    pattern: &Expr,
    pos: &RewritePosition,
    acc: &mut Vec<RewritePosition>,
) {
    if expr == pattern {
        acc.push(pos.clone());
        return;
    }
    match expr {
        Expr::App(f, a) => {
            find_positions_rec(f, pattern, &pos.extend(0), acc);
            find_positions_rec(a, pattern, &pos.extend(1), acc);
        }
        Expr::Lam(_, _, ty, body) => {
            find_positions_rec(ty, pattern, &pos.extend(3), acc);
            find_positions_rec(body, pattern, &pos.extend(2), acc);
        }
        Expr::Pi(_, _, ty, body) => {
            find_positions_rec(ty, pattern, &pos.extend(3), acc);
            find_positions_rec(body, pattern, &pos.extend(2), acc);
        }
        Expr::Let(_, ty, val, body) => {
            find_positions_rec(ty, pattern, &pos.extend(3), acc);
            find_positions_rec(val, pattern, &pos.extend(1), acc);
            find_positions_rec(body, pattern, &pos.extend(2), acc);
        }
        Expr::Proj(_, _, e) => {
            find_positions_rec(e, pattern, &pos.extend(0), acc);
        }
        _ => {}
    }
}
/// Replace only the n-th occurrence (0-indexed) of `from` with `to`.
#[allow(dead_code)]
pub fn replace_nth_occurrence(expr: &Expr, from: &Expr, to: &Expr, n: usize) -> (Expr, bool) {
    let mut counter = 0usize;
    replace_nth_rec(expr, from, to, n, &mut counter)
}
pub(super) fn replace_nth_rec(
    expr: &Expr,
    from: &Expr,
    to: &Expr,
    target: usize,
    counter: &mut usize,
) -> (Expr, bool) {
    if expr == from {
        if *counter == target {
            *counter += 1;
            return (to.clone(), true);
        }
        *counter += 1;
    }
    match expr {
        Expr::App(f, a) => {
            let (f2, done) = replace_nth_rec(f, from, to, target, counter);
            if done {
                return (Expr::App(Box::new(f2), a.clone()), true);
            }
            let (a2, done) = replace_nth_rec(a, from, to, target, counter);
            (Expr::App(Box::new(f2), Box::new(a2)), done)
        }
        Expr::Lam(bi, n, ty, body) => {
            let (ty2, done) = replace_nth_rec(ty, from, to, target, counter);
            if done {
                return (Expr::Lam(*bi, n.clone(), Box::new(ty2), body.clone()), true);
            }
            let (body2, done) = replace_nth_rec(body, from, to, target, counter);
            (
                Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2)),
                done,
            )
        }
        Expr::Pi(bi, n, ty, body) => {
            let (ty2, done) = replace_nth_rec(ty, from, to, target, counter);
            if done {
                return (Expr::Pi(*bi, n.clone(), Box::new(ty2), body.clone()), true);
            }
            let (body2, done) = replace_nth_rec(body, from, to, target, counter);
            (
                Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2)),
                done,
            )
        }
        other => (other.clone(), false),
    }
}
/// Try to rewrite using a rule that has conditions.
///
/// The conditions must each be provable as `True` (trivially) for the rule to fire.
#[allow(dead_code)]
pub fn try_conditional_rewrite(rule: &RewriteRule, expr: &Expr) -> Option<Expr> {
    for cond in &rule.conditions {
        if !is_trivially_true(cond) {
            return None;
        }
    }
    rule.apply(expr)
}
/// Check whether an expression is trivially `True`.
#[allow(dead_code)]
pub fn is_trivially_true(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(n, _) if * n == Name::str("True"))
}
/// Check whether an expression is trivially `False`.
#[allow(dead_code)]
pub fn is_trivially_false(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(n, _) if * n == Name::str("False"))
}
/// Run the rewrite system exhaustively, collecting a trace.
#[allow(dead_code)]
pub fn run_rewrite_loop(
    system: &RewriteSystem,
    expr: &Expr,
    config: &RewriteLoopConfig,
) -> RewriteTrace {
    let mut trace = RewriteTrace::new();
    let mut current = expr.clone();
    let mut steps_taken = 0;
    loop {
        if steps_taken >= config.max_steps {
            break;
        }
        let rewritten = system.apply_anywhere_once(&current);
        match rewritten {
            Some(new_expr) => {
                trace.push(RewriteStep {
                    rule_name: Name::str("?"),
                    direction: RewriteDirection::Forward,
                    occurrence: 0,
                    before: current.clone(),
                    after: new_expr.clone(),
                });
                current = new_expr;
                steps_taken += 1;
                if config.stop_after_first {
                    break;
                }
            }
            None => break,
        }
    }
    trace.final_expr = Some(current);
    trace
}
/// Build an `Eq.congr f h` proof: `a = b → f a = f b`.
#[allow(dead_code)]
pub fn build_congr_proof(f: Expr, h: Expr) -> Expr {
    let congr_arg = Expr::Const(Name::str("congrArg"), vec![Level::zero()]);
    Expr::App(
        Box::new(Expr::App(Box::new(congr_arg), Box::new(f))),
        Box::new(h),
    )
}
/// Build an `Eq.trans h1 h2` proof.
#[allow(dead_code)]
pub fn build_trans_proof(h1: Expr, h2: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Eq.trans"), vec![Level::zero()])),
            Box::new(h1),
        )),
        Box::new(h2),
    )
}
/// Build an `Eq.symm h` proof.
#[allow(dead_code)]
pub fn build_symm_proof(h: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Eq.symm"), vec![Level::zero()])),
        Box::new(h),
    )
}
/// Describe the effect of a rewrite on an expression.
#[allow(dead_code)]
pub fn describe_rewrite(
    before: &Expr,
    after: &Expr,
    rule_name: &Name,
    direction: RewriteDirection,
) -> String {
    let dir_str = match direction {
        RewriteDirection::Forward => "→",
        RewriteDirection::Backward => "←",
    };
    format!(
        "rw [{}{}]: {} ↦ {}",
        dir_str,
        rule_name,
        expr_display(before),
        expr_display(after)
    )
}
/// Simple display helper for expressions.
#[allow(dead_code)]
pub(super) fn expr_display(expr: &Expr) -> String {
    match expr {
        Expr::Const(n, _) => n.to_string(),
        Expr::BVar(i) => format!("#{}", i),
        Expr::App(f, a) => format!("({} {})", expr_display(f), expr_display(a)),
        Expr::Lam(_, n, _, b) => format!("(fun {} => ...{}...)", n, expr_display(b)),
        Expr::Pi(_, n, _, b) => format!("(Π {} : _ . {})", n, expr_display(b)),
        Expr::Let(n, _, _, b) => format!("(let {} := _ in {})", n, expr_display(b)),
        Expr::Sort(l) => format!("Sort({})", l),
        Expr::Lit(oxilean_kernel::Literal::Nat(n)) => format!("{}", n),
        Expr::Lit(oxilean_kernel::Literal::Str(s)) => format!("{:?}", s),
        Expr::Proj(_, i, e) => format!("{}.{}", expr_display(e), i),
        Expr::FVar(id) => format!("fv({})", id.0),
    }
}
/// Beta reduce an expression (one step at the top).
#[allow(dead_code)]
pub fn beta_reduce_once(expr: &Expr) -> Option<Expr> {
    if let Expr::App(f, a) = expr {
        if let Expr::Lam(_, _, _, body) = f.as_ref() {
            return Some(subst_bvar(body, 0, a));
        }
    }
    None
}
/// Beta reduce an expression to normal form (up to `limit` steps).
#[allow(dead_code)]
pub fn beta_normalize(expr: &Expr, limit: usize) -> Expr {
    let mut current = expr.clone();
    for _ in 0..limit {
        if let Some(reduced) = beta_reduce_once(&current) {
            current = reduced;
        } else {
            break;
        }
    }
    current
}
/// Substitute `replacement` for BVar(depth) in `expr`.
#[allow(dead_code)]
pub fn subst_bvar(expr: &Expr, depth: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i == depth {
                replacement.clone()
            } else if *i > depth {
                Expr::BVar(i - 1)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f2 = subst_bvar(f, depth, replacement);
            let a2 = subst_bvar(a, depth, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = subst_bvar(ty, depth, replacement);
            let body2 = subst_bvar(body, depth + 1, replacement);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = subst_bvar(ty, depth, replacement);
            let body2 = subst_bvar(body, depth + 1, replacement);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(n, ty, val, body) => {
            let ty2 = subst_bvar(ty, depth, replacement);
            let val2 = subst_bvar(val, depth, replacement);
            let body2 = subst_bvar(body, depth + 1, replacement);
            Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = subst_bvar(e, depth, replacement);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Try all equality hypotheses in the context and rewrite the goal.
///
/// Returns the name of the first hypothesis that successfully rewrote.
#[allow(dead_code)]
pub fn tac_auto_rewrite(
    direction: RewriteDirection,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> crate::tactic::state::TacticResult<Option<Name>> {
    let hyps = ctx.get_local_hyps();
    let eq_hyps: Vec<Name> = hyps
        .iter()
        .filter_map(|(n, ty)| {
            if EqualityInfo::from_expr(ty).is_some() {
                Some(n.clone())
            } else {
                None
            }
        })
        .collect();
    for name in eq_hyps {
        if tac_rewrite_named(&name, direction, state, ctx).is_ok() {
            return Ok(Some(name));
        }
    }
    Ok(None)
}
/// Try all equality hypotheses in sequence until one succeeds.
#[allow(dead_code)]
pub fn tac_rewrite_any(
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> crate::tactic::state::TacticResult<()> {
    let result = tac_auto_rewrite(RewriteDirection::Forward, state, ctx)?;
    if result.is_none() {
        return Err(crate::tactic::state::TacticError::Failed(
            "rewrite_any: no applicable equality hypothesis found".into(),
        ));
    }
    Ok(())
}
/// Replace at a specific position in an expression.
///
/// The `path` is a sequence of child indices as described in `RewritePosition`.
#[allow(dead_code)]
pub fn replace_at_position(expr: &Expr, path: &[usize], replacement: &Expr) -> Option<Expr> {
    if path.is_empty() {
        return Some(replacement.clone());
    }
    let (head, tail) = path
        .split_first()
        .expect("path is non-empty; checked above");
    match expr {
        Expr::App(f, a) => match head {
            0 => {
                let new_f = replace_at_position(f, tail, replacement)?;
                Some(Expr::App(Box::new(new_f), a.clone()))
            }
            1 => {
                let new_a = replace_at_position(a, tail, replacement)?;
                Some(Expr::App(f.clone(), Box::new(new_a)))
            }
            _ => None,
        },
        Expr::Lam(bi, n, ty, body) => match head {
            2 => {
                let new_body = replace_at_position(body, tail, replacement)?;
                Some(Expr::Lam(*bi, n.clone(), ty.clone(), Box::new(new_body)))
            }
            3 => {
                let new_ty = replace_at_position(ty, tail, replacement)?;
                Some(Expr::Lam(*bi, n.clone(), Box::new(new_ty), body.clone()))
            }
            _ => None,
        },
        Expr::Pi(bi, n, ty, body) => match head {
            2 => {
                let new_body = replace_at_position(body, tail, replacement)?;
                Some(Expr::Pi(*bi, n.clone(), ty.clone(), Box::new(new_body)))
            }
            3 => {
                let new_ty = replace_at_position(ty, tail, replacement)?;
                Some(Expr::Pi(*bi, n.clone(), Box::new(new_ty), body.clone()))
            }
            _ => None,
        },
        _ => None,
    }
}
/// Build a proof that if `h : a = b` and `h2 : b = c` then `a = c`.
#[allow(dead_code)]
pub fn chain_equalities(proofs: &[Expr]) -> Option<Expr> {
    match proofs {
        [] => None,
        [p] => Some(p.clone()),
        [first, rest @ ..] => {
            let mut acc = first.clone();
            for proof in rest {
                acc = build_trans_proof(acc, proof.clone());
            }
            Some(acc)
        }
    }
}
/// Flip an equality proof: `h : a = b` → `h.symm : b = a`.
#[allow(dead_code)]
pub fn flip_eq_proof(proof: Expr) -> Expr {
    build_symm_proof(proof)
}
/// Check if two expressions are equal modulo rewriting by the given system.
#[allow(dead_code)]
pub fn equal_modulo_rewriting(
    lhs: &Expr,
    rhs: &Expr,
    system: &RewriteSystem,
    steps: usize,
) -> bool {
    let (lhs_nf, _) = system.normalize(lhs, steps);
    let (rhs_nf, _) = system.normalize(rhs, steps);
    lhs_nf == rhs_nf
}
/// Check if an expression is in normal form w.r.t. a rewrite system.
#[allow(dead_code)]
pub fn is_normal_form(expr: &Expr, system: &RewriteSystem) -> bool {
    system.apply_anywhere_once(expr).is_none()
}
/// Rewrite in all goals, not just the current one.
#[allow(dead_code)]
pub fn tac_rewrite_all_goals(
    eq_name: &Name,
    direction: RewriteDirection,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> crate::tactic::state::TacticResult<usize> {
    let num_goals = state.num_goals();
    let mut count = 0;
    for _i in 0..num_goals {
        if tac_rewrite_named(eq_name, direction, state, ctx).is_ok() {
            count += 1;
        }
    }
    Ok(count)
}
/// `subst h` — substitute a hypothesis of the form `x = t` or `t = x`.
///
/// Replaces all occurrences of `x` with `t` in the goal and all hypotheses.
#[allow(dead_code)]
pub fn tac_subst(
    hyp_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> crate::tactic::state::TacticResult<()> {
    let hyps = ctx.get_local_hyps();
    let hyp_ty = hyps
        .iter()
        .find(|(n, _)| n == hyp_name)
        .map(|(_, ty)| ty.clone())
        .ok_or_else(|| crate::tactic::state::TacticError::UnknownHyp(hyp_name.clone()))?;
    let _eq_info = EqualityInfo::from_expr(&hyp_ty).ok_or_else(|| {
        crate::tactic::state::TacticError::Failed(format!(
            "subst: '{}' is not an equality",
            hyp_name
        ))
    })?;
    tac_rewrite_named(hyp_name, RewriteDirection::Forward, state, ctx)
}
/// Check if an expression is a numeral literal.
#[allow(dead_code)]
pub fn is_numeral(expr: &Expr) -> bool {
    matches!(expr, Expr::Lit(oxilean_kernel::Literal::Nat(_)))
}
/// Get the numeral value of an expression, if it is one.
#[allow(dead_code)]
pub fn get_numeral(expr: &Expr) -> Option<u64> {
    match expr {
        Expr::Lit(oxilean_kernel::Literal::Nat(n)) => Some(*n),
        _ => None,
    }
}
/// Check if an expression represents a Nat.zero.
#[allow(dead_code)]
pub fn is_nat_zero(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(n, _) if * n == Name::str("Nat.zero"))
        || matches!(expr, Expr::Lit(oxilean_kernel::Literal::Nat(0)))
}
/// Check if an expression represents Nat.succ applied to something.
#[allow(dead_code)]
pub fn is_nat_succ(expr: &Expr) -> bool {
    if let Expr::App(f, _) = expr {
        if let Expr::Const(n, _) = f.as_ref() {
            return *n == Name::str("Nat.succ");
        }
    }
    false
}
#[cfg(test)]
mod rewrite_extended_tests {
    use super::*;
    use crate::tactic::rewrite::*;
    use oxilean_kernel::{Expr, Name};
    #[test]
    fn test_rewrite_rule_apply_match() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let rule = RewriteRule::new(Name::str("h"), a.clone(), b.clone());
        assert_eq!(rule.apply(&a), Some(b));
    }
    #[test]
    fn test_rewrite_rule_apply_no_match() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let rule = RewriteRule::new(Name::str("h"), a, b);
        assert_eq!(rule.apply(&c), None);
    }
    #[test]
    fn test_rewrite_system_normalize() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let mut sys = RewriteSystem::new();
        sys.add_rule(RewriteRule::new(Name::str("h"), a.clone(), b.clone()));
        let (result, steps) = sys.normalize(&a, 10);
        assert_eq!(result, b);
        assert_eq!(steps, 1);
    }
    #[test]
    fn test_find_positions_none() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let positions = find_positions(&a, &b);
        assert!(positions.is_empty());
    }
    #[test]
    fn test_find_positions_root() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let positions = find_positions(&a, &a);
        assert_eq!(positions.len(), 1);
        assert!(positions[0].is_root());
    }
    #[test]
    fn test_find_positions_app() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let app = Expr::App(Box::new(a.clone()), Box::new(a.clone()));
        let positions = find_positions(&app, &a);
        assert!(!positions.is_empty());
    }
    #[test]
    fn test_replace_nth_occurrence_first() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let expr = Expr::App(Box::new(a.clone()), Box::new(a.clone()));
        let (result, ok) = replace_nth_occurrence(&expr, &a, &b, 0);
        assert!(ok);
        if let Expr::App(f, arg) = result {
            assert_eq!(*f, b);
            assert_eq!(*arg, a);
        }
    }
    #[test]
    fn test_beta_reduce_once() {
        let body = Expr::BVar(0);
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let lam = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(body),
        );
        let a = Expr::Const(Name::str("a"), vec![]);
        let app = Expr::App(Box::new(lam), Box::new(a.clone()));
        let result = beta_reduce_once(&app);
        assert_eq!(result, Some(a));
    }
    #[test]
    fn test_subst_bvar_hit() {
        let body = Expr::BVar(0);
        let replacement = Expr::Const(Name::str("x"), vec![]);
        let result = subst_bvar(&body, 0, &replacement);
        assert_eq!(result, replacement);
    }
    #[test]
    fn test_subst_bvar_miss() {
        let body = Expr::BVar(1);
        let replacement = Expr::Const(Name::str("x"), vec![]);
        let result = subst_bvar(&body, 0, &replacement);
        assert_eq!(result, Expr::BVar(0));
    }
    #[test]
    fn test_is_numeral() {
        let n = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        assert!(is_numeral(&n));
        let c = Expr::Const(Name::str("Nat"), vec![]);
        assert!(!is_numeral(&c));
    }
    #[test]
    fn test_is_nat_zero_lit() {
        let z = Expr::Lit(oxilean_kernel::Literal::Nat(0));
        assert!(is_nat_zero(&z));
    }
    #[test]
    fn test_is_nat_zero_const() {
        let z = Expr::Const(Name::str("Nat.zero"), vec![]);
        assert!(is_nat_zero(&z));
    }
    #[test]
    fn test_is_nat_succ() {
        let succ = Expr::Const(Name::str("Nat.succ"), vec![]);
        let zero = Expr::Const(Name::str("Nat.zero"), vec![]);
        let one = Expr::App(Box::new(succ), Box::new(zero));
        assert!(is_nat_succ(&one));
    }
    #[test]
    fn test_rewrite_hint_db() {
        let mut db = RewriteHintDb::new();
        db.add(Name::str("h1"), 10, RewriteDirection::Forward);
        db.add(Name::str("h2"), 20, RewriteDirection::Backward);
        assert_eq!(db.len(), 2);
        db.sort_by_priority();
        assert_eq!(db.hints[0].lemma, Name::str("h2"));
    }
    #[test]
    fn test_rewrite_trace_summary() {
        let t = RewriteTrace::new();
        assert_eq!(t.summary(), "no rewrites");
    }
    #[test]
    fn test_chain_equalities_none() {
        assert_eq!(chain_equalities(&[]), None);
    }
    #[test]
    fn test_chain_equalities_single() {
        let h = Expr::Const(Name::str("h"), vec![]);
        let result = chain_equalities(std::slice::from_ref(&h));
        assert_eq!(result, Some(h));
    }
    #[test]
    fn test_equal_modulo_rewriting_trivial() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let sys = RewriteSystem::new();
        assert!(equal_modulo_rewriting(&a, &a, &sys, 10));
    }
    #[test]
    fn test_is_normal_form_empty_system() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let sys = RewriteSystem::new();
        assert!(is_normal_form(&a, &sys));
    }
    #[test]
    fn test_setoid_rewrite_describe() {
        let sr = SetoidRewrite::new("Setoid", Name::str("h"), RewriteDirection::Forward);
        let desc = sr.describe();
        assert!(desc.contains("Setoid"));
        assert!(desc.contains("h"));
    }
    #[test]
    fn test_position_display() {
        let p = RewritePosition(vec![0, 1, 2]);
        let s = format!("{}", p);
        assert_eq!(s, "[0.1.2]");
    }
    #[test]
    fn test_rewrite_stats() {
        let mut s = RewriteTacticStats::new();
        assert!(!s.any_progress());
        s.record_success();
        assert!(s.any_progress());
        let summary = s.summary();
        assert!(summary.contains("applied=1"));
    }
    #[test]
    fn test_is_trivially_true() {
        let t = Expr::Const(Name::str("True"), vec![]);
        assert!(is_trivially_true(&t));
        let f = Expr::Const(Name::str("False"), vec![]);
        assert!(!is_trivially_true(&f));
    }
    #[test]
    fn test_is_trivially_false() {
        let f = Expr::Const(Name::str("False"), vec![]);
        assert!(is_trivially_false(&f));
    }
    #[test]
    fn test_match_result_assign_conflict() {
        let mut mr = MatchResult::empty();
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        assert!(mr.assign(Name::str("x"), a.clone()));
        assert!(mr.assign(Name::str("x"), a.clone()));
        assert!(!mr.assign(Name::str("x"), b));
    }
    #[test]
    fn test_flip_eq_proof() {
        let h = Expr::Const(Name::str("h"), vec![]);
        let flipped = flip_eq_proof(h.clone());
        assert!(matches!(flipped, Expr::App(_, _)));
    }
    #[test]
    fn test_rewrite_hint_annotation() {
        let ann = RewriteHypAnnotation::default_for(Name::str("h"))
            .once()
            .reverse();
        assert_eq!(ann.direction(), RewriteDirection::Backward);
        assert!(ann.once);
        assert!(ann.reverse);
    }
    #[test]
    fn test_replace_at_position_root() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let result = replace_at_position(&a, &[], &b);
        assert_eq!(result, Some(b));
    }
    #[test]
    fn test_replace_at_position_app_arg() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let app = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let result = replace_at_position(&app, &[1], &b);
        assert_eq!(result, Some(Expr::App(Box::new(f), Box::new(b))));
    }
    #[test]
    fn test_run_rewrite_loop_empty_system() {
        let sys = RewriteSystem::new();
        let config = RewriteLoopConfig::default_config();
        let a = Expr::Const(Name::str("a"), vec![]);
        let trace = run_rewrite_loop(&sys, &a, &config);
        assert!(trace.is_empty());
        assert_eq!(trace.result(), Some(&a));
    }
    #[test]
    fn test_build_congr_proof() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let h = Expr::Const(Name::str("h"), vec![]);
        let proof = build_congr_proof(f, h);
        assert!(matches!(proof, Expr::App(_, _)));
    }
    #[test]
    fn test_build_trans_proof() {
        let h1 = Expr::Const(Name::str("h1"), vec![]);
        let h2 = Expr::Const(Name::str("h2"), vec![]);
        let proof = build_trans_proof(h1, h2);
        assert!(matches!(proof, Expr::App(_, _)));
    }
    #[test]
    fn test_rewrite_rule_backward() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let rule = RewriteRule::new(Name::str("h"), a.clone(), b.clone())
            .with_direction(RewriteDirection::Backward);
        assert_eq!(rule.apply(&b), Some(a.clone()));
        assert_eq!(rule.apply(&a), None);
    }
    #[test]
    fn test_rewrite_loop_single_step() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let mut sys = RewriteSystem::new();
        sys.add_rule(RewriteRule::new(Name::str("h"), a.clone(), b.clone()));
        let config = RewriteLoopConfig::single_step();
        let trace = run_rewrite_loop(&sys, &a, &config);
        assert_eq!(trace.len(), 1);
        assert_eq!(trace.result(), Some(&b));
    }
    #[test]
    fn test_describe_rewrite_forward() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let desc = describe_rewrite(&a, &b, &Name::str("h"), RewriteDirection::Forward);
        assert!(desc.contains("h"));
        assert!(desc.contains("a"));
        assert!(desc.contains("b"));
    }
    #[test]
    fn test_get_numeral() {
        let n = Expr::Lit(oxilean_kernel::Literal::Nat(7));
        assert_eq!(get_numeral(&n), Some(7));
        let c = Expr::Const(Name::str("x"), vec![]);
        assert_eq!(get_numeral(&c), None);
    }
}
