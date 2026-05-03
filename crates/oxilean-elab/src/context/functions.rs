//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::elabcontext_type::ElabContext;
use super::types::{
    ContextCheckpoint, ContextDiff, ContextStats, ContextSummary, ContextValidation, DepthGuard,
    ElabContextBuilder, FVarIdGen, GoalPriority, GoalQueue, HypGroup, HypothesisIter,
    LetBindingIter, LocalContextStats, LocalEntry, LocalEntryBuilder, LocalKind, MetaDependency,
    PrioritizedGoal, RenameMap, TypeSnapshot,
};
use oxilean_kernel::{BinderInfo, Environment, Expr, FVarId, Level, Name};

/// Execute a closure with a temporarily pushed local variable.
pub fn with_local<'env, F, R>(ctx: &mut ElabContext<'env>, name: Name, ty: Expr, f: F) -> R
where
    F: FnOnce(&mut ElabContext<'env>, FVarId) -> R,
{
    let fvar = ctx.push_local(name, ty, None);
    let result = f(ctx, fvar);
    ctx.pop_local();
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::*;
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    #[test]
    fn test_push_pop_local() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let fvar = ctx.push_local(Name::str("x"), sort(), None);
        assert_eq!(ctx.local_count(), 1);
        assert_eq!(
            ctx.lookup_fvar(fvar)
                .expect("test operation should succeed")
                .name,
            Name::str("x")
        );
        ctx.pop_local();
        assert_eq!(ctx.local_count(), 0);
    }
    #[test]
    fn test_lookup_local() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_local(Name::str("x"), sort(), None);
        ctx.push_local(Name::str("y"), sort(), None);
        assert!(ctx.lookup_local(&Name::str("x")).is_some());
        assert!(ctx.lookup_local(&Name::str("y")).is_some());
        assert!(ctx.lookup_local(&Name::str("z")).is_none());
    }
    #[test]
    fn test_fresh_meta() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let (id1, _) = ctx.fresh_meta(sort());
        let (id2, _) = ctx.fresh_meta(sort());
        assert_ne!(id1, id2);
    }
    #[test]
    fn test_assign_meta() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let (id, _) = ctx.fresh_meta(sort());
        ctx.assign_meta(id, sort());
        assert_eq!(ctx.get_meta(id), Some(&sort()));
        assert!(ctx.is_meta_assigned(id));
    }
    #[test]
    fn test_univ_params() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_univ_param(Name::str("u"));
        assert!(ctx.is_univ_param(&Name::str("u")));
        assert!(!ctx.is_univ_param(&Name::str("v")));
        ctx.pop_univ_param();
        assert!(!ctx.is_univ_param(&Name::str("u")));
    }
    #[test]
    fn test_depth() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        assert_eq!(ctx.depth(), 0);
        ctx.push_depth();
        ctx.push_depth();
        assert_eq!(ctx.depth(), 2);
        ctx.pop_depth();
        assert_eq!(ctx.depth(), 1);
    }
    #[test]
    fn test_let_binding() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let fvar = ctx.push_let(Name::str("n"), sort(), sort());
        let entry = ctx
            .lookup_fvar(fvar)
            .expect("test operation should succeed");
        assert!(entry.is_let());
        assert!(entry.val.is_some());
    }
    #[test]
    fn test_expected_type() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        assert!(ctx.expected_type().is_none());
        ctx.push_expected_type(Some(sort()));
        assert_eq!(ctx.expected_type(), Some(&sort()));
        ctx.pop_expected_type();
        assert!(ctx.expected_type().is_none());
    }
    #[test]
    fn test_goals() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        assert!(!ctx.has_goals());
        ctx.add_goal(sort());
        assert!(ctx.has_goals());
        assert_eq!(ctx.goal_count(), 1);
        ctx.clear_goals();
        assert!(!ctx.has_goals());
    }
    #[test]
    fn test_hypotheses_filter() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("h"), sort());
        ctx.push_let(Name::str("n"), sort(), sort());
        let hyps = ctx.hypotheses();
        assert_eq!(hyps.len(), 1);
        assert_eq!(*hyps[0].0, Name::str("h"));
    }
    #[test]
    fn test_pop_to_depth() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("x"), sort());
        ctx.push_depth();
        ctx.push_hypothesis(Name::str("y"), sort());
        assert_eq!(ctx.local_count(), 2);
        ctx.pop_to_depth(1);
        assert_eq!(ctx.local_count(), 1);
    }
    #[test]
    fn test_with_local_helper() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut captured_fvar = None;
        let _ = with_local(&mut ctx, Name::str("z"), sort(), |_ctx, fvar| {
            captured_fvar = Some(fvar);
            42
        });
        assert_eq!(ctx.local_count(), 0);
        assert!(captured_fvar.is_some());
    }
}
/// Execute a closure in a scoped elaboration context that is restored afterward.
///
/// Useful for speculative elaboration: attempt something, then either keep or
/// discard the results.
pub fn with_scope<'env, F, R>(ctx: &mut ElabContext<'env>, f: F) -> (R, bool)
where
    F: FnOnce(&mut ElabContext<'env>) -> R,
{
    let snap = ctx.snapshot();
    let result = f(ctx);
    let clean = ctx.pending_goals.is_empty();
    if !clean {
        ctx.restore(snap);
    }
    (result, clean)
}
/// Temporarily set the expected type and run a closure.
pub fn with_expected_type<'env, F, R>(
    ctx: &mut ElabContext<'env>,
    ty: Option<oxilean_kernel::Expr>,
    f: F,
) -> R
where
    F: FnOnce(&mut ElabContext<'env>) -> R,
{
    ctx.push_expected_type(ty);
    let result = f(ctx);
    ctx.pop_expected_type();
    result
}
/// Describe the current state of an `ElabContext` as a debug string.
pub fn describe_context(ctx: &ElabContext<'_>) -> String {
    format!(
        "ElabContext {{ locals: {}, metas: {}/{} assigned, depth: {}, goals: {} }}",
        ctx.local_count(),
        ctx.assigned_meta_count(),
        ctx.meta_count(),
        ctx.depth(),
        ctx.goal_count(),
    )
}
#[cfg(test)]
mod context_extended_tests_1 {
    use super::*;
    use crate::context::*;
    fn sort() -> oxilean_kernel::Expr {
        oxilean_kernel::Expr::Sort(Level::zero())
    }
    #[test]
    fn test_snapshot_restore() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("x"), sort());
        let snap = ctx.snapshot();
        ctx.push_hypothesis(Name::str("y"), sort());
        assert_eq!(ctx.local_count(), 2);
        ctx.restore(snap);
        assert_eq!(ctx.local_count(), 1);
    }
    #[test]
    fn test_pop_goal() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.add_goal(sort());
        ctx.add_goal(sort());
        assert_eq!(ctx.goal_count(), 2);
        let g = ctx.pop_goal();
        assert!(g.is_some());
        assert_eq!(ctx.goal_count(), 1);
    }
    #[test]
    fn test_take_first_goal() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.add_goal(sort());
        let g = ctx.take_first_goal();
        assert!(g.is_some());
        assert!(ctx.pending_goals.is_empty());
    }
    #[test]
    fn test_hypothesis_count() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("h"), sort());
        ctx.push_let(Name::str("n"), sort(), sort());
        assert_eq!(ctx.hypothesis_count(), 1);
        assert_eq!(ctx.let_count(), 1);
    }
    #[test]
    fn test_is_in_scope() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let fvar = ctx.push_hypothesis(Name::str("x"), sort());
        assert!(ctx.is_in_scope(fvar));
        ctx.remove_fvar(fvar);
        assert!(!ctx.is_in_scope(fvar));
    }
    #[test]
    fn test_update_local_type() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let fvar = ctx.push_hypothesis(Name::str("x"), sort());
        let new_ty = oxilean_kernel::Expr::Const(Name::str("Nat"), vec![]);
        assert!(ctx.update_local_type(fvar, new_ty.clone()));
        assert_eq!(
            ctx.lookup_fvar(fvar)
                .expect("test operation should succeed")
                .ty,
            new_ty
        );
    }
    #[test]
    fn test_builder_allow_sorry() {
        let env = Environment::new();
        let ctx = ElabContextBuilder::new(&env).allow_sorry().build();
        assert!(ctx.options().allow_sorry);
    }
    #[test]
    fn test_describe_context() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        let desc = describe_context(&ctx);
        assert!(desc.contains("ElabContext"));
    }
    #[test]
    fn test_with_expected_type() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        with_expected_type(&mut ctx, Some(sort()), |c| {
            assert!(c.has_expected_type());
        });
        assert!(!ctx.has_expected_type());
    }
    #[test]
    fn test_let_bindings() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_let(Name::str("n"), sort(), sort());
        let lets = ctx.let_bindings();
        assert_eq!(lets.len(), 1);
    }
    #[test]
    fn test_is_clean() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        assert!(ctx.is_clean());
        ctx.add_goal(sort());
        assert!(!ctx.is_clean());
    }
    #[test]
    fn test_meta_assignments() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let (id, _) = ctx.fresh_meta(sort());
        ctx.assign_meta(id, sort());
        let assignments = ctx.meta_assignments();
        assert_eq!(assignments.len(), 1);
    }
    #[test]
    fn test_clear_metas() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let (id, _) = ctx.fresh_meta(sort());
        ctx.assign_meta(id, sort());
        ctx.clear_metas();
        assert_eq!(ctx.assigned_meta_count(), 0);
    }
}
/// Check whether an expression contains a given constant by name.
pub fn type_contains_const(expr: &oxilean_kernel::Expr, name: &Name) -> bool {
    match expr {
        oxilean_kernel::Expr::Const(n, _) => n == name,
        oxilean_kernel::Expr::App(f, a) => {
            type_contains_const(f, name) || type_contains_const(a, name)
        }
        oxilean_kernel::Expr::Pi(_, _, ty, body) | oxilean_kernel::Expr::Lam(_, _, ty, body) => {
            type_contains_const(ty, name) || type_contains_const(body, name)
        }
        oxilean_kernel::Expr::Let(_, ty, val, body) => {
            type_contains_const(ty, name)
                || type_contains_const(val, name)
                || type_contains_const(body, name)
        }
        _ => false,
    }
}
/// Merge the hypotheses of one context into another.
pub fn copy_hypotheses<'env>(src: &ElabContext<'_>, dst: &mut ElabContext<'env>) {
    for entry in src.locals() {
        if entry.is_hypothesis() {
            dst.push_hypothesis(entry.name.clone(), entry.ty.clone());
        }
    }
}
#[cfg(test)]
mod context_view_tests {
    use super::*;
    use crate::context::*;
    fn sort() -> oxilean_kernel::Expr {
        oxilean_kernel::Expr::Sort(Level::zero())
    }
    #[test]
    fn test_local_view() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("h"), sort());
        let view = ctx.local_view();
        assert_eq!(view.len(), 1);
        assert!(view.find(&Name::str("h")).is_some());
    }
    #[test]
    fn test_rename_local() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let fvar = ctx.push_hypothesis(Name::str("x"), sort());
        assert!(ctx.rename_local(fvar, Name::str("y")));
        assert!(ctx.lookup_local(&Name::str("y")).is_some());
        assert!(ctx.lookup_local(&Name::str("x")).is_none());
    }
    #[test]
    fn test_context_stats() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("h"), sort());
        ctx.add_goal(sort());
        let stats = ContextStats::compute(&ctx);
        assert_eq!(stats.local_count, 1);
        assert_eq!(stats.goal_count, 1);
    }
    #[test]
    fn test_meta_completion() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let (id, _) = ctx.fresh_meta(sort());
        let stats = ContextStats::compute(&ctx);
        assert!((stats.meta_completion() - 0.0).abs() < 1e-10);
        ctx.assign_meta(id, sort());
        let stats2 = ContextStats::compute(&ctx);
        assert!((stats2.meta_completion() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_locals_with_type_const() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let nat = oxilean_kernel::Expr::Const(Name::str("Nat"), vec![]);
        ctx.push_hypothesis(Name::str("n"), nat);
        ctx.push_hypothesis(Name::str("b"), sort());
        let found = ctx.locals_with_type_const(&Name::str("Nat"));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, Name::str("n"));
    }
    #[test]
    fn test_copy_hypotheses() {
        let env = Environment::new();
        let mut src = ElabContext::new(&env);
        src.push_hypothesis(Name::str("h"), sort());
        let mut dst = ElabContext::new(&env);
        copy_hypotheses(&src, &mut dst);
        assert_eq!(dst.local_count(), 1);
    }
}
/// A helper for building the type of a local context as a telescope.
///
/// A telescope is a sequence of Pi binders: `(x₁ : T₁) → (x₂ : T₂) → ... → body`.
#[allow(dead_code)]
pub fn build_telescope(locals: &[LocalEntry], body: oxilean_kernel::Expr) -> oxilean_kernel::Expr {
    use oxilean_kernel::{BinderInfo, Expr};
    let mut result = body;
    for entry in locals.iter().rev() {
        result = Expr::Pi(
            BinderInfo::Default,
            entry.name.clone(),
            Box::new(entry.ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Collect the free variables (FVarId) in all local types.
#[allow(dead_code)]
pub fn collect_free_vars_in_locals(locals: &[LocalEntry]) -> Vec<FVarId> {
    let mut result = Vec::new();
    for entry in locals {
        collect_fvars_in_expr(&entry.ty, &mut result);
        if let Some(val) = &entry.val {
            collect_fvars_in_expr(val, &mut result);
        }
    }
    result
}
fn collect_fvars_in_expr(expr: &oxilean_kernel::Expr, acc: &mut Vec<FVarId>) {
    match expr {
        Expr::FVar(id) if !acc.contains(id) => {
            acc.push(*id);
        }
        Expr::App(f, a) => {
            collect_fvars_in_expr(f, acc);
            collect_fvars_in_expr(a, acc);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars_in_expr(ty, acc);
            collect_fvars_in_expr(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_in_expr(ty, acc);
            collect_fvars_in_expr(val, acc);
            collect_fvars_in_expr(body, acc);
        }
        Expr::Proj(_, _, inner) => collect_fvars_in_expr(inner, acc),
        _ => {}
    }
}
/// Check whether a local entry is "used" (its FVar appears in a given expression).
#[allow(dead_code)]
pub fn is_local_used_in(entry: &LocalEntry, expr: &oxilean_kernel::Expr) -> bool {
    let mut fvars = Vec::new();
    collect_fvars_in_expr(expr, &mut fvars);
    fvars.contains(&entry.fvar)
}
/// Filter a list of locals to only those whose FVars appear in `expr`.
#[allow(dead_code)]
pub fn relevant_locals<'a>(
    locals: &'a [LocalEntry],
    expr: &oxilean_kernel::Expr,
) -> Vec<&'a LocalEntry> {
    let mut fvars = Vec::new();
    collect_fvars_in_expr(expr, &mut fvars);
    locals.iter().filter(|e| fvars.contains(&e.fvar)).collect()
}
#[cfg(test)]
mod extra_context_tests {
    use super::*;
    use crate::context::*;
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_checkpoint_take() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("h"), sort());
        let cp = ContextCheckpoint::take(&ctx);
        assert_eq!(cp.local_len, 1);
        assert_eq!(cp.depth, ctx.depth());
    }
    #[test]
    fn test_hyp_group_contains() {
        let fv1 = FVarId(1);
        let fv2 = FVarId(2);
        let group = HypGroup::new(Name::str("g"), vec![fv1, fv2]);
        assert!(group.contains(fv1));
        assert!(!group.contains(FVarId(3)));
    }
    #[test]
    fn test_hyp_group_len() {
        let group = HypGroup::new(Name::str("g"), vec![FVarId(0), FVarId(1), FVarId(2)]);
        assert_eq!(group.len(), 3);
        assert!(!group.is_empty());
    }
    #[test]
    fn test_context_summary_clean() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        let summary = ContextSummary::of(&ctx);
        assert!(summary.is_clean());
    }
    #[test]
    fn test_context_summary_has_goals() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.add_goal(sort());
        let summary = ContextSummary::of(&ctx);
        assert!(!summary.is_clean());
    }
    #[test]
    fn test_build_telescope_empty() {
        let body = sort();
        let result = build_telescope(&[], body.clone());
        assert_eq!(result, body);
    }
    #[test]
    fn test_build_telescope_one() {
        let entry = LocalEntry::hypothesis(FVarId(0), Name::str("x"), nat(), 0);
        let body = sort();
        let result = build_telescope(&[entry], body);
        assert!(matches!(result, Expr::Pi(_, _, _, _)));
    }
    #[test]
    fn test_collect_free_vars_none() {
        let entry = LocalEntry::hypothesis(FVarId(0), Name::str("x"), sort(), 0);
        let vars = collect_free_vars_in_locals(&[entry]);
        assert!(vars.is_empty());
    }
    #[test]
    fn test_collect_free_vars_some() {
        let fv = FVarId(42);
        let ty = Expr::FVar(fv);
        let entry = LocalEntry::hypothesis(FVarId(1), Name::str("x"), ty, 0);
        let vars = collect_free_vars_in_locals(&[entry]);
        assert!(vars.contains(&fv));
    }
    #[test]
    fn test_is_local_used_in_true() {
        let fv = FVarId(7);
        let entry = LocalEntry::hypothesis(fv, Name::str("x"), sort(), 0);
        let expr = Expr::FVar(fv);
        assert!(is_local_used_in(&entry, &expr));
    }
    #[test]
    fn test_is_local_used_in_false() {
        let entry = LocalEntry::hypothesis(FVarId(5), Name::str("x"), sort(), 0);
        let expr = sort();
        assert!(!is_local_used_in(&entry, &expr));
    }
    #[test]
    fn test_relevant_locals() {
        let fv = FVarId(3);
        let e1 = LocalEntry::hypothesis(fv, Name::str("x"), sort(), 0);
        let e2 = LocalEntry::hypothesis(FVarId(4), Name::str("y"), sort(), 0);
        let expr = Expr::FVar(fv);
        let locals = [e1, e2];
        let relevant = relevant_locals(&locals, &expr);
        assert_eq!(relevant.len(), 1);
        assert_eq!(relevant[0].name, Name::str("x"));
    }
    #[test]
    fn test_context_summary_fields() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("h1"), sort());
        ctx.push_let(Name::str("x"), nat(), sort());
        let summary = ContextSummary::of(&ctx);
        assert_eq!(summary.hypotheses.len(), 1);
        assert_eq!(summary.let_bindings.len(), 1);
    }
}
/// Filter local entries by kind.
#[allow(dead_code)]
pub fn filter_locals_by_kind<'a>(
    entries: &'a [LocalEntry],
    kind: &LocalKind,
) -> Vec<&'a LocalEntry> {
    entries.iter().filter(|e| &e.kind == kind).collect()
}
/// Partition locals into hypotheses and let-bindings.
#[allow(dead_code)]
pub fn partition_locals(entries: &[LocalEntry]) -> (Vec<&LocalEntry>, Vec<&LocalEntry>) {
    let hyps = entries
        .iter()
        .filter(|e| matches!(e.kind, LocalKind::Hypothesis))
        .collect();
    let lets = entries
        .iter()
        .filter(|e| matches!(e.kind, LocalKind::LetBinding))
        .collect();
    (hyps, lets)
}
/// Sort locals by depth (outermost first).
#[allow(dead_code)]
pub fn sort_locals_by_depth(entries: &mut Vec<LocalEntry>) {
    entries.sort_by_key(|e| e.depth);
}
/// Return locals in reverse depth order (innermost first).
#[allow(dead_code)]
pub fn locals_innermost_first(entries: &[LocalEntry]) -> Vec<&LocalEntry> {
    let mut refs: Vec<&LocalEntry> = entries.iter().collect();
    refs.sort_by_key(|b| std::cmp::Reverse(b.depth));
    refs
}
/// Find all locals at a specific depth.
#[allow(dead_code)]
pub fn locals_at_depth(entries: &[LocalEntry], depth: u32) -> Vec<&LocalEntry> {
    entries.iter().filter(|e| e.depth == depth).collect()
}
/// Find all locals introduced since a given depth.
#[allow(dead_code)]
pub fn locals_since_depth(entries: &[LocalEntry], min_depth: u32) -> Vec<&LocalEntry> {
    entries.iter().filter(|e| e.depth >= min_depth).collect()
}
/// Validate a list of local entries for basic well-formedness.
#[allow(dead_code)]
pub fn validate_locals(entries: &[LocalEntry]) -> ContextValidation {
    let mut result = ContextValidation::ok();
    let mut seen_fvars = std::collections::HashSet::new();
    for entry in entries {
        if !seen_fvars.insert(entry.fvar.0) {
            result.add_error(format!("Duplicate FVar ID {} in context", entry.fvar.0));
        }
    }
    let mut seen_names: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for entry in entries {
        let name_str = entry.name.to_string();
        let name_str_static: &str = unsafe { std::mem::transmute::<&str, &str>(name_str.as_str()) };
        *seen_names.entry(name_str_static).or_insert(0) += 1;
    }
    for (name, count) in &seen_names {
        if *count > 1 {
            result.add_warning(format!(
                "Name '{}' appears {} times in context",
                name, count
            ));
        }
    }
    result
}
/// Format a local entry as a hypothesis declaration string.
#[allow(dead_code)]
pub fn format_local_entry(entry: &LocalEntry) -> String {
    let kind_str = match entry.kind {
        LocalKind::Hypothesis => "hyp",
        LocalKind::LetBinding => "let",
        LocalKind::Have => "have",
        LocalKind::Auxiliary => "aux",
    };
    match &entry.val {
        Some(val) => {
            format!(
                "[{}] {} : {:?} := {:?}",
                kind_str, entry.name, entry.ty, val
            )
        }
        None => format!("[{}] {} : {:?}", kind_str, entry.name, entry.ty),
    }
}
/// Format all local entries as a context display.
#[allow(dead_code)]
pub fn format_context(entries: &[LocalEntry]) -> String {
    entries
        .iter()
        .map(format_local_entry)
        .collect::<Vec<_>>()
        .join("\n")
}
/// Generalize an expression by abstracting a free variable into a bound variable.
///
/// Replaces all occurrences of `fvar` in `body` with `BVar(0)`, producing
/// a lambda over the body.
#[allow(dead_code)]
pub fn abstract_fvar(
    fvar: FVarId,
    name: Name,
    ty: oxilean_kernel::Expr,
    body: &oxilean_kernel::Expr,
) -> oxilean_kernel::Expr {
    let abstracted = abstract_fvar_in_expr(fvar, body, 0);
    Expr::Lam(
        BinderInfo::Default,
        name,
        Box::new(ty),
        Box::new(abstracted),
    )
}
fn abstract_fvar_in_expr(
    fvar: FVarId,
    expr: &oxilean_kernel::Expr,
    depth: u32,
) -> oxilean_kernel::Expr {
    match expr {
        Expr::FVar(id) if *id == fvar => Expr::BVar(depth),
        Expr::App(f, a) => Expr::App(
            Box::new(abstract_fvar_in_expr(fvar, f, depth)),
            Box::new(abstract_fvar_in_expr(fvar, a, depth)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(abstract_fvar_in_expr(fvar, ty, depth)),
            Box::new(abstract_fvar_in_expr(fvar, body, depth + 1)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(abstract_fvar_in_expr(fvar, ty, depth)),
            Box::new(abstract_fvar_in_expr(fvar, body, depth + 1)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(abstract_fvar_in_expr(fvar, ty, depth)),
            Box::new(abstract_fvar_in_expr(fvar, val, depth)),
            Box::new(abstract_fvar_in_expr(fvar, body, depth + 1)),
        ),
        Expr::Proj(n, idx, inner) => Expr::Proj(
            n.clone(),
            *idx,
            Box::new(abstract_fvar_in_expr(fvar, inner, depth)),
        ),
        _ => expr.clone(),
    }
}
/// Apply a rename to all local entries in a slice.
#[allow(dead_code)]
pub fn apply_rename_to_locals(entries: &mut Vec<LocalEntry>, rename_map: &RenameMap) {
    for entry in entries.iter_mut() {
        if let Some(new_name) = rename_map.get(entry.fvar) {
            entry.name = new_name.clone();
        }
    }
}
/// Remove all locals at depth >= given threshold.
#[allow(dead_code)]
pub fn remove_locals_from_depth(entries: &mut Vec<LocalEntry>, min_depth: u32) {
    entries.retain(|e| e.depth < min_depth);
}
/// Count locals with a value (let-style).
#[allow(dead_code)]
pub fn count_valued_locals(entries: &[LocalEntry]) -> usize {
    entries.iter().filter(|e| e.val.is_some()).count()
}
/// Extract the names of all locals in order.
#[allow(dead_code)]
pub fn local_names(entries: &[LocalEntry]) -> Vec<&Name> {
    entries.iter().map(|e| &e.name).collect()
}
/// Extract the types of all locals in order.
#[allow(dead_code)]
pub fn local_types(entries: &[LocalEntry]) -> Vec<&oxilean_kernel::Expr> {
    entries.iter().map(|e| &e.ty).collect()
}
/// Merge two lists of local entries, deduplicating by FVarId.
///
/// Entries from `extra` that are not in `base` are appended.
#[allow(dead_code)]
pub fn merge_locals(base: &[LocalEntry], extra: &[LocalEntry]) -> Vec<LocalEntry> {
    let base_ids: std::collections::HashSet<u64> = base.iter().map(|e| e.fvar.0).collect();
    let mut result = base.to_vec();
    for entry in extra {
        if !base_ids.contains(&entry.fvar.0) {
            result.push(entry.clone());
        }
    }
    result
}
/// Remove duplicates from a local entry list (keeping the last occurrence).
#[allow(dead_code)]
pub fn dedup_locals(entries: Vec<LocalEntry>) -> Vec<LocalEntry> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for entry in entries.into_iter().rev() {
        if seen.insert(entry.fvar.0) {
            result.push(entry);
        }
    }
    result.reverse();
    result
}
#[cfg(test)]
mod context_extended_tests {
    use super::*;
    use crate::context::*;
    use oxilean_kernel::{Environment, Expr, FVarId, Level, Name};
    fn sort() -> Expr {
        Expr::Sort(Level::zero())
    }
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    #[test]
    fn test_type_snapshot_empty() {
        let snap = TypeSnapshot::new(0);
        assert!(snap.is_empty());
    }
    #[test]
    fn test_type_snapshot_record_lookup() {
        let mut snap = TypeSnapshot::new(1);
        let fv = FVarId(10);
        snap.record(fv, nat());
        assert_eq!(snap.len(), 1);
        assert_eq!(snap.lookup(fv), Some(&nat()));
    }
    #[test]
    fn test_type_snapshot_from_locals() {
        let fv = FVarId(0);
        let e = LocalEntry::hypothesis(fv, Name::str("x"), nat(), 1);
        let snap = TypeSnapshot::from_locals(&[e]);
        assert_eq!(snap.len(), 1);
        assert_eq!(snap.lookup(fv), Some(&nat()));
    }
    #[test]
    fn test_filter_locals_by_kind() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::let_binding(FVarId(1), Name::str("x"), nat(), sort(), 0),
        ];
        let hyps = filter_locals_by_kind(&entries, &LocalKind::Hypothesis);
        assert_eq!(hyps.len(), 1);
        let lets = filter_locals_by_kind(&entries, &LocalKind::LetBinding);
        assert_eq!(lets.len(), 1);
    }
    #[test]
    fn test_partition_locals() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::let_binding(FVarId(1), Name::str("x"), nat(), sort(), 0),
            LocalEntry::hypothesis(FVarId(2), Name::str("h2"), bool_ty(), 1),
        ];
        let (hyps, lets) = partition_locals(&entries);
        assert_eq!(hyps.len(), 2);
        assert_eq!(lets.len(), 1);
    }
    #[test]
    fn test_locals_at_depth() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("a"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("b"), nat(), 1),
            LocalEntry::hypothesis(FVarId(2), Name::str("c"), nat(), 1),
        ];
        let depth1 = locals_at_depth(&entries, 1);
        assert_eq!(depth1.len(), 2);
    }
    #[test]
    fn test_locals_since_depth() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("a"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("b"), nat(), 2),
            LocalEntry::hypothesis(FVarId(2), Name::str("c"), nat(), 3),
        ];
        let since2 = locals_since_depth(&entries, 2);
        assert_eq!(since2.len(), 2);
    }
    #[test]
    fn test_context_stats_empty() {
        let stats = LocalContextStats::of(&[]);
        assert!(stats.is_empty());
        assert_eq!(stats.let_fraction(), 0.0);
    }
    #[test]
    fn test_context_stats_mixed() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::let_binding(FVarId(1), Name::str("x"), nat(), sort(), 0),
            LocalEntry::let_binding(FVarId(2), Name::str("y"), nat(), sort(), 1),
        ];
        let stats = LocalContextStats::of(&entries);
        assert_eq!(stats.total_locals, 3);
        assert_eq!(stats.hypothesis_count, 1);
        assert_eq!(stats.let_binding_count, 2);
        assert_eq!(stats.max_depth, 1);
        assert!((stats.let_fraction() - 2.0 / 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_fvar_id_gen() {
        let mut gen = FVarIdGen::new();
        let a = gen.fresh();
        let b = gen.fresh();
        assert_ne!(a, b);
        assert_eq!(gen.peek_next(), 2);
        gen.reset();
        assert_eq!(gen.peek_next(), 0);
    }
    #[test]
    fn test_goal_queue_priority_order() {
        let mut q = GoalQueue::new();
        q.push(PrioritizedGoal::low(nat(), 0));
        q.push(PrioritizedGoal::high(sort(), 0));
        q.push(PrioritizedGoal::normal(bool_ty(), 0));
        let first = q.pop().expect("collection should not be empty");
        assert_eq!(first.priority, GoalPriority::High);
    }
    #[test]
    fn test_goal_queue_count_by_priority() {
        let mut q = GoalQueue::new();
        q.push(PrioritizedGoal::normal(nat(), 0));
        q.push(PrioritizedGoal::normal(nat(), 0));
        q.push(PrioritizedGoal::high(sort(), 0));
        assert_eq!(q.count_by_priority(GoalPriority::Normal), 2);
        assert_eq!(q.count_by_priority(GoalPriority::High), 1);
        assert_eq!(q.count_by_priority(GoalPriority::Low), 0);
    }
    #[test]
    fn test_goal_queue_label() {
        let g = PrioritizedGoal::normal(nat(), 0).with_label("main goal");
        assert_eq!(g.label, Some("main goal".to_string()));
    }
    #[test]
    fn test_context_validation_ok() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("g"), sort(), 0),
        ];
        let result = validate_locals(&entries);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }
    #[test]
    fn test_context_validation_duplicate_fvar() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(5), Name::str("h"), nat(), 0),
            LocalEntry::hypothesis(FVarId(5), Name::str("g"), sort(), 0),
        ];
        let result = validate_locals(&entries);
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }
    #[test]
    fn test_context_diff_empty() {
        let entries = vec![LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0)];
        let diff = ContextDiff::compute(&entries, &entries);
        assert!(diff.is_empty());
    }
    #[test]
    fn test_context_diff_added() {
        let old = vec![LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0)];
        let new = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("g"), sort(), 1),
        ];
        let diff = ContextDiff::compute(&old, &new);
        assert_eq!(diff.added_count(), 1);
        assert_eq!(diff.removed_count(), 0);
    }
    #[test]
    fn test_context_diff_removed() {
        let old = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("g"), sort(), 1),
        ];
        let new = vec![LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0)];
        let diff = ContextDiff::compute(&old, &new);
        assert_eq!(diff.added_count(), 0);
        assert_eq!(diff.removed_count(), 1);
    }
    #[test]
    fn test_rename_map_apply() {
        let mut rm = RenameMap::new();
        let fv = FVarId(3);
        rm.add(fv, Name::str("new_name"));
        let entry = LocalEntry::hypothesis(fv, Name::str("old_name"), nat(), 0);
        let renamed = rm.apply_to_entry(&entry);
        assert_eq!(renamed.name, Name::str("new_name"));
    }
    #[test]
    fn test_rename_map_merge() {
        let mut a = RenameMap::new();
        a.add(FVarId(0), Name::str("x"));
        let mut b = RenameMap::new();
        b.add(FVarId(1), Name::str("y"));
        a.merge(&b);
        assert_eq!(a.len(), 2);
    }
    #[test]
    fn test_local_entry_builder_hypothesis() {
        let entry = LocalEntryBuilder::hypothesis(FVarId(0), Name::str("h"), nat())
            .with_depth(3)
            .build();
        assert_eq!(entry.depth, 3);
        assert_eq!(entry.kind, LocalKind::Hypothesis);
        assert!(entry.val.is_none());
    }
    #[test]
    fn test_local_entry_builder_let() {
        let entry = LocalEntryBuilder::let_binding(FVarId(1), Name::str("x"), nat(), sort())
            .with_depth(1)
            .build();
        assert_eq!(entry.kind, LocalKind::LetBinding);
        assert!(entry.val.is_some());
    }
    #[test]
    fn test_hypothesis_iter() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::let_binding(FVarId(1), Name::str("x"), nat(), sort(), 0),
            LocalEntry::hypothesis(FVarId(2), Name::str("g"), sort(), 1),
        ];
        let hyps: Vec<_> = HypothesisIter::new(&entries).collect();
        assert_eq!(hyps.len(), 2);
    }
    #[test]
    fn test_let_binding_iter() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::let_binding(FVarId(1), Name::str("x"), nat(), sort(), 0),
            LocalEntry::let_binding(FVarId(2), Name::str("y"), nat(), sort(), 1),
        ];
        let lets: Vec<_> = LetBindingIter::new(&entries).collect();
        assert_eq!(lets.len(), 2);
    }
    #[test]
    fn test_abstract_fvar_simple() {
        let fv = FVarId(99);
        let body = Expr::FVar(fv);
        let result = abstract_fvar(fv, Name::str("x"), nat(), &body);
        assert!(matches!(result, Expr::Lam(_, _, _, _)));
        if let Expr::Lam(_, _, _, body_inner) = &result {
            assert_eq!(*body_inner.as_ref(), Expr::BVar(0));
        }
    }
    #[test]
    fn test_abstract_fvar_not_present() {
        let fv = FVarId(99);
        let body = Expr::Const(Name::str("Nat"), vec![]);
        let result = abstract_fvar(fv, Name::str("x"), nat(), &body);
        if let Expr::Lam(_, _, _, inner) = &result {
            assert_eq!(*inner.as_ref(), Expr::Const(Name::str("Nat"), vec![]));
        } else {
            panic!("Expected Lam");
        }
    }
    #[test]
    fn test_apply_rename_to_locals() {
        let mut entries = vec![LocalEntry::hypothesis(
            FVarId(0),
            Name::str("old"),
            nat(),
            0,
        )];
        let mut rm = RenameMap::new();
        rm.add(FVarId(0), Name::str("new"));
        apply_rename_to_locals(&mut entries, &rm);
        assert_eq!(entries[0].name, Name::str("new"));
    }
    #[test]
    fn test_remove_locals_from_depth() {
        let mut entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("a"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("b"), nat(), 2),
            LocalEntry::hypothesis(FVarId(2), Name::str("c"), nat(), 3),
        ];
        remove_locals_from_depth(&mut entries, 2);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, Name::str("a"));
    }
    #[test]
    fn test_count_valued_locals() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::let_binding(FVarId(1), Name::str("x"), nat(), sort(), 0),
        ];
        assert_eq!(count_valued_locals(&entries), 1);
    }
    #[test]
    fn test_local_names_and_types() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("g"), sort(), 0),
        ];
        let names = local_names(&entries);
        assert_eq!(names.len(), 2);
        let types = local_types(&entries);
        assert_eq!(types.len(), 2);
    }
    #[test]
    fn test_merge_locals_no_duplicates() {
        let base = vec![LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0)];
        let extra = vec![LocalEntry::hypothesis(FVarId(1), Name::str("g"), sort(), 0)];
        let merged = merge_locals(&base, &extra);
        assert_eq!(merged.len(), 2);
    }
    #[test]
    fn test_merge_locals_with_overlap() {
        let base = vec![LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0)];
        let extra = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("g"), sort(), 0),
        ];
        let merged = merge_locals(&base, &extra);
        assert_eq!(merged.len(), 2);
    }
    #[test]
    fn test_dedup_locals() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::hypothesis(FVarId(0), Name::str("h2"), sort(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("g"), sort(), 0),
        ];
        let deduped = dedup_locals(entries);
        assert_eq!(deduped.len(), 2);
    }
    #[test]
    fn test_depth_guard() {
        let guard = DepthGuard::save(42);
        assert_eq!(guard.depth(), 42);
    }
    #[test]
    fn test_meta_dependency() {
        let dep = MetaDependency::new(0, vec![1, 2]);
        assert!(!dep.is_independent());
        assert!(dep.depends_on_id(1));
        assert!(!dep.depends_on_id(3));
        let indep = MetaDependency::new(3, vec![]);
        assert!(indep.is_independent());
    }
    #[test]
    fn test_format_context() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("h"), nat(), 0),
            LocalEntry::let_binding(FVarId(1), Name::str("x"), nat(), sort(), 0),
        ];
        let s = format_context(&entries);
        assert!(s.contains("hyp"));
        assert!(s.contains("let"));
    }
    #[test]
    fn test_elab_context_new() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        assert_eq!(ctx.depth(), 0);
        assert_eq!(ctx.goal_count(), 0);
    }
    #[test]
    fn test_elab_context_push_pop_hypothesis() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        ctx.push_hypothesis(Name::str("h"), nat());
        assert_eq!(ctx.depth(), 1);
        ctx.pop_local();
        assert_eq!(ctx.depth(), 0);
    }
    #[test]
    fn test_innermost_first() {
        let entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("a"), nat(), 0),
            LocalEntry::hypothesis(FVarId(1), Name::str("b"), nat(), 3),
            LocalEntry::hypothesis(FVarId(2), Name::str("c"), nat(), 1),
        ];
        let ordered = locals_innermost_first(&entries);
        assert_eq!(ordered[0].depth, 3);
        assert_eq!(ordered[1].depth, 1);
        assert_eq!(ordered[2].depth, 0);
    }
    #[test]
    fn test_sort_locals_by_depth() {
        let mut entries = vec![
            LocalEntry::hypothesis(FVarId(0), Name::str("c"), nat(), 3),
            LocalEntry::hypothesis(FVarId(1), Name::str("a"), nat(), 0),
            LocalEntry::hypothesis(FVarId(2), Name::str("b"), nat(), 1),
        ];
        sort_locals_by_depth(&mut entries);
        assert_eq!(entries[0].depth, 0);
        assert_eq!(entries[1].depth, 1);
        assert_eq!(entries[2].depth, 3);
    }
}
