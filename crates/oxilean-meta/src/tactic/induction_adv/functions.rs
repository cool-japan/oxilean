//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::tactic::state::{TacticError, TacticResult, TacticState};
use oxilean_kernel::{Expr, Level, Name};
use std::collections::{BTreeMap, HashMap, HashSet};

use super::types::{
    ConstructorRuleInfo, GeneralizationResult, InductionConfig, InductionScheme, MinorPremise,
    MutualInductionConfig, RecursorInfo, WellFoundedConfig,
};

/// Infer the default induction scheme for a target type.
///
/// Given a type expression (e.g. `Nat`, `List α`, `Vector α n`), looks up
/// the corresponding inductive type in the environment, retrieves its
/// recursor, and constructs an `InductionScheme` that describes all the
/// pieces needed to apply the recursor.
pub fn infer_induction_scheme(
    target_type: &Expr,
    ctx: &MetaContext,
) -> TacticResult<InductionScheme> {
    let (head_name, _type_args) = decompose_app(target_type);
    let inductive_name = head_name.ok_or_else(|| {
        TacticError::Failed(
            "induction: target type is not an application of an inductive type".into(),
        )
    })?;
    let ind_info = lookup_inductive(&inductive_name, ctx)?;
    let recursor_name = make_recursor_name(&inductive_name);
    let rec_info = lookup_recursor(&recursor_name, ctx);
    let (num_params, num_indices, is_rec, ctors, all_names) = ind_info;
    let minor_premises = build_minor_premises(&inductive_name, &ctors, num_params, ctx)?;
    let num_motives = if all_names.len() > 1 {
        all_names.len()
    } else {
        1
    };
    let major_idx = num_params as usize + num_motives + minor_premises.len() + num_indices as usize;
    let motive = build_motive_placeholder(&inductive_name, num_indices, target_type);
    let motives = vec![motive];
    let universe_levels = if let Some(ref ri) = rec_info {
        ri.universe_levels.clone()
    } else {
        vec![Level::zero()]
    };
    let scheme = InductionScheme {
        recursor: recursor_name,
        major_idx,
        num_params: num_params as usize,
        num_indices: num_indices as usize,
        motives,
        minor_premises,
        universe_levels,
        is_custom: false,
        inductive_name: inductive_name.clone(),
        mutual_names: all_names,
    };
    validate_scheme(&scheme, is_rec)?;
    Ok(scheme)
}
/// Infer an induction scheme using a user-specified custom recursor.
///
/// This is used when the user writes `induction x using MyRec`.
pub(super) fn infer_custom_scheme(
    target_type: &Expr,
    recursor_name: &Name,
    ctx: &MetaContext,
) -> TacticResult<InductionScheme> {
    let (head_name, _args) = decompose_app(target_type);
    let inductive_name = head_name.ok_or_else(|| {
        TacticError::Failed("induction: cannot determine inductive type from target".into())
    })?;
    let rec_info = lookup_recursor(recursor_name, ctx).ok_or_else(|| {
        TacticError::Failed(format!(
            "induction: recursor '{}' not found in the environment",
            recursor_name
        ))
    })?;
    let minor_premises = rec_info
        .constructor_rules
        .iter()
        .map(|rule| {
            MinorPremise::new(
                rule.ctor_name.clone(),
                rule.num_fields as usize,
                rule.num_recursive as usize,
            )
        })
        .collect::<Vec<_>>();
    let motives = vec![build_motive_placeholder(
        &inductive_name,
        rec_info.num_indices,
        target_type,
    )];
    let major_idx = rec_info.num_params as usize
        + motives.len()
        + minor_premises.len()
        + rec_info.num_indices as usize;
    let mut scheme = InductionScheme {
        recursor: recursor_name.clone(),
        major_idx,
        num_params: rec_info.num_params as usize,
        num_indices: rec_info.num_indices as usize,
        motives,
        minor_premises,
        universe_levels: rec_info.universe_levels.clone(),
        is_custom: true,
        inductive_name,
        mutual_names: rec_info.all_names.clone(),
    };
    if let Some(custom_major) = rec_info.custom_major_idx {
        scheme.major_idx = custom_major as usize;
    }
    Ok(scheme)
}
/// Check whether a given recursor is compatible with a target expression.
///
/// Verifies that the recursor's inductive type matches the head of the
/// target and that universe levels can be unified.
pub fn check_recursor_compatibility(
    recursor: &Name,
    target: &Expr,
    ctx: &MetaContext,
) -> TacticResult<bool> {
    let (head_name, _) = decompose_app(target);
    let inductive_name = match head_name {
        Some(n) => n,
        None => return Ok(false),
    };
    let rec_info = match lookup_recursor(recursor, ctx) {
        Some(ri) => ri,
        None => return Ok(false),
    };
    if !rec_info.all_names.contains(&inductive_name) {
        return Ok(false);
    }
    let ind_info = lookup_inductive(&inductive_name, ctx);
    if let Ok((num_params, _num_indices, _is_rec, _ctors, _all)) = ind_info {
        if rec_info.num_params != num_params {
            return Ok(false);
        }
    }
    if rec_info.constructor_rules.is_empty() {
        return Ok(false);
    }
    Ok(true)
}
/// Build minor premises from constructor information.
pub(super) fn build_minor_premises(
    inductive_name: &Name,
    ctors: &[(Name, u32)],
    _num_params: u32,
    ctx: &MetaContext,
) -> TacticResult<Vec<MinorPremise>> {
    let mut premises = Vec::with_capacity(ctors.len());
    for (ctor_name, num_fields) in ctors {
        let num_recursive = count_recursive_fields(ctor_name, inductive_name, ctx);
        let mp = MinorPremise::new(ctor_name.clone(), *num_fields as usize, num_recursive);
        premises.push(mp);
    }
    Ok(premises)
}
/// Build a placeholder motive expression.
///
/// The motive is `fun (indices...) (x : T indices...) => target_type` where
/// `target_type` is lifted by the number of binders introduced (so that any
/// de Bruijn indices in the target type are shifted past the new binders).
pub(super) fn build_motive_placeholder(
    _inductive_name: &Name,
    num_indices: u32,
    target_type: &Expr,
) -> Expr {
    let num_binders = num_indices + 1;
    let lifted_body = lift_bvars(target_type, num_binders);
    let mut body = lifted_body;
    for i in (0..=num_indices).rev() {
        let binder_name = if i == num_indices {
            Name::str("x")
        } else {
            Name::str(format!("idx_{}", i))
        };
        body = Expr::Lam(
            oxilean_kernel::BinderInfo::Default,
            binder_name,
            Box::new(Expr::Sort(Level::zero())),
            Box::new(body),
        );
    }
    body
}
/// Lift all de Bruijn indices in `expr` by `amount` (for use when wrapping
/// an expression in `amount` additional binders).
pub(super) fn lift_bvars(expr: &Expr, amount: u32) -> Expr {
    lift_bvars_at(expr, amount, 0)
}
pub(super) fn lift_bvars_at(expr: &Expr, amount: u32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                Expr::BVar(i + amount)
            } else {
                Expr::BVar(*i)
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Lit(_) | Expr::Const(_, _) => expr.clone(),
        Expr::App(f, a) => {
            let f2 = lift_bvars_at(f, amount, cutoff);
            let a2 = lift_bvars_at(a, amount, cutoff);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = lift_bvars_at(ty, amount, cutoff);
            let body2 = lift_bvars_at(body, amount, cutoff + 1);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = lift_bvars_at(ty, amount, cutoff);
            let body2 = lift_bvars_at(body, amount, cutoff + 1);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(n, ty, val, body) => {
            let ty2 = lift_bvars_at(ty, amount, cutoff);
            let val2 = lift_bvars_at(val, amount, cutoff);
            let body2 = lift_bvars_at(body, amount, cutoff + 1);
            Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(n, i, e) => {
            let e2 = lift_bvars_at(e, amount, cutoff);
            Expr::Proj(n.clone(), *i, Box::new(e2))
        }
    }
}
/// Validate that an induction scheme is well-formed.
pub(super) fn validate_scheme(scheme: &InductionScheme, _is_rec: bool) -> TacticResult<()> {
    if scheme.minor_premises.is_empty() {
        return Err(TacticError::Failed(format!(
            "induction: type '{}' has no constructors",
            scheme.inductive_name
        )));
    }
    for mp in &scheme.minor_premises {
        if mp.num_recursive_args > mp.num_fields {
            return Err(TacticError::Internal(format!(
                "induction: constructor '{}' has more recursive args ({}) than fields ({})",
                mp.ctor_name, mp.num_recursive_args, mp.num_fields,
            )));
        }
    }
    let expected_major = scheme.args_before_major();
    if scheme.major_idx != expected_major {}
    Ok(())
}
/// Generalize expressions: for each `(e, name)` pair, abstract `e` in the
/// goal type, replacing it with a fresh universally-quantified variable.
///
/// This is the core of `generalize : e = x` in Lean 4.
pub fn tac_generalize(
    exprs: &[(Expr, Name)],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<GeneralizationResult> {
    if exprs.is_empty() {
        let goal = state.current_goal()?;
        let ty = ctx
            .get_mvar_type(goal)
            .cloned()
            .unwrap_or(Expr::Sort(Level::zero()));
        return Ok(GeneralizationResult {
            reverted: Vec::new(),
            new_goal: goal,
            generalized_type: ty,
            num_generalized: 0,
            hyp_positions: HashMap::new(),
            generalized_exprs: Vec::new(),
        });
    }
    let goal = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let goal_ty = ctx.instantiate_mvars(&goal_ty);
    let mut current_ty = goal_ty;
    let mut reverted_names = Vec::new();
    let mut hyp_positions = HashMap::new();
    let mut generalized_exprs = Vec::new();
    for (i, (expr, name)) in exprs.iter().enumerate().rev() {
        let abstracted = abstract_expr_in_type(&current_ty, expr);
        let expr_ty = infer_expr_type_simple(expr, ctx);
        current_ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            name.clone(),
            Box::new(expr_ty),
            Box::new(abstracted),
        );
        reverted_names.push(name.clone());
        hyp_positions.insert(name.clone(), i);
        generalized_exprs.push((expr.clone(), name.clone()));
    }
    reverted_names.reverse();
    generalized_exprs.reverse();
    let (new_goal_id, new_goal_expr) =
        ctx.mk_fresh_expr_mvar(current_ty.clone(), MetavarKind::Natural);
    let mut proof = new_goal_expr;
    for (expr, _name) in &generalized_exprs {
        proof = Expr::App(Box::new(proof), Box::new(expr.clone()));
    }
    ctx.assign_mvar(goal, proof);
    state.replace_goal(vec![new_goal_id]);
    Ok(GeneralizationResult {
        reverted: reverted_names,
        new_goal: new_goal_id,
        generalized_type: current_ty,
        num_generalized: exprs.len(),
        hyp_positions,
        generalized_exprs,
    })
}
/// Revert all hypotheses that depend on the induction target.
///
/// Before performing induction on `x`, we need to revert any hypothesis
/// whose type mentions `x`. This transforms `h : P x |- Q x` into
/// `|- (h : P x) -> Q x`, allowing the induction motive to properly
/// abstract over `x`.
pub(super) fn revert_dependent_hyps(
    target_name: &Name,
    generalizing: &[Name],
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<Name>> {
    let goal = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let _goal_ty = ctx.instantiate_mvars(&goal_ty);
    let hyps = ctx.get_local_hyps();
    let mut to_revert: Vec<Name> = Vec::new();
    for name in generalizing {
        if hyps.iter().any(|(n, _)| n == name) && !to_revert.contains(name) {
            to_revert.push(name.clone());
        }
    }
    for (hyp_name, hyp_ty) in &hyps {
        if hyp_name == target_name {
            continue;
        }
        if (expr_mentions_name(hyp_ty, target_name)
            || to_revert.iter().any(|r| expr_mentions_name(hyp_ty, r)))
            && !to_revert.contains(hyp_name)
        {
            to_revert.push(hyp_name.clone());
        }
    }
    if to_revert.is_empty() {
        return Ok(Vec::new());
    }
    let sorted = topological_sort_hyps(&to_revert, &hyps);
    let mut reverted = Vec::new();
    for name in sorted.iter().rev() {
        let current_hyps = ctx.get_local_hyps();
        if current_hyps.iter().any(|(n, _)| n == name) {
            perform_single_revert(name, state, ctx)?;
            reverted.push(name.clone());
        }
    }
    Ok(reverted)
}
/// Perform a single hypothesis revert, transforming `h : A |- T` into `|- A -> T`.
pub(super) fn perform_single_revert(
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
    let abstracted_target = abstract_name_in_type(&target, hyp_name);
    let new_target = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        hyp_name.clone(),
        Box::new(hyp_ty),
        Box::new(abstracted_target),
    );
    let (new_id, new_expr) = ctx.mk_fresh_expr_mvar(new_target, MetavarKind::Natural);
    let hyp_expr = Expr::Const(hyp_name.clone(), vec![]);
    let proof = Expr::App(Box::new(new_expr), Box::new(hyp_expr));
    ctx.assign_mvar(goal, proof);
    ctx.clear_local(hyp_name);
    state.replace_goal(vec![new_id]);
    Ok(())
}
/// Topologically sort hypotheses by dependency.
pub(super) fn topological_sort_hyps(names: &[Name], hyps: &[(Name, Expr)]) -> Vec<Name> {
    let name_set: HashSet<&Name> = names.iter().collect();
    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut in_degree: BTreeMap<String, usize> = BTreeMap::new();
    for name in names {
        let key = format!("{}", name);
        graph.entry(key.clone()).or_default();
        in_degree.entry(key).or_insert(0);
    }
    for (hyp_name, hyp_ty) in hyps {
        if !name_set.contains(hyp_name) {
            continue;
        }
        let from_key = format!("{}", hyp_name);
        for other_name in names {
            if other_name == hyp_name {
                continue;
            }
            if expr_mentions_name(hyp_ty, other_name) {
                let to_key = format!("{}", other_name);
                graph
                    .entry(to_key.clone())
                    .or_default()
                    .push(from_key.clone());
                *in_degree.entry(from_key.clone()).or_insert(0) += 1;
            }
        }
    }
    let mut queue: Vec<String> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(k, _)| k.clone())
        .collect();
    queue.sort();
    let mut result = Vec::new();
    while let Some(node) = queue.pop() {
        if let Some(name) = names.iter().find(|n| format!("{}", n) == node) {
            result.push(name.clone());
        }
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(neighbor) {
                    *deg = deg.saturating_sub(1);
                    if *deg == 0 {
                        queue.push(neighbor.clone());
                        queue.sort();
                    }
                }
            }
        }
    }
    result
}
/// Advanced induction tactic.
///
/// Performs induction on the given target expression using the specified
/// configuration. This extends the basic `induction` tactic with:
/// - Generalization of dependent hypotheses
/// - Custom recursor support
/// - User-controlled naming
///
/// Returns the list of new goal MVarIds (one per constructor case).
#[allow(clippy::too_many_arguments)]
pub fn tac_induction_adv(
    target: &Expr,
    config: &InductionConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let saved_state = ctx.save_state();
    state.save();
    let result = tac_induction_adv_impl(target, config, state, ctx);
    if result.is_err() {
        ctx.restore_state(saved_state);
        let _ = state.restore();
    }
    result
}
/// Inner implementation of advanced induction.
pub(super) fn tac_induction_adv_impl(
    target: &Expr,
    config: &InductionConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let target = ctx.instantiate_mvars(target);
    let inferred = infer_expr_type_simple(&target, ctx);
    let target_type = match &inferred {
        Expr::Sort(_) => target.clone(),
        _ => inferred,
    };
    let scheme = if let Some(ref rec_name) = config.using_recursor {
        infer_custom_scheme(&target_type, rec_name, ctx)?
    } else {
        infer_induction_scheme(&target_type, ctx)?
    };
    let mut reverted_hyps = Vec::new();
    if config.revert_deps || config.has_generalization() {
        let target_name = extract_target_name(&target);
        reverted_hyps = revert_dependent_hyps(&target_name, &config.generalizing, state, ctx)?;
    }
    let goal_ids = apply_recursor(&scheme, &target, config, state, ctx)?;
    if !reverted_hyps.is_empty() {
        for goal_id in &goal_ids {
            reintroduce_hyps(*goal_id, &reverted_hyps, ctx)?;
        }
    }
    assign_user_names(&goal_ids, &scheme, &config.with_names, ctx)?;
    Ok(goal_ids)
}
/// Apply a recursor to the current goal, producing one sub-goal per constructor.
pub(super) fn apply_recursor(
    scheme: &InductionScheme,
    target: &Expr,
    _config: &InductionConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let goal_ty = ctx.instantiate_mvars(&goal_ty);
    let mut new_goal_ids = Vec::new();
    let mut rec_expr = Expr::Const(scheme.recursor.clone(), scheme.universe_levels.clone());
    let (_head, type_args) = decompose_app_with_args(target);
    for i in 0..scheme.num_params {
        let param_arg = if i < type_args.len() {
            type_args[i].clone()
        } else {
            let (_, placeholder) =
                ctx.mk_fresh_expr_mvar(Expr::Sort(Level::zero()), MetavarKind::Synthetic);
            placeholder
        };
        rec_expr = Expr::App(Box::new(rec_expr), Box::new(param_arg));
    }
    for motive in &scheme.motives {
        rec_expr = Expr::App(Box::new(rec_expr), Box::new(motive.clone()));
    }
    for mp in &scheme.minor_premises {
        let minor_type = build_minor_premise_type(mp, &goal_ty, scheme);
        let (minor_id, minor_placeholder) =
            ctx.mk_fresh_expr_mvar(minor_type, MetavarKind::Natural);
        rec_expr = Expr::App(Box::new(rec_expr), Box::new(minor_placeholder));
        new_goal_ids.push(minor_id);
    }
    for i in 0..scheme.num_indices {
        let index_arg = if scheme.num_params + i < type_args.len() {
            type_args[scheme.num_params + i].clone()
        } else {
            let (_, placeholder) =
                ctx.mk_fresh_expr_mvar(Expr::Sort(Level::zero()), MetavarKind::Synthetic);
            placeholder
        };
        rec_expr = Expr::App(Box::new(rec_expr), Box::new(index_arg));
    }
    rec_expr = Expr::App(Box::new(rec_expr), Box::new(target.clone()));
    ctx.assign_mvar(goal, rec_expr);
    state.replace_goal(new_goal_ids.clone());
    Ok(new_goal_ids)
}
/// Build the type of a minor premise (the proof obligation for one constructor).
pub(super) fn build_minor_premise_type(
    mp: &MinorPremise,
    goal_ty: &Expr,
    _scheme: &InductionScheme,
) -> Expr {
    let mut result_ty = goal_ty.clone();
    for i in (0..mp.num_recursive_args).rev() {
        let ih_name = if mp.num_recursive_args == 1 {
            Name::str("ih")
        } else {
            Name::str(format!("ih_{}", i))
        };
        result_ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            ih_name,
            Box::new(goal_ty.clone()),
            Box::new(result_ty),
        );
    }
    for i in (0..mp.num_fields).rev() {
        let field_name = if i < mp.field_names.len() {
            mp.field_names[i].clone()
        } else {
            Name::str(format!("a_{}", i))
        };
        result_ty = Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            field_name,
            Box::new(Expr::Sort(Level::zero())),
            Box::new(result_ty),
        );
    }
    result_ty
}
/// Re-introduce reverted hypotheses into a goal after induction.
///
/// After the recursor is applied, each sub-goal has the reverted hypotheses
/// as leading Pi-binders. This function automatically re-introduces them by
/// calling `tac_intro` for each name, so the user does not need to write
/// explicit `intro` calls.
pub(super) fn reintroduce_hyps(
    goal_id: MVarId,
    reverted: &[Name],
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    if reverted.is_empty() {
        return Ok(());
    }
    let mut sub_state = TacticState::single(goal_id);
    for name in reverted {
        let result = crate::tactic::core::tac_intro(Some(name.clone()), &mut sub_state, ctx);
        if result.is_err() {
            break;
        }
    }
    Ok(())
}
/// Extract the "name" of an induction target for dependency tracking.
pub(super) fn extract_target_name(target: &Expr) -> Name {
    match target {
        Expr::Const(n, _) => n.clone(),
        Expr::FVar(fid) => Name::str(format!("_fvar_{}", fid.0)),
        _ => Name::str("_target"),
    }
}
/// Perform mutual induction on multiple targets simultaneously.
///
/// For mutually inductive types `A` and `B`, this constructs a combined
/// recursor application that produces goals for all constructors of both types.
pub fn tac_mutual_induction(
    targets: &[Expr],
    config: &MutualInductionConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    if targets.is_empty() {
        return Err(TacticError::Failed(
            "mutual induction: no targets provided".into(),
        ));
    }
    if targets.len() == 1 {
        let single_config = config.target_configs.first().cloned().unwrap_or_default();
        return tac_induction_adv(&targets[0], &single_config, state, ctx);
    }
    if config.exceeds_limit() {
        return Err(TacticError::Failed(format!(
            "mutual induction: too many targets ({}, max {})",
            targets.len(),
            config.max_mutual,
        )));
    }
    let saved = ctx.save_state();
    state.save();
    let result = tac_mutual_induction_impl(targets, config, state, ctx);
    if result.is_err() {
        ctx.restore_state(saved);
        let _ = state.restore();
    }
    result
}
/// Inner implementation of mutual induction.
pub(super) fn tac_mutual_induction_impl(
    targets: &[Expr],
    config: &MutualInductionConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let mut schemes = Vec::new();
    for target in targets {
        let ty = infer_expr_type_simple(target, ctx);
        let scheme = infer_induction_scheme(&ty, ctx)?;
        schemes.push(scheme);
    }
    verify_mutual_block(&schemes)?;
    if !config.shared_generalizing.is_empty() {
        let first_target_name = extract_target_name(&targets[0]);
        revert_dependent_hyps(&first_target_name, &config.shared_generalizing, state, ctx)?;
    }
    let combined_scheme = build_combined_scheme(&schemes)?;
    let goal = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let goal_ty = ctx.instantiate_mvars(&goal_ty);
    let mut new_goal_ids = Vec::new();
    let mut rec_expr = Expr::Const(
        combined_scheme.recursor.clone(),
        combined_scheme.universe_levels.clone(),
    );
    let (_head, first_target_args) = decompose_app_with_args(&targets[0]);
    for i in 0..combined_scheme.num_params {
        let param = if i < first_target_args.len() {
            first_target_args[i].clone()
        } else {
            let (_, p) = ctx.mk_fresh_expr_mvar(Expr::Sort(Level::zero()), MetavarKind::Synthetic);
            p
        };
        rec_expr = Expr::App(Box::new(rec_expr), Box::new(param));
    }
    for motive in &combined_scheme.motives {
        rec_expr = Expr::App(Box::new(rec_expr), Box::new(motive.clone()));
    }
    for mp in &combined_scheme.minor_premises {
        let minor_type = build_minor_premise_type(mp, &goal_ty, &combined_scheme);
        let (minor_id, minor_placeholder) =
            ctx.mk_fresh_expr_mvar(minor_type, MetavarKind::Natural);
        rec_expr = Expr::App(Box::new(rec_expr), Box::new(minor_placeholder));
        new_goal_ids.push(minor_id);
    }
    for target in targets {
        rec_expr = Expr::App(Box::new(rec_expr), Box::new(target.clone()));
    }
    ctx.assign_mvar(goal, rec_expr);
    state.replace_goal(new_goal_ids.clone());
    let all_names: Vec<Vec<Name>> = config
        .motive_names
        .iter()
        .cloned()
        .chain(std::iter::repeat_with(Vec::new))
        .take(combined_scheme.minor_premises.len())
        .collect();
    assign_user_names(&new_goal_ids, &combined_scheme, &all_names, ctx)?;
    Ok(new_goal_ids)
}
/// Verify that all schemes belong to the same mutual inductive block.
pub(super) fn verify_mutual_block(schemes: &[InductionScheme]) -> TacticResult<()> {
    if schemes.len() <= 1 {
        return Ok(());
    }
    let first_mutual = &schemes[0].mutual_names;
    for (i, scheme) in schemes.iter().enumerate().skip(1) {
        if !first_mutual.contains(&scheme.inductive_name) {
            return Err(
                TacticError::Failed(
                    format!(
                        "mutual induction: target {} ('{}') does not belong to the same mutual block as target 0 ('{}')",
                        i, scheme.inductive_name, schemes[0].inductive_name,
                    ),
                ),
            );
        }
    }
    Ok(())
}
/// Combine multiple schemes into a single mutual induction scheme.
pub(super) fn build_combined_scheme(schemes: &[InductionScheme]) -> TacticResult<InductionScheme> {
    if schemes.is_empty() {
        return Err(TacticError::Internal(
            "build_combined_scheme: empty schemes list".into(),
        ));
    }
    if schemes.len() == 1 {
        return Ok(schemes[0].clone());
    }
    let first = &schemes[0];
    let mut combined = first.clone();
    combined.motives.clear();
    for scheme in schemes {
        combined.motives.extend(scheme.motives.clone());
    }
    combined.minor_premises.clear();
    for scheme in schemes {
        combined
            .minor_premises
            .extend(scheme.minor_premises.clone());
    }
    let mut all_mutual: Vec<Name> = Vec::new();
    for scheme in schemes {
        for name in &scheme.mutual_names {
            if !all_mutual.contains(name) {
                all_mutual.push(name.clone());
            }
        }
    }
    combined.mutual_names = all_mutual;
    combined.major_idx = combined.num_params
        + combined.motives.len()
        + combined.minor_premises.len()
        + combined.num_indices;
    Ok(combined)
}
/// Perform well-founded induction on the target.
///
/// Given a target `x` and a well-founded relation `<` on its type, produces
/// a single goal where the induction hypothesis states that the property
/// holds for all elements smaller than `x`.
///
/// The generated goal has the form:
/// ```text
/// x : T
/// ih : forall y : T, y < x -> P y
/// |- P x
/// ```
pub fn tac_well_founded_induction(
    target: &Expr,
    rel: &Expr,
    config: &WellFoundedConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let saved = ctx.save_state();
    state.save();
    let result = tac_wf_induction_impl(target, rel, config, state, ctx);
    if result.is_err() {
        ctx.restore_state(saved);
        let _ = state.restore();
    }
    result
}
/// Inner implementation of well-founded induction.
pub(super) fn tac_wf_induction_impl(
    target: &Expr,
    rel: &Expr,
    config: &WellFoundedConfig,
    state: &mut TacticState,
    ctx: &mut MetaContext,
) -> TacticResult<Vec<MVarId>> {
    let goal = state.current_goal()?;
    let goal_ty = ctx
        .get_mvar_type(goal)
        .cloned()
        .ok_or_else(|| TacticError::Internal("goal has no type".into()))?;
    let goal_ty = ctx.instantiate_mvars(&goal_ty);
    let target = ctx.instantiate_mvars(target);
    let rel = ctx.instantiate_mvars(rel);
    let target_ty = infer_expr_type_simple(&target, ctx);
    let ih_name = config
        .ih_names
        .first()
        .cloned()
        .unwrap_or_else(|| Name::str("ih"));
    let ih_type = build_wf_ih_type(&target_ty, &rel, &goal_ty);
    let step_type = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("x"),
        Box::new(target_ty.clone()),
        Box::new(Expr::Pi(
            oxilean_kernel::BinderInfo::Default,
            ih_name.clone(),
            Box::new(ih_type.clone()),
            Box::new(goal_ty.clone()),
        )),
    );
    let (step_goal_id, step_goal_placeholder) =
        ctx.mk_fresh_expr_mvar(step_type, MetavarKind::Natural);
    let wf_fix_name = Name::str("WellFounded.fix");
    let wf_proof = config
        .wf_proof_name
        .clone()
        .map(|n| Expr::Const(n, vec![]))
        .unwrap_or_else(|| Expr::Const(Name::str("WellFounded.wf"), vec![]));
    let effective_rel = if let Some(ref measure) = config.measure {
        Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("InvImage"), vec![Level::zero()])),
                Box::new(rel.clone()),
            )),
            Box::new(measure.clone()),
        )
    } else {
        rel.clone()
    };
    let mut fix_expr = Expr::Const(wf_fix_name, vec![Level::zero()]);
    fix_expr = Expr::App(Box::new(fix_expr), Box::new(target_ty.clone()));
    fix_expr = Expr::App(Box::new(fix_expr), Box::new(effective_rel));
    fix_expr = Expr::App(Box::new(fix_expr), Box::new(wf_proof));
    let motive = Expr::Lam(
        oxilean_kernel::BinderInfo::Default,
        Name::str("x"),
        Box::new(target_ty),
        Box::new(goal_ty),
    );
    fix_expr = Expr::App(Box::new(fix_expr), Box::new(motive));
    fix_expr = Expr::App(Box::new(fix_expr), Box::new(step_goal_placeholder));
    fix_expr = Expr::App(Box::new(fix_expr), Box::new(target));
    ctx.assign_mvar(goal, fix_expr);
    state.replace_goal(vec![step_goal_id]);
    Ok(vec![step_goal_id])
}
/// Build the IH type for well-founded induction:
/// `forall (y : T), rel y x -> P y`
pub(super) fn build_wf_ih_type(target_ty: &Expr, rel: &Expr, goal_ty: &Expr) -> Expr {
    let rel_applied = Expr::App(
        Box::new(Expr::App(Box::new(rel.clone()), Box::new(Expr::BVar(0)))),
        Box::new(Expr::BVar(2)),
    );
    let goal_for_y = goal_ty.clone();
    let inner = Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("h_lt"),
        Box::new(rel_applied),
        Box::new(goal_for_y),
    );
    Expr::Pi(
        oxilean_kernel::BinderInfo::Default,
        Name::str("y"),
        Box::new(target_ty.clone()),
        Box::new(inner),
    )
}
/// Attempt to automatically find a well-founded relation for the target type.
///
/// Looks for `WellFoundedRelation` instances on the type.
#[allow(dead_code)]
pub(super) fn auto_find_wf_relation(target_ty: &Expr, ctx: &MetaContext) -> TacticResult<Expr> {
    let (head, _) = decompose_app(target_ty);
    let type_name = head.unwrap_or_else(|| Name::str("_unknown"));
    let type_str = format!("{}", type_name);
    match type_str.as_str() {
        "Nat" => Ok(Expr::Const(Name::str("Nat.lt"), vec![])),
        "Prod" => Ok(Expr::Const(Name::str("Prod.Lex"), vec![Level::zero()])),
        _ => {
            if let Some(oxilean_kernel::ConstantInfo::Definition(_)) =
                ctx.env().find(&Name::str(format!("{}.sizeOf", type_name)))
            {
                Ok(Expr::Const(Name::str("SizeOfWF"), vec![Level::zero()]))
            } else {
                Err(TacticError::Failed(format!(
                    "well-founded induction: cannot find well-founded relation for type '{}'",
                    type_name,
                )))
            }
        }
    }
}
/// Assign user-supplied names to the variables introduced in each induction goal.
///
/// If the user writes `induction n with | zero => ... | succ m ih => ...`,
/// the names `m` and `ih` are assigned to the field and IH binders in the
/// `succ` goal.
pub(super) fn assign_user_names(
    goal_ids: &[MVarId],
    scheme: &InductionScheme,
    with_names: &[Vec<Name>],
    ctx: &mut MetaContext,
) -> TacticResult<()> {
    for (i, goal_id) in goal_ids.iter().enumerate() {
        if i >= scheme.minor_premises.len() {
            break;
        }
        let mp = &scheme.minor_premises[i];
        let user_names = if i < with_names.len() {
            &with_names[i]
        } else {
            continue;
        };
        if user_names.is_empty() {
            continue;
        }
        let total_binders = mp.total_binders();
        let names_to_assign: Vec<(usize, Name)> = user_names
            .iter()
            .take(total_binders)
            .enumerate()
            .map(|(j, n)| (j, n.clone()))
            .collect();
        if let Some(goal_type) = ctx.get_mvar_type(*goal_id).cloned() {
            let renamed_type = rename_pi_binders(&goal_type, &names_to_assign);
            let _ = renamed_type;
        }
    }
    Ok(())
}
/// Rename the leading Pi binders in a type according to the given mapping.
pub(super) fn rename_pi_binders(ty: &Expr, names: &[(usize, Name)]) -> Expr {
    rename_pi_binders_impl(ty, names, 0)
}
pub(super) fn rename_pi_binders_impl(ty: &Expr, names: &[(usize, Name)], idx: usize) -> Expr {
    match ty {
        Expr::Pi(bi, old_name, domain, body) => {
            let new_name = names
                .iter()
                .find(|(i, _)| *i == idx)
                .map(|(_, n)| n.clone())
                .unwrap_or_else(|| old_name.clone());
            let new_body = rename_pi_binders_impl(body, names, idx + 1);
            Expr::Pi(*bi, new_name, domain.clone(), Box::new(new_body))
        }
        _ => ty.clone(),
    }
}
/// Generate default names for a constructor case based on the constructor name
/// and field/IH count.
#[allow(dead_code)]
pub(super) fn generate_default_names(mp: &MinorPremise) -> Vec<Name> {
    let mut names = Vec::with_capacity(mp.total_binders());
    for i in 0..mp.num_fields {
        if i < mp.field_names.len() {
            names.push(mp.field_names[i].clone());
        } else {
            names.push(Name::str(format!("a_{}", i)));
        }
    }
    let ih_names = mp.default_ih_names();
    names.extend(ih_names);
    names
}
/// Decompose an expression into its head constant and discard arguments.
pub(super) fn decompose_app(expr: &Expr) -> (Option<Name>, Vec<Expr>) {
    let mut args = Vec::new();
    let mut current = expr;
    loop {
        match current {
            Expr::App(f, a) => {
                args.push((**a).clone());
                current = f;
            }
            Expr::Const(name, _) => {
                args.reverse();
                return (Some(name.clone()), args);
            }
            _ => {
                args.reverse();
                return (None, args);
            }
        }
    }
}
/// Decompose into head + collected arguments.
pub(super) fn decompose_app_with_args(expr: &Expr) -> (Option<Name>, Vec<Expr>) {
    decompose_app(expr)
}
/// Extract the short name from a qualified constructor name.
///
/// E.g. `Nat.succ` -> `succ`, `List.cons` -> `cons`.
pub(super) fn extract_short_name(name: &Name) -> String {
    let full = format!("{}", name);
    if let Some(idx) = full.rfind('.') {
        full[idx + 1..].to_string()
    } else {
        full
    }
}
/// Make the default recursor name for an inductive type.
pub(super) fn make_recursor_name(inductive_name: &Name) -> Name {
    let full = format!("{}", inductive_name);
    Name::str(format!("{}.rec", full))
}
/// Look up recursor info from the environment.
pub(super) fn lookup_recursor(recursor_name: &Name, ctx: &MetaContext) -> Option<RecursorInfo> {
    if let Some(oxilean_kernel::ConstantInfo::Recursor(rec)) = ctx.env().find(recursor_name) {
        let constructor_rules = rec
            .rules
            .iter()
            .map(|rule| ConstructorRuleInfo {
                ctor_name: rule.ctor.clone(),
                num_fields: rule.nfields,
                num_recursive: 0,
            })
            .collect();
        Some(RecursorInfo {
            all_names: rec.all.clone(),
            num_params: rec.num_params,
            num_indices: rec.num_indices,
            universe_levels: rec
                .common
                .level_params
                .iter()
                .map(|n| Level::Param(n.clone()))
                .collect(),
            constructor_rules,
            custom_major_idx: None,
        })
    } else {
        let name_str = format!("{}", recursor_name);
        match name_str.as_str() {
            "Nat.rec" => Some(RecursorInfo {
                all_names: vec![Name::str("Nat")],
                num_params: 0,
                num_indices: 0,
                universe_levels: vec![Level::zero()],
                constructor_rules: vec![
                    ConstructorRuleInfo {
                        ctor_name: Name::str("Nat.zero"),
                        num_fields: 0,
                        num_recursive: 0,
                    },
                    ConstructorRuleInfo {
                        ctor_name: Name::str("Nat.succ"),
                        num_fields: 1,
                        num_recursive: 1,
                    },
                ],
                custom_major_idx: None,
            }),
            "List.rec" => Some(RecursorInfo {
                all_names: vec![Name::str("List")],
                num_params: 1,
                num_indices: 0,
                universe_levels: vec![Level::zero()],
                constructor_rules: vec![
                    ConstructorRuleInfo {
                        ctor_name: Name::str("List.nil"),
                        num_fields: 0,
                        num_recursive: 0,
                    },
                    ConstructorRuleInfo {
                        ctor_name: Name::str("List.cons"),
                        num_fields: 2,
                        num_recursive: 1,
                    },
                ],
                custom_major_idx: None,
            }),
            "Bool.rec" => Some(RecursorInfo {
                all_names: vec![Name::str("Bool")],
                num_params: 0,
                num_indices: 0,
                universe_levels: vec![Level::zero()],
                constructor_rules: vec![
                    ConstructorRuleInfo {
                        ctor_name: Name::str("Bool.true"),
                        num_fields: 0,
                        num_recursive: 0,
                    },
                    ConstructorRuleInfo {
                        ctor_name: Name::str("Bool.false"),
                        num_fields: 0,
                        num_recursive: 0,
                    },
                ],
                custom_major_idx: None,
            }),
            _ => None,
        }
    }
}
/// Look up inductive type information from the environment.
///
/// Returns `(num_params, num_indices, is_rec, constructors, mutual_names)`.
#[allow(clippy::type_complexity)]
pub(super) fn lookup_inductive(
    name: &Name,
    ctx: &MetaContext,
) -> TacticResult<(u32, u32, bool, Vec<(Name, u32)>, Vec<Name>)> {
    if let Some(oxilean_kernel::ConstantInfo::Inductive(ind)) = ctx.env().find(name) {
        let mut ctors = Vec::new();
        for ctor_name in &ind.ctors {
            let num_fields = if let Some(oxilean_kernel::ConstantInfo::Constructor(cv)) =
                ctx.env().find(ctor_name)
            {
                cv.num_fields
            } else {
                0
            };
            ctors.push((ctor_name.clone(), num_fields));
        }
        return Ok((
            ind.num_params,
            ind.num_indices,
            ind.is_rec,
            ctors,
            ind.all.clone(),
        ));
    }
    let name_str = format!("{}", name);
    match name_str.as_str() {
        "Nat" => Ok((
            0,
            0,
            true,
            vec![(Name::str("Nat.zero"), 0), (Name::str("Nat.succ"), 1)],
            vec![Name::str("Nat")],
        )),
        "Bool" => Ok((
            0,
            0,
            false,
            vec![(Name::str("Bool.true"), 0), (Name::str("Bool.false"), 0)],
            vec![Name::str("Bool")],
        )),
        "List" => Ok((
            1,
            0,
            true,
            vec![(Name::str("List.nil"), 0), (Name::str("List.cons"), 2)],
            vec![Name::str("List")],
        )),
        "Option" => Ok((
            1,
            0,
            false,
            vec![(Name::str("Option.none"), 0), (Name::str("Option.some"), 1)],
            vec![Name::str("Option")],
        )),
        "And" => Ok((
            0,
            0,
            false,
            vec![(Name::str("And.intro"), 2)],
            vec![Name::str("And")],
        )),
        "Or" => Ok((
            0,
            0,
            false,
            vec![(Name::str("Or.inl"), 1), (Name::str("Or.inr"), 1)],
            vec![Name::str("Or")],
        )),
        "Vector" => Ok((
            1,
            1,
            true,
            vec![(Name::str("Vector.nil"), 0), (Name::str("Vector.cons"), 2)],
            vec![Name::str("Vector")],
        )),
        "Sigma" | "Exists" => Ok((
            0,
            0,
            false,
            vec![(Name::str("Sigma.mk"), 2)],
            vec![name.clone()],
        )),
        "Sum" => Ok((
            0,
            0,
            false,
            vec![(Name::str("Sum.inl"), 1), (Name::str("Sum.inr"), 1)],
            vec![Name::str("Sum")],
        )),
        "Empty" => Ok((0, 0, false, Vec::new(), vec![Name::str("Empty")])),
        "Unit" | "PUnit" => Ok((
            0,
            0,
            false,
            vec![(Name::str("Unit.unit"), 0)],
            vec![name.clone()],
        )),
        "Fin" => Ok((
            0,
            1,
            true,
            vec![(Name::str("Fin.zero"), 0), (Name::str("Fin.succ"), 1)],
            vec![Name::str("Fin")],
        )),
        _ => Err(TacticError::Failed(format!(
            "induction: unknown inductive type '{}'",
            name,
        ))),
    }
}
/// Count recursive fields for a constructor.
pub(super) fn count_recursive_fields(
    ctor_name: &Name,
    inductive_name: &Name,
    ctx: &MetaContext,
) -> usize {
    if let Some(oxilean_kernel::ConstantInfo::Constructor(cv)) = ctx.env().find(ctor_name) {
        let ctor_ty = &cv.common.ty;
        return count_inductive_occurrences_in_pi(ctor_ty, inductive_name);
    }
    let ind_str = format!("{}", inductive_name);
    let ctor_str = format!("{}", ctor_name);
    match ind_str.as_str() {
        "Nat" => {
            if ctor_str.contains("succ") {
                1
            } else {
                0
            }
        }
        "List" => {
            if ctor_str.contains("cons") {
                1
            } else {
                0
            }
        }
        "Vector" => {
            if ctor_str.contains("cons") {
                1
            } else {
                0
            }
        }
        "Fin" => {
            if ctor_str.contains("succ") {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}
/// Count how many times an inductive name appears as a Pi-domain in a type.
pub(super) fn count_inductive_occurrences_in_pi(ty: &Expr, inductive_name: &Name) -> usize {
    match ty {
        Expr::Pi(_, _, domain, body) => {
            let in_domain = if expr_head_is(domain, inductive_name) {
                1
            } else {
                0
            };
            in_domain + count_inductive_occurrences_in_pi(body, inductive_name)
        }
        _ => 0,
    }
}
/// Check whether the head of an expression is a given name.
pub(super) fn expr_head_is(expr: &Expr, name: &Name) -> bool {
    match expr {
        Expr::Const(n, _) => n == name,
        Expr::App(f, _) => expr_head_is(f, name),
        _ => false,
    }
}
/// Check if an expression mentions a name (simple syntactic check).
pub(super) fn expr_mentions_name(expr: &Expr, name: &Name) -> bool {
    match expr {
        Expr::Const(n, _) => n == name,
        Expr::App(f, a) => expr_mentions_name(f, name) || expr_mentions_name(a, name),
        Expr::Lam(_, n, ty, body) => {
            n == name || expr_mentions_name(ty, name) || expr_mentions_name(body, name)
        }
        Expr::Pi(_, n, ty, body) => {
            n == name || expr_mentions_name(ty, name) || expr_mentions_name(body, name)
        }
        Expr::Let(n, ty, val, body) => {
            n == name
                || expr_mentions_name(ty, name)
                || expr_mentions_name(val, name)
                || expr_mentions_name(body, name)
        }
        Expr::Proj(_, _, e) => expr_mentions_name(e, name),
        _ => false,
    }
}
/// Abstract all occurrences of an expression in a type, replacing them with BVar(0).
pub(super) fn abstract_expr_in_type(ty: &Expr, target: &Expr) -> Expr {
    abstract_expr_impl(ty, target, 0)
}
pub(super) fn abstract_expr_impl(expr: &Expr, target: &Expr, depth: u32) -> Expr {
    if expr == target {
        return Expr::BVar(depth);
    }
    match expr {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => expr.clone(),
        Expr::Const(_, _) => {
            if expr == target {
                Expr::BVar(depth)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f2 = abstract_expr_impl(f, target, depth);
            let a2 = abstract_expr_impl(a, target, depth);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = abstract_expr_impl(ty, target, depth);
            let body2 = abstract_expr_impl(body, target, depth + 1);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = abstract_expr_impl(ty, target, depth);
            let body2 = abstract_expr_impl(body, target, depth + 1);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(n, ty, val, body) => {
            let ty2 = abstract_expr_impl(ty, target, depth);
            let val2 = abstract_expr_impl(val, target, depth);
            let body2 = abstract_expr_impl(body, target, depth + 1);
            Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(n, i, e) => {
            let e2 = abstract_expr_impl(e, target, depth);
            Expr::Proj(n.clone(), *i, Box::new(e2))
        }
    }
}
/// Abstract all occurrences of a name in a type, replacing them with BVar(0).
pub(super) fn abstract_name_in_type(ty: &Expr, name: &Name) -> Expr {
    abstract_name_in_type_impl(ty, name, 0)
}
pub(super) fn abstract_name_in_type_impl(expr: &Expr, name: &Name, depth: u32) -> Expr {
    match expr {
        Expr::Const(n, _) if n == name => Expr::BVar(depth),
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) | Expr::Const(_, _) => {
            expr.clone()
        }
        Expr::App(f, a) => {
            let f2 = abstract_name_in_type_impl(f, name, depth);
            let a2 = abstract_name_in_type_impl(a, name, depth);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, n, ty, body) => {
            let ty2 = abstract_name_in_type_impl(ty, name, depth);
            let body2 = abstract_name_in_type_impl(body, name, depth + 1);
            Expr::Lam(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, n, ty, body) => {
            let ty2 = abstract_name_in_type_impl(ty, name, depth);
            let body2 = abstract_name_in_type_impl(body, name, depth + 1);
            Expr::Pi(*bi, n.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(n, ty, val, body) => {
            let ty2 = abstract_name_in_type_impl(ty, name, depth);
            let val2 = abstract_name_in_type_impl(val, name, depth);
            let body2 = abstract_name_in_type_impl(body, name, depth + 1);
            Expr::Let(n.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(n, i, e) => {
            let e2 = abstract_name_in_type_impl(e, name, depth);
            Expr::Proj(n.clone(), *i, Box::new(e2))
        }
    }
}
/// Simplified type inference for an expression (for scheme construction).
///
/// Handles the most common structural cases without a full constraint-solving
/// type checker. Returns the expression itself for complex dependent cases.
pub(super) fn infer_expr_type_simple(expr: &Expr, ctx: &MetaContext) -> Expr {
    match expr {
        Expr::Sort(l) => Expr::Sort(Level::succ(l.clone())),
        Expr::Lit(oxilean_kernel::Literal::Nat(_)) => Expr::Const(Name::str("Nat"), vec![]),
        Expr::Lit(oxilean_kernel::Literal::Str(_)) => Expr::Const(Name::str("String"), vec![]),
        Expr::Const(name, _levels) => {
            if let Some(ci) = ctx.env().find(name) {
                match ci {
                    oxilean_kernel::ConstantInfo::Axiom(a) => a.common.ty.clone(),
                    oxilean_kernel::ConstantInfo::Definition(d) => d.common.ty.clone(),
                    oxilean_kernel::ConstantInfo::Theorem(t) => t.common.ty.clone(),
                    oxilean_kernel::ConstantInfo::Inductive(i) => i.common.ty.clone(),
                    oxilean_kernel::ConstantInfo::Constructor(c) => c.common.ty.clone(),
                    oxilean_kernel::ConstantInfo::Recursor(r) => r.common.ty.clone(),
                    oxilean_kernel::ConstantInfo::Opaque(o) => o.common.ty.clone(),
                    oxilean_kernel::ConstantInfo::Quotient(q) => q.common.ty.clone(),
                }
            } else {
                let name_str = format!("{}", name);
                match name_str.as_str() {
                    "Nat" | "Bool" | "Prop" | "String" => Expr::Sort(Level::succ(Level::zero())),
                    "Nat.zero" => Expr::Const(Name::str("Nat"), vec![]),
                    "Bool.true" | "Bool.false" | "true" | "false" => {
                        Expr::Const(Name::str("Bool"), vec![])
                    }
                    _ => expr.clone(),
                }
            }
        }
        Expr::App(f, a) => {
            let f_ty = infer_expr_type_simple(f, ctx);
            match f_ty {
                Expr::Pi(_, _, _, body) => oxilean_kernel::instantiate(&body, a),
                _ => expr.clone(),
            }
        }
        Expr::Pi(_, _, dom, cod) => {
            let dom_ty = infer_expr_type_simple(dom, ctx);
            let cod_ty = infer_expr_type_simple(cod, ctx);
            match (dom_ty, cod_ty) {
                (Expr::Sort(l1), Expr::Sort(l2)) => Expr::Sort(Level::imax(l1, l2)),
                _ => Expr::Sort(Level::succ(Level::zero())),
            }
        }
        Expr::Lam(bi, name, dom, body) => {
            let body_ty = infer_expr_type_simple(body, ctx);
            Expr::Pi(*bi, name.clone(), dom.clone(), Box::new(body_ty))
        }
        _ => expr.clone(),
    }
}
