//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use crate::metavar::MetaVarContext;
use oxilean_kernel::{alpha_equiv, const_name, expr_head, BinderInfo, Expr, FVarId, Name};

use super::types::{
    ArgClass, AutoImplicitScope, CheckDirection, ImplicitArg, ImplicitArgSummary, ImplicitCache,
    ImplicitElabResult, ImplicitError, ImplicitGuard, ImplicitInsertResult, ImplicitInsertionStats,
    ImplicitMode, ImplicitPipelineConfig, ImplicitPipelineResult, ImplicitPositionIndex,
    ImplicitScopeStack, ImplicitStats, InsertedImplicit, InstanceTable, PendingImplicit,
    PendingImplicitQueue,
};

/// Collect all leading implicit / instance arguments from a Pi-type,
/// returning them in order together with the remaining (non-implicit) type.
///
/// `max_depth` caps how many implicit arguments are stripped; use `usize::MAX`
/// for unbounded traversal.
pub fn collect_implicit_prefix(ty: &Expr, max_depth: usize) -> (Vec<ImplicitArg>, Expr) {
    let mut args = Vec::new();
    let mut current = ty.clone();
    for _ in 0..max_depth {
        match &current {
            Expr::Pi(bi, name, dom, cod) => {
                let is_implicit = matches!(bi, BinderInfo::Implicit | BinderInfo::InstImplicit);
                if !is_implicit {
                    break;
                }
                let is_instance = matches!(bi, BinderInfo::InstImplicit);
                args.push(ImplicitArg {
                    name: name.to_string(),
                    ty: (**dom).clone(),
                    is_instance,
                });
                current = (**cod).clone();
            }
            _ => break,
        }
    }
    (args, current)
}
/// Count the number of leading implicit arguments in a Pi-type.
pub fn count_implicit_prefix(ty: &Expr) -> usize {
    let (args, _) = collect_implicit_prefix(ty, usize::MAX);
    args.len()
}
/// Check whether a Pi-type begins with at least one implicit argument.
pub fn has_implicit_prefix(ty: &Expr) -> bool {
    matches!(
        ty,
        Expr::Pi(BinderInfo::Implicit | BinderInfo::InstImplicit, ..)
    )
}
/// Count total number of explicit arguments in a Pi-type (ignoring implicits).
pub fn count_explicit_args(ty: &Expr) -> usize {
    let mut count = 0;
    let mut current = ty;
    while let Expr::Pi(bi, _, _, cod) = current {
        if matches!(bi, BinderInfo::Default | BinderInfo::StrictImplicit) {
            count += 1;
        }
        current = cod;
    }
    count
}
/// Resolve implicit arguments for a function application.
///
/// Walks the head type `fun_type` and, for every leading implicit/instance Pi,
/// creates a fresh metavariable and instantiates the codomain with it.
///
/// Returns the list of implicit argument expressions (metavariable proxies).
pub fn resolve_implicits(
    _ctx: &mut ElabContext,
    metas: &mut MetaVarContext,
    fun_type: &Expr,
) -> Vec<Expr> {
    let mut implicits = Vec::new();
    let mut ty = fun_type.clone();
    while let Expr::Pi(bi, _name, dom, cod) = &ty {
        if matches!(bi, BinderInfo::Implicit | BinderInfo::InstImplicit) {
            let meta_id = metas.fresh((**dom).clone());
            let meta_expr = Expr::FVar(FVarId(1_000_000 + meta_id));
            implicits.push(meta_expr.clone());
            ty = oxilean_kernel::instantiate(cod, &meta_expr);
        } else {
            break;
        }
    }
    implicits
}
/// Insert implicit arguments for an application node `f : fun_type` applied
/// to explicit argument `arg`.
///
/// Returns `(applied, remaining_type)` where `applied` is the expression `f`
/// with all leading implicits filled in, ready to accept `arg`.
pub fn insert_implicits_for_app(
    ctx: &mut ElabContext,
    metas: &mut MetaVarContext,
    f: Expr,
    fun_type: &Expr,
) -> (Expr, Expr) {
    let implicit_vals = resolve_implicits(ctx, metas, fun_type);
    let mut expr = f;
    let mut ty = fun_type.clone();
    for val in implicit_vals {
        if let Expr::Pi(_, _, _, cod) = &ty {
            ty = oxilean_kernel::instantiate(cod, &val);
        }
        expr = Expr::App(Box::new(expr), Box::new(val));
    }
    (expr, ty)
}
/// Determine whether a name should be treated as an auto-implicit variable.
///
/// In Lean 4, single-letter Greek or Latin identifiers that begin with a
/// lowercase letter and are not in scope are auto-bound.  This is a simplified
/// heuristic used during elaboration.
pub fn is_auto_implicit_candidate(name: &str) -> bool {
    let mut chars = name.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => c.is_alphabetic() && c.is_lowercase(),
        _ => false,
    }
}
/// Collect the set of free variable *names* that appear in `expr` and are
/// plausible auto-implicit candidates (single lowercase letter).
pub fn collect_auto_implicit_candidates(expr: &Expr) -> Vec<String> {
    let mut found = Vec::new();
    collect_candidates_rec(expr, &mut found);
    found.sort();
    found.dedup();
    found
}
fn collect_candidates_rec(expr: &Expr, out: &mut Vec<String>) {
    match expr {
        Expr::Const(name, _) => {
            let s = name.to_string();
            if is_auto_implicit_candidate(&s) {
                out.push(s);
            }
        }
        Expr::App(f, a) => {
            collect_candidates_rec(f, out);
            collect_candidates_rec(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_candidates_rec(ty, out);
            collect_candidates_rec(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_candidates_rec(ty, out);
            collect_candidates_rec(val, out);
            collect_candidates_rec(body, out);
        }
        _ => {}
    }
}
/// Try to infer an implicit argument from local context.
///
/// Searches the local context for a variable whose declared type is
/// alpha-equivalent to `ty`.  If exactly one such variable is found, it is
/// returned as `Expr::FVar(fvar_id)`.  If zero or more than one variable
/// matches the type is considered ambiguous and `None` is returned.
pub fn infer_implicit(ctx: &ElabContext, ty: &Expr) -> Option<Expr> {
    let matches: Vec<_> = ctx
        .locals()
        .iter()
        .filter(|entry| alpha_equiv(&entry.ty, ty))
        .collect();
    if matches.len() == 1 {
        Some(Expr::FVar(matches[0].fvar))
    } else {
        None
    }
}
/// Try to resolve a type-class instance.
///
/// Searches for an instance of the class denoted by the head constant of
/// `class_ty`.  Resolution proceeds in two phases:
///
/// 1. **Local search** — scan the local context for a hypothesis whose type
///    shares the same head class constant.  This handles instance arguments
///    that were introduced as local hypotheses (e.g. `[inst : Add α]`).
/// 2. **Global search** — scan the kernel environment for a constant whose
///    type shares the same head class constant.  This handles globally
///    registered instances (e.g. `instAddNat : Add Nat`).
///
/// Returns the first match found, encoded as `Expr::FVar` (for locals) or
/// `Expr::Const` (for globals), or `None` if no instance is available.
pub fn resolve_instance(ctx: &ElabContext, class_ty: &Expr) -> Option<Expr> {
    let class_name = match expr_head(class_ty) {
        Expr::Const(name, _) => name,
        _ => return None,
    };
    for entry in ctx.locals() {
        if let Some(head) = const_name(expr_head(&entry.ty)) {
            if head == class_name {
                return Some(Expr::FVar(entry.fvar));
            }
        }
    }
    for (name, ci) in ctx.env().constant_infos() {
        let ci_ty = ci.ty();
        if let Some(head) = const_name(expr_head(ci_ty)) {
            if head == class_name {
                return Some(Expr::Const(name.clone(), vec![]));
            }
        }
    }
    None
}
/// Resolve implicits and record which ones were inserted.
pub fn resolve_implicits_tracked(
    _ctx: &mut ElabContext,
    metas: &mut MetaVarContext,
    fun_type: &Expr,
) -> (Vec<Expr>, Vec<InsertedImplicit>) {
    let mut implicit_exprs = Vec::new();
    let mut records = Vec::new();
    let mut ty = fun_type.clone();
    while let Expr::Pi(bi, name, dom, cod) = &ty {
        if let Some(mode) = ImplicitMode::from_binder(bi) {
            let meta_id = metas.fresh((**dom).clone());
            let meta_expr = Expr::FVar(FVarId(1_000_000 + meta_id));
            implicit_exprs.push(meta_expr.clone());
            records.push(InsertedImplicit::new(name.to_string(), meta_id, mode));
            ty = oxilean_kernel::instantiate(cod, &meta_expr);
        } else {
            break;
        }
    }
    (implicit_exprs, records)
}
/// Collect the names of all binders in a Pi-type, in order.
pub fn binder_names(ty: &Expr) -> Vec<Name> {
    let mut names = Vec::new();
    let mut current = ty;
    while let Expr::Pi(_, name, _, cod) = current {
        names.push(name.clone());
        current = cod;
    }
    names
}
/// Strip the first `n` explicit binders from a Pi-type.
///
/// Implicit binders that precede each explicit one are also stripped.
/// Returns the remaining type.
pub fn strip_n_explicit(ty: &Expr, n: usize) -> Expr {
    let mut current = ty.clone();
    let mut stripped = 0;
    while stripped < n {
        match current {
            Expr::Pi(bi, _, _, cod) => {
                if matches!(bi, BinderInfo::Default | BinderInfo::StrictImplicit) {
                    stripped += 1;
                }
                current = *cod;
            }
            other => return other,
        }
    }
    current
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::implicit::*;
    use oxilean_kernel::{BinderInfo, Environment, Level, Name};
    fn make_pi(bi: BinderInfo, name: &str, dom: Expr, cod: Expr) -> Expr {
        Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(cod))
    }
    fn type0() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    #[test]
    fn test_resolve_implicits_none() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut metas = MetaVarContext::new();
        let ty = Expr::Sort(Level::zero());
        let implicits = resolve_implicits(&mut ctx, &mut metas, &ty);
        assert_eq!(implicits.len(), 0);
    }
    #[test]
    fn test_resolve_implicits_one() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut metas = MetaVarContext::new();
        let pi = make_pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            make_pi(BinderInfo::Default, "x", Expr::BVar(0), Expr::BVar(1)),
        );
        let implicits = resolve_implicits(&mut ctx, &mut metas, &pi);
        assert_eq!(implicits.len(), 1);
    }
    #[test]
    fn test_collect_implicit_prefix() {
        let pi = make_pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            make_pi(
                BinderInfo::InstImplicit,
                "_",
                type0(),
                make_pi(BinderInfo::Default, "x", type0(), type0()),
            ),
        );
        let (args, _rest) = collect_implicit_prefix(&pi, usize::MAX);
        assert_eq!(args.len(), 2);
        assert!(!args[0].is_instance);
        assert!(args[1].is_instance);
    }
    #[test]
    fn test_count_implicit_prefix() {
        let pi = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(BinderInfo::Default, "x", type0(), type0()),
        );
        assert_eq!(count_implicit_prefix(&pi), 1);
    }
    #[test]
    fn test_has_implicit_prefix() {
        let implicit_pi = make_pi(BinderInfo::Implicit, "a", type0(), type0());
        let explicit_pi = make_pi(BinderInfo::Default, "a", type0(), type0());
        assert!(has_implicit_prefix(&implicit_pi));
        assert!(!has_implicit_prefix(&explicit_pi));
    }
    #[test]
    fn test_count_explicit_args() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            make_pi(
                BinderInfo::Default,
                "x",
                type0(),
                make_pi(BinderInfo::Default, "y", type0(), type0()),
            ),
        );
        assert_eq!(count_explicit_args(&ty), 2);
    }
    #[test]
    fn test_is_auto_implicit_candidate() {
        assert!(is_auto_implicit_candidate("α"));
        assert!(is_auto_implicit_candidate("x"));
        assert!(!is_auto_implicit_candidate("foo"));
        assert!(!is_auto_implicit_candidate("X"));
    }
    #[test]
    fn test_implicit_mode_from_binder() {
        assert_eq!(
            ImplicitMode::from_binder(&BinderInfo::Implicit),
            Some(ImplicitMode::Unification)
        );
        assert_eq!(
            ImplicitMode::from_binder(&BinderInfo::InstImplicit),
            Some(ImplicitMode::TypeClass)
        );
        assert_eq!(ImplicitMode::from_binder(&BinderInfo::Default), None);
    }
    #[test]
    fn test_implicit_mode_needs() {
        assert!(ImplicitMode::Unification.needs_unification());
        assert!(!ImplicitMode::Unification.needs_synthesis());
        assert!(ImplicitMode::TypeClass.needs_synthesis());
        assert!(!ImplicitMode::TypeClass.needs_unification());
    }
    #[test]
    fn test_resolve_implicits_tracked() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut metas = MetaVarContext::new();
        let pi = make_pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            make_pi(BinderInfo::Default, "x", Expr::BVar(0), Expr::BVar(1)),
        );
        let (exprs, records) = resolve_implicits_tracked(&mut ctx, &mut metas, &pi);
        assert_eq!(exprs.len(), 1);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].mode, ImplicitMode::Unification);
    }
    #[test]
    fn test_strip_n_explicit() {
        let ty = make_pi(
            BinderInfo::Default,
            "x",
            type0(),
            make_pi(BinderInfo::Default, "y", type0(), type0()),
        );
        let rest = strip_n_explicit(&ty, 1);
        assert!(matches!(rest, Expr::Pi(BinderInfo::Default, _, _, _)));
        let rest2 = strip_n_explicit(&ty, 2);
        assert!(matches!(rest2, Expr::Sort(_)));
    }
    #[test]
    fn test_binder_names() {
        let ty = make_pi(
            BinderInfo::Default,
            "a",
            type0(),
            make_pi(BinderInfo::Default, "b", type0(), type0()),
        );
        let names = binder_names(&ty);
        assert_eq!(names.len(), 2);
        assert_eq!(names[0].to_string(), "a");
        assert_eq!(names[1].to_string(), "b");
    }
    #[test]
    fn test_infer_implicit_returns_none() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        assert!(infer_implicit(&ctx, &type0()).is_none());
    }
    #[test]
    fn test_resolve_instance_returns_none() {
        let env = Environment::new();
        let ctx = ElabContext::new(&env);
        assert!(resolve_instance(&ctx, &type0()).is_none());
    }
}
/// Full implicit-insertion pipeline.
///
/// Given `f : fun_type`, inserts all leading implicit arguments and returns
/// the augmented expression with insertion records.
pub fn insert_all_implicits(
    ctx: &mut ElabContext,
    metas: &mut MetaVarContext,
    f: Expr,
    fun_type: &Expr,
) -> ImplicitInsertResult {
    let (implicit_exprs, inserted) = resolve_implicits_tracked(ctx, metas, fun_type);
    let mut expr = f;
    let mut ty = fun_type.clone();
    for val in &implicit_exprs {
        if let Expr::Pi(_, _, _, cod) = &ty {
            ty = oxilean_kernel::instantiate(cod, val);
        }
        expr = Expr::App(Box::new(expr), Box::new(val.clone()));
    }
    ImplicitInsertResult {
        expr,
        remaining_ty: ty,
        inserted,
    }
}
/// Produce a sequence of `_`-like hole expressions for all implicit arguments
/// of a Pi-type.  These are represented as metavariables in the given context.
pub fn generate_implicit_holes(metas: &mut MetaVarContext, fun_type: &Expr) -> Vec<Expr> {
    let (args, _) = collect_implicit_prefix(fun_type, usize::MAX);
    args.into_iter()
        .map(|arg| {
            let id = metas.fresh(arg.ty.clone());
            Expr::FVar(FVarId(1_000_000 + id))
        })
        .collect()
}
/// Return `true` if a `BinderInfo` represents any form of implicit.
pub fn is_implicit_bi(bi: &BinderInfo) -> bool {
    matches!(
        bi,
        BinderInfo::Implicit | BinderInfo::InstImplicit | BinderInfo::StrictImplicit
    )
}
/// Return `true` if a `BinderInfo` is the strict `⦃…⦄` form.
pub fn is_strict_implicit_bi(bi: &BinderInfo) -> bool {
    matches!(bi, BinderInfo::StrictImplicit)
}
/// Return `true` if a `BinderInfo` is the instance `[…]` form.
pub fn is_inst_implicit_bi(bi: &BinderInfo) -> bool {
    matches!(bi, BinderInfo::InstImplicit)
}
/// Walk a spine application and count how many arguments were likely implicit
/// (heuristic: `FVar` in the meta-variable range `≥ 1_000_000`).
pub fn count_likely_implicit_args(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => {
            let is_meta_like = matches!(
                a.as_ref(), Expr::FVar(FVarId(id)) if * id >= 1_000_000
            );
            let n = if is_meta_like { 1 } else { 0 };
            n + count_likely_implicit_args(f)
        }
        _ => 0,
    }
}
#[cfg(test)]
mod implicit_extra_tests {
    use super::*;
    use crate::implicit::*;
    use oxilean_kernel::{BinderInfo, Environment, Level};
    fn type0() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn make_pi(bi: BinderInfo, name: &str, dom: Expr, cod: Expr) -> Expr {
        Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(cod))
    }
    #[test]
    fn test_insert_all_implicits() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut metas = MetaVarContext::new();
        let f = Expr::Const(Name::str("f"), vec![]);
        let pi = make_pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            make_pi(BinderInfo::Default, "x", Expr::BVar(0), Expr::BVar(1)),
        );
        let result = insert_all_implicits(&mut ctx, &mut metas, f, &pi);
        assert_eq!(result.inserted.len(), 1);
        assert!(matches!(result.expr, Expr::App(_, _)));
    }
    #[test]
    fn test_generate_implicit_holes() {
        let mut metas = MetaVarContext::new();
        let pi = make_pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            make_pi(BinderInfo::Implicit, "β", type0(), type0()),
        );
        let holes = generate_implicit_holes(&mut metas, &pi);
        assert_eq!(holes.len(), 2);
    }
    #[test]
    fn test_is_implicit_bi() {
        assert!(is_implicit_bi(&BinderInfo::Implicit));
        assert!(is_implicit_bi(&BinderInfo::InstImplicit));
        assert!(is_implicit_bi(&BinderInfo::StrictImplicit));
        assert!(!is_implicit_bi(&BinderInfo::Default));
    }
    #[test]
    fn test_is_strict_implicit() {
        assert!(is_strict_implicit_bi(&BinderInfo::StrictImplicit));
        assert!(!is_strict_implicit_bi(&BinderInfo::Implicit));
    }
    #[test]
    fn test_is_inst_implicit() {
        assert!(is_inst_implicit_bi(&BinderInfo::InstImplicit));
        assert!(!is_inst_implicit_bi(&BinderInfo::Implicit));
    }
    #[test]
    fn test_count_likely_implicit_args() {
        let inner = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::FVar(FVarId(1_000_001))),
        );
        let outer = Expr::App(
            Box::new(inner),
            Box::new(Expr::Const(Name::str("x"), vec![])),
        );
        assert_eq!(count_likely_implicit_args(&outer), 1);
    }
    #[test]
    fn test_instance_table_register_lookup() {
        let mut table = InstanceTable::new();
        table.register("Ord", Expr::Const(Name::str("Nat.Ord"), vec![]));
        table.register("Ord", Expr::Const(Name::str("Int.Ord"), vec![]));
        assert_eq!(table.lookup("Ord").len(), 2);
        assert!(table.has_instances("Ord"));
        assert!(!table.has_instances("Eq"));
        assert_eq!(table.total(), 2);
    }
    #[test]
    fn test_implicit_insert_result_fields() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut metas = MetaVarContext::new();
        let f = Expr::Const(Name::str("g"), vec![]);
        let ty = type0();
        let result = insert_all_implicits(&mut ctx, &mut metas, f.clone(), &ty);
        assert_eq!(result.inserted.len(), 0);
        assert_eq!(result.expr, f);
    }
}
/// Return `true` if `fun_type` has any trailing (non-leading) implicit
/// arguments after the first explicit binder.
#[allow(dead_code)]
pub fn has_trailing_implicits(fun_type: &Expr) -> bool {
    let mut current = fun_type;
    let mut seen_explicit = false;
    loop {
        match current {
            Expr::Pi(bi, _, _, cod) => {
                let is_impl = matches!(bi, BinderInfo::Implicit | BinderInfo::InstImplicit);
                if !is_impl {
                    seen_explicit = true;
                } else if seen_explicit {
                    return true;
                }
                current = cod;
            }
            _ => return false,
        }
    }
}
#[cfg(test)]
mod implicit_final_tests {
    use super::*;
    use crate::implicit::*;
    use oxilean_kernel::{BinderInfo, Level};
    fn type0() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    #[test]
    fn test_has_trailing_implicits_false() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(type0()),
            Box::new(type0()),
        );
        assert!(!has_trailing_implicits(&pi));
    }
    #[test]
    fn test_has_trailing_implicits_true() {
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(type0()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("y"),
                Box::new(type0()),
                Box::new(type0()),
            )),
        );
        assert!(has_trailing_implicits(&pi));
    }
}
/// Classify all binders in a Pi-type.
#[allow(dead_code)]
pub fn classify_binders(ty: &Expr) -> Vec<(String, ArgClass)> {
    let mut result = Vec::new();
    let mut current = ty;
    while let Expr::Pi(bi, name, _, cod) = current {
        result.push((name.to_string(), ArgClass::from_binder(bi)));
        current = cod;
    }
    result
}
/// Count explicit vs implicit binders in a Pi-type.
#[allow(dead_code)]
pub fn binder_counts(ty: &Expr) -> (usize, usize) {
    let classes = classify_binders(ty);
    let explicit = classes.iter().filter(|(_, c)| !c.is_implicit()).count();
    let implicit = classes.len() - explicit;
    (explicit, implicit)
}
#[cfg(test)]
mod extra_implicit_tests {
    use super::*;
    use crate::implicit::*;
    use oxilean_kernel::{BinderInfo, Level};
    fn type0() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn make_pi(bi: BinderInfo, name: &str, dom: Expr, cod: Expr) -> Expr {
        Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(cod))
    }
    #[test]
    fn test_pending_implicit_queue_basic() {
        let mut q = PendingImplicitQueue::new();
        assert!(q.is_empty());
        q.push(PendingImplicit::new(0, type0(), ImplicitMode::Unification));
        assert_eq!(q.len(), 1);
        assert!(q.pop().is_some());
        assert!(q.is_empty());
    }
    #[test]
    fn test_pending_implicit_user_provided() {
        let p = PendingImplicit::new(0, type0(), ImplicitMode::TypeClass).mark_user_provided();
        assert!(p.user_provided);
    }
    #[test]
    fn test_pending_implicit_queue_tc_count() {
        let mut q = PendingImplicitQueue::new();
        q.push(PendingImplicit::new(0, type0(), ImplicitMode::TypeClass));
        q.push(PendingImplicit::new(1, type0(), ImplicitMode::Unification));
        q.push(PendingImplicit::new(2, type0(), ImplicitMode::TypeClass).mark_user_provided());
        assert_eq!(q.tc_pending_count(), 1);
        assert_eq!(q.user_provided_count(), 1);
    }
    #[test]
    fn test_arg_class_from_binder() {
        assert_eq!(
            ArgClass::from_binder(&BinderInfo::Default),
            ArgClass::Explicit
        );
        assert_eq!(
            ArgClass::from_binder(&BinderInfo::Implicit),
            ArgClass::Implicit
        );
        assert_eq!(
            ArgClass::from_binder(&BinderInfo::InstImplicit),
            ArgClass::Instance
        );
        assert_eq!(
            ArgClass::from_binder(&BinderInfo::StrictImplicit),
            ArgClass::Strict
        );
    }
    #[test]
    fn test_arg_class_is_implicit() {
        assert!(!ArgClass::Explicit.is_implicit());
        assert!(ArgClass::Implicit.is_implicit());
        assert!(ArgClass::Instance.is_implicit());
        assert!(ArgClass::Strict.is_implicit());
    }
    #[test]
    fn test_classify_binders() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            make_pi(BinderInfo::Default, "x", type0(), type0()),
        );
        let classes = classify_binders(&ty);
        assert_eq!(classes.len(), 2);
        assert_eq!(classes[0].1, ArgClass::Implicit);
        assert_eq!(classes[1].1, ArgClass::Explicit);
    }
    #[test]
    fn test_binder_counts() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "α",
            type0(),
            make_pi(
                BinderInfo::Default,
                "x",
                type0(),
                make_pi(BinderInfo::Default, "y", type0(), type0()),
            ),
        );
        let (explicit, implicit) = binder_counts(&ty);
        assert_eq!(explicit, 2);
        assert_eq!(implicit, 1);
    }
    #[test]
    fn test_auto_implicit_scope() {
        let mut scope = AutoImplicitScope::new();
        assert!(scope.is_empty());
        scope.bind("α");
        scope.bind("β");
        scope.bind("α");
        assert_eq!(scope.len(), 2);
        assert!(scope.is_bound("α"));
        assert!(!scope.is_bound("γ"));
    }
    #[test]
    fn test_auto_implicit_scope_names() {
        let mut scope = AutoImplicitScope::new();
        scope.bind("x");
        scope.bind("y");
        let names = scope.names();
        assert_eq!(names, &["x".to_string(), "y".to_string()]);
    }
    #[test]
    fn test_pending_implicit_queue_clear() {
        let mut q = PendingImplicitQueue::new();
        q.push(PendingImplicit::new(0, type0(), ImplicitMode::Unification));
        q.clear();
        assert!(q.is_empty());
    }
    #[test]
    fn test_pending_implicit_peek() {
        let mut q = PendingImplicitQueue::new();
        q.push(PendingImplicit::new(5, type0(), ImplicitMode::Strict));
        let p = q.peek().expect("test operation should succeed");
        assert_eq!(p.arg_index, 5);
    }
    #[test]
    fn test_instance_table_class_names() {
        let mut table = InstanceTable::new();
        table.register("Ord", Expr::Const(Name::str("Nat.Ord"), vec![]));
        table.register("Eq", Expr::Const(Name::str("Nat.Eq"), vec![]));
        let names: Vec<_> = table.class_names().collect();
        assert_eq!(names.len(), 2);
    }
}
/// Run the full implicit argument insertion pipeline.
#[allow(dead_code)]
pub fn run_implicit_pipeline(
    ctx: &mut ElabContext,
    metas: &mut MetaVarContext,
    f: Expr,
    fun_type: &Expr,
    cfg: &ImplicitPipelineConfig,
    stats: &mut ImplicitStats,
    _cache: &mut ImplicitCache,
) -> ImplicitPipelineResult {
    let mut expr = f;
    let mut ty = fun_type.clone();
    let mut inserted = Vec::new();
    let mut count = 0;
    while count < cfg.max_implicit_args {
        match &ty.clone() {
            Expr::Pi(bi, name, dom, cod) => {
                let mode_opt = ImplicitMode::from_binder(bi);
                let mode = match mode_opt {
                    Some(m) => m,
                    None => break,
                };
                if mode == ImplicitMode::Strict && !cfg.insert_strict {
                    break;
                }
                if mode == ImplicitMode::TypeClass && !cfg.enable_tc_synthesis {
                    break;
                }
                let meta_id = metas.fresh((**dom).clone());
                let meta_expr = Expr::FVar(FVarId(1_000_000 + meta_id));
                inserted.push(InsertedImplicit::new(name.to_string(), meta_id, mode));
                stats.record_insertion(mode);
                ty = oxilean_kernel::instantiate(cod, &meta_expr);
                expr = Expr::App(Box::new(expr), Box::new(meta_expr));
                count += 1;
                if mode == ImplicitMode::TypeClass {
                    if infer_implicit(ctx, &(**dom).clone()).is_some() {
                        stats.local_instance_hits += 1;
                    } else if resolve_instance(ctx, &(**dom).clone()).is_some() {
                        stats.global_instance_hits += 1;
                    }
                }
            }
            _ => break,
        }
    }
    ImplicitPipelineResult {
        expr,
        remaining_ty: ty,
        inserted,
    }
}
/// Eta-expand a function expression with respect to its leading implicit binders.
#[allow(dead_code)]
pub fn eta_expand_implicits(metas: &mut MetaVarContext, f: Expr, fun_type: &Expr) -> (Expr, Expr) {
    let (implicit_args, rest_ty) = collect_implicit_prefix(fun_type, usize::MAX);
    if implicit_args.is_empty() {
        return (f, fun_type.clone());
    }
    let mut result = f;
    for arg in &implicit_args {
        let meta_id = metas.fresh(arg.ty.clone());
        let meta_expr = Expr::FVar(FVarId(1_000_000 + meta_id));
        result = Expr::App(Box::new(result), Box::new(meta_expr));
    }
    (result, rest_ty)
}
/// Remove redundant explicit wrapping of implicit arguments.
#[allow(dead_code)]
pub fn strip_implicit_apps(expr: &Expr) -> &Expr {
    let mut current = expr;
    loop {
        match current {
            Expr::App(f, a) => {
                let is_meta = matches!(
                    a.as_ref(), Expr::FVar(FVarId(id)) if * id >= 1_000_000
                );
                if is_meta {
                    current = f;
                } else {
                    break current;
                }
            }
            _ => break current,
        }
    }
}
/// Return the "head" of an expression after stripping all applications.
#[allow(dead_code)]
pub fn expr_head_strip(expr: &Expr) -> &Expr {
    let mut current = expr;
    while let Expr::App(f, _) = current {
        current = f;
    }
    current
}
/// Analyse which implicit arguments depend on earlier implicit arguments.
#[allow(dead_code)]
pub fn analyze_implicit_dependencies(ty: &Expr) -> Vec<(usize, usize)> {
    let mut deps = Vec::new();
    let mut current = ty;
    let mut pos: usize = 0;
    while let Expr::Pi(bi, _, dom, cod) = current {
        if matches!(bi, BinderInfo::Implicit | BinderInfo::InstImplicit) {
            for j in 0..pos {
                let depth_to_check = (pos - j - 1) as u32;
                if contains_bvar(dom, depth_to_check) {
                    deps.push((pos, j));
                }
            }
        }
        current = cod;
        pos += 1;
    }
    deps
}
/// Return true if expr contains BVar(depth) anywhere.
#[allow(dead_code)]
pub fn contains_bvar(expr: &Expr, depth: u32) -> bool {
    match expr {
        Expr::BVar(n) => *n == depth,
        Expr::App(f, a) => contains_bvar(f, depth) || contains_bvar(a, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_bvar(ty, depth) || contains_bvar(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            contains_bvar(ty, depth) || contains_bvar(val, depth) || contains_bvar(body, depth + 1)
        }
        _ => false,
    }
}
/// Return true if expr contains any metavariable-like FVar (ID >= 1_000_000).
#[allow(dead_code)]
pub fn contains_meta_fvar(expr: &Expr) -> bool {
    match expr {
        Expr::FVar(FVarId(id)) => *id >= 1_000_000,
        Expr::App(f, a) => contains_meta_fvar(f) || contains_meta_fvar(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_meta_fvar(ty) || contains_meta_fvar(body)
        }
        Expr::Let(_, ty, val, body) => {
            contains_meta_fvar(ty) || contains_meta_fvar(val) || contains_meta_fvar(body)
        }
        _ => false,
    }
}
/// Return the total arity of a Pi-type.
#[allow(dead_code)]
pub fn total_arity(ty: &Expr) -> usize {
    let mut count = 0;
    let mut current = ty;
    while let Expr::Pi(_, _, _, cod) = current {
        count += 1;
        current = cod;
    }
    count
}
/// Return the number of leading implicit binders before the first explicit.
#[allow(dead_code)]
pub fn leading_implicit_arity(ty: &Expr) -> usize {
    let (args, _) = collect_implicit_prefix(ty, usize::MAX);
    args.len()
}
/// Return the return type of a Pi-type after stripping all binders.
#[allow(dead_code)]
pub fn return_type(ty: &Expr) -> &Expr {
    let mut current = ty;
    while let Expr::Pi(_, _, _, cod) = current {
        current = cod;
    }
    current
}
/// Count the number of applications in an expression spine.
#[allow(dead_code)]
pub fn count_app_spine(expr: &Expr) -> usize {
    let mut count = 0;
    let mut current = expr;
    while let Expr::App(f, _) = current {
        count += 1;
        current = f;
    }
    count
}
/// Check whether an expression is a metavariable placeholder (FVar with ID >= 1_000_000).
#[allow(dead_code)]
pub fn is_meta_placeholder(expr: &Expr) -> bool {
    matches!(expr, Expr::FVar(FVarId(id)) if * id >= 1_000_000)
}
/// Replace all metavariable placeholders in expr with a given replacement.
#[allow(dead_code)]
pub fn replace_meta_placeholders(expr: &Expr, replacement: &Expr) -> Expr {
    match expr {
        Expr::FVar(FVarId(id)) if *id >= 1_000_000 => replacement.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(replace_meta_placeholders(f, replacement)),
            Box::new(replace_meta_placeholders(a, replacement)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(replace_meta_placeholders(ty, replacement)),
            Box::new(replace_meta_placeholders(body, replacement)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(replace_meta_placeholders(ty, replacement)),
            Box::new(replace_meta_placeholders(body, replacement)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(replace_meta_placeholders(ty, replacement)),
            Box::new(replace_meta_placeholders(val, replacement)),
            Box::new(replace_meta_placeholders(body, replacement)),
        ),
        _ => expr.clone(),
    }
}
/// A single pass of implicit argument normalisation.
#[allow(dead_code)]
pub fn normalise_implicit_pass(expr: &Expr, default_val: &Expr) -> (Expr, usize) {
    match expr {
        Expr::App(f, a) if is_meta_placeholder(a) => {
            let (f2, n) = normalise_implicit_pass(f, default_val);
            (
                Expr::App(Box::new(f2), Box::new(default_val.clone())),
                n + 1,
            )
        }
        Expr::App(f, a) => {
            let (f2, n1) = normalise_implicit_pass(f, default_val);
            let (a2, n2) = normalise_implicit_pass(a, default_val);
            (Expr::App(Box::new(f2), Box::new(a2)), n1 + n2)
        }
        Expr::Lam(bi, name, ty, body) => {
            let (ty2, n1) = normalise_implicit_pass(ty, default_val);
            let (body2, n2) = normalise_implicit_pass(body, default_val);
            (
                Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2)),
                n1 + n2,
            )
        }
        Expr::Pi(bi, name, ty, body) => {
            let (ty2, n1) = normalise_implicit_pass(ty, default_val);
            let (body2, n2) = normalise_implicit_pass(body, default_val);
            (
                Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2)),
                n1 + n2,
            )
        }
        _ => (expr.clone(), 0),
    }
}
/// Produce a human-readable description of the implicit arguments of a Pi-type.
#[allow(dead_code)]
pub fn describe_implicits(ty: &Expr) -> String {
    let (args, _) = collect_implicit_prefix(ty, usize::MAX);
    if args.is_empty() {
        return "(no implicit arguments)".to_string();
    }
    let parts: Vec<String> = args
        .iter()
        .map(|a| {
            let bracket = if a.is_instance { "[" } else { "{" };
            let close = if a.is_instance { "]" } else { "}" };
            format!("{}{}{}", bracket, a.name, close)
        })
        .collect();
    parts.join(" ")
}
/// Reorder implicit arguments to move all leading implicits before explicit ones.
#[allow(dead_code)]
pub fn reorder_implicits_to_front(ty: &Expr) -> Expr {
    let mut implicits: Vec<(BinderInfo, Name, Expr)> = Vec::new();
    let mut explicits: Vec<(BinderInfo, Name, Expr)> = Vec::new();
    let mut current = ty;
    while let Expr::Pi(bi, name, dom, cod) = current {
        if matches!(bi, BinderInfo::Implicit | BinderInfo::InstImplicit) {
            implicits.push((*bi, name.clone(), (**dom).clone()));
        } else {
            explicits.push((*bi, name.clone(), (**dom).clone()));
        }
        current = cod;
    }
    let ret = current.clone();
    let mut result = ret;
    for (bi, name, dom) in explicits.into_iter().rev() {
        result = Expr::Pi(bi, name, Box::new(dom), Box::new(result));
    }
    for (bi, name, dom) in implicits.into_iter().rev() {
        result = Expr::Pi(bi, name, Box::new(dom), Box::new(result));
    }
    result
}
/// Detect whether an expression has had some implicit arguments explicitly supplied.
#[allow(dead_code)]
pub fn detect_explicit_implicits(expr: &Expr, fun_type: &Expr) -> bool {
    let app_count = count_app_spine(expr);
    let leading = leading_implicit_arity(fun_type);
    let explicit = count_explicit_args(fun_type);
    app_count > explicit && app_count >= leading + explicit
}
#[cfg(test)]
mod pipeline_tests {
    use super::*;
    use crate::implicit::*;
    use oxilean_kernel::{BinderInfo, Environment, Level};
    fn type0() -> Expr {
        Expr::Sort(Level::succ(Level::zero()))
    }
    fn make_pi(bi: BinderInfo, name: &str, dom: Expr, cod: Expr) -> Expr {
        Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(cod))
    }
    #[test]
    fn test_pipeline_config_default() {
        let cfg = ImplicitPipelineConfig::default();
        assert!(cfg.enable_tc_synthesis);
        assert_eq!(cfg.max_implicit_args, 64);
    }
    #[test]
    fn test_pipeline_config_full() {
        let cfg = ImplicitPipelineConfig::full();
        assert!(cfg.insert_strict);
        assert_eq!(cfg.max_implicit_args, 256);
    }
    #[test]
    fn test_pipeline_config_minimal() {
        let cfg = ImplicitPipelineConfig::minimal();
        assert!(!cfg.enable_tc_synthesis);
        assert!(cfg.allow_partial);
    }
    #[test]
    fn test_implicit_stats_new() {
        let s = ImplicitStats::new();
        assert_eq!(s.total_insertions, 0);
        assert!(!s.any_inserted());
    }
    #[test]
    fn test_implicit_stats_record() {
        let mut s = ImplicitStats::new();
        s.record_insertion(ImplicitMode::Unification);
        s.record_insertion(ImplicitMode::TypeClass);
        s.record_insertion(ImplicitMode::Strict);
        assert_eq!(s.total_insertions, 3);
        assert_eq!(s.tc_insertions, 1);
        assert_eq!(s.strict_insertions, 1);
        assert!(s.any_inserted());
    }
    #[test]
    fn test_implicit_stats_reset() {
        let mut s = ImplicitStats::new();
        s.record_insertion(ImplicitMode::Unification);
        s.reset();
        assert_eq!(s.total_insertions, 0);
    }
    #[test]
    fn test_implicit_cache_new() {
        let c = ImplicitCache::new();
        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }
    #[test]
    fn test_implicit_cache_insert_get() {
        let mut c = ImplicitCache::new();
        c.insert("Nat", Expr::Const(Name::str("Nat"), vec![]));
        assert_eq!(c.len(), 1);
        assert!(c.get("Nat").is_some());
        assert!(c.get("Int").is_none());
    }
    #[test]
    fn test_implicit_cache_clear() {
        let mut c = ImplicitCache::new();
        c.insert("k1", type0());
        c.insert("k2", type0());
        c.clear();
        assert!(c.is_empty());
    }
    #[test]
    fn test_run_implicit_pipeline_no_implicits() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut metas = MetaVarContext::new();
        let f = Expr::Const(Name::str("f"), vec![]);
        let ty = type0();
        let cfg = ImplicitPipelineConfig::default();
        let mut stats = ImplicitStats::new();
        let mut cache = ImplicitCache::new();
        let result = run_implicit_pipeline(
            &mut ctx,
            &mut metas,
            f.clone(),
            &ty,
            &cfg,
            &mut stats,
            &mut cache,
        );
        assert_eq!(result.inserted.len(), 0);
        assert_eq!(result.expr, f);
        assert_eq!(stats.total_insertions, 0);
    }
    #[test]
    fn test_run_implicit_pipeline_one_implicit() {
        let env = Environment::new();
        let mut ctx = ElabContext::new(&env);
        let mut metas = MetaVarContext::new();
        let f = Expr::Const(Name::str("id"), vec![]);
        let pi = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(BinderInfo::Default, "x", Expr::BVar(0), Expr::BVar(1)),
        );
        let cfg = ImplicitPipelineConfig::default();
        let mut stats = ImplicitStats::new();
        let mut cache = ImplicitCache::new();
        let result =
            run_implicit_pipeline(&mut ctx, &mut metas, f, &pi, &cfg, &mut stats, &mut cache);
        assert_eq!(result.inserted.len(), 1);
        assert_eq!(stats.total_insertions, 1);
        assert!(matches!(result.expr, Expr::App(_, _)));
    }
    #[test]
    fn test_implicit_position_index_empty() {
        let ty = type0();
        let idx = ImplicitPositionIndex::from_type(&ty);
        assert!(idx.is_empty());
    }
    #[test]
    fn test_implicit_position_index_mixed() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(
                BinderInfo::Default,
                "x",
                type0(),
                make_pi(BinderInfo::InstImplicit, "inst", type0(), type0()),
            ),
        );
        let idx = ImplicitPositionIndex::from_type(&ty);
        assert_eq!(idx.len(), 2);
        let imp_pos = idx.implicit_positions();
        assert!(imp_pos.contains(&0));
        let inst_pos = idx.instance_positions();
        assert!(inst_pos.contains(&2));
    }
    #[test]
    fn test_eta_expand_no_implicits() {
        let mut metas = MetaVarContext::new();
        let f = Expr::Const(Name::str("f"), vec![]);
        let ty = type0();
        let (result, _) = eta_expand_implicits(&mut metas, f.clone(), &ty);
        assert_eq!(result, f);
    }
    #[test]
    fn test_eta_expand_one_implicit() {
        let mut metas = MetaVarContext::new();
        let f = Expr::Const(Name::str("f"), vec![]);
        let ty = make_pi(BinderInfo::Implicit, "a", type0(), type0());
        let (result, _) = eta_expand_implicits(&mut metas, f, &ty);
        assert!(matches!(result, Expr::App(_, _)));
    }
    #[test]
    fn test_contains_bvar_true() {
        let e = Expr::BVar(2);
        assert!(contains_bvar(&e, 2));
        assert!(!contains_bvar(&e, 3));
    }
    #[test]
    fn test_contains_meta_fvar_true() {
        let expr = Expr::FVar(FVarId(1_000_001));
        assert!(contains_meta_fvar(&expr));
    }
    #[test]
    fn test_contains_meta_fvar_false() {
        let expr = Expr::FVar(FVarId(5));
        assert!(!contains_meta_fvar(&expr));
    }
    #[test]
    fn test_total_arity() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(BinderInfo::Default, "b", type0(), type0()),
        );
        assert_eq!(total_arity(&ty), 2);
    }
    #[test]
    fn test_leading_implicit_arity() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(BinderInfo::Default, "b", type0(), type0()),
        );
        assert_eq!(leading_implicit_arity(&ty), 1);
    }
    #[test]
    fn test_return_type() {
        let ty = make_pi(BinderInfo::Default, "x", type0(), type0());
        let ret = return_type(&ty);
        assert!(matches!(ret, Expr::Sort(_)));
    }
    #[test]
    fn test_is_meta_placeholder() {
        assert!(is_meta_placeholder(&Expr::FVar(FVarId(1_000_000))));
        assert!(!is_meta_placeholder(&Expr::FVar(FVarId(999_999))));
    }
    #[test]
    fn test_describe_implicits_none() {
        let desc = describe_implicits(&type0());
        assert_eq!(desc, "(no implicit arguments)");
    }
    #[test]
    fn test_describe_implicits_one() {
        let ty = make_pi(BinderInfo::Implicit, "a", type0(), type0());
        let desc = describe_implicits(&ty);
        assert!(desc.contains('a'));
        assert!(desc.contains('{'));
    }
    #[test]
    fn test_implicit_scope_stack_empty() {
        let stack = ImplicitScopeStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.depth(), 0);
    }
    #[test]
    fn test_implicit_scope_stack_push_pop() {
        let mut stack = ImplicitScopeStack::new();
        stack.push_scope();
        assert_eq!(stack.depth(), 1);
        stack.bind_current("a");
        assert!(stack.is_bound_anywhere("a"));
        let scope = stack.pop_scope();
        assert!(scope.is_some());
        assert_eq!(stack.depth(), 0);
    }
    #[test]
    fn test_check_direction_is_infer() {
        let d = CheckDirection::Infer;
        assert!(d.is_infer());
        assert!(!d.is_check());
    }
    #[test]
    fn test_check_direction_is_check() {
        let d = CheckDirection::Check;
        assert!(d.is_check());
        assert!(!d.is_infer());
    }
    #[test]
    fn test_implicit_guard_enable_disable() {
        let mut guard = ImplicitGuard::new();
        assert!(guard.is_enabled());
        guard.disable();
        assert!(!guard.is_enabled());
        guard.enable();
        assert!(guard.is_enabled());
    }
    #[test]
    fn test_implicit_guard_nested_disable() {
        let mut guard = ImplicitGuard::new();
        guard.disable();
        guard.disable();
        guard.enable();
        assert!(!guard.is_enabled());
        guard.enable();
        assert!(guard.is_enabled());
    }
    #[test]
    fn test_implicit_elab_result_inferred() {
        let metas = MetaVarContext::new();
        let r = ImplicitElabResult::Inferred(type0());
        assert!(!r.is_pending());
        let e = r.into_expr(&metas);
        assert!(e.is_some());
    }
    #[test]
    fn test_implicit_elab_result_pending() {
        let metas = MetaVarContext::new();
        let r = ImplicitElabResult::Pending(42);
        assert!(r.is_pending());
        let e = r.into_expr(&metas);
        assert!(e.is_none());
    }
    #[test]
    fn test_implicit_error_display() {
        let e = ImplicitError::CannotInfer("a".to_string());
        assert!(e.to_string().contains("a"));
        let e2 = ImplicitError::InstanceNotFound("Add".to_string());
        assert!(e2.to_string().contains("Add"));
        let e3 = ImplicitError::TooManyImplicits(10);
        assert!(e3.to_string().contains("10"));
        let e4 = ImplicitError::CircularDependency(0, 1);
        assert!(e4.to_string().contains('0'));
    }
    #[test]
    fn test_implicit_arg_summary() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(
                BinderInfo::InstImplicit,
                "inst",
                type0(),
                make_pi(BinderInfo::Default, "x", type0(), type0()),
            ),
        );
        let summary = ImplicitArgSummary::of(&ty);
        assert_eq!(summary.leading_implicit, 1);
        assert_eq!(summary.leading_instance, 1);
        assert_eq!(summary.explicit, 1);
        assert_eq!(summary.total, 3);
        assert_eq!(summary.total_leading(), 2);
        assert!(summary.has_leading());
    }
    #[test]
    fn test_analyze_implicit_dependencies_none() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(BinderInfo::Implicit, "b", type0(), type0()),
        );
        let deps = analyze_implicit_dependencies(&ty);
        assert!(deps.is_empty());
    }
    #[test]
    fn test_analyze_implicit_dependencies_dependent() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(
                BinderInfo::Implicit,
                "f",
                make_pi(BinderInfo::Default, "_", Expr::BVar(0), Expr::BVar(1)),
                type0(),
            ),
        );
        let deps = analyze_implicit_dependencies(&ty);
        assert!(!deps.is_empty());
    }
    #[test]
    fn test_reorder_implicits_to_front() {
        let ty = make_pi(
            BinderInfo::Implicit,
            "a",
            type0(),
            make_pi(BinderInfo::Default, "x", type0(), type0()),
        );
        let reordered = reorder_implicits_to_front(&ty);
        assert!(matches!(reordered, Expr::Pi(BinderInfo::Implicit, _, _, _)));
    }
    #[test]
    fn test_strip_implicit_apps_strips_meta() {
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::FVar(FVarId(1_000_005))),
        );
        let stripped = strip_implicit_apps(&expr);
        assert!(matches!(stripped, Expr::Const(_, _)));
    }
    #[test]
    fn test_count_app_spine() {
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("f"), vec![])),
                Box::new(type0()),
            )),
            Box::new(type0()),
        );
        assert_eq!(count_app_spine(&e), 2);
    }
    #[test]
    fn test_normalise_implicit_pass_substitutes() {
        let default_val = Expr::Const(Name::str("default"), vec![]);
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::FVar(FVarId(1_000_002))),
        );
        let (result, n) = normalise_implicit_pass(&expr, &default_val);
        assert_eq!(n, 1);
        match &result {
            Expr::App(_, a) => assert_eq!(*a.as_ref(), default_val),
            _ => panic!("expected App"),
        }
    }
}
#[cfg(test)]
mod implicit_ext_tests {
    use super::*;
    use crate::implicit::*;
    #[test]
    fn test_implicit_insertion_stats_record() {
        let mut s = ImplicitInsertionStats::new();
        s.record(3, true);
        s.record(0, false);
        assert_eq!(s.inserted, 3);
        assert_eq!(s.exprs_processed, 2);
        assert_eq!(s.failures, 1);
    }
    #[test]
    fn test_implicit_insertion_stats_avg() {
        let mut s = ImplicitInsertionStats::new();
        s.record(4, true);
        s.record(2, true);
        assert!((s.avg_inserted() - 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_implicit_insertion_stats_summary() {
        let mut s = ImplicitInsertionStats::new();
        s.record(1, true);
        let sum = s.summary();
        assert!(sum.contains("inserted=1"));
    }
    #[test]
    fn test_implicit_insertion_stats_empty() {
        let s = ImplicitInsertionStats::new();
        assert!((s.avg_inserted() - 0.0).abs() < 1e-10);
    }
}
