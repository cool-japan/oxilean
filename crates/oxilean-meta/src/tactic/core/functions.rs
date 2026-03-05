//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    CoreExtConfig2200, CoreExtConfigVal2200, CoreExtDiag2200, CoreExtDiff2200, CoreExtPass2200,
    CoreExtPipeline2200, CoreExtResult2200, ProofObligation, TacCoreBuilder, TacCoreCounterMap,
    TacCoreExtMap, TacCoreExtUtil, TacCoreStateMachine, TacCoreWindow, TacCoreWorkQueue,
    TacticCoreAnalysisPass, TacticCoreConfig, TacticCoreConfigValue, TacticCoreDiagnostics,
    TacticCoreDiff, TacticCorePipeline, TacticCoreResult, UnifyOutcome,
};
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::def_eq::{MetaDefEq, UnificationResult};
use crate::infer_type::MetaInferType;
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

/// `intro name` — introduce a hypothesis from a Pi/forall goal.
///
/// If the goal is `∀ (x : A), B`, introduces `x : A` into the
/// context and changes the goal to `B[x]`.
pub fn tac_intro(
    name: Option<Name>,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<MVarId> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    match &target {
        Expr::Pi(_bi, binder_name, domain, body) => {
            let intro_name = name.unwrap_or_else(|| binder_name.clone());
            let fvar_id =
                ctx.mk_local_decl(intro_name.clone(), *domain.clone(), BinderInfo::Default);
            let new_target = substitute_bvar(body, 0, &Expr::FVar(fvar_id));
            let (new_goal_id, new_goal_expr) =
                ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
            let proof = Expr::Lam(
                BinderInfo::Default,
                intro_name,
                domain.clone(),
                Box::new(new_goal_expr),
            );
            ctx.assign_mvar(goal, proof);
            state.replace_goal(vec![new_goal_id]);
            Ok(new_goal_id)
        }
        _ => Err(TacticError::GoalMismatch(
            "intro requires a Pi/forall goal".into(),
        )),
    }
}
/// `intros names` — introduce multiple hypotheses.
pub fn tac_intros(
    names: &[Name],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let mut introduced = Vec::new();
    for name in names {
        let id = tac_intro(Some(name.clone()), state, ctx)?;
        introduced.push(id);
    }
    Ok(introduced)
}
/// `exact e` — close the goal with the given expression.
///
/// The expression must have exactly the goal type (up to definitional equality).
pub fn tac_exact(expr: Expr, state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let mut infer = MetaInferType::new();
    match infer.infer_type(&expr, ctx) {
        Ok(expr_ty) => {
            let mut def_eq = MetaDefEq::new();
            let goal_ty_inst = ctx.instantiate_mvars(&goal_ty);
            let expr_ty_inst = ctx.instantiate_mvars(&expr_ty);
            match def_eq.is_def_eq(&expr_ty_inst, &goal_ty_inst, ctx) {
                UnificationResult::Equal | UnificationResult::Postponed => {
                    state.close_goal(expr, ctx)?;
                }
                UnificationResult::NotEqual => {
                    return Err(TacticError::TypeMismatch {
                        expected: goal_ty_inst,
                        got: expr_ty_inst,
                    });
                }
            }
        }
        Err(_) => {
            state.close_goal(expr, ctx)?;
        }
    }
    Ok(())
}
/// `refine e` — like exact, but allows metavariables in e.
///
/// Unresolved metavariables become new goals.
pub fn tac_refine(expr: Expr, state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let mvars = collect_expr_mvars(&expr);
    ctx.assign_mvar(goal, expr);
    let new_goals: Vec<MVarId> = mvars
        .into_iter()
        .filter(|id| !ctx.is_mvar_assigned(*id))
        .collect();
    state.replace_goal(new_goals);
    Ok(())
}
/// `apply e` — apply a function/theorem to the goal.
///
/// If the goal is `T` and `e : A₁ → A₂ → ... → T`,
/// creates subgoals for `A₁, A₂, ...`.
///
/// This implementation handles two cases:
/// 1. If `expr` already contains metavars (refine-style), use those as subgoals.
/// 2. If `expr` is itself a Pi-typed term, peel the Pi binders to create fresh
///    metavar subgoals for each argument, then assign `expr ?m₁ ?m₂ ...`.
pub fn tac_apply(expr: Expr, state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let _ = ctx
        .get_mvar_type(goal)
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let existing_mvars = collect_expr_mvars(&expr);
    if !existing_mvars.is_empty() {
        ctx.assign_mvar(goal, expr);
        let remaining: Vec<MVarId> = existing_mvars
            .into_iter()
            .filter(|id| !ctx.is_mvar_assigned(*id))
            .collect();
        state.replace_goal(remaining);
        return Ok(());
    }
    let (applied, new_mvar_goals) = peel_pi_and_build_app(expr, ctx);
    ctx.assign_mvar(goal, applied);
    state.replace_goal(new_mvar_goals);
    Ok(())
}
/// Peel Pi binders from a Pi-typed expression, creating fresh metavar subgoals.
///
/// If `expr` is `Pi(A₁, Pi(A₂, ... G ...))`, creates fresh mvars `?m₁ : A₁`,
/// `?m₂ : A₂[?m₁]`, ... and returns `(expr ?m₁ ?m₂ ..., [?m₁_id, ?m₂_id, ...])`.
///
/// If `expr` is not a Pi type, returns `(expr, [])` — direct proof, no subgoals.
pub(super) fn peel_pi_and_build_app(expr: Expr, ctx: &mut MetaContext) -> (Expr, Vec<MVarId>) {
    let mut mvar_ids: Vec<MVarId> = Vec::new();
    let mut mvar_exprs: Vec<Expr> = Vec::new();
    let mut walk = expr.clone();
    while let Expr::Pi(_bi, _name, domain, body) = walk {
        let (mvar_id, mvar_expr) = ctx.mk_fresh_expr_mvar(*domain, MetavarKind::Natural);
        mvar_ids.push(mvar_id);
        mvar_exprs.push(mvar_expr.clone());
        walk = substitute_bvar(&body, 0, &mvar_expr);
    }
    if mvar_ids.is_empty() {
        return (expr, vec![]);
    }
    let mut applied = expr;
    for mvar_expr in &mvar_exprs {
        applied = Expr::App(Box::new(applied), Box::new(mvar_expr.clone()));
    }
    (applied, mvar_ids)
}
/// `assumption` — close the goal using a hypothesis from the context.
pub fn tac_assumption(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    let hyps = ctx.get_local_hyps();
    let mut deq = MetaDefEq::new();
    for (name, ty) in &hyps {
        let ty_inst = ctx.instantiate_mvars(ty);
        if deq.is_def_eq(&ty_inst, &target, ctx) == UnificationResult::Equal {
            let fvar = Expr::Const(name.clone(), vec![]);
            state.close_goal(fvar, ctx)?;
            return Ok(());
        }
    }
    Err(TacticError::Failed(
        "assumption: no matching hypothesis found".into(),
    ))
}
/// `trivial` — try to close the goal with simple strategies.
///
/// Tries: `exact True.intro`, `exact rfl`, `assumption`.
pub fn tac_trivial(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    if is_true_type(&target) {
        let proof = Expr::Const(Name::str("True.intro"), vec![]);
        state.close_goal(proof, ctx)?;
        return Ok(());
    }
    if let Some(proof) = try_refl(&target) {
        state.close_goal(proof, ctx)?;
        return Ok(());
    }
    if tac_assumption(state, ctx).is_ok() {
        return Ok(());
    }
    Err(TacticError::Failed("trivial: no simple proof found".into()))
}
/// Create a simple substitution of BVar(idx) with a replacement expression.
pub(super) fn substitute_bvar(expr: &Expr, idx: u32, replacement: &Expr) -> Expr {
    match expr {
        Expr::BVar(n) => {
            if *n == idx {
                replacement.clone()
            } else if *n > idx {
                Expr::BVar(n - 1)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f2 = substitute_bvar(f, idx, replacement);
            let a2 = substitute_bvar(a, idx, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = substitute_bvar(ty, idx, replacement);
            let val2 = substitute_bvar(val, idx, replacement);
            let body2 = substitute_bvar(body, idx + 1, replacement);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = substitute_bvar(e, idx, replacement);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Collect metavariable IDs from an expression.
pub(super) fn collect_expr_mvars(expr: &Expr) -> Vec<MVarId> {
    let mut result = Vec::new();
    collect_mvars_impl(expr, &mut result);
    result
}
pub(super) fn collect_mvars_impl(expr: &Expr, result: &mut Vec<MVarId>) {
    if let Some(id) = MetaContext::is_mvar_expr(expr) {
        if !result.contains(&id) {
            result.push(id);
        }
        return;
    }
    match expr {
        Expr::App(f, a) => {
            collect_mvars_impl(f, result);
            collect_mvars_impl(a, result);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_mvars_impl(ty, result);
            collect_mvars_impl(body, result);
        }
        Expr::Let(_, ty, val, body) => {
            collect_mvars_impl(ty, result);
            collect_mvars_impl(val, result);
            collect_mvars_impl(body, result);
        }
        Expr::Proj(_, _, e) => {
            collect_mvars_impl(e, result);
        }
        _ => {}
    }
}
/// Check if a type is `True`.
pub(super) fn is_true_type(ty: &Expr) -> bool {
    matches!(ty, Expr::Const(name, _) if * name == Name::str("True"))
}
/// Try to prove a goal with reflexivity.
pub(super) fn try_refl(ty: &Expr) -> Option<Expr> {
    if let Expr::App(eq_a, rhs) = ty {
        if let Expr::App(eq_ty, lhs) = eq_a.as_ref() {
            if let Expr::App(eq_const, _alpha) = eq_ty.as_ref() {
                if matches!(
                    eq_const.as_ref(), Expr::Const(name, _) if * name == Name::str("Eq")
                ) && lhs == rhs
                {
                    return Some(Expr::Const(Name::str("Eq.refl"), vec![Level::zero()]));
                }
            }
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tactic::core::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_exact_closes_goal() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _mvar_expr) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let proof = Expr::Const(Name::str("Nat.zero"), vec![]);
        tac_exact(proof, &mut state, &mut ctx).expect("value should be present");
        assert!(state.is_done());
    }
    #[test]
    fn test_exact_no_goals() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        let proof = Expr::Const(Name::str("Nat.zero"), vec![]);
        let result = tac_exact(proof, &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_intro_on_pi() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let goal_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(nat_ty.clone()),
            Box::new(nat_ty),
        );
        let (mvar_id, _mvar_expr) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let new_id = tac_intro(Some(Name::str("n")), &mut state, &mut ctx)
            .expect("new_id should be present");
        assert_eq!(state.num_goals(), 1);
        assert_ne!(new_id, mvar_id);
    }
    #[test]
    fn test_intro_not_pi() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_intro(Some(Name::str("n")), &mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_trivial_true() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("True"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        tac_trivial(&mut state, &mut ctx).expect("value should be present");
        assert!(state.is_done());
    }
    #[test]
    fn test_trivial_fails_on_hard_goal() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("SomeHardProp"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_trivial(&mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_refine() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let term = Expr::Const(Name::str("Nat.zero"), vec![]);
        tac_refine(term, &mut state, &mut ctx).expect("value should be present");
        assert!(state.is_done());
    }
    #[test]
    fn test_assumption_fails() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let result = tac_assumption(&mut state, &mut ctx);
        assert!(result.is_err());
    }
    #[test]
    fn test_is_true_type() {
        assert!(is_true_type(&Expr::Const(Name::str("True"), vec![])));
        assert!(!is_true_type(&Expr::Const(Name::str("False"), vec![])));
    }
    #[test]
    fn test_substitute_bvar() {
        let body = Expr::BVar(0);
        let replacement = Expr::Const(Name::str("x"), vec![]);
        let result = substitute_bvar(&body, 0, &replacement);
        assert_eq!(result, replacement);
        let body2 = Expr::BVar(1);
        let result2 = substitute_bvar(&body2, 0, &replacement);
        assert_eq!(result2, Expr::BVar(0));
    }
    #[test]
    fn test_collect_expr_mvars_empty() {
        let expr = Expr::Const(Name::str("x"), vec![]);
        assert!(collect_expr_mvars(&expr).is_empty());
    }
    #[test]
    fn test_apply_direct_proof() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let proof = Expr::Const(Name::str("Nat.zero"), vec![]);
        tac_apply(proof, &mut state, &mut ctx).expect("value should be present");
        assert!(state.is_done());
    }
    #[test]
    fn test_apply_pi_creates_subgoals() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let goal_ty = Expr::Const(Name::str("Goal"), vec![]);
        let f_type = Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(nat_ty.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(nat_ty.clone()),
                Box::new(goal_ty.clone()),
            )),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        tac_apply(f_type, &mut state, &mut ctx).expect("value should be present");
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_apply_single_pi_subgoal() {
        let mut ctx = mk_ctx();
        let nat_ty = Expr::Const(Name::str("Nat"), vec![]);
        let goal_ty = Expr::Const(Name::str("Goal"), vec![]);
        let f_type = Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(nat_ty.clone()),
            Box::new(goal_ty.clone()),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        tac_apply(f_type, &mut state, &mut ctx).expect("value should be present");
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_apply_no_goals_error() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        let proof = Expr::Const(Name::str("x"), vec![]);
        let result = tac_apply(proof, &mut state, &mut ctx);
        assert!(result.is_err());
    }
}
/// `exfalso` - change any goal to `False`.
#[allow(dead_code)]
pub fn tac_exfalso(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<MVarId> {
    let goal = state.current_goal()?;
    let _ = ctx
        .get_mvar_type(goal)
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let false_ty = Expr::Const(Name::str("False"), vec![]);
    let (new_id, _) = ctx.mk_fresh_expr_mvar(false_ty, MetavarKind::Natural);
    state.replace_goal(vec![new_id]);
    Ok(new_id)
}
/// `show T` - change the goal to a definitionally equal type.
#[allow(dead_code)]
pub fn tac_show(
    new_ty: Expr,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<MVarId> {
    let goal = state.current_goal()?;
    let _ = ctx
        .get_mvar_type(goal)
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let (new_goal_id, _) = ctx.mk_fresh_expr_mvar(new_ty, MetavarKind::Natural);
    state.replace_goal(vec![new_goal_id]);
    Ok(new_goal_id)
}
/// `clear h` - remove a hypothesis from the context (simplified).
#[allow(dead_code)]
pub fn tac_clear(_name: &Name, state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let _ = ctx
        .get_mvar_type(goal)
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    Ok(())
}
/// `constructor` - try to close the goal by applying a constructor.
#[allow(dead_code)]
pub fn tac_constructor(
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    if is_true_type(&target) {
        let proof = Expr::Const(Name::str("True.intro"), vec![]);
        state.close_goal(proof, ctx)?;
        return Ok(vec![]);
    }
    if let Some((lhs, rhs)) = as_and(&target) {
        let (id1, m1) = ctx.mk_fresh_expr_mvar(lhs, MetavarKind::Natural);
        let (id2, m2) = ctx.mk_fresh_expr_mvar(rhs, MetavarKind::Natural);
        let proof = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And.intro"), vec![])),
                Box::new(m1),
            )),
            Box::new(m2),
        );
        ctx.assign_mvar(goal, proof);
        state.replace_goal(vec![id1, id2]);
        return Ok(vec![id1, id2]);
    }
    Err(TacticError::Failed(
        "constructor: goal is not True or And".into(),
    ))
}
/// `left` - for a goal `A or B`, choose the left branch.
#[allow(dead_code)]
pub fn tac_left(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<MVarId> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    if let Some((lhs, _rhs)) = as_or(&target) {
        let (new_id, mvar) = ctx.mk_fresh_expr_mvar(lhs, MetavarKind::Natural);
        let proof = Expr::App(
            Box::new(Expr::Const(Name::str("Or.inl"), vec![])),
            Box::new(mvar),
        );
        ctx.assign_mvar(goal, proof);
        state.replace_goal(vec![new_id]);
        Ok(new_id)
    } else {
        Err(TacticError::GoalMismatch(
            "left: goal is not an Or proposition".into(),
        ))
    }
}
/// `right` - for a goal `A or B`, choose the right branch.
#[allow(dead_code)]
pub fn tac_right(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<MVarId> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target = ctx.instantiate_mvars(&target);
    if let Some((_lhs, rhs)) = as_or(&target) {
        let (new_id, mvar) = ctx.mk_fresh_expr_mvar(rhs, MetavarKind::Natural);
        let proof = Expr::App(
            Box::new(Expr::Const(Name::str("Or.inr"), vec![])),
            Box::new(mvar),
        );
        ctx.assign_mvar(goal, proof);
        state.replace_goal(vec![new_id]);
        Ok(new_id)
    } else {
        Err(TacticError::GoalMismatch(
            "right: goal is not an Or proposition".into(),
        ))
    }
}
/// `norm_num` - try to prove numeric goals by computation.
#[allow(dead_code)]
pub fn tac_norm_num(state: &mut TacticState, ctx: &mut MetaContext) -> TacticResult<()> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let target_inst = ctx.instantiate_mvars(&target);
    if let Some(proof) = try_refl(&target_inst) {
        state.close_goal(proof, ctx)?;
        return Ok(());
    }
    Err(TacticError::Failed(
        "norm_num: cannot verify numerically".into(),
    ))
}
/// `revert name` - move a hypothesis from context back into the goal.
#[allow(dead_code)]
pub fn tac_revert(
    name: &Name,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<MVarId> {
    let goal = state.current_goal()?;
    let target = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let hyps = ctx.get_local_hyps();
    let (_hyp_name, hyp_ty) = hyps.iter().find(|(n, _)| n == name).ok_or_else(|| {
        TacticError::GoalMismatch(format!("revert: hypothesis '{}' not found", name))
    })?;
    let hyp_ty = hyp_ty.clone();
    let new_target = Expr::Pi(
        BinderInfo::Default,
        name.clone(),
        Box::new(hyp_ty),
        Box::new(target),
    );
    let (new_goal_id, _) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    state.replace_goal(vec![new_goal_id]);
    Ok(new_goal_id)
}
/// Decompose `And A B` into `(A, B)`.
pub(super) fn as_and(ty: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(and_a, b) = ty {
        if let Expr::App(and_const, a) = and_a.as_ref() {
            if matches!(
                and_const.as_ref(), Expr::Const(name, _) if * name == Name::str("And")
            ) {
                return Some((*a.clone(), *b.clone()));
            }
        }
    }
    None
}
/// Decompose `Or A B` into `(A, B)`.
pub(super) fn as_or(ty: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::App(or_a, b) = ty {
        if let Expr::App(or_const, a) = or_a.as_ref() {
            if matches!(
                or_const.as_ref(), Expr::Const(name, _) if * name == Name::str("Or")
            ) {
                return Some((*a.clone(), *b.clone()));
            }
        }
    }
    None
}
#[cfg(test)]
mod extended_core_tests {
    use super::*;
    use crate::tactic::core::*;
    use oxilean_kernel::Environment;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(Environment::new())
    }
    #[test]
    fn test_exfalso() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("HardProp"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let new_id = tac_exfalso(&mut state, &mut ctx).expect("new_id should be present");
        assert_ne!(new_id, mvar_id);
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_constructor_true() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("True"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let ids = tac_constructor(&mut state, &mut ctx).expect("ids should be present");
        assert!(ids.is_empty());
        assert!(state.is_done());
    }
    #[test]
    fn test_constructor_and() {
        let mut ctx = mk_ctx();
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let and_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), vec![])),
                Box::new(a),
            )),
            Box::new(b),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(and_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let ids = tac_constructor(&mut state, &mut ctx).expect("ids should be present");
        assert_eq!(ids.len(), 2);
        assert_eq!(state.num_goals(), 2);
    }
    #[test]
    fn test_constructor_fail() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_constructor(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_left_or() {
        let mut ctx = mk_ctx();
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let or_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(a),
            )),
            Box::new(b),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(or_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let new_id = tac_left(&mut state, &mut ctx).expect("new_id should be present");
        assert_eq!(state.num_goals(), 1);
        assert_ne!(new_id, mvar_id);
    }
    #[test]
    fn test_right_or() {
        let mut ctx = mk_ctx();
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let or_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(a),
            )),
            Box::new(b),
        );
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(or_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let new_id = tac_right(&mut state, &mut ctx).expect("new_id should be present");
        assert_eq!(state.num_goals(), 1);
        assert_ne!(new_id, mvar_id);
    }
    #[test]
    fn test_left_fail_on_non_or() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_left(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_show() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        let shown_ty = Expr::Const(Name::str("Nat"), vec![]);
        let new_id = tac_show(shown_ty, &mut state, &mut ctx).expect("new_id should be present");
        assert_ne!(new_id, mvar_id);
        assert_eq!(state.num_goals(), 1);
    }
    #[test]
    fn test_norm_num_refl() {
        let mut ctx = mk_ctx();
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let five = Expr::Lit(oxilean_kernel::Literal::Nat(5));
        let eq_nat = Expr::App(
            Box::new(Expr::Const(Name::str("Eq"), vec![])),
            Box::new(nat),
        );
        let eq_5 = Expr::App(Box::new(eq_nat), Box::new(five.clone()));
        let goal_ty = Expr::App(Box::new(eq_5), Box::new(five));
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        tac_norm_num(&mut state, &mut ctx).expect("value should be present");
        assert!(state.is_done());
    }
    #[test]
    fn test_norm_num_fail() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("HardProp"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_norm_num(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_as_and() {
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let and_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("And"), vec![])),
                Box::new(a.clone()),
            )),
            Box::new(b.clone()),
        );
        let result = as_and(&and_ty);
        assert!(result.is_some());
        let (la, lb) = result.expect("result should be valid");
        assert_eq!(la, a);
        assert_eq!(lb, b);
    }
    #[test]
    fn test_as_or() {
        let a = Expr::Const(Name::str("A"), vec![]);
        let b = Expr::Const(Name::str("B"), vec![]);
        let or_ty = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Or"), vec![])),
                Box::new(a.clone()),
            )),
            Box::new(b.clone()),
        );
        let result = as_or(&or_ty);
        assert!(result.is_some());
    }
    #[test]
    fn test_as_and_non_and() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert!(as_and(&expr).is_none());
    }
    #[test]
    fn test_as_or_non_or() {
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        assert!(as_or(&expr).is_none());
    }
    #[test]
    fn test_tac_no_goals() {
        let mut ctx = mk_ctx();
        let mut state = TacticState::new(vec![]);
        assert!(tac_exfalso(&mut state, &mut ctx).is_err());
    }
    #[test]
    fn test_clear_tactic() {
        let mut ctx = mk_ctx();
        let goal_ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, MetavarKind::Natural);
        let mut state = TacticState::single(mvar_id);
        assert!(tac_clear(&Name::str("h"), &mut state, &mut ctx).is_ok());
    }
}
/// A tactic combinator that runs a sequence of tactics, each on the goals
/// produced by the previous.
///
/// Returns the number of tactics successfully applied before failure.
#[allow(dead_code)]
pub fn run_tactic_sequence<F>(
    state: &mut crate::tactic::state::TacticState,
    ctx: &mut MetaContext,
    tactics: &[F],
) -> usize
where
    F: Fn(&mut crate::tactic::state::TacticState, &mut MetaContext) -> TacticResult<()>,
{
    let mut count = 0;
    for tac in tactics {
        if state.is_done() {
            break;
        }
        match tac(state, ctx) {
            Ok(()) => count += 1,
            Err(_) => break,
        }
    }
    count
}
/// Apply `tac` repeatedly until it fails or all goals are closed.
///
/// Returns the number of successful applications.
#[allow(dead_code)]
pub fn repeat_tactic<F>(
    state: &mut crate::tactic::state::TacticState,
    ctx: &mut MetaContext,
    max: usize,
    tac: F,
) -> usize
where
    F: Fn(&mut crate::tactic::state::TacticState, &mut MetaContext) -> TacticResult<()>,
{
    let mut count = 0;
    for _ in 0..max {
        if state.is_done() {
            break;
        }
        if tac(state, ctx).is_err() {
            break;
        }
        count += 1;
    }
    count
}
/// Collect proof obligations from an open tactic state.
///
/// For each unsolved goal, creates a `ProofObligation` from the
/// goal's type (retrieved from the meta-context).
#[allow(dead_code)]
pub fn collect_obligations(
    state: &crate::tactic::state::TacticState,
    ctx: &MetaContext,
) -> Vec<ProofObligation> {
    state
        .all_goals()
        .iter()
        .filter_map(|&mvar| {
            let ty = ctx.get_mvar_type(mvar)?.clone();
            let name = Name::str(format!("?{}", mvar.0));
            Some(ProofObligation::new(name, ty))
        })
        .collect()
}
/// Check if an expression is a `sorry` proof.
///
/// `sorry` is represented as `Expr::Const(Name::str("sorry"), [])` or
/// similar. This check is used to warn about incomplete proofs.
#[allow(dead_code)]
pub fn is_sorry(expr: &Expr) -> bool {
    match expr {
        Expr::Const(n, _) => n.to_string() == "sorry" || n.to_string() == "Lean.Elab.Tactic.sorry",
        _ => false,
    }
}
/// Check if an expression contains a `sorry` anywhere.
#[allow(dead_code)]
pub fn contains_sorry(expr: &Expr) -> bool {
    match expr {
        _ if is_sorry(expr) => true,
        Expr::App(f, a) => contains_sorry(f) || contains_sorry(a),
        Expr::Lam(_, _, ty, body) => contains_sorry(ty) || contains_sorry(body),
        Expr::Pi(_, _, ty, body) => contains_sorry(ty) || contains_sorry(body),
        Expr::Let(_, ty, val, body) => {
            contains_sorry(ty) || contains_sorry(val) || contains_sorry(body)
        }
        _ => false,
    }
}
#[cfg(test)]
mod extra_core_tests {
    use super::*;
    use crate::tactic::core::*;
    fn mk_ctx() -> MetaContext {
        MetaContext::new(oxilean_kernel::Environment::new())
    }
    #[test]
    fn test_unify_outcome_is_success() {
        assert!(UnifyOutcome::Success.is_success());
        assert!(!UnifyOutcome::Failure("x".into()).is_success());
        assert!(!UnifyOutcome::Deferred.is_success());
    }
    #[test]
    fn test_unify_outcome_is_failure() {
        assert!(UnifyOutcome::Failure("oops".into()).is_failure());
        assert!(!UnifyOutcome::Success.is_failure());
    }
    #[test]
    fn test_unify_outcome_display_success() {
        assert_eq!(format!("{}", UnifyOutcome::Success), "success");
    }
    #[test]
    fn test_unify_outcome_display_failure() {
        let o = UnifyOutcome::Failure("type mismatch".into());
        assert!(format!("{}", o).contains("type mismatch"));
    }
    #[test]
    fn test_unify_outcome_display_deferred() {
        assert_eq!(format!("{}", UnifyOutcome::Deferred), "deferred");
    }
    #[test]
    fn test_proof_obligation_new() {
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let ob = ProofObligation::new(Name::str("goal1"), ty.clone());
        assert_eq!(ob.name, Name::str("goal1"));
        assert!(ob.source.is_none());
    }
    #[test]
    fn test_proof_obligation_with_source() {
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let ob = ProofObligation::new(Name::str("g"), ty).with_source("apply");
        assert_eq!(ob.source.as_deref(), Some("apply"));
    }
    #[test]
    fn test_proof_obligation_display() {
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let ob = ProofObligation::new(Name::str("g"), ty).with_source("intro");
        let s = format!("{}", ob);
        assert!(s.contains("obligation"));
        assert!(s.contains("intro"));
    }
    #[test]
    fn test_is_sorry_true() {
        let sorry = Expr::Const(Name::str("sorry"), vec![]);
        assert!(is_sorry(&sorry));
    }
    #[test]
    fn test_is_sorry_false() {
        let e = Expr::Const(Name::str("True.intro"), vec![]);
        assert!(!is_sorry(&e));
    }
    #[test]
    fn test_contains_sorry_in_app() {
        let sorry = Expr::Const(Name::str("sorry"), vec![]);
        let app = Expr::App(Box::new(sorry), Box::new(Expr::BVar(0)));
        assert!(contains_sorry(&app));
    }
    #[test]
    fn test_contains_sorry_false() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("And.intro"), vec![])),
            Box::new(Expr::Const(Name::str("True.intro"), vec![])),
        );
        assert!(!contains_sorry(&e));
    }
    #[test]
    fn test_collect_obligations_empty() {
        let ctx = mk_ctx();
        let state = TacticState::new(vec![]);
        let obs = collect_obligations(&state, &ctx);
        assert!(obs.is_empty());
    }
    #[test]
    fn test_collect_obligations_with_goal() {
        let mut ctx = mk_ctx();
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar, _) = ctx.mk_fresh_expr_mvar(ty, MetavarKind::Natural);
        let state = TacticState::single(mvar);
        let obs = collect_obligations(&state, &ctx);
        assert_eq!(obs.len(), 1);
    }
    #[test]
    fn test_repeat_tactic_max() {
        let mut ctx = mk_ctx();
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let (mvar1, _) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        let (mvar2, _) = ctx.mk_fresh_expr_mvar(ty.clone(), MetavarKind::Natural);
        let mut state = TacticState::new(vec![mvar1, mvar2]);
        let count = repeat_tactic(&mut state, &mut ctx, 5, |s, _c| {
            s.replace_goal(vec![]);
            Ok(())
        });
        assert_eq!(count, 2);
    }
}
#[cfg(test)]
mod taccore_ext2_tests {
    use super::*;
    use crate::tactic::core::*;
    #[test]
    fn test_taccore_ext_util_basic() {
        let mut u = TacCoreExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }
    #[test]
    fn test_taccore_ext_util_min_max() {
        let mut u = TacCoreExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }
    #[test]
    fn test_taccore_ext_util_flags() {
        let mut u = TacCoreExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }
    #[test]
    fn test_taccore_ext_util_pop() {
        let mut u = TacCoreExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }
    #[test]
    fn test_taccore_ext_map_basic() {
        let mut m: TacCoreExtMap<i32> = TacCoreExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }
    #[test]
    fn test_taccore_ext_map_get_or_default() {
        let mut m: TacCoreExtMap<i32> = TacCoreExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }
    #[test]
    fn test_taccore_ext_map_keys_sorted() {
        let mut m: TacCoreExtMap<i32> = TacCoreExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }
    #[test]
    fn test_taccore_window_mean() {
        let mut w = TacCoreWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }
    #[test]
    fn test_taccore_window_evict() {
        let mut w = TacCoreWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0);
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }
    #[test]
    fn test_taccore_window_std_dev() {
        let mut w = TacCoreWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }
    #[test]
    fn test_taccore_builder_basic() {
        let b = TacCoreBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }
    #[test]
    fn test_taccore_builder_summary() {
        let b = TacCoreBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }
    #[test]
    fn test_taccore_state_machine_start() {
        let mut sm = TacCoreStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }
    #[test]
    fn test_taccore_state_machine_complete() {
        let mut sm = TacCoreStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }
    #[test]
    fn test_taccore_state_machine_fail() {
        let mut sm = TacCoreStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }
    #[test]
    fn test_taccore_state_machine_no_transition_after_terminal() {
        let mut sm = TacCoreStateMachine::new();
        sm.complete();
        assert!(!sm.start());
    }
    #[test]
    fn test_taccore_work_queue_basic() {
        let mut wq = TacCoreWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }
    #[test]
    fn test_taccore_work_queue_capacity() {
        let mut wq = TacCoreWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }
    #[test]
    fn test_taccore_counter_map_basic() {
        let mut cm = TacCoreCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }
    #[test]
    fn test_taccore_counter_map_frequency() {
        let mut cm = TacCoreCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_taccore_counter_map_most_common() {
        let mut cm = TacCoreCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}
#[cfg(test)]
mod tacticcore_analysis_tests {
    use super::*;
    use crate::tactic::core::*;
    #[test]
    fn test_tacticcore_result_ok() {
        let r = TacticCoreResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcore_result_err() {
        let r = TacticCoreResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcore_result_partial() {
        let r = TacticCoreResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_tacticcore_result_skipped() {
        let r = TacticCoreResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_tacticcore_analysis_pass_run() {
        let mut p = TacticCoreAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_tacticcore_analysis_pass_empty_input() {
        let mut p = TacticCoreAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_tacticcore_analysis_pass_success_rate() {
        let mut p = TacticCoreAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_tacticcore_analysis_pass_disable() {
        let mut p = TacticCoreAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_tacticcore_pipeline_basic() {
        let mut pipeline = TacticCorePipeline::new("main_pipeline");
        pipeline.add_pass(TacticCoreAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticCoreAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_tacticcore_pipeline_disabled_pass() {
        let mut pipeline = TacticCorePipeline::new("partial");
        let mut p = TacticCoreAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticCoreAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_tacticcore_diff_basic() {
        let mut d = TacticCoreDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_tacticcore_diff_summary() {
        let mut d = TacticCoreDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_tacticcore_config_set_get() {
        let mut cfg = TacticCoreConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_tacticcore_config_read_only() {
        let mut cfg = TacticCoreConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_tacticcore_config_remove() {
        let mut cfg = TacticCoreConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_tacticcore_diagnostics_basic() {
        let mut diag = TacticCoreDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_tacticcore_diagnostics_max_errors() {
        let mut diag = TacticCoreDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_tacticcore_diagnostics_clear() {
        let mut diag = TacticCoreDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_tacticcore_config_value_types() {
        let b = TacticCoreConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = TacticCoreConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = TacticCoreConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = TacticCoreConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = TacticCoreConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod core_ext_tests_2200 {
    use super::*;
    use crate::tactic::core::*;
    #[test]
    fn test_core_ext_result_ok_2200() {
        let r = CoreExtResult2200::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_core_ext_result_err_2200() {
        let r = CoreExtResult2200::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_core_ext_result_partial_2200() {
        let r = CoreExtResult2200::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_core_ext_result_skipped_2200() {
        let r = CoreExtResult2200::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_core_ext_pass_run_2200() {
        let mut p = CoreExtPass2200::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_core_ext_pass_empty_2200() {
        let mut p = CoreExtPass2200::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_core_ext_pass_rate_2200() {
        let mut p = CoreExtPass2200::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_core_ext_pass_disable_2200() {
        let mut p = CoreExtPass2200::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_core_ext_pipeline_basic_2200() {
        let mut pipeline = CoreExtPipeline2200::new("main_pipeline");
        pipeline.add_pass(CoreExtPass2200::new("pass1"));
        pipeline.add_pass(CoreExtPass2200::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_core_ext_pipeline_disabled_2200() {
        let mut pipeline = CoreExtPipeline2200::new("partial");
        let mut p = CoreExtPass2200::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(CoreExtPass2200::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_core_ext_diff_basic_2200() {
        let mut d = CoreExtDiff2200::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_core_ext_config_set_get_2200() {
        let mut cfg = CoreExtConfig2200::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_core_ext_config_read_only_2200() {
        let mut cfg = CoreExtConfig2200::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_core_ext_config_remove_2200() {
        let mut cfg = CoreExtConfig2200::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_core_ext_diagnostics_basic_2200() {
        let mut diag = CoreExtDiag2200::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_core_ext_diagnostics_max_errors_2200() {
        let mut diag = CoreExtDiag2200::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_core_ext_diagnostics_clear_2200() {
        let mut diag = CoreExtDiag2200::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_core_ext_config_value_types_2200() {
        let b = CoreExtConfigVal2200::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = CoreExtConfigVal2200::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = CoreExtConfigVal2200::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = CoreExtConfigVal2200::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = CoreExtConfigVal2200::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
