//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    EqInfo, HypSummary, StructuralExtConfig2400, StructuralExtConfigVal2400, StructuralExtDiag2400,
    StructuralExtDiff2400, StructuralExtPass2400, StructuralExtPipeline2400,
    StructuralExtResult2400, TacStructBuilder, TacStructCounterMap, TacStructExtMap,
    TacStructExtUtil, TacStructStateMachine, TacStructWindow, TacStructWorkQueue,
    TacticStructuralAnalysisPass, TacticStructuralConfig, TacticStructuralConfigValue,
    TacticStructuralDiagnostics, TacticStructuralDiff, TacticStructuralPipeline,
    TacticStructuralResult,
};
use crate::basic::{MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{BinderInfo, Expr, Name};

/// `clear h` — remove a hypothesis from the context.
///
/// Fails if the hypothesis is used in the goal or other hypotheses.
pub fn tac_clear(
    hyp_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let hyps = ctx.get_local_hyps();
    let found = hyps.iter().any(|(name, _)| name == hyp_name);
    if !found {
        return Err(TacticError::UnknownHyp(hyp_name.clone()));
    }
    if expr_contains_name(&target, hyp_name) {
        return Err(TacticError::Failed(format!(
            "clear: hypothesis {} is used in the goal",
            hyp_name
        )));
    }
    ctx.clear_local(hyp_name);
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(target, MetavarKind::Natural);
    ctx.assign_mvar(goal, new_expr);
    state.replace_goal(vec![new_id]);
    Ok(())
}
/// `revert h` — move a hypothesis back into the goal as a Pi binder.
///
/// If `h : A` and goal is `T`, produces goal `A → T`.
pub fn tac_revert(
    hyp_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let hyps = ctx.get_local_hyps();
    let hyp_ty = hyps
        .iter()
        .find(|(name, _)| name == hyp_name)
        .map(|(_, ty)| ty.clone())
        .ok_or_else(|| TacticError::UnknownHyp(hyp_name.clone()))?;
    let new_target = Expr::Pi(
        BinderInfo::Default,
        hyp_name.clone(),
        Box::new(hyp_ty),
        Box::new(abstract_name_in_expr(&target, hyp_name)),
    );
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    let hyp_expr = Expr::Const(hyp_name.clone(), vec![]);
    let proof = Expr::App(Box::new(new_expr), Box::new(hyp_expr));
    ctx.assign_mvar(goal, proof);
    ctx.clear_local(hyp_name);
    state.replace_goal(vec![new_id]);
    Ok(())
}
/// `subst h` — substitute an equality hypothesis.
///
/// Given `h : x = e` where `x` is a free variable, replaces all
/// occurrences of `x` with `e` in the goal and removes `h` and `x`.
pub fn tac_subst(
    hyp_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let hyps = ctx.get_local_hyps();
    let hyp_ty = hyps
        .iter()
        .find(|(name, _)| name == hyp_name)
        .map(|(_, ty)| ty.clone())
        .ok_or_else(|| TacticError::UnknownHyp(hyp_name.clone()))?;
    let eq_info = parse_equality(&hyp_ty).ok_or_else(|| {
        TacticError::GoalMismatch(format!("subst: {} is not an equality hypothesis", hyp_name))
    })?;
    let (var_name, replacement) = if let Expr::Const(name, _) = &eq_info.lhs {
        (name.clone(), eq_info.rhs.clone())
    } else if let Expr::Const(name, _) = &eq_info.rhs {
        (name.clone(), eq_info.lhs.clone())
    } else {
        return Err(TacticError::GoalMismatch(
            "subst: neither side of equality is a variable".into(),
        ));
    };
    let new_target = replace_name_in_expr(&target, &var_name, &replacement);
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    ctx.assign_mvar(goal, new_expr);
    ctx.clear_local(hyp_name);
    state.replace_goal(vec![new_id]);
    Ok(())
}
/// Parse an equality expression.
pub(super) fn parse_equality(ty: &Expr) -> Option<EqInfo> {
    if let Expr::App(eq_a_lhs, rhs) = ty {
        if let Expr::App(eq_a, lhs) = eq_a_lhs.as_ref() {
            if let Expr::App(eq_const, _alpha) = eq_a.as_ref() {
                if matches!(
                    eq_const.as_ref(), Expr::Const(name, _) if * name == Name::str("Eq")
                ) {
                    return Some(EqInfo {
                        lhs: *lhs.clone(),
                        rhs: *rhs.clone(),
                    });
                }
            }
        }
    }
    None
}
/// Check if an expression contains a reference to a name.
pub(super) fn expr_contains_name(expr: &Expr, name: &Name) -> bool {
    match expr {
        Expr::Const(n, _) => n == name,
        Expr::App(f, a) => expr_contains_name(f, name) || expr_contains_name(a, name),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            expr_contains_name(ty, name) || expr_contains_name(body, name)
        }
        Expr::Let(_, ty, val, body) => {
            expr_contains_name(ty, name)
                || expr_contains_name(val, name)
                || expr_contains_name(body, name)
        }
        Expr::Proj(_, _, e) => expr_contains_name(e, name),
        _ => false,
    }
}
/// Replace all occurrences of a named constant with a replacement expression.
pub(super) fn replace_name_in_expr(expr: &Expr, name: &Name, replacement: &Expr) -> Expr {
    match expr {
        Expr::Const(n, _) if n == name => replacement.clone(),
        Expr::App(f, a) => {
            let f2 = replace_name_in_expr(f, name, replacement);
            let a2 = replace_name_in_expr(a, name, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = replace_name_in_expr(ty, name, replacement);
            let body2 = replace_name_in_expr(body, name, replacement);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = replace_name_in_expr(ty, name, replacement);
            let body2 = replace_name_in_expr(body, name, replacement);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(n, ty, val, body) => {
            let ty2 = replace_name_in_expr(ty, name, replacement);
            let val2 = replace_name_in_expr(val, name, replacement);
            let body2 = replace_name_in_expr(body, name, replacement);
            Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(n, i, e) => {
            let e2 = replace_name_in_expr(e, name, replacement);
            Expr::Proj(n.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Abstract over a named constant, converting it to BVar(0).
pub(super) fn abstract_name_in_expr(expr: &Expr, name: &Name) -> Expr {
    abstract_name_impl(expr, name, 0)
}
pub(super) fn abstract_name_impl(expr: &Expr, name: &Name, depth: u32) -> Expr {
    match expr {
        Expr::Const(n, _) if n == name => Expr::BVar(depth),
        Expr::App(f, a) => {
            let f2 = abstract_name_impl(f, name, depth);
            let a2 = abstract_name_impl(a, name, depth);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = abstract_name_impl(ty, name, depth);
            let body2 = abstract_name_impl(body, name, depth + 1);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = abstract_name_impl(ty, name, depth);
            let body2 = abstract_name_impl(body, name, depth + 1);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(n, ty, val, body) => {
            let ty2 = abstract_name_impl(ty, name, depth);
            let val2 = abstract_name_impl(val, name, depth);
            let body2 = abstract_name_impl(body, name, depth + 1);
            Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(n, i, e) => {
            let e2 = abstract_name_impl(e, name, depth);
            Expr::Proj(n.clone(), *i, Box::new(e2))
        }
        Expr::BVar(idx) => {
            if *idx >= depth {
                Expr::BVar(idx + 1)
            } else {
                expr.clone()
            }
        }
        _ => expr.clone(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::structural::*;
    use oxilean_kernel::{Environment, Level};
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_clear_unknown_hyp() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_clear(&Name::str("nonexistent"), &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_revert_unknown_hyp() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_revert(&Name::str("nonexistent"), &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_subst_unknown_hyp() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_subst(&Name::str("nonexistent"), &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_expr_contains_name() {
        let f_a = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        assert!(expr_contains_name(&f_a, &Name::str("a")));
        assert!(expr_contains_name(&f_a, &Name::str("f")));
        assert!(!expr_contains_name(&f_a, &Name::str("b")));
    }
    #[test]
    fn test_replace_name_in_expr() {
        let b = Expr::Const(Name::str("b"), vec![]);
        let f_a = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        let result = replace_name_in_expr(&f_a, &Name::str("a"), &b);
        let expected = Expr::App(Box::new(Expr::Const(Name::str("f"), vec![])), Box::new(b));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_abstract_name_in_expr() {
        let f_x = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("x"), vec![])),
        );
        let result = abstract_name_in_expr(&f_x, &Name::str("x"));
        let expected = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_parse_equality() {
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let eq_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Eq"), vec![Level::zero()])),
                    Box::new(nat_ty),
                )),
                Box::new(a.clone()),
            )),
            Box::new(b.clone()),
        );
        let info = parse_equality(&eq_ty).expect("info should be present");
        assert_eq!(info.lhs, a);
        assert_eq!(info.rhs, b);
    }
    #[test]
    fn test_parse_equality_non_eq() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert!(parse_equality(&expr).is_none());
    }
    #[test]
    fn test_abstract_nested() {
        let x = Expr::Const(Name::str("x"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("add"), vec![])),
                Box::new(x.clone()),
            )),
            Box::new(x),
        );
        let result = abstract_name_in_expr(&expr, &Name::str("x"));
        let expected = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("add"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(result, expected);
    }
}
/// `rename_hyp old_name new_name` — rename a hypothesis.
pub fn tac_rename(
    old_name: &Name,
    new_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let hyps = ctx.get_local_hyps();
    let hyp_ty = hyps
        .iter()
        .find(|(n, _)| n == old_name)
        .map(|(_, ty)| ty.clone())
        .ok_or_else(|| TacticError::UnknownHyp(old_name.clone()))?;
    ctx.clear_local(old_name);
    ctx.mk_local_decl(new_name.clone(), hyp_ty, BinderInfo::Default);
    let new_target =
        replace_name_in_expr(&target, old_name, &Expr::Const(new_name.clone(), vec![]));
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    ctx.assign_mvar(goal, new_expr);
    state.replace_goal(vec![new_id]);
    Ok(())
}
/// `intro_all` — introduce all leading Pi-binders.
pub fn tac_intro_all(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<Vec<Name>> {
    let mut introduced = Vec::new();
    loop {
        let goal = state.current_goal()?;
        let target = ctx
            .get_mvar_type(goal)
            .cloned()
            .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
        let target = ctx.instantiate_mvars(&target);
        match &target {
            Expr::Pi(_, binder_name, binder_ty, body) => {
                let name = binder_name.clone();
                let ty = *binder_ty.clone();
                let new_target = *body.clone();
                ctx.mk_local_decl(name.clone(), ty, BinderInfo::Default);
                let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
                ctx.assign_mvar(goal, new_expr);
                state.replace_goal(vec![new_id]);
                introduced.push(name);
            }
            _ => break,
        }
    }
    Ok(introduced)
}
/// Substitute `BVar(depth)` with `replacement` in an expression.
pub(super) fn substitute_bvar(expr: &Expr, depth: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(i) if *i == depth => replacement.clone(),
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => {
            expr.clone()
        }
        Expr::App(f, a) => {
            let f2 = substitute_bvar(f, depth, replacement);
            let a2 = substitute_bvar(a, depth, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = substitute_bvar(ty, depth, replacement);
            let body2 = substitute_bvar(body, depth + 1, replacement);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = substitute_bvar(ty, depth, replacement);
            let body2 = substitute_bvar(body, depth + 1, replacement);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(n, ty, val, body) => {
            let ty2 = substitute_bvar(ty, depth, replacement);
            let val2 = substitute_bvar(val, depth, replacement);
            let body2 = substitute_bvar(body, depth + 1, replacement);
            Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(n, i, e) => {
            let e2 = substitute_bvar(e, depth, replacement);
            Expr::Proj(n.clone(), *i, Box::new(e2))
        }
    }
}
/// `specialize h with arg` — specialize a Pi hypothesis.
pub fn tac_specialize(
    hyp_name: &Name,
    arg: Expr,
    spec_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let hyps = ctx.get_local_hyps();
    let hyp_ty = hyps
        .iter()
        .find(|(n, _)| n == hyp_name)
        .map(|(_, ty)| ty.clone())
        .ok_or_else(|| TacticError::UnknownHyp(hyp_name.clone()))?;
    let spec_ty = match &hyp_ty {
        Expr::Pi(_, _, _, body) => substitute_bvar(body, 0, &arg),
        _ => return Err(TacticError::GoalMismatch(format!("{} is not Pi", hyp_name))),
    };
    ctx.clear_local(hyp_name);
    ctx.mk_local_decl(spec_name.clone(), spec_ty, BinderInfo::Default);
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(target, MetavarKind::Natural);
    ctx.assign_mvar(goal, new_expr);
    state.replace_goal(vec![new_id]);
    Ok(())
}
/// `generalize e as x` — generalize over an expression.
pub fn tac_generalize(
    expr: &Expr,
    var_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let generalized = generalize_expr(&target, expr);
    let var_ty = Expr::Sort(oxilean_kernel::Level::zero());
    let new_target = Expr::Pi(
        BinderInfo::Default,
        var_name.clone(),
        Box::new(var_ty),
        Box::new(generalized),
    );
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    ctx.assign_mvar(goal, new_expr);
    state.replace_goal(vec![new_id]);
    Ok(())
}
pub(super) fn generalize_expr(expr: &Expr, target_expr: &Expr) -> Expr {
    if expr == target_expr {
        return Expr::BVar(0);
    }
    match expr {
        Expr::App(f, a) => {
            let f2 = generalize_expr(f, target_expr);
            let a2 = generalize_expr(a, target_expr);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = generalize_expr(ty, target_expr);
            let body2 = generalize_expr(body, target_expr);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = generalize_expr(ty, target_expr);
            let body2 = generalize_expr(body, target_expr);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        _ => expr.clone(),
    }
}
/// Check whether two names are equal.
pub fn names_eq(a: &Name, b: &Name) -> bool {
    a == b
}
#[cfg(test)]
mod extended_structural_tests {
    use super::*;
    use crate::tactic::structural::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_rename_hyp() {
        let mut ctx = mk_ctx();
        ctx.mk_local_decl(
            Name::str("old"),
            Expr::Const(Name::str("Nat"), vec![]),
            BinderInfo::Default,
        );
        let goal_ty = Expr::Const(Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_rename(&Name::str("old"), &Name::str("new"), &mut state, &mut ctx);
        assert!(result.is_ok());
    }
    #[test]
    fn test_rename_hyp_not_found() {
        let mut ctx = mk_ctx();
        let (mvar_id, _) =
            ctx.mk_fresh_expr_mvar(Expr::Const(Name::str("P"), vec![]), MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_rename(
            &Name::str("nonexistent"),
            &Name::str("new"),
            &mut state,
            &mut ctx
        )
        .is_err());
    }
    #[test]
    fn test_intro_all_non_pi() {
        let mut ctx = mk_ctx();
        let (mvar_id, _) =
            ctx.mk_fresh_expr_mvar(Expr::Const(Name::str("P"), vec![]), MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let introduced = tac_intro_all(&mut state, &mut ctx).expect("introduced should be present");
        assert_eq!(introduced.len(), 0);
    }
    #[test]
    fn test_intro_all_pi() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let goal_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty),
            Box::new(Expr::BVar(0)),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let introduced = tac_intro_all(&mut state, &mut ctx).expect("introduced should be present");
        assert_eq!(introduced.len(), 1);
    }
    #[test]
    fn test_generalize() {
        let mut ctx = mk_ctx();
        let n = Expr::Const(Name::str("n"), vec![]);
        let goal_ty = Expr::App(
            Box::new(Expr::Const(Name::str("P"), vec![])),
            Box::new(n.clone()),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_generalize(&n, &Name::str("x"), &mut state, &mut ctx).is_ok());
    }
    #[test]
    fn test_generalize_expr() {
        let n = Expr::Const(Name::str("n"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("P"), vec![])),
            Box::new(n.clone()),
        );
        let result = generalize_expr(&expr, &n);
        assert_eq!(
            result,
            Expr::App(
                Box::new(Expr::Const(Name::str("P"), vec![])),
                Box::new(Expr::BVar(0)),
            )
        );
    }
    #[test]
    fn test_substitute_bvar() {
        let arg = Expr::Const(Name::str("a"), vec![]);
        let body = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let result = substitute_bvar(&body, 0, &arg);
        assert_eq!(result, Expr::App(Box::new(arg), Box::new(Expr::BVar(1))));
    }
    #[test]
    fn test_names_eq() {
        assert!(names_eq(&Name::str("a"), &Name::str("a")));
        assert!(!names_eq(&Name::str("a"), &Name::str("b")));
    }
    #[test]
    fn test_specialize_non_pi() {
        let mut ctx = mk_ctx();
        ctx.mk_local_decl(
            Name::str("h"),
            Expr::Const(Name::str("Nat"), vec![]),
            BinderInfo::Default,
        );
        let (mvar_id, _) =
            ctx.mk_fresh_expr_mvar(Expr::Const(Name::str("P"), vec![]), MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let arg = Expr::Const(Name::str("zero"), vec![]);
        assert!(tac_specialize(
            &Name::str("h"),
            arg,
            &Name::str("h_spec"),
            &mut state,
            &mut ctx
        )
        .is_err());
    }
    #[test]
    fn test_abstract_name_in_expr() {
        let f_x = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("x"), vec![])),
        );
        let result = abstract_name_in_expr(&f_x, &Name::str("x"));
        let expected = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_abstract_nested() {
        let x = Expr::Const(Name::str("x"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("add"), vec![])),
                Box::new(x.clone()),
            )),
            Box::new(x),
        );
        let result = abstract_name_in_expr(&expr, &Name::str("x"));
        let expected = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("add"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_clear_unknown_hyp() {
        let mut ctx = mk_ctx();
        let (mvar_id, _) =
            ctx.mk_fresh_expr_mvar(Expr::Const(Name::str("P"), vec![]), MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_clear(&Name::str("nonexistent"), &mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_specialize_pi() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let p_x = Expr::App(
            Box::new(Expr::Const(Name::str("P"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        let hyp_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty),
            Box::new(p_x),
        );
        ctx.mk_local_decl(Name::str("h"), hyp_ty, BinderInfo::Default);
        let (mvar_id, _) =
            ctx.mk_fresh_expr_mvar(Expr::Const(Name::str("Q"), vec![]), MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let arg = Expr::Const(Name::str("zero"), vec![]);
        assert!(tac_specialize(
            &Name::str("h"),
            arg,
            &Name::str("h_spec"),
            &mut state,
            &mut ctx
        )
        .is_ok());
    }
}
/// Count the total number of local hypotheses currently in context.
#[allow(dead_code)]
pub fn hyp_count(ctx: &MetaContext) -> usize {
    ctx.get_local_hyps().len()
}
/// Return the names of all current local hypotheses.
#[allow(dead_code)]
pub fn hyp_names(ctx: &MetaContext) -> Vec<Name> {
    ctx.get_local_hyps()
        .iter()
        .map(|(n, _)| n.clone())
        .collect()
}
/// Check whether a hypothesis with the given name exists in context.
#[allow(dead_code)]
pub fn has_hyp(name: &Name, ctx: &MetaContext) -> bool {
    ctx.get_local_hyps().iter().any(|(n, _)| n == name)
}
/// Return the type of a hypothesis by name, or `None` if not found.
#[allow(dead_code)]
pub fn hyp_type(name: &Name, ctx: &MetaContext) -> Option<Expr> {
    ctx.get_local_hyps()
        .iter()
        .find_map(|(n, ty)| if n == name { Some(ty.clone()) } else { None })
}
/// Introduce up to `limit` Pi-binders from the current goal, stopping early
/// if the goal no longer has a Pi type.
///
/// Returns the number of introductions performed.
#[allow(dead_code)]
pub fn tac_intro_n(
    limit: usize,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<usize> {
    let mut count = 0;
    for _ in 0..limit {
        let goal = match state.current_goal() {
            Ok(g) => g,
            Err(_) => break,
        };
        let target = match ctx.get_mvar_type(goal) {
            Some(t) => ctx.instantiate_mvars(t),
            None => break,
        };
        if !matches!(&target, Expr::Pi(_, _, _, _)) {
            break;
        }
        crate::tactic::core::tac_intro(None, state, ctx)?;
        count += 1;
    }
    Ok(count)
}
/// Check whether the current goal target starts with a `Pi` binder.
#[allow(dead_code)]
pub fn goal_has_pi(state: &TacticState, ctx: &MetaContext) -> bool {
    state
        .current_goal()
        .ok()
        .and_then(|g| ctx.get_mvar_type(g))
        .map(|t| {
            let inst = ctx.instantiate_mvars(t);
            matches!(inst, Expr::Pi(_, _, _, _))
        })
        .unwrap_or(false)
}
/// Duplicate a hypothesis under a new name.
///
/// Adds `new_name : T` where `T` is the type of the hypothesis named `orig`.
#[allow(dead_code)]
pub fn tac_dup_hyp(
    orig: &Name,
    new_name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let orig_ty = hyp_type(orig, ctx).ok_or_else(|| TacticError::UnknownHyp(orig.clone()))?;
    ctx.mk_local_decl(new_name.clone(), orig_ty, BinderInfo::Default);
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(target, MetavarKind::Natural);
    ctx.assign_mvar(goal, new_expr);
    state.replace_goal(vec![new_id]);
    Ok(())
}
/// Remove all hypotheses that are of sort `Prop` (universe 0).
#[allow(dead_code)]
pub fn tac_clear_props(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let prop_hyps: Vec<Name> = ctx
        .get_local_hyps()
        .iter()
        .filter(|(_, ty)| is_prop_expr(ty))
        .map(|(n, _)| n.clone())
        .collect();
    for name in prop_hyps {
        let _ = tac_clear(&name, state, ctx);
    }
    Ok(())
}
/// Check whether an expression is of sort `Prop` (i.e., `Sort 0`).
#[allow(dead_code)]
pub(super) fn is_prop_expr(expr: &Expr) -> bool {
    matches!(
        expr, Expr::Sort(l) if oxilean_kernel::level::is_equivalent(l, &
        oxilean_kernel::Level::zero())
    )
}
/// Check whether the expression `needle` appears anywhere in `haystack`.
#[allow(dead_code)]
pub fn expr_contains(haystack: &Expr, needle: &Expr) -> bool {
    if haystack == needle {
        return true;
    }
    match haystack {
        Expr::App(f, a) => expr_contains(f, needle) || expr_contains(a, needle),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            expr_contains(dom, needle) || expr_contains(body, needle)
        }
        Expr::Let(_, ty, val, body) => {
            expr_contains(ty, needle) || expr_contains(val, needle) || expr_contains(body, needle)
        }
        _ => false,
    }
}
#[cfg(test)]
mod structural_extra_tests {
    use super::*;
    use crate::tactic::state::TacticState;
    use crate::tactic::structural::*;
    use oxilean_kernel::{Expr, Level, Name};
    fn mk_ctx() -> MetaContext {
        MetaContext::new(oxilean_kernel::Environment::new())
    }
    #[test]
    fn test_hyp_count_empty() {
        let ctx = mk_ctx();
        assert_eq!(hyp_count(&ctx), 0);
    }
    #[test]
    fn test_hyp_names_empty() {
        let ctx = mk_ctx();
        assert!(hyp_names(&ctx).is_empty());
    }
    #[test]
    fn test_has_hyp_false() {
        let ctx = mk_ctx();
        assert!(!has_hyp(&Name::str("h"), &ctx));
    }
    #[test]
    fn test_hyp_type_none() {
        let ctx = mk_ctx();
        assert!(hyp_type(&Name::str("h"), &ctx).is_none());
    }
    #[test]
    fn test_goal_has_pi_false() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Sort(Level::succ(Level::zero()));
        let (mv, _) = ctx.mk_fresh_expr_mvar(nat_ty, MetavarKind::Natural);
        let state = TacticState::single(mv);
        assert!(!goal_has_pi(&state, &ctx));
    }
    #[test]
    fn test_goal_has_pi_true() {
        let mut ctx = mk_ctx();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat.clone()),
            Box::new(nat),
        );
        let (mv, _) = ctx.mk_fresh_expr_mvar(pi, MetavarKind::Natural);
        let state = TacticState::single(mv);
        assert!(goal_has_pi(&state, &ctx));
    }
    #[test]
    fn test_tac_intro_n_no_pi() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Sort(Level::succ(Level::zero()));
        let (mv, _) = ctx.mk_fresh_expr_mvar(nat_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mv);
        let count = tac_intro_n(5, &mut state, &mut ctx).expect("count should be present");
        assert_eq!(count, 0);
    }
    #[test]
    fn test_expr_contains_self() {
        let e = Expr::Const(Name::str("x"), vec![]);
        assert!(expr_contains(&e, &e));
    }
    #[test]
    fn test_expr_contains_subexpr() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let e = Expr::App(Box::new(a.clone()), Box::new(b));
        assert!(expr_contains(&e, &a));
    }
    #[test]
    fn test_expr_contains_false() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let c = Expr::Const(Name::str("c"), vec![]);
        let e = Expr::App(Box::new(a), Box::new(b));
        assert!(!expr_contains(&e, &c));
    }
    #[test]
    fn test_has_hyp_after_add() {
        let mut ctx = mk_ctx();
        let ty = Expr::Sort(Level::zero());
        ctx.mk_local_decl(Name::str("h"), ty, BinderInfo::Default);
        assert!(has_hyp(&Name::str("h"), &ctx));
    }
    #[test]
    fn test_hyp_type_after_add() {
        let mut ctx = mk_ctx();
        let ty = Expr::Sort(Level::zero());
        ctx.mk_local_decl(Name::str("h"), ty.clone(), BinderInfo::Default);
        let found = hyp_type(&Name::str("h"), &ctx);
        assert!(found.is_some());
    }
}
/// Check whether an expression is a bound variable at depth 0.
pub fn is_bvar_zero(expr: &Expr) -> bool {
    matches!(expr, Expr::BVar(0))
}
/// Check whether an expression is a free variable (`FVar`).
pub fn is_fvar(expr: &Expr) -> bool {
    matches!(expr, Expr::FVar(_))
}
/// Check whether an expression is a `Sort`.
pub fn is_sort_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(_))
}
/// Check whether an expression is `Sort 0` (Prop).
pub fn is_prop_sort(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(l) if * l == oxilean_kernel::Level::zero())
}
/// Check whether an expression is `Sort 1` (Type).
pub fn is_type_zero_expr(expr: &Expr) -> bool {
    use oxilean_kernel::Level;
    matches!(expr, Expr::Sort(l) if * l == Level::succ(Level::zero()))
}
/// Check whether `name` appears in a list of names.
pub fn name_in_list(name: &Name, list: &[Name]) -> bool {
    list.contains(name)
}
/// Build a chain of `n` Pi-binders with domain `ty` and a final body.
pub fn build_pi_chain(n: usize, ty: Expr, body: Expr) -> Expr {
    let mut result = body;
    for i in (0..n).rev() {
        let nm = Name::str(format!("x_{}", i));
        result = Expr::Pi(
            BinderInfo::Default,
            nm,
            Box::new(ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Build a chain of `n` lambda binders with domain `ty` and a final body.
pub fn build_lam_chain(n: usize, ty: Expr, body: Expr) -> Expr {
    let mut result = body;
    for i in (0..n).rev() {
        let nm = Name::str(format!("x_{}", i));
        result = Expr::Lam(
            BinderInfo::Default,
            nm,
            Box::new(ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Count the number of Pi-binders at the head of an expression.
pub fn count_pi_binders_unique(expr: &Expr) -> usize {
    let mut count = 0;
    let mut cur = expr;
    while let Expr::Pi(_, _, _, body) = cur {
        count += 1;
        cur = body;
    }
    count
}
/// Count the number of lambda binders at the head of an expression.
pub fn count_lam_binders_unique(expr: &Expr) -> usize {
    let mut count = 0;
    let mut cur = expr;
    while let Expr::Lam(_, _, _, body) = cur {
        count += 1;
        cur = body;
    }
    count
}
/// Strip the outermost `n` lambda binders from an expression.
pub fn strip_lam_n(expr: &Expr, n: usize) -> &Expr {
    let mut cur = expr;
    for _ in 0..n {
        match cur {
            Expr::Lam(_, _, _, body) => cur = body,
            _ => break,
        }
    }
    cur
}
/// Peel off `n` Pi-binders, returning binders and body.
pub fn peel_pi_n(expr: &Expr, n: usize) -> (Vec<(&BinderInfo, &Name, &Expr)>, &Expr) {
    let mut binders = Vec::new();
    let mut cur = expr;
    for _ in 0..n {
        match cur {
            Expr::Pi(bi, name, ty, body) => {
                binders.push((bi, name, ty.as_ref()));
                cur = body;
            }
            _ => break,
        }
    }
    (binders, cur)
}
/// Collect constants from an expression (depth-first).
pub fn collect_consts_from_expr(expr: &Expr) -> Vec<Name> {
    let mut out = Vec::new();
    collect_consts_inner(expr, &mut out);
    out
}
pub(super) fn collect_consts_inner(expr: &Expr, out: &mut Vec<Name>) {
    match expr {
        Expr::Const(n, _) => out.push(n.clone()),
        Expr::App(f, a) => {
            collect_consts_inner(f, out);
            collect_consts_inner(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_consts_inner(ty, out);
            collect_consts_inner(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_consts_inner(ty, out);
            collect_consts_inner(val, out);
            collect_consts_inner(body, out);
        }
        _ => {}
    }
}
#[cfg(test)]
mod structural_new_tests {
    use super::*;
    use crate::basic::MetaContext;
    use crate::tactic::structural::*;
    use oxilean_kernel::Level;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(oxilean_kernel::Environment::new())
    }
    #[test]
    fn test_is_bvar_zero_true() {
        assert!(is_bvar_zero(&Expr::BVar(0)));
    }
    #[test]
    fn test_is_bvar_zero_false() {
        assert!(!is_bvar_zero(&Expr::BVar(1)));
        assert!(!is_bvar_zero(&Expr::Const(Name::str("x"), vec![])));
    }
    #[test]
    fn test_is_sort_expr_prop() {
        let p = Expr::Sort(Level::zero());
        assert!(is_sort_expr(&p));
        assert!(is_prop_sort(&p));
        assert!(!is_type_zero_expr(&p));
    }
    #[test]
    fn test_is_type_zero_expr() {
        let t = Expr::Sort(Level::succ(Level::zero()));
        assert!(is_sort_expr(&t));
        assert!(!is_prop_sort(&t));
        assert!(is_type_zero_expr(&t));
    }
    #[test]
    fn test_hyp_summary_from_ctx_empty() {
        let ctx = mk_ctx();
        let summary = HypSummary::from_ctx(&ctx);
        assert_eq!(summary.count, 0);
        assert!(summary.names.is_empty());
    }
    #[test]
    fn test_hyp_summary_has() {
        let mut ctx = mk_ctx();
        ctx.mk_local_decl(
            Name::str("h"),
            Expr::Sort(Level::zero()),
            BinderInfo::Default,
        );
        let summary = HypSummary::from_ctx(&ctx);
        assert!(summary.has(&Name::str("h")));
        assert!(!summary.has(&Name::str("x")));
    }
    #[test]
    fn test_name_in_list_found() {
        let names = vec![Name::str("a"), Name::str("b")];
        assert!(name_in_list(&Name::str("a"), &names));
    }
    #[test]
    fn test_name_in_list_not_found() {
        let names = vec![Name::str("a")];
        assert!(!name_in_list(&Name::str("z"), &names));
    }
    #[test]
    fn test_build_pi_chain() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let chain = build_pi_chain(3, nat, body);
        assert_eq!(count_pi_binders_unique(&chain), 3);
    }
    #[test]
    fn test_build_lam_chain() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let chain = build_lam_chain(2, nat, body);
        assert_eq!(count_lam_binders_unique(&chain), 2);
    }
    #[test]
    fn test_strip_lam_n() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let body = Expr::BVar(0);
        let chain = build_lam_chain(2, nat, body.clone());
        let stripped = strip_lam_n(&chain, 2);
        assert_eq!(stripped, &body);
    }
    #[test]
    fn test_peel_pi_n() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let chain = build_pi_chain(3, nat.clone(), nat);
        let (binders, _body) = peel_pi_n(&chain, 2);
        assert_eq!(binders.len(), 2);
    }
    #[test]
    fn test_collect_consts_from_expr() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let e = Expr::App(Box::new(a), Box::new(b));
        let cs = collect_consts_from_expr(&e);
        assert_eq!(cs.len(), 2);
    }
    #[test]
    fn test_is_fvar() {
        use oxilean_kernel::FVarId;
        let fv = Expr::FVar(FVarId::new(0));
        assert!(is_fvar(&fv));
        assert!(!is_fvar(&Expr::BVar(0)));
    }
}
#[cfg(test)]
mod tacstruct_ext2_tests {
    use super::*;
    use crate::tactic::structural::*;
    #[test]
    fn test_tacstruct_ext_util_basic() {
        let mut u = TacStructExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_tacstruct_ext_util_min_max() {
        let mut u = TacStructExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_tacstruct_ext_util_flags() {
        let mut u = TacStructExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_tacstruct_ext_util_pop() {
        let mut u = TacStructExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_tacstruct_ext_map_basic() {
        let mut m: TacStructExtMap<i32> = TacStructExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_tacstruct_ext_map_get_or_default() {
        let mut m: TacStructExtMap<i32> = TacStructExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_tacstruct_ext_map_keys_sorted() {
        let mut m: TacStructExtMap<i32> = TacStructExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_tacstruct_window_mean() {
        let mut w = TacStructWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacstruct_window_evict() {
        let mut w = TacStructWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacstruct_window_std_dev() {
        let mut w = TacStructWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_tacstruct_builder_basic() {
        let b = TacStructBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_tacstruct_builder_summary() {
        let b = TacStructBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_tacstruct_state_machine_start() {
        let mut sm = TacStructStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_tacstruct_state_machine_complete() {
        let mut sm = TacStructStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_tacstruct_state_machine_fail() {
        let mut sm = TacStructStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_tacstruct_state_machine_no_transition_after_terminal() {
        let mut sm = TacStructStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_tacstruct_work_queue_basic() {
        let mut wq = TacStructWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_tacstruct_work_queue_capacity() {
        let mut wq = TacStructWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_tacstruct_counter_map_basic() {
        let mut cm = TacStructCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_tacstruct_counter_map_frequency() {
        let mut cm = TacStructCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacstruct_counter_map_most_common() {
        let mut cm = TacStructCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticstructural_analysis_tests {
    use super::*;
    use crate::tactic::structural::*;
    #[test]
    fn test_tacticstructural_result_ok() {
        let r = TacticStructuralResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticstructural_result_err() {
        let r = TacticStructuralResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticstructural_result_partial() {
        let r = TacticStructuralResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticstructural_result_skipped() {
        let r = TacticStructuralResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticstructural_analysis_pass_run() {
        let mut p = TacticStructuralAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticstructural_analysis_pass_empty_input() {
        let mut p = TacticStructuralAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticstructural_analysis_pass_success_rate() {
        let mut p = TacticStructuralAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticstructural_analysis_pass_disable() {
        let mut p = TacticStructuralAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticstructural_pipeline_basic() {
        let mut pipeline = TacticStructuralPipeline::new("main_pipeline");
        pipeline.add_pass(TacticStructuralAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticStructuralAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticstructural_pipeline_disabled_pass() {
        let mut pipeline = TacticStructuralPipeline::new("partial");
        let mut p = TacticStructuralAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticStructuralAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticstructural_diff_basic() {
        let mut d = TacticStructuralDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticstructural_diff_summary() {
        let mut d = TacticStructuralDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticstructural_config_set_get() {
        let mut cfg = TacticStructuralConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticstructural_config_read_only() {
        let mut cfg = TacticStructuralConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticstructural_config_remove() {
        let mut cfg = TacticStructuralConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticstructural_diagnostics_basic() {
        let mut diag = TacticStructuralDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticstructural_diagnostics_max_errors() {
        let mut diag = TacticStructuralDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticstructural_diagnostics_clear() {
        let mut diag = TacticStructuralDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticstructural_config_value_types() {
        let b = TacticStructuralConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticStructuralConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticStructuralConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticStructuralConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticStructuralConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod structural_ext_tests_2400 {
    use super::*;
    use crate::tactic::structural::*;
    #[test]
    fn test_structural_ext_result_ok_2400() {
        let r = StructuralExtResult2400::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_structural_ext_result_err_2400() {
        let r = StructuralExtResult2400::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_structural_ext_result_partial_2400() {
        let r = StructuralExtResult2400::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_structural_ext_result_skipped_2400() {
        let r = StructuralExtResult2400::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_structural_ext_pass_run_2400() {
        let mut p = StructuralExtPass2400::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_structural_ext_pass_empty_2400() {
        let mut p = StructuralExtPass2400::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_structural_ext_pass_rate_2400() {
        let mut p = StructuralExtPass2400::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_structural_ext_pass_disable_2400() {
        let mut p = StructuralExtPass2400::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_structural_ext_pipeline_basic_2400() {
        let mut pipeline = StructuralExtPipeline2400::new("main_pipeline");
        pipeline.add_pass(StructuralExtPass2400::new("pass1"));
        pipeline.add_pass(StructuralExtPass2400::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_structural_ext_pipeline_disabled_2400() {
        let mut pipeline = StructuralExtPipeline2400::new("partial");
        let mut p = StructuralExtPass2400::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(StructuralExtPass2400::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_structural_ext_diff_basic_2400() {
        let mut d = StructuralExtDiff2400::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_structural_ext_config_set_get_2400() {
        let mut cfg = StructuralExtConfig2400::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_structural_ext_config_read_only_2400() {
        let mut cfg = StructuralExtConfig2400::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_structural_ext_config_remove_2400() {
        let mut cfg = StructuralExtConfig2400::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_structural_ext_diagnostics_basic_2400() {
        let mut diag = StructuralExtDiag2400::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_structural_ext_diagnostics_max_errors_2400() {
        let mut diag = StructuralExtDiag2400::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_structural_ext_diagnostics_clear_2400() {
        let mut diag = StructuralExtDiag2400::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_structural_ext_config_value_types_2400() {
        let b = StructuralExtConfigVal2400::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = StructuralExtConfigVal2400::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = StructuralExtConfigVal2400::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = StructuralExtConfigVal2400::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = StructuralExtConfigVal2400::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
