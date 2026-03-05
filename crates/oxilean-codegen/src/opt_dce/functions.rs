//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{
    ConstValue, DCEAnalysisCache, DCEConstantFoldingHelper, DCEDepGraph, DCEDominatorTree,
    DCELivenessInfo, DCEPassConfig, DCEPassPhase, DCEPassRegistry, DCEPassStats, DCEWorklist,
    DceConfig, DceStats, UsageInfo,
};

/// Collect usage information for every variable referenced in `expr`.
///
/// This is an occurrence-analysis pass similar to GHC's.  It walks the
/// expression tree once, counting references for each `LcnfVarId` and
/// noting whether any use escapes or is inside a loop.
pub fn collect_usage_info(expr: &LcnfExpr) -> HashMap<LcnfVarId, UsageInfo> {
    let mut info: HashMap<LcnfVarId, UsageInfo> = HashMap::new();
    collect_usage_expr(expr, &mut info, false);
    info
}
/// Internal recursive walker for usage analysis.
pub(super) fn collect_usage_expr(
    expr: &LcnfExpr,
    info: &mut HashMap<LcnfVarId, UsageInfo>,
    in_loop: bool,
) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_usage_value(value, info, in_loop);
            collect_usage_expr(body, info, in_loop);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            record_use(info, *scrutinee, in_loop, false);
            for alt in alts {
                collect_usage_expr(&alt.body, info, in_loop);
            }
            if let Some(def) = default {
                collect_usage_expr(def, info, in_loop);
            }
        }
        LcnfExpr::Return(arg) => {
            record_arg_use(info, arg, in_loop, false);
        }
        LcnfExpr::TailCall(func, args) => {
            record_arg_use(info, func, in_loop, false);
            for a in args {
                record_arg_use(info, a, in_loop, false);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
/// Record uses from a let-bound value.
pub(super) fn collect_usage_value(
    value: &LcnfLetValue,
    info: &mut HashMap<LcnfVarId, UsageInfo>,
    in_loop: bool,
) {
    match value {
        LcnfLetValue::App(func, args) => {
            record_arg_use(info, func, in_loop, false);
            for a in args {
                record_arg_use(info, a, in_loop, false);
            }
        }
        LcnfLetValue::Proj(_, _, v) => {
            record_use(info, *v, in_loop, false);
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                record_arg_use(info, a, in_loop, true);
            }
        }
        LcnfLetValue::FVar(v) => {
            record_use(info, *v, in_loop, false);
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
/// Increment the use count for a variable, with optional flags.
pub(super) fn record_use(
    info: &mut HashMap<LcnfVarId, UsageInfo>,
    var: LcnfVarId,
    in_loop: bool,
    escaping: bool,
) {
    let entry = info.entry(var).or_default();
    entry.add_use();
    if in_loop {
        entry.mark_in_loop();
    }
    if escaping {
        entry.mark_escaping();
    }
}
/// Record a use for an argument (only `LcnfArg::Var` contributes).
pub(super) fn record_arg_use(
    info: &mut HashMap<LcnfVarId, UsageInfo>,
    arg: &LcnfArg,
    in_loop: bool,
    escaping: bool,
) {
    if let LcnfArg::Var(v) = arg {
        record_use(info, *v, in_loop, escaping);
    }
}
/// Count total variable references in an expression (quick version
/// that does not distinguish escaping / loop context).
pub(super) fn count_refs(expr: &LcnfExpr) -> HashMap<LcnfVarId, usize> {
    let mut counts: HashMap<LcnfVarId, usize> = HashMap::new();
    count_refs_expr(expr, &mut counts);
    counts
}
pub(super) fn count_refs_expr(expr: &LcnfExpr, counts: &mut HashMap<LcnfVarId, usize>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            count_refs_value(value, counts);
            count_refs_expr(body, counts);
        }
        LcnfExpr::Case {
            scrutinee,
            alts,
            default,
            ..
        } => {
            *counts.entry(*scrutinee).or_insert(0) += 1;
            for alt in alts {
                count_refs_expr(&alt.body, counts);
            }
            if let Some(d) = default {
                count_refs_expr(d, counts);
            }
        }
        LcnfExpr::Return(arg) => {
            count_refs_arg(arg, counts);
        }
        LcnfExpr::TailCall(func, args) => {
            count_refs_arg(func, counts);
            for a in args {
                count_refs_arg(a, counts);
            }
        }
        LcnfExpr::Unreachable => {}
    }
}
pub(super) fn count_refs_value(value: &LcnfLetValue, counts: &mut HashMap<LcnfVarId, usize>) {
    match value {
        LcnfLetValue::App(func, args) => {
            count_refs_arg(func, counts);
            for a in args {
                count_refs_arg(a, counts);
            }
        }
        LcnfLetValue::Proj(_, _, v) => {
            *counts.entry(*v).or_insert(0) += 1;
        }
        LcnfLetValue::Ctor(_, _, args) => {
            for a in args {
                count_refs_arg(a, counts);
            }
        }
        LcnfLetValue::FVar(v) => {
            *counts.entry(*v).or_insert(0) += 1;
        }
        LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
pub(super) fn count_refs_arg(arg: &LcnfArg, counts: &mut HashMap<LcnfVarId, usize>) {
    if let LcnfArg::Var(v) = arg {
        *counts.entry(*v).or_insert(0) += 1;
    }
}
/// Remove let-bindings whose bound variable is never used in the
/// continuation body.  Only pure (side-effect-free) bindings are removed;
/// applications are conservatively kept because they may diverge or have
/// side-effects.
///
/// This is a single bottom-up pass; run it inside a fixed-point loop if
/// earlier passes may create new dead code.
pub fn eliminate_dead_lets(expr: &LcnfExpr) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_body = eliminate_dead_lets(body);
            let refs = count_refs(&new_body);
            let used = refs.get(id).copied().unwrap_or(0) > 0;
            if !used && is_pure_let_value(value) {
                new_body
            } else {
                LcnfExpr::Let {
                    id: *id,
                    name: name.clone(),
                    ty: ty.clone(),
                    value: value.clone(),
                    body: Box::new(new_body),
                }
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_alts: Vec<LcnfAlt> = alts
                .iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name.clone(),
                    ctor_tag: alt.ctor_tag,
                    params: alt.params.clone(),
                    body: eliminate_dead_lets(&alt.body),
                })
                .collect();
            let new_default = default.as_ref().map(|d| Box::new(eliminate_dead_lets(d)));
            LcnfExpr::Case {
                scrutinee: *scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        other => other.clone(),
    }
}
/// Returns `true` if the let-value is guaranteed pure (no side-effects,
/// no divergence).  We conservatively treat function application as
/// impure since the callee may diverge or perform IO.
pub(super) fn is_pure_let_value(value: &LcnfLetValue) -> bool {
    matches!(
        value,
        LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::FVar(_)
            | LcnfLetValue::Proj(_, _, _)
            | LcnfLetValue::Ctor(_, _, _)
    )
}
/// Propagate literal constants: when `let x = <lit>`, replace every
/// occurrence of `Var(x)` with `Lit(<lit>)` in the continuation and
/// remove the binding when it becomes dead.
///
/// Only literal values (`LcnfLetValue::Lit`) are propagated; constructor
/// constants are handled by `fold_known_case` instead.
pub fn propagate_constants(expr: &LcnfExpr) -> LcnfExpr {
    propagate_constants_env(expr, &HashMap::new())
}
/// Propagate constants with an environment mapping variables to their
/// known literal values.
pub(super) fn propagate_constants_env(
    expr: &LcnfExpr,
    env: &HashMap<LcnfVarId, LcnfLit>,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            if let LcnfLetValue::Lit(lit) = value {
                let mut new_env = env.clone();
                new_env.insert(*id, lit.clone());
                let new_body = propagate_constants_env(body, &new_env);
                let refs = count_refs(&new_body);
                if refs.get(id).copied().unwrap_or(0) == 0 {
                    new_body
                } else {
                    LcnfExpr::Let {
                        id: *id,
                        name: name.clone(),
                        ty: ty.clone(),
                        value: value.clone(),
                        body: Box::new(new_body),
                    }
                }
            } else {
                let new_value = subst_value_constants(value, env);
                let new_body = propagate_constants_env(body, env);
                LcnfExpr::Let {
                    id: *id,
                    name: name.clone(),
                    ty: ty.clone(),
                    value: new_value,
                    body: Box::new(new_body),
                }
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_alts: Vec<LcnfAlt> = alts
                .iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name.clone(),
                    ctor_tag: alt.ctor_tag,
                    params: alt.params.clone(),
                    body: propagate_constants_env(&alt.body, env),
                })
                .collect();
            let new_default = default
                .as_ref()
                .map(|d| Box::new(propagate_constants_env(d, env)));
            LcnfExpr::Case {
                scrutinee: *scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(subst_arg_constant(arg, env)),
        LcnfExpr::TailCall(func, args) => {
            let new_func = subst_arg_constant(func, env);
            let new_args: Vec<LcnfArg> = args.iter().map(|a| subst_arg_constant(a, env)).collect();
            LcnfExpr::TailCall(new_func, new_args)
        }
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
/// Substitute known constant literals inside a `LcnfLetValue`.
pub(super) fn subst_value_constants(
    value: &LcnfLetValue,
    env: &HashMap<LcnfVarId, LcnfLit>,
) -> LcnfLetValue {
    match value {
        LcnfLetValue::App(func, args) => {
            let new_func = subst_arg_constant(func, env);
            let new_args: Vec<LcnfArg> = args.iter().map(|a| subst_arg_constant(a, env)).collect();
            LcnfLetValue::App(new_func, new_args)
        }
        LcnfLetValue::Ctor(name, tag, args) => {
            let new_args: Vec<LcnfArg> = args.iter().map(|a| subst_arg_constant(a, env)).collect();
            LcnfLetValue::Ctor(name.clone(), *tag, new_args)
        }
        LcnfLetValue::FVar(v) => {
            if let Some(lit) = env.get(v) {
                LcnfLetValue::Lit(lit.clone())
            } else {
                value.clone()
            }
        }
        LcnfLetValue::Proj(name, idx, v) => LcnfLetValue::Proj(name.clone(), *idx, *v),
        other => other.clone(),
    }
}
/// Replace a variable argument with its known literal if available.
pub(super) fn subst_arg_constant(arg: &LcnfArg, env: &HashMap<LcnfVarId, LcnfLit>) -> LcnfArg {
    match arg {
        LcnfArg::Var(v) => {
            if let Some(lit) = env.get(v) {
                LcnfArg::Lit(lit.clone())
            } else {
                arg.clone()
            }
        }
        other => other.clone(),
    }
}
/// Propagate copies: when `let x = y` (i.e. `LcnfLetValue::FVar(y)`),
/// replace every use of `x` with `y` in the continuation and drop the
/// binding.
///
/// This is particularly effective after lambda lifting and join point
/// optimization which often introduce trivial copy bindings.
pub fn propagate_copies(expr: &LcnfExpr) -> LcnfExpr {
    propagate_copies_env(expr, &HashMap::new())
}
/// Copy propagation with an accumulated substitution environment.
pub(super) fn propagate_copies_env(
    expr: &LcnfExpr,
    env: &HashMap<LcnfVarId, LcnfVarId>,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            if let LcnfLetValue::FVar(src) = value {
                let resolved = resolve_copy(env, *src);
                let mut new_env = env.clone();
                new_env.insert(*id, resolved);
                let new_body = propagate_copies_env(body, &new_env);
                let refs = count_refs(&new_body);
                if refs.get(id).copied().unwrap_or(0) == 0 {
                    new_body
                } else {
                    LcnfExpr::Let {
                        id: *id,
                        name: name.clone(),
                        ty: ty.clone(),
                        value: LcnfLetValue::FVar(resolved),
                        body: Box::new(new_body),
                    }
                }
            } else {
                let new_value = subst_value_copies(value, env);
                let new_body = propagate_copies_env(body, env);
                LcnfExpr::Let {
                    id: *id,
                    name: name.clone(),
                    ty: ty.clone(),
                    value: new_value,
                    body: Box::new(new_body),
                }
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let resolved_scrutinee = resolve_copy(env, *scrutinee);
            let new_alts: Vec<LcnfAlt> = alts
                .iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name.clone(),
                    ctor_tag: alt.ctor_tag,
                    params: alt.params.clone(),
                    body: propagate_copies_env(&alt.body, env),
                })
                .collect();
            let new_default = default
                .as_ref()
                .map(|d| Box::new(propagate_copies_env(d, env)));
            LcnfExpr::Case {
                scrutinee: resolved_scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(subst_arg_copy(arg, env)),
        LcnfExpr::TailCall(func, args) => {
            let new_func = subst_arg_copy(func, env);
            let new_args: Vec<LcnfArg> = args.iter().map(|a| subst_arg_copy(a, env)).collect();
            LcnfExpr::TailCall(new_func, new_args)
        }
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
/// Follow a chain of copy substitutions to find the ultimate source.
/// Detects cycles via a visited set.
pub(super) fn resolve_copy(env: &HashMap<LcnfVarId, LcnfVarId>, mut var: LcnfVarId) -> LcnfVarId {
    let mut visited = HashSet::new();
    while let Some(&target) = env.get(&var) {
        if !visited.insert(var) {
            break;
        }
        var = target;
    }
    var
}
/// Substitute copy-renamed variables inside a let-value.
pub(super) fn subst_value_copies(
    value: &LcnfLetValue,
    env: &HashMap<LcnfVarId, LcnfVarId>,
) -> LcnfLetValue {
    match value {
        LcnfLetValue::App(func, args) => {
            let new_func = subst_arg_copy(func, env);
            let new_args: Vec<LcnfArg> = args.iter().map(|a| subst_arg_copy(a, env)).collect();
            LcnfLetValue::App(new_func, new_args)
        }
        LcnfLetValue::Ctor(name, tag, args) => {
            let new_args: Vec<LcnfArg> = args.iter().map(|a| subst_arg_copy(a, env)).collect();
            LcnfLetValue::Ctor(name.clone(), *tag, new_args)
        }
        LcnfLetValue::Proj(name, idx, v) => {
            LcnfLetValue::Proj(name.clone(), *idx, resolve_copy(env, *v))
        }
        LcnfLetValue::FVar(v) => LcnfLetValue::FVar(resolve_copy(env, *v)),
        other => other.clone(),
    }
}
/// Replace a variable argument with its copy-target if available.
pub(super) fn subst_arg_copy(arg: &LcnfArg, env: &HashMap<LcnfVarId, LcnfVarId>) -> LcnfArg {
    match arg {
        LcnfArg::Var(v) => LcnfArg::Var(resolve_copy(env, *v)),
        other => other.clone(),
    }
}
/// Eliminate case alternatives that are statically unreachable.
///
/// Currently detects three patterns:
/// 1. Any alternative whose body is `Unreachable` is removed.
/// 2. If the default is `Unreachable`, it is removed.
/// 3. If after trimming there are no alternatives left but a default
///    exists, the case is replaced with the default body.  If there are
///    no alternatives and no default, the result is `Unreachable`.
pub fn eliminate_unreachable_alts(expr: &LcnfExpr) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => LcnfExpr::Let {
            id: *id,
            name: name.clone(),
            ty: ty.clone(),
            value: value.clone(),
            body: Box::new(eliminate_unreachable_alts(body)),
        },
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let mut new_alts: Vec<LcnfAlt> = alts
                .iter()
                .map(|alt| LcnfAlt {
                    ctor_name: alt.ctor_name.clone(),
                    ctor_tag: alt.ctor_tag,
                    params: alt.params.clone(),
                    body: eliminate_unreachable_alts(&alt.body),
                })
                .collect();
            let mut new_default = default
                .as_ref()
                .map(|d| Box::new(eliminate_unreachable_alts(d)));
            new_alts.retain(|alt| !matches!(alt.body, LcnfExpr::Unreachable));
            if let Some(ref d) = new_default {
                if matches!(d.as_ref(), LcnfExpr::Unreachable) {
                    new_default = None;
                }
            }
            if new_alts.is_empty() {
                if let Some(def) = new_default {
                    return *def;
                }
                return LcnfExpr::Unreachable;
            }
            LcnfExpr::Case {
                scrutinee: *scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        other => other.clone(),
    }
}
/// Fold a case expression when the scrutinee is a known constructor.
///
/// When we can determine (via a local constant environment) that the
/// scrutinee variable was bound to a specific constructor, we select the
/// matching alternative and substitute the constructor's fields for the
/// alt's parameters.
pub fn fold_known_case(expr: &LcnfExpr) -> LcnfExpr {
    fold_known_case_env(expr, &HashMap::new())
}
/// Known-case folding with an environment mapping variables to their
/// known `ConstValue`.
pub(super) fn fold_known_case_env(
    expr: &LcnfExpr,
    env: &HashMap<LcnfVarId, ConstValue>,
) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let mut new_env = env.clone();
            match value {
                LcnfLetValue::Ctor(ctor_name, tag, args) => {
                    new_env.insert(*id, ConstValue::Ctor(ctor_name.clone(), *tag, args.clone()));
                }
                LcnfLetValue::Lit(lit) => {
                    new_env.insert(*id, ConstValue::Lit(lit.clone()));
                }
                _ => {}
            }
            let new_body = fold_known_case_env(body, &new_env);
            LcnfExpr::Let {
                id: *id,
                name: name.clone(),
                ty: ty.clone(),
                value: value.clone(),
                body: Box::new(new_body),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            if let Some(ConstValue::Ctor(_, known_tag, ctor_args)) = env.get(scrutinee) {
                if let Some(matching_alt) = alts.iter().find(|a| a.ctor_tag == *known_tag) {
                    let mut result = matching_alt.body.clone();
                    result = substitute_alt_params(&result, &matching_alt.params, ctor_args);
                    return fold_known_case_env(&result, env);
                }
                if let Some(def) = default {
                    return fold_known_case_env(def, env);
                }
                return LcnfExpr::Unreachable;
            }
            let new_alts: Vec<LcnfAlt> = alts
                .iter()
                .map(|alt| {
                    let mut branch_env = env.clone();
                    branch_env.insert(
                        *scrutinee,
                        ConstValue::Ctor(
                            alt.ctor_name.clone(),
                            alt.ctor_tag,
                            alt.params.iter().map(|p| LcnfArg::Var(p.id)).collect(),
                        ),
                    );
                    LcnfAlt {
                        ctor_name: alt.ctor_name.clone(),
                        ctor_tag: alt.ctor_tag,
                        params: alt.params.clone(),
                        body: fold_known_case_env(&alt.body, &branch_env),
                    }
                })
                .collect();
            let new_default = default
                .as_ref()
                .map(|d| Box::new(fold_known_case_env(d, env)));
            LcnfExpr::Case {
                scrutinee: *scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(arg.clone()),
        LcnfExpr::TailCall(func, args) => LcnfExpr::TailCall(func.clone(), args.clone()),
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
/// Substitute constructor field values for the alt-bound parameters in
/// an expression.  For each parameter `p_i` in `params`, if `ctor_args`
/// has a matching argument at position `i`, we replace `Var(p_i.id)`
/// with that argument throughout `expr`.
pub(super) fn substitute_alt_params(
    expr: &LcnfExpr,
    params: &[LcnfParam],
    ctor_args: &[LcnfArg],
) -> LcnfExpr {
    let mut subst: HashMap<LcnfVarId, LcnfArg> = HashMap::new();
    for (param, arg) in params.iter().zip(ctor_args.iter()) {
        subst.insert(param.id, arg.clone());
    }
    apply_arg_subst(expr, &subst)
}
/// Apply an argument-level substitution throughout an expression.
pub(super) fn apply_arg_subst(expr: &LcnfExpr, subst: &HashMap<LcnfVarId, LcnfArg>) -> LcnfExpr {
    match expr {
        LcnfExpr::Let {
            id,
            name,
            ty,
            value,
            body,
        } => {
            let new_value = apply_value_subst(value, subst);
            let mut inner_subst = subst.clone();
            inner_subst.remove(id);
            let new_body = apply_arg_subst(body, &inner_subst);
            LcnfExpr::Let {
                id: *id,
                name: name.clone(),
                ty: ty.clone(),
                value: new_value,
                body: Box::new(new_body),
            }
        }
        LcnfExpr::Case {
            scrutinee,
            scrutinee_ty,
            alts,
            default,
        } => {
            let new_scrutinee = resolve_var_subst(subst, *scrutinee);
            let new_alts: Vec<LcnfAlt> = alts
                .iter()
                .map(|alt| {
                    let mut alt_subst = subst.clone();
                    for p in &alt.params {
                        alt_subst.remove(&p.id);
                    }
                    LcnfAlt {
                        ctor_name: alt.ctor_name.clone(),
                        ctor_tag: alt.ctor_tag,
                        params: alt.params.clone(),
                        body: apply_arg_subst(&alt.body, &alt_subst),
                    }
                })
                .collect();
            let new_default = default
                .as_ref()
                .map(|d| Box::new(apply_arg_subst(d, subst)));
            LcnfExpr::Case {
                scrutinee: new_scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            }
        }
        LcnfExpr::Return(arg) => LcnfExpr::Return(do_subst_arg(arg, subst)),
        LcnfExpr::TailCall(func, args) => {
            let new_func = do_subst_arg(func, subst);
            let new_args: Vec<LcnfArg> = args.iter().map(|a| do_subst_arg(a, subst)).collect();
            LcnfExpr::TailCall(new_func, new_args)
        }
        LcnfExpr::Unreachable => LcnfExpr::Unreachable,
    }
}
/// Apply substitution to a let-value.
pub(super) fn apply_value_subst(
    value: &LcnfLetValue,
    subst: &HashMap<LcnfVarId, LcnfArg>,
) -> LcnfLetValue {
    match value {
        LcnfLetValue::App(func, args) => {
            let new_func = do_subst_arg(func, subst);
            let new_args: Vec<LcnfArg> = args.iter().map(|a| do_subst_arg(a, subst)).collect();
            LcnfLetValue::App(new_func, new_args)
        }
        LcnfLetValue::Ctor(name, tag, args) => {
            let new_args: Vec<LcnfArg> = args.iter().map(|a| do_subst_arg(a, subst)).collect();
            LcnfLetValue::Ctor(name.clone(), *tag, new_args)
        }
        LcnfLetValue::Proj(name, idx, v) => {
            let resolved = resolve_var_subst(subst, *v);
            LcnfLetValue::Proj(name.clone(), *idx, resolved)
        }
        LcnfLetValue::FVar(v) => {
            if let Some(replacement) = subst.get(v) {
                match replacement {
                    LcnfArg::Var(new_v) => LcnfLetValue::FVar(*new_v),
                    LcnfArg::Lit(lit) => LcnfLetValue::Lit(lit.clone()),
                    _ => value.clone(),
                }
            } else {
                value.clone()
            }
        }
        other => other.clone(),
    }
}
/// Substitute an argument via the substitution map.
pub(super) fn do_subst_arg(arg: &LcnfArg, subst: &HashMap<LcnfVarId, LcnfArg>) -> LcnfArg {
    match arg {
        LcnfArg::Var(v) => {
            if let Some(replacement) = subst.get(v) {
                replacement.clone()
            } else {
                arg.clone()
            }
        }
        other => other.clone(),
    }
}
/// Resolve a variable through a substitution map; returns the original
/// if not present, or the target variable if the substitution maps to a Var.
pub(super) fn resolve_var_subst(subst: &HashMap<LcnfVarId, LcnfArg>, var: LcnfVarId) -> LcnfVarId {
    if let Some(LcnfArg::Var(target)) = subst.get(&var) {
        *target
    } else {
        var
    }
}
/// Remove function declarations from `module` that are not reachable from
/// the given `roots`.  A function is reachable if it is named in `roots`
/// or transitively called by a reachable function.
///
/// The call-graph is approximated conservatively: any mention of a
/// function name in the body of another function counts as a reference.
pub fn eliminate_dead_functions(module: &LcnfModule, roots: &[String]) -> LcnfModule {
    let name_to_idx: HashMap<&str, usize> = module
        .fun_decls
        .iter()
        .enumerate()
        .map(|(i, d)| (d.name.as_str(), i))
        .collect();
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); module.fun_decls.len()];
    for (i, decl) in module.fun_decls.iter().enumerate() {
        let mentioned = collect_mentioned_names(&decl.body);
        for name in &mentioned {
            if let Some(&j) = name_to_idx.get(name.as_str()) {
                adj[i].insert(j);
            }
        }
    }
    let mut reachable: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    for root in roots {
        if let Some(&idx) = name_to_idx.get(root.as_str()) {
            if reachable.insert(idx) {
                queue.push_back(idx);
            }
        }
    }
    for (i, decl) in module.fun_decls.iter().enumerate() {
        if !decl.is_lifted && reachable.insert(i) {
            queue.push_back(i);
        }
    }
    while let Some(idx) = queue.pop_front() {
        for &callee in &adj[idx] {
            if reachable.insert(callee) {
                queue.push_back(callee);
            }
        }
    }
    let kept_decls: Vec<LcnfFunDecl> = module
        .fun_decls
        .iter()
        .enumerate()
        .filter(|(i, _)| reachable.contains(i))
        .map(|(_, d)| d.clone())
        .collect();
    let eliminated_count = module.fun_decls.len() - kept_decls.len();
    LcnfModule {
        fun_decls: kept_decls,
        extern_decls: module.extern_decls.clone(),
        name: module.name.clone(),
        metadata: LcnfModuleMetadata {
            decl_count: module.metadata.decl_count.saturating_sub(eliminated_count),
            ..module.metadata.clone()
        },
    }
}
/// Collect all constructor / function names mentioned in an expression.
/// This is a conservative over-approximation used for reachability.
pub(super) fn collect_mentioned_names(expr: &LcnfExpr) -> HashSet<String> {
    let mut names = HashSet::new();
    collect_names_inner(expr, &mut names);
    names
}
pub(super) fn collect_names_inner(expr: &LcnfExpr, names: &mut HashSet<String>) {
    match expr {
        LcnfExpr::Let { value, body, .. } => {
            collect_names_value(value, names);
            collect_names_inner(body, names);
        }
        LcnfExpr::Case { alts, default, .. } => {
            for alt in alts {
                names.insert(alt.ctor_name.clone());
                collect_names_inner(&alt.body, names);
            }
            if let Some(d) = default {
                collect_names_inner(d, names);
            }
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
    }
}
pub(super) fn collect_names_value(value: &LcnfLetValue, names: &mut HashSet<String>) {
    match value {
        LcnfLetValue::Ctor(name, _, _) => {
            names.insert(name.clone());
        }
        LcnfLetValue::Proj(name, _, _) => {
            names.insert(name.clone());
        }
        LcnfLetValue::App(_, _)
        | LcnfLetValue::FVar(_)
        | LcnfLetValue::Lit(_)
        | LcnfLetValue::Erased
        | LcnfLetValue::Reset(_)
        | LcnfLetValue::Reuse(_, _, _, _) => {}
    }
}
/// Run the complete DCE + constant propagation pipeline on a module.
///
/// The optimizer runs the enabled passes in a fixed-point loop:
///   1. Constant propagation
///   2. Copy propagation
///   3. Known case folding
///   4. Dead let elimination
///   5. Unreachable alt elimination
///
/// After the fixed point, dead function elimination is applied once
/// (interprocedural).
///
/// Returns the optimized module and accumulated statistics.
pub fn optimize_dce(module: &LcnfModule, config: &DceConfig) -> (LcnfModule, DceStats) {
    let mut stats = DceStats::default();
    let mut result = module.clone();
    for decl in &mut result.fun_decls {
        let fn_stats = optimize_function_body(&mut decl.body, config);
        stats.merge(&fn_stats);
    }
    let roots: Vec<String> = result
        .fun_decls
        .iter()
        .filter(|d| !d.is_lifted)
        .map(|d| d.name.clone())
        .collect();
    let before_count = result.fun_decls.len();
    result = eliminate_dead_functions(&result, &roots);
    stats.functions_eliminated += before_count.saturating_sub(result.fun_decls.len());
    (result, stats)
}
/// Optimize a single function body using intraprocedural passes.
pub(super) fn optimize_function_body(body: &mut LcnfExpr, config: &DceConfig) -> DceStats {
    let mut total_stats = DceStats::default();
    for _iteration in 0..config.max_iterations {
        total_stats.iterations += 1;
        let before = count_let_bindings(body);
        if config.propagate_constants {
            *body = propagate_constants(body);
        }
        if config.propagate_copies {
            *body = propagate_copies(body);
        }
        if config.fold_known_calls {
            *body = fold_known_case(body);
        }
        if config.eliminate_unused_lets {
            *body = eliminate_dead_lets(body);
        }
        if config.eliminate_unreachable_alts {
            *body = eliminate_unreachable_alts(body);
        }
        let after = count_let_bindings(body);
        let eliminated = before.saturating_sub(after);
        total_stats.lets_eliminated += eliminated;
        if eliminated == 0 {
            break;
        }
    }
    total_stats
}
/// Count the number of let-bindings in an expression (used for convergence
/// checking in the fixed-point loop).
pub(super) fn count_let_bindings(expr: &LcnfExpr) -> usize {
    match expr {
        LcnfExpr::Let { body, .. } => 1 + count_let_bindings(body),
        LcnfExpr::Case { alts, default, .. } => {
            let alt_count: usize = alts.iter().map(|a| count_let_bindings(&a.body)).sum();
            let def_count = default.as_ref().map(|d| count_let_bindings(d)).unwrap_or(0);
            alt_count + def_count
        }
        LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => 0,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn vid(n: u64) -> LcnfVarId {
        LcnfVarId(n)
    }
    pub(super) fn mk_param(n: u64, name: &str) -> LcnfParam {
        LcnfParam {
            id: vid(n),
            name: name.to_string(),
            ty: LcnfType::Nat,
            erased: false,
            borrowed: false,
        }
    }
    pub(super) fn mk_let(id: u64, value: LcnfLetValue, body: LcnfExpr) -> LcnfExpr {
        LcnfExpr::Let {
            id: vid(id),
            name: format!("x{}", id),
            ty: LcnfType::Nat,
            value,
            body: Box::new(body),
        }
    }
    pub(super) fn mk_decl(name: &str, body: LcnfExpr) -> LcnfFunDecl {
        LcnfFunDecl {
            name: name.to_string(),
            original_name: None,
            params: vec![mk_param(0, "a")],
            ret_type: LcnfType::Nat,
            body,
            is_recursive: false,
            is_lifted: false,
            inline_cost: 1,
        }
    }
    pub(super) fn mk_module(decls: Vec<LcnfFunDecl>) -> LcnfModule {
        LcnfModule {
            fun_decls: decls,
            extern_decls: vec![],
            name: "test_mod".to_string(),
            metadata: LcnfModuleMetadata::default(),
        }
    }
    #[test]
    pub(super) fn test_config_default() {
        let cfg = DceConfig::default();
        assert!(cfg.eliminate_unused_lets);
        assert!(cfg.eliminate_unreachable_alts);
        assert!(cfg.propagate_constants);
        assert!(cfg.propagate_copies);
        assert!(cfg.fold_known_calls);
        assert_eq!(cfg.max_iterations, 10);
    }
    #[test]
    pub(super) fn test_config_display() {
        let cfg = DceConfig::default();
        let s = cfg.to_string();
        assert!(s.contains("unused_lets=true"));
        assert!(s.contains("max_iter=10"));
    }
    #[test]
    pub(super) fn test_stats_default() {
        let stats = DceStats::default();
        assert_eq!(stats.total_changes(), 0);
    }
    #[test]
    pub(super) fn test_stats_merge() {
        let mut a = DceStats {
            lets_eliminated: 3,
            ..Default::default()
        };
        let b = DceStats {
            lets_eliminated: 2,
            constants_propagated: 1,
            ..Default::default()
        };
        a.merge(&b);
        assert_eq!(a.lets_eliminated, 5);
        assert_eq!(a.constants_propagated, 1);
    }
    #[test]
    pub(super) fn test_stats_display() {
        let stats = DceStats {
            lets_eliminated: 7,
            ..Default::default()
        };
        let s = stats.to_string();
        assert!(s.contains("lets_elim=7"));
    }
    #[test]
    pub(super) fn test_const_value_lit() {
        let cv = ConstValue::Lit(LcnfLit::Nat(42));
        assert!(cv.is_known());
        assert_eq!(cv.as_lit(), Some(&LcnfLit::Nat(42)));
        assert!(cv.as_ctor().is_none());
    }
    #[test]
    pub(super) fn test_const_value_ctor() {
        let cv = ConstValue::Ctor("Nil".to_string(), 0, vec![]);
        assert!(cv.is_known());
        assert!(cv.as_lit().is_none());
        let (name, tag, args) = cv.as_ctor().expect("expected Some/Ok value");
        assert_eq!(name, "Nil");
        assert_eq!(tag, 0);
        assert!(args.is_empty());
    }
    #[test]
    pub(super) fn test_const_value_unknown() {
        let cv = ConstValue::Unknown;
        assert!(!cv.is_known());
    }
    #[test]
    pub(super) fn test_const_value_display() {
        assert!(ConstValue::Unknown.to_string().contains("unknown"));
        let lit = ConstValue::Lit(LcnfLit::Nat(99));
        assert!(lit.to_string().contains("99"));
    }
    #[test]
    pub(super) fn test_usage_info_basic() {
        let mut u = UsageInfo::new();
        assert!(u.is_dead());
        assert!(!u.is_once());
        u.add_use();
        assert!(!u.is_dead());
        assert!(u.is_once());
        u.add_use();
        assert!(!u.is_once());
        assert_eq!(u.use_count, 2);
    }
    #[test]
    pub(super) fn test_usage_info_flags() {
        let mut u = UsageInfo::new();
        assert!(!u.is_escaping);
        assert!(!u.is_in_loop);
        u.mark_escaping();
        assert!(u.is_escaping);
        u.mark_in_loop();
        assert!(u.is_in_loop);
    }
    #[test]
    pub(super) fn test_usage_info_display() {
        let u = UsageInfo {
            use_count: 3,
            is_escaping: true,
            is_in_loop: false,
        };
        let s = u.to_string();
        assert!(s.contains("uses=3"));
        assert!(s.contains("escaping=true"));
    }
    #[test]
    pub(super) fn test_collect_usage_simple_return() {
        let expr = LcnfExpr::Return(LcnfArg::Var(vid(5)));
        let info = collect_usage_info(&expr);
        assert_eq!(
            info.get(&vid(5))
                .expect("value should be present in map")
                .use_count,
            1
        );
    }
    #[test]
    pub(super) fn test_collect_usage_let_chain() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            mk_let(
                2,
                LcnfLetValue::FVar(vid(1)),
                LcnfExpr::Return(LcnfArg::Var(vid(2))),
            ),
        );
        let info = collect_usage_info(&expr);
        assert_eq!(
            info.get(&vid(1))
                .expect("value should be present in map")
                .use_count,
            1
        );
        assert_eq!(
            info.get(&vid(2))
                .expect("value should be present in map")
                .use_count,
            1
        );
    }
    #[test]
    pub(super) fn test_collect_usage_ctor_escaping() {
        let expr = mk_let(
            1,
            LcnfLetValue::Ctor("Cons".into(), 1, vec![LcnfArg::Var(vid(0))]),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        let info = collect_usage_info(&expr);
        assert!(
            info.get(&vid(0))
                .expect("value should be present")
                .is_escaping
        );
    }
    #[test]
    pub(super) fn test_collect_usage_tail_call() {
        let expr = LcnfExpr::TailCall(
            LcnfArg::Var(vid(10)),
            vec![LcnfArg::Var(vid(0)), LcnfArg::Var(vid(1))],
        );
        let info = collect_usage_info(&expr);
        assert_eq!(
            info.get(&vid(10))
                .expect("value should be present in map")
                .use_count,
            1
        );
        assert_eq!(
            info.get(&vid(0))
                .expect("value should be present in map")
                .use_count,
            1
        );
        assert_eq!(
            info.get(&vid(1))
                .expect("value should be present in map")
                .use_count,
            1
        );
    }
    #[test]
    pub(super) fn test_eliminate_dead_lets_simple() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(vid(0))),
        );
        let result = eliminate_dead_lets(&expr);
        assert!(matches!(result, LcnfExpr::Return(LcnfArg::Var(v)) if v == vid(0)));
    }
    #[test]
    pub(super) fn test_eliminate_dead_lets_keeps_used() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        let result = eliminate_dead_lets(&expr);
        assert!(matches!(result, LcnfExpr::Let { id, .. } if id == vid(1)));
    }
    #[test]
    pub(super) fn test_eliminate_dead_lets_chain() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            mk_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                mk_let(
                    3,
                    LcnfLetValue::Lit(LcnfLit::Nat(3)),
                    LcnfExpr::Return(LcnfArg::Var(vid(3))),
                ),
            ),
        );
        let result = eliminate_dead_lets(&expr);
        assert!(matches!(& result, LcnfExpr::Let { id, .. } if * id == vid(3)));
    }
    #[test]
    pub(super) fn test_eliminate_dead_lets_keeps_app() {
        let expr = mk_let(
            1,
            LcnfLetValue::App(LcnfArg::Var(vid(10)), vec![LcnfArg::Var(vid(0))]),
            LcnfExpr::Return(LcnfArg::Var(vid(0))),
        );
        let result = eliminate_dead_lets(&expr);
        assert!(matches!(result, LcnfExpr::Let { .. }));
    }
    #[test]
    pub(super) fn test_eliminate_dead_lets_in_case() {
        let expr = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![
                LcnfAlt {
                    ctor_name: "A".into(),
                    ctor_tag: 0,
                    params: vec![],
                    body: mk_let(
                        5,
                        LcnfLetValue::Lit(LcnfLit::Nat(99)),
                        LcnfExpr::Return(LcnfArg::Var(vid(0))),
                    ),
                },
                LcnfAlt {
                    ctor_name: "B".into(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Var(vid(0))),
                },
            ],
            default: None,
        };
        let result = eliminate_dead_lets(&expr);
        if let LcnfExpr::Case { alts, .. } = &result {
            assert!(matches!(&alts[0].body, LcnfExpr::Return(_)));
        } else {
            panic!("expected Case");
        }
    }
    #[test]
    pub(super) fn test_propagate_constants_simple() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        let result = propagate_constants(&expr);
        assert!(matches!(
            result,
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42)))
        ));
    }
    #[test]
    pub(super) fn test_propagate_constants_in_app() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(10)),
            mk_let(
                2,
                LcnfLetValue::App(LcnfArg::Var(vid(99)), vec![LcnfArg::Var(vid(1))]),
                LcnfExpr::Return(LcnfArg::Var(vid(2))),
            ),
        );
        let result = propagate_constants(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::App(_, args) = value {
                assert!(matches!(args[0], LcnfArg::Lit(LcnfLit::Nat(10))));
            } else {
                panic!("expected App");
            }
        } else {
            panic!("expected Let");
        }
    }
    #[test]
    pub(super) fn test_propagate_constants_in_tail_call() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(5)),
            LcnfExpr::TailCall(LcnfArg::Var(vid(99)), vec![LcnfArg::Var(vid(1))]),
        );
        let result = propagate_constants(&expr);
        if let LcnfExpr::TailCall(_, args) = &result {
            assert!(matches!(args[0], LcnfArg::Lit(LcnfLit::Nat(5))));
        } else {
            panic!("expected TailCall");
        }
    }
    #[test]
    pub(super) fn test_propagate_copies_simple() {
        let expr = mk_let(
            2,
            LcnfLetValue::FVar(vid(1)),
            LcnfExpr::Return(LcnfArg::Var(vid(2))),
        );
        let result = propagate_copies(&expr);
        assert!(matches!(result, LcnfExpr::Return(LcnfArg::Var(v)) if v == vid(1)));
    }
    #[test]
    pub(super) fn test_propagate_copies_transitive() {
        let expr = mk_let(
            2,
            LcnfLetValue::FVar(vid(1)),
            mk_let(
                3,
                LcnfLetValue::FVar(vid(2)),
                LcnfExpr::Return(LcnfArg::Var(vid(3))),
            ),
        );
        let result = propagate_copies(&expr);
        assert!(matches!(result, LcnfExpr::Return(LcnfArg::Var(v)) if v == vid(1)));
    }
    #[test]
    pub(super) fn test_propagate_copies_in_case_scrutinee() {
        let expr = mk_let(
            2,
            LcnfLetValue::FVar(vid(1)),
            LcnfExpr::Case {
                scrutinee: vid(2),
                scrutinee_ty: LcnfType::Nat,
                alts: vec![LcnfAlt {
                    ctor_name: "Zero".into(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                }],
                default: None,
            },
        );
        let result = propagate_copies(&expr);
        if let LcnfExpr::Case { scrutinee, .. } = &result {
            assert_eq!(*scrutinee, vid(1));
        } else {
            panic!("expected Case");
        }
    }
    #[test]
    pub(super) fn test_propagate_copies_in_value() {
        let expr = mk_let(
            2,
            LcnfLetValue::FVar(vid(1)),
            mk_let(
                3,
                LcnfLetValue::Ctor(
                    "Pair".into(),
                    0,
                    vec![LcnfArg::Var(vid(2)), LcnfArg::Var(vid(2))],
                ),
                LcnfExpr::Return(LcnfArg::Var(vid(3))),
            ),
        );
        let result = propagate_copies(&expr);
        if let LcnfExpr::Let { value, .. } = &result {
            if let LcnfLetValue::Ctor(_, _, args) = value {
                assert!(matches!(args[0], LcnfArg::Var(v) if v == vid(1)));
                assert!(matches!(args[1], LcnfArg::Var(v) if v == vid(1)));
            } else {
                panic!("expected Ctor");
            }
        } else {
            panic!("expected Let");
        }
    }
    #[test]
    pub(super) fn test_eliminate_unreachable_alts_removes_unreachable_default() {
        let expr = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "Zero".into(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
            }],
            default: Some(Box::new(LcnfExpr::Unreachable)),
        };
        let result = eliminate_unreachable_alts(&expr);
        if let LcnfExpr::Case { default, .. } = &result {
            assert!(default.is_none());
        } else {
            panic!("expected Case");
        }
    }
    #[test]
    pub(super) fn test_eliminate_unreachable_alts_removes_unreachable_alt() {
        let expr = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![
                LcnfAlt {
                    ctor_name: "Zero".into(),
                    ctor_tag: 0,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                },
                LcnfAlt {
                    ctor_name: "Dead".into(),
                    ctor_tag: 99,
                    params: vec![],
                    body: LcnfExpr::Unreachable,
                },
            ],
            default: None,
        };
        let result = eliminate_unreachable_alts(&expr);
        if let LcnfExpr::Case { alts, .. } = &result {
            assert_eq!(alts.len(), 1);
            assert_eq!(alts[0].ctor_name, "Zero");
        } else {
            panic!("expected Case");
        }
    }
    #[test]
    pub(super) fn test_eliminate_unreachable_alts_inline_default_when_no_alts() {
        let expr = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "Dead".into(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Unreachable,
            }],
            default: Some(Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(77))))),
        };
        let result = eliminate_unreachable_alts(&expr);
        assert!(matches!(
            result,
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(77)))
        ));
    }
    #[test]
    pub(super) fn test_eliminate_unreachable_alts_all_dead() {
        let expr = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "X".into(),
                ctor_tag: 0,
                params: vec![],
                body: LcnfExpr::Unreachable,
            }],
            default: None,
        };
        let result = eliminate_unreachable_alts(&expr);
        assert!(matches!(result, LcnfExpr::Unreachable));
    }
    #[test]
    pub(super) fn test_fold_known_case_simple() {
        let expr = mk_let(
            1,
            LcnfLetValue::Ctor("Nil".into(), 0, vec![]),
            LcnfExpr::Case {
                scrutinee: vid(1),
                scrutinee_ty: LcnfType::Ctor("List".into(), vec![LcnfType::Nat]),
                alts: vec![
                    LcnfAlt {
                        ctor_name: "Nil".into(),
                        ctor_tag: 0,
                        params: vec![],
                        body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                    },
                    LcnfAlt {
                        ctor_name: "Cons".into(),
                        ctor_tag: 1,
                        params: vec![mk_param(10, "h"), mk_param(11, "t")],
                        body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                    },
                ],
                default: None,
            },
        );
        let result = fold_known_case(&expr);
        if let LcnfExpr::Let { body, .. } = &result {
            assert!(matches!(
                body.as_ref(),
                LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))
            ));
        } else {
            panic!("expected Let wrapping the folded result");
        }
    }
    #[test]
    pub(super) fn test_fold_known_case_with_params() {
        let expr = mk_let(
            1,
            LcnfLetValue::Ctor(
                "Cons".into(),
                1,
                vec![LcnfArg::Var(vid(10)), LcnfArg::Var(vid(11))],
            ),
            LcnfExpr::Case {
                scrutinee: vid(1),
                scrutinee_ty: LcnfType::Ctor("List".into(), vec![LcnfType::Nat]),
                alts: vec![
                    LcnfAlt {
                        ctor_name: "Nil".into(),
                        ctor_tag: 0,
                        params: vec![],
                        body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0))),
                    },
                    LcnfAlt {
                        ctor_name: "Cons".into(),
                        ctor_tag: 1,
                        params: vec![mk_param(20, "h"), mk_param(21, "t")],
                        body: LcnfExpr::Return(LcnfArg::Var(vid(20))),
                    },
                ],
                default: None,
            },
        );
        let result = fold_known_case(&expr);
        if let LcnfExpr::Let { body, .. } = &result {
            assert!(
                matches!(body.as_ref(), LcnfExpr::Return(LcnfArg::Var(v)) if * v ==
                vid(10))
            );
        } else {
            panic!("expected Let wrapping folded result");
        }
    }
    #[test]
    pub(super) fn test_fold_known_case_falls_to_default() {
        let expr = mk_let(
            1,
            LcnfLetValue::Ctor("Nil".into(), 0, vec![]),
            LcnfExpr::Case {
                scrutinee: vid(1),
                scrutinee_ty: LcnfType::Nat,
                alts: vec![LcnfAlt {
                    ctor_name: "Cons".into(),
                    ctor_tag: 1,
                    params: vec![],
                    body: LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
                }],
                default: Some(Box::new(LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(99))))),
            },
        );
        let result = fold_known_case(&expr);
        if let LcnfExpr::Let { body, .. } = &result {
            assert!(matches!(
                body.as_ref(),
                LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(99)))
            ));
        } else {
            panic!("expected Let wrapping default");
        }
    }
    #[test]
    pub(super) fn test_eliminate_dead_functions_keeps_roots() {
        let decls = vec![
            mk_decl("main", LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
            mk_decl("helper", LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1)))),
        ];
        let module = mk_module(decls);
        let result = eliminate_dead_functions(&module, &["main".to_string()]);
        assert_eq!(result.fun_decls.len(), 2);
    }
    #[test]
    pub(super) fn test_eliminate_dead_functions_removes_lifted_unreachable() {
        let mut lifted = mk_decl(
            "lifted_helper",
            LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(1))),
        );
        lifted.is_lifted = true;
        let decls = vec![
            mk_decl("main", LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)))),
            lifted,
        ];
        let module = mk_module(decls);
        let result = eliminate_dead_functions(&module, &["main".to_string()]);
        assert_eq!(result.fun_decls.len(), 1);
        assert_eq!(result.fun_decls[0].name, "main");
    }
    #[test]
    pub(super) fn test_optimize_dce_full_pipeline() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            mk_let(
                2,
                LcnfLetValue::FVar(vid(1)),
                mk_let(
                    3,
                    LcnfLetValue::Lit(LcnfLit::Nat(99)),
                    LcnfExpr::Return(LcnfArg::Var(vid(2))),
                ),
            ),
        );
        let module = mk_module(vec![mk_decl("test", body)]);
        let config = DceConfig::default();
        let (result, stats) = optimize_dce(&module, &config);
        assert_eq!(result.fun_decls.len(), 1);
        let final_body = &result.fun_decls[0].body;
        assert!(
            matches!(final_body, LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(42)))),
            "expected return 42, got: {:?}",
            final_body,
        );
        assert!(stats.lets_eliminated > 0);
    }
    #[test]
    pub(super) fn test_optimize_dce_no_passes() {
        let body = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(42)),
            LcnfExpr::Return(LcnfArg::Var(vid(0))),
        );
        let module = mk_module(vec![mk_decl("test", body)]);
        let config = DceConfig {
            eliminate_unused_lets: false,
            eliminate_unreachable_alts: false,
            propagate_constants: false,
            propagate_copies: false,
            fold_known_calls: false,
            max_iterations: 10,
        };
        let (result, _stats) = optimize_dce(&module, &config);
        assert!(matches!(result.fun_decls[0].body, LcnfExpr::Let { .. }));
    }
    #[test]
    pub(super) fn test_count_let_bindings() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(1)),
            mk_let(
                2,
                LcnfLetValue::Lit(LcnfLit::Nat(2)),
                LcnfExpr::Return(LcnfArg::Var(vid(2))),
            ),
        );
        assert_eq!(count_let_bindings(&expr), 2);
    }
    #[test]
    pub(super) fn test_count_let_bindings_terminal() {
        let expr = LcnfExpr::Return(LcnfArg::Lit(LcnfLit::Nat(0)));
        assert_eq!(count_let_bindings(&expr), 0);
    }
    #[test]
    pub(super) fn test_count_let_bindings_case() {
        let expr = LcnfExpr::Case {
            scrutinee: vid(0),
            scrutinee_ty: LcnfType::Nat,
            alts: vec![LcnfAlt {
                ctor_name: "A".into(),
                ctor_tag: 0,
                params: vec![],
                body: mk_let(
                    5,
                    LcnfLetValue::Lit(LcnfLit::Nat(5)),
                    LcnfExpr::Return(LcnfArg::Var(vid(5))),
                ),
            }],
            default: Some(Box::new(mk_let(
                6,
                LcnfLetValue::Lit(LcnfLit::Nat(6)),
                LcnfExpr::Return(LcnfArg::Var(vid(6))),
            ))),
        };
        assert_eq!(count_let_bindings(&expr), 2);
    }
    #[test]
    pub(super) fn test_resolve_copy_chain() {
        let mut env = HashMap::new();
        env.insert(vid(3), vid(2));
        env.insert(vid(2), vid(1));
        assert_eq!(resolve_copy(&env, vid(3)), vid(1));
    }
    #[test]
    pub(super) fn test_resolve_copy_cycle() {
        let mut env = HashMap::new();
        env.insert(vid(1), vid(2));
        env.insert(vid(2), vid(1));
        let _ = resolve_copy(&env, vid(1));
    }
    #[test]
    pub(super) fn test_resolve_copy_identity() {
        let env: HashMap<LcnfVarId, LcnfVarId> = HashMap::new();
        assert_eq!(resolve_copy(&env, vid(7)), vid(7));
    }
    #[test]
    pub(super) fn test_is_pure_let_value() {
        assert!(is_pure_let_value(&LcnfLetValue::Lit(LcnfLit::Nat(0))));
        assert!(is_pure_let_value(&LcnfLetValue::Erased));
        assert!(is_pure_let_value(&LcnfLetValue::FVar(vid(0))));
        assert!(is_pure_let_value(&LcnfLetValue::Proj(
            "S".into(),
            0,
            vid(0)
        )));
        assert!(is_pure_let_value(&LcnfLetValue::Ctor(
            "X".into(),
            0,
            vec![]
        )));
        assert!(!is_pure_let_value(&LcnfLetValue::App(
            LcnfArg::Var(vid(0)),
            vec![]
        )));
    }
    #[test]
    pub(super) fn test_count_refs_multiple_uses() {
        let expr = mk_let(
            1,
            LcnfLetValue::App(
                LcnfArg::Var(vid(99)),
                vec![LcnfArg::Var(vid(0)), LcnfArg::Var(vid(0))],
            ),
            LcnfExpr::Return(LcnfArg::Var(vid(1))),
        );
        let refs = count_refs(&expr);
        assert_eq!(refs.get(&vid(0)).copied().unwrap_or(0), 2);
        assert_eq!(refs.get(&vid(1)).copied().unwrap_or(0), 1);
        assert_eq!(refs.get(&vid(99)).copied().unwrap_or(0), 1);
    }
    #[test]
    pub(super) fn test_const_prop_then_dead_let() {
        let expr = mk_let(
            1,
            LcnfLetValue::Lit(LcnfLit::Nat(10)),
            mk_let(
                2,
                LcnfLetValue::App(LcnfArg::Var(vid(99)), vec![LcnfArg::Var(vid(1))]),
                LcnfExpr::Return(LcnfArg::Var(vid(2))),
            ),
        );
        let after_const = propagate_constants(&expr);
        let after_dce = eliminate_dead_lets(&after_const);
        if let LcnfExpr::Let { id, .. } = &after_dce {
            assert_eq!(*id, vid(2), "only x2 should remain");
        } else {
            panic!("expected a Let for x2");
        }
    }
    #[test]
    pub(super) fn test_copy_prop_then_dead_let() {
        let expr = mk_let(
            2,
            LcnfLetValue::FVar(vid(0)),
            mk_let(
                3,
                LcnfLetValue::App(LcnfArg::Var(vid(99)), vec![LcnfArg::Var(vid(2))]),
                LcnfExpr::Return(LcnfArg::Var(vid(3))),
            ),
        );
        let after_copy = propagate_copies(&expr);
        let after_dce = eliminate_dead_lets(&after_copy);
        if let LcnfExpr::Let { id, value, .. } = &after_dce {
            assert_eq!(*id, vid(3));
            if let LcnfLetValue::App(_, args) = value {
                assert!(matches!(args[0], LcnfArg::Var(v) if v == vid(0)));
            } else {
                panic!("expected App");
            }
        } else {
            panic!("expected Let for x3");
        }
    }
    #[test]
    pub(super) fn test_tail_call_not_affected_by_dce() {
        let expr = LcnfExpr::TailCall(
            LcnfArg::Var(vid(10)),
            vec![LcnfArg::Var(vid(0)), LcnfArg::Var(vid(1))],
        );
        let result = eliminate_dead_lets(&expr);
        assert!(matches!(result, LcnfExpr::TailCall(_, _)));
        let result2 = propagate_constants(&expr);
        assert!(matches!(result2, LcnfExpr::TailCall(_, _)));
    }
    #[test]
    pub(super) fn test_unreachable_preserved() {
        let expr = LcnfExpr::Unreachable;
        assert!(matches!(eliminate_dead_lets(&expr), LcnfExpr::Unreachable));
        assert!(matches!(propagate_constants(&expr), LcnfExpr::Unreachable));
        assert!(matches!(propagate_copies(&expr), LcnfExpr::Unreachable));
        assert!(matches!(fold_known_case(&expr), LcnfExpr::Unreachable));
        assert!(matches!(
            eliminate_unreachable_alts(&expr),
            LcnfExpr::Unreachable
        ));
    }
}
#[cfg(test)]
mod DCE_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = DCEPassConfig::new("test_pass", DCEPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = DCEPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = DCEPassRegistry::new();
        reg.register(DCEPassConfig::new("pass_a", DCEPassPhase::Analysis));
        reg.register(DCEPassConfig::new("pass_b", DCEPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = DCEAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = DCEWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = DCEDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = DCELivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(DCEConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(DCEConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(DCEConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            DCEConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(DCEConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = DCEDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
