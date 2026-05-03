//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::context::ElabContext;
use crate::elaborate::ElabError;
use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Name};
use oxilean_parse::{Binder, BinderKind, Located, SurfaceExpr};

use super::types::{
    AutoBoundImplicitInfo, BinderDep, BinderElabResult, BinderKindCount, BinderTypeInference,
    BinderUniverse, BinderValidationError, BinderWithDefault, Telescope,
};

/// Elaborate a sequence of surface-level binders.
///
/// Processes each binder in order:
/// 1. If the binder has a type annotation, elaborate it
/// 2. If not, create a fresh metavariable for the type
/// 3. Push each binder into the local context
/// 4. Return the elaborated binder info
///
/// The caller is responsible for popping the binders from the context
/// when they go out of scope.
pub fn elaborate_binders(
    ctx: &mut ElabContext,
    binders: &[Binder],
) -> Result<Vec<BinderElabResult>, ElabError> {
    let mut results = Vec::with_capacity(binders.len());
    for binder in binders {
        let ty = if let Some(ty_surf) = &binder.ty {
            crate::elaborate::elaborate_expr(ctx, ty_surf)?
        } else {
            let sort_ty = Expr::Sort(Level::succ(Level::zero()));
            let (_id, meta) = ctx.fresh_meta(sort_ty);
            meta
        };
        let info = convert_binder_kind(&binder.info);
        let name = Name::str(&binder.name);
        let fvar = ctx.push_local(name.clone(), ty.clone(), None);
        results.push(BinderElabResult {
            name,
            ty,
            info,
            fvar,
        });
    }
    Ok(results)
}
/// Elaborate only the types of binders, without pushing them to context.
///
/// This is useful when you need to know the types of binders before
/// deciding what to do with them.
#[allow(dead_code)]
pub fn elaborate_binder_types(
    ctx: &mut ElabContext,
    binders: &[Binder],
) -> Result<Vec<(Name, Expr, BinderInfo)>, ElabError> {
    let mut results = Vec::with_capacity(binders.len());
    for binder in binders {
        let ty = if let Some(ty_surf) = &binder.ty {
            crate::elaborate::elaborate_expr(ctx, ty_surf)?
        } else {
            let sort_ty = Expr::Sort(Level::succ(Level::zero()));
            let (_id, meta) = ctx.fresh_meta(sort_ty);
            meta
        };
        let info = convert_binder_kind(&binder.info);
        let name = Name::str(&binder.name);
        results.push((name, ty, info));
    }
    Ok(results)
}
/// Elaborate a single binder and push it to the context.
///
/// Returns the elaborated result along with the free variable ID.
#[allow(dead_code)]
pub fn elaborate_single_binder(
    ctx: &mut ElabContext,
    binder: &Binder,
) -> Result<BinderElabResult, ElabError> {
    let ty = if let Some(ty_surf) = &binder.ty {
        crate::elaborate::elaborate_expr(ctx, ty_surf)?
    } else {
        let sort_ty = Expr::Sort(Level::succ(Level::zero()));
        let (_id, meta) = ctx.fresh_meta(sort_ty);
        meta
    };
    let info = convert_binder_kind(&binder.info);
    let name = Name::str(&binder.name);
    let fvar = ctx.push_local(name.clone(), ty.clone(), None);
    Ok(BinderElabResult {
        name,
        ty,
        info,
        fvar,
    })
}
/// Detect free variables in a type that aren't in scope and add implicit
/// binders for them.
///
/// This implements Lean 4's auto-bound implicit feature where type
/// variables can be used without explicit binding:
///
/// ```text
/// def id (x : α) : α := x
/// -- becomes: def id {α : Sort u} (x : α) : α := x
/// ```
///
/// Returns the new expression and type with implicit binders prepended.
pub fn auto_bind_implicits(ctx: &mut ElabContext, expr: Expr, ty: Expr) -> (Expr, Expr) {
    let free_vars = collect_unbound_vars_from_expr(ctx, &ty);
    if free_vars.is_empty() {
        return (expr, ty);
    }
    let mut new_expr = expr;
    let mut new_ty = ty;
    for var_name in free_vars.iter().rev() {
        let name = Name::str(var_name);
        let var_ty = Expr::Sort(Level::succ(Level::zero()));
        new_expr = Expr::Lam(
            BinderInfo::Implicit,
            name.clone(),
            Box::new(var_ty.clone()),
            Box::new(new_expr),
        );
        new_ty = Expr::Pi(
            BinderInfo::Implicit,
            name,
            Box::new(var_ty),
            Box::new(new_ty),
        );
    }
    (new_expr, new_ty)
}
/// Collect variables in an expression that are not bound in the current context.
///
/// This walks the expression looking for `Var` references that don't correspond
/// to any local in the context or any global constant.
#[allow(dead_code)]
fn collect_unbound_vars_from_expr(ctx: &ElabContext, expr: &Expr) -> Vec<String> {
    let mut unbound = Vec::new();
    collect_unbound_helper(ctx, expr, &mut unbound);
    let mut seen = std::collections::HashSet::new();
    unbound.retain(|v| seen.insert(v.clone()));
    unbound
}
fn collect_unbound_helper(ctx: &ElabContext, expr: &Expr, unbound: &mut Vec<String>) {
    match expr {
        Expr::FVar(fvar) if ctx.lookup_fvar(*fvar).is_none() => {
            unbound.push(format!("_fvar_{}", fvar.0));
        }
        Expr::App(f, a) => {
            collect_unbound_helper(ctx, f, unbound);
            collect_unbound_helper(ctx, a, unbound);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_unbound_helper(ctx, ty, unbound);
            collect_unbound_helper(ctx, body, unbound);
        }
        Expr::Let(_, ty, val, body) => {
            collect_unbound_helper(ctx, ty, unbound);
            collect_unbound_helper(ctx, val, unbound);
            collect_unbound_helper(ctx, body, unbound);
        }
        Expr::Proj(_, _, e) => {
            collect_unbound_helper(ctx, e, unbound);
        }
        _ => {}
    }
}
/// For instance-implicit binders `[inst : C α]`, try to synthesize an instance.
///
/// Returns a vector of expressions to insert as implicit arguments for
/// each instance-implicit parameter encountered when traversing the
/// function type.
///
/// Falls back to creating a metavariable if instance synthesis fails.
#[allow(dead_code)]
pub fn insert_instance_implicits(ctx: &mut ElabContext, fun_ty: &Expr) -> Vec<Expr> {
    let mut results = Vec::new();
    let mut ty = fun_ty.clone();
    loop {
        match &ty {
            Expr::Pi(BinderInfo::InstImplicit, _name, dom, cod) => {
                let instance_expr = try_synthesize_instance(ctx, dom);
                results.push(instance_expr.clone());
                ty = oxilean_kernel::instantiate(cod, &instance_expr);
            }
            Expr::Pi(BinderInfo::Implicit, _name, _dom, cod) => {
                let sort_ty = Expr::Sort(Level::succ(Level::zero()));
                let (_id, meta) = ctx.fresh_meta(sort_ty);
                results.push(meta.clone());
                ty = oxilean_kernel::instantiate(cod, &meta);
            }
            _ => break,
        }
    }
    results
}
/// Try to synthesize a type class instance for the given type.
///
/// Returns the instance expression, or a fresh metavariable if synthesis fails.
fn try_synthesize_instance(ctx: &mut ElabContext, class_ty: &Expr) -> Expr {
    if let Some(class_name) = extract_class_name(class_ty) {
        if let Some(decl) = ctx.env().get(&class_name) {
            return Expr::Const(decl.name().clone(), vec![]);
        }
    }
    let sort_ty = Expr::Sort(Level::succ(Level::zero()));
    let (_id, meta) = ctx.fresh_meta(sort_ty);
    meta
}
/// Extract the class name from a type class constraint type.
///
/// For `Add Nat`, extracts `Add`.
/// For `Monad IO`, extracts `Monad`.
fn extract_class_name(ty: &Expr) -> Option<Name> {
    match ty {
        Expr::Const(name, _) => Some(name.clone()),
        Expr::App(f, _) => extract_class_name(f),
        _ => None,
    }
}
/// Create nested lambda abstractions from binder results.
///
/// Given binders `[x : A, y : B, z : C]` and `body`, produces:
/// `λ (x : A), λ (y : B), λ (z : C), body`
pub fn abstract_binders(binders: &[BinderElabResult], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, binder| {
        Expr::Lam(
            binder.info,
            binder.name.clone(),
            Box::new(binder.ty.clone()),
            Box::new(acc),
        )
    })
}
/// Create nested Pi types from binder results.
///
/// Given binders `[x : A, y : B]` and `body`, produces:
/// `Π (x : A), Π (y : B), body`
pub fn pi_binders(binders: &[BinderElabResult], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, binder| {
        Expr::Pi(
            binder.info,
            binder.name.clone(),
            Box::new(binder.ty.clone()),
            Box::new(acc),
        )
    })
}
/// Create nested let bindings.
///
/// Given binders `[x : A, y : B]`, values `[v1, v2]`, and `body`, produces:
/// `let x : A := v1 in let y : B := v2 in body`
///
/// The lengths of `binders` and `vals` must match.
#[allow(dead_code)]
pub fn let_binders(binders: &[BinderElabResult], vals: &[Expr], body: Expr) -> Expr {
    assert_eq!(
        binders.len(),
        vals.len(),
        "binder and value counts must match"
    );
    binders
        .iter()
        .zip(vals.iter())
        .rev()
        .fold(body, |acc, (binder, val)| {
            Expr::Let(
                binder.name.clone(),
                Box::new(binder.ty.clone()),
                Box::new(val.clone()),
                Box::new(acc),
            )
        })
}
/// Create nested lambda abstractions from tuples.
///
/// This is a convenience function that works with `(Name, Expr, BinderInfo)`
/// tuples instead of `BinderElabResult`.
#[allow(dead_code)]
pub fn abstract_binders_tuple(binders: &[(Name, Expr, BinderInfo)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (name, ty, info)| {
        Expr::Lam(*info, name.clone(), Box::new(ty.clone()), Box::new(acc))
    })
}
/// Create nested Pi types from tuples.
#[allow(dead_code)]
pub fn pi_binders_tuple(binders: &[(Name, Expr, BinderInfo)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (name, ty, info)| {
        Expr::Pi(*info, name.clone(), Box::new(ty.clone()), Box::new(acc))
    })
}
/// Collect all free variable IDs referenced in an expression.
#[allow(dead_code)]
pub fn collect_binder_fvars(expr: &Expr) -> Vec<FVarId> {
    let mut fvars = Vec::new();
    collect_fvars_helper(expr, &mut fvars);
    let mut seen = std::collections::HashSet::new();
    fvars.retain(|v| seen.insert(*v));
    fvars
}
fn collect_fvars_helper(expr: &Expr, fvars: &mut Vec<FVarId>) {
    match expr {
        Expr::FVar(fvar) => {
            fvars.push(*fvar);
        }
        Expr::App(f, a) => {
            collect_fvars_helper(f, fvars);
            collect_fvars_helper(a, fvars);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars_helper(ty, fvars);
            collect_fvars_helper(body, fvars);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_helper(ty, fvars);
            collect_fvars_helper(val, fvars);
            collect_fvars_helper(body, fvars);
        }
        Expr::Proj(_, _, e) => {
            collect_fvars_helper(e, fvars);
        }
        _ => {}
    }
}
/// Collect variable names that appear in a surface expression but are not
/// bound in the current context.
///
/// This walks the surface expression looking for `Var` references that
/// are not in the local context or global environment.
pub fn collect_unbound_vars(ctx: &ElabContext, expr: &Located<SurfaceExpr>) -> Vec<String> {
    let mut unbound = Vec::new();
    collect_unbound_surface_helper(ctx, &expr.value, &mut unbound);
    let mut seen = std::collections::HashSet::new();
    unbound.retain(|v| seen.insert(v.clone()));
    unbound
}
fn collect_unbound_surface_helper(
    ctx: &ElabContext,
    expr: &SurfaceExpr,
    unbound: &mut Vec<String>,
) {
    match expr {
        SurfaceExpr::Var(name) => {
            let kernel_name = Name::str(name);
            if ctx.lookup_local(&kernel_name).is_none() && ctx.env().get(&kernel_name).is_none() {
                unbound.push(name.clone());
            }
        }
        SurfaceExpr::App(f, a) => {
            collect_unbound_surface_helper(ctx, &f.value, unbound);
            collect_unbound_surface_helper(ctx, &a.value, unbound);
        }
        SurfaceExpr::Lam(binders, body) => {
            for b in binders {
                if let Some(ty) = &b.ty {
                    collect_unbound_surface_helper(ctx, &ty.value, unbound);
                }
            }
            collect_unbound_surface_helper(ctx, &body.value, unbound);
        }
        SurfaceExpr::Pi(binders, body) => {
            for b in binders {
                if let Some(ty) = &b.ty {
                    collect_unbound_surface_helper(ctx, &ty.value, unbound);
                }
            }
            collect_unbound_surface_helper(ctx, &body.value, unbound);
        }
        SurfaceExpr::Let(_, ty_opt, val, body) => {
            if let Some(ty) = ty_opt {
                collect_unbound_surface_helper(ctx, &ty.value, unbound);
            }
            collect_unbound_surface_helper(ctx, &val.value, unbound);
            collect_unbound_surface_helper(ctx, &body.value, unbound);
        }
        SurfaceExpr::Ann(e, ty) => {
            collect_unbound_surface_helper(ctx, &e.value, unbound);
            collect_unbound_surface_helper(ctx, &ty.value, unbound);
        }
        SurfaceExpr::If(c, t, e) => {
            collect_unbound_surface_helper(ctx, &c.value, unbound);
            collect_unbound_surface_helper(ctx, &t.value, unbound);
            collect_unbound_surface_helper(ctx, &e.value, unbound);
        }
        SurfaceExpr::Match(scrutinee, arms) => {
            collect_unbound_surface_helper(ctx, &scrutinee.value, unbound);
            for arm in arms {
                collect_unbound_surface_helper(ctx, &arm.rhs.value, unbound);
            }
        }
        SurfaceExpr::Proj(e, _) => {
            collect_unbound_surface_helper(ctx, &e.value, unbound);
        }
        SurfaceExpr::Have(_, ty, val, body) => {
            collect_unbound_surface_helper(ctx, &ty.value, unbound);
            collect_unbound_surface_helper(ctx, &val.value, unbound);
            collect_unbound_surface_helper(ctx, &body.value, unbound);
        }
        SurfaceExpr::Show(ty, e) => {
            collect_unbound_surface_helper(ctx, &ty.value, unbound);
            collect_unbound_surface_helper(ctx, &e.value, unbound);
        }
        SurfaceExpr::ListLit(elems) => {
            for e in elems {
                collect_unbound_surface_helper(ctx, &e.value, unbound);
            }
        }
        SurfaceExpr::Tuple(elems) => {
            for e in elems {
                collect_unbound_surface_helper(ctx, &e.value, unbound);
            }
        }
        SurfaceExpr::AnonymousCtor(elems) => {
            for e in elems {
                collect_unbound_surface_helper(ctx, &e.value, unbound);
            }
        }
        SurfaceExpr::Return(e) => {
            collect_unbound_surface_helper(ctx, &e.value, unbound);
        }
        SurfaceExpr::Do(actions) => {
            for action in actions {
                match action {
                    oxilean_parse::DoAction::Let(_, val) => {
                        collect_unbound_surface_helper(ctx, &val.value, unbound);
                    }
                    oxilean_parse::DoAction::LetTyped(_, ty, val) => {
                        collect_unbound_surface_helper(ctx, &ty.value, unbound);
                        collect_unbound_surface_helper(ctx, &val.value, unbound);
                    }
                    oxilean_parse::DoAction::Bind(_, e) => {
                        collect_unbound_surface_helper(ctx, &e.value, unbound);
                    }
                    oxilean_parse::DoAction::Expr(e) => {
                        collect_unbound_surface_helper(ctx, &e.value, unbound);
                    }
                    oxilean_parse::DoAction::Return(e) => {
                        collect_unbound_surface_helper(ctx, &e.value, unbound);
                    }
                }
            }
        }
        SurfaceExpr::Suffices(_, ty, body) => {
            collect_unbound_surface_helper(ctx, &ty.value, unbound);
            collect_unbound_surface_helper(ctx, &body.value, unbound);
        }
        SurfaceExpr::NamedArg(f, _, val) => {
            collect_unbound_surface_helper(ctx, &f.value, unbound);
            collect_unbound_surface_helper(ctx, &val.value, unbound);
        }
        SurfaceExpr::Range(lo, hi) => {
            if let Some(lo) = lo {
                collect_unbound_surface_helper(ctx, &lo.value, unbound);
            }
            if let Some(hi) = hi {
                collect_unbound_surface_helper(ctx, &hi.value, unbound);
            }
        }
        SurfaceExpr::Calc(steps) => {
            for step in steps {
                collect_unbound_surface_helper(ctx, &step.lhs.value, unbound);
                collect_unbound_surface_helper(ctx, &step.rhs.value, unbound);
                collect_unbound_surface_helper(ctx, &step.proof.value, unbound);
            }
        }
        _ => {}
    }
}
/// Pop `n` binders from the context.
///
/// This is a convenience function for cleaning up after `elaborate_binders`.
#[allow(dead_code)]
pub fn pop_binders(ctx: &mut ElabContext, count: usize) {
    for _ in 0..count {
        ctx.pop_local();
    }
}
/// Convert a parse `BinderKind` to a kernel `BinderInfo`.
pub fn convert_binder_kind(kind: &BinderKind) -> BinderInfo {
    match kind {
        BinderKind::Default => BinderInfo::Default,
        BinderKind::Implicit => BinderInfo::Implicit,
        BinderKind::Instance => BinderInfo::InstImplicit,
        BinderKind::StrictImplicit => BinderInfo::StrictImplicit,
    }
}
/// Check if a binder kind represents an implicit argument.
#[allow(dead_code)]
pub fn is_implicit_binder(kind: &BinderKind) -> bool {
    matches!(
        kind,
        BinderKind::Implicit | BinderKind::Instance | BinderKind::StrictImplicit
    )
}
/// Make a simple binder (for testing and convenience).
#[allow(dead_code)]
pub fn make_binder(name: &str, ty: Option<Located<SurfaceExpr>>, kind: BinderKind) -> Binder {
    Binder {
        name: name.to_string(),
        ty: ty.map(Box::new),
        info: kind,
    }
}
/// Attempt to infer a binder type from context.
///
/// Inspects the surrounding `Pi`-type (if any) to determine what type the next
/// binder should have, enabling better type inference for anonymous binders.
#[allow(dead_code)]
pub fn infer_binder_type_from_context(
    ctx: &ElabContext,
    expected_ty: Option<&Expr>,
    binder_index: usize,
) -> (Option<Expr>, BinderTypeInference) {
    if let Some(expected) = expected_ty {
        let mut ty = expected.clone();
        for i in 0..=binder_index {
            match ty.clone() {
                Expr::Pi(_, _, dom, cod) => {
                    if i == binder_index {
                        return (Some(*dom), BinderTypeInference::FromExpected);
                    }
                    ty = *cod;
                }
                _ => break,
            }
        }
    }
    if !ctx.locals().is_empty() && binder_index < ctx.locals().len() {
        let local = &ctx.locals()[binder_index];
        return (
            Some(local.ty.clone()),
            BinderTypeInference::FromSibling(binder_index),
        );
    }
    (None, BinderTypeInference::Fresh)
}
/// Elaborate a binder with context-driven type inference.
#[allow(dead_code)]
pub fn elaborate_binder_with_inference(
    ctx: &mut ElabContext,
    binder: &Binder,
    expected_ty: Option<&Expr>,
    binder_index: usize,
) -> Result<BinderElabResult, ElabError> {
    let ty = if let Some(ty_surf) = &binder.ty {
        crate::elaborate::elaborate_expr(ctx, ty_surf)?
    } else {
        let (inferred, _strategy) = infer_binder_type_from_context(ctx, expected_ty, binder_index);
        inferred.unwrap_or_else(|| {
            let sort_ty = Expr::Sort(Level::succ(Level::zero()));
            let (_id, meta) = ctx.fresh_meta(sort_ty);
            meta
        })
    };
    let info = convert_binder_kind(&binder.info);
    let name = Name::str(&binder.name);
    let fvar = ctx.push_local(name.clone(), ty.clone(), None);
    Ok(BinderElabResult {
        name,
        ty,
        info,
        fvar,
    })
}
/// Check whether a character is a Greek letter (common type variable notation).
#[allow(dead_code)]
pub fn is_greek_letter(c: char) -> bool {
    ('\u{0370}'..='\u{03FF}').contains(&c)
}
/// Collect heuristic info about auto-bound candidate variables.
#[allow(dead_code)]
pub fn detect_auto_bound_candidates(
    ctx: &ElabContext,
    expr: &Located<SurfaceExpr>,
) -> Vec<AutoBoundImplicitInfo> {
    collect_unbound_vars(ctx, expr)
        .into_iter()
        .map(|n| AutoBoundImplicitInfo::for_name(&n))
        .collect()
}
/// Elaborate a full telescope from surface binders.
#[allow(dead_code)]
pub fn elaborate_telescope(
    ctx: &mut ElabContext,
    binders: &[Binder],
) -> Result<Telescope, ElabError> {
    let results = elaborate_binders(ctx, binders)?;
    let mut tel = Telescope::new();
    for r in results {
        tel.push(r);
    }
    Ok(tel)
}
/// Pop the entire telescope from the local context.
#[allow(dead_code)]
pub fn pop_telescope(ctx: &mut ElabContext, telescope: &Telescope) {
    pop_binders(ctx, telescope.len());
}
/// Check whether binder `index` is dependent (later binders reference it).
#[allow(dead_code)]
pub fn is_dependent_binder(binders: &[BinderElabResult], index: usize) -> bool {
    if index >= binders.len() {
        return false;
    }
    let fvar = binders[index].fvar;
    binders[index + 1..]
        .iter()
        .any(|b| expr_contains_fvar(&b.ty, fvar))
}
/// Check whether any binder in a slice is dependent.
#[allow(dead_code)]
pub fn has_dependent_binders(binders: &[BinderElabResult]) -> bool {
    (0..binders.len()).any(|i| is_dependent_binder(binders, i))
}
/// Return indices of all dependent binders.
#[allow(dead_code)]
pub fn dependent_binder_indices(binders: &[BinderElabResult]) -> Vec<usize> {
    (0..binders.len())
        .filter(|&i| is_dependent_binder(binders, i))
        .collect()
}
/// Check whether an expression contains a specific free variable.
#[allow(dead_code)]
pub fn expr_contains_fvar(expr: &Expr, fvar: FVarId) -> bool {
    match expr {
        Expr::FVar(id) => *id == fvar,
        Expr::App(f, a) => expr_contains_fvar(f, fvar) || expr_contains_fvar(a, fvar),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            expr_contains_fvar(ty, fvar) || expr_contains_fvar(body, fvar)
        }
        Expr::Let(_, ty, val, body) => {
            expr_contains_fvar(ty, fvar)
                || expr_contains_fvar(val, fvar)
                || expr_contains_fvar(body, fvar)
        }
        Expr::Proj(_, _, e) => expr_contains_fvar(e, fvar),
        _ => false,
    }
}
/// Create an anonymous binder for non-dependent Pi-types.
#[allow(dead_code)]
pub fn make_anonymous_binder(ty: Located<SurfaceExpr>) -> Binder {
    Binder {
        name: "_".to_string(),
        ty: Some(Box::new(ty)),
        info: BinderKind::Default,
    }
}
/// Check whether a binder is anonymous.
#[allow(dead_code)]
pub fn is_anonymous_binder(binder: &Binder) -> bool {
    binder.name == "_" || binder.name.starts_with("_x_") || binder.name.is_empty()
}
/// Check whether an elaborated binder is anonymous.
#[allow(dead_code)]
pub fn is_anonymous_result(result: &BinderElabResult) -> bool {
    result.name == Name::Anonymous || result.name == Name::str("_")
}
/// Create an elaborated anonymous binder result directly.
#[allow(dead_code)]
pub fn make_anonymous_result(ty: Expr, fvar: FVarId) -> BinderElabResult {
    BinderElabResult {
        name: Name::Anonymous,
        ty,
        info: BinderInfo::Default,
        fvar,
    }
}
/// Resolve a named binder reference from a slice.
#[allow(dead_code)]
pub fn resolve_named_binder<'a>(
    binders: &'a [BinderElabResult],
    name: &Name,
) -> Option<(usize, &'a BinderElabResult)> {
    binders.iter().enumerate().find(|(_, b)| &b.name == name)
}
/// Reorder named arguments to match declared binder positions.
#[allow(dead_code)]
pub fn reorder_named_args(
    binders: &[BinderElabResult],
    named_args: &[(Name, Expr)],
) -> Vec<Option<Expr>> {
    let mut result: Vec<Option<Expr>> = vec![None; binders.len()];
    for (name, val) in named_args {
        if let Some((idx, _)) = resolve_named_binder(binders, name) {
            result[idx] = Some(val.clone());
        }
    }
    result
}
/// Elaborate binders with optional default values.
#[allow(dead_code)]
pub fn elaborate_binders_with_defaults(
    ctx: &mut ElabContext,
    binders: &[Binder],
    defaults: &[Option<Located<SurfaceExpr>>],
) -> Result<Vec<BinderWithDefault>, ElabError> {
    assert_eq!(binders.len(), defaults.len());
    let mut results = Vec::with_capacity(binders.len());
    for (binder, default_opt) in binders.iter().zip(defaults.iter()) {
        let result = elaborate_single_binder(ctx, binder)?;
        let default_val = if let Some(default_expr) = default_opt {
            Some(crate::elaborate::elaborate_expr(ctx, default_expr)?)
        } else {
            None
        };
        results.push(BinderWithDefault {
            result,
            default_val,
        });
    }
    Ok(results)
}
/// Reserved names that cannot be used as binder names.
#[allow(dead_code)]
static RESERVED_NAMES: &[&str] = &[
    "def",
    "theorem",
    "lemma",
    "axiom",
    "fun",
    "let",
    "in",
    "match",
    "with",
    "forall",
    "exists",
    "if",
    "then",
    "else",
    "return",
    "do",
    "by",
    "have",
    "show",
    "from",
    "import",
    "namespace",
    "end",
    "section",
];
/// Validate a single binder for syntax correctness.
#[allow(dead_code)]
pub fn validate_binder(binder: &Binder) -> Result<(), BinderValidationError> {
    if binder.name.is_empty() {
        return Err(BinderValidationError::EmptyName);
    }
    if binder.name != "_" && RESERVED_NAMES.contains(&binder.name.as_str()) {
        return Err(BinderValidationError::ReservedName(binder.name.clone()));
    }
    if binder.info == BinderKind::Instance && binder.ty.is_none() {
        return Err(BinderValidationError::InstanceBinderWithoutType);
    }
    Ok(())
}
/// Validate a sequence of binders.
#[allow(dead_code)]
pub fn validate_binders(binders: &[Binder]) -> Result<(), BinderValidationError> {
    let mut has_named = false;
    let mut has_anon = false;
    for binder in binders {
        validate_binder(binder)?;
        if binder.name == "_" {
            has_anon = true;
        } else {
            has_named = true;
        }
    }
    if has_named && has_anon && binders.len() > 2 {
        return Err(BinderValidationError::AmbiguousMixedBinders);
    }
    Ok(())
}
/// Count surface binders by kind.
#[allow(dead_code)]
pub fn count_binder_kinds(binders: &[Binder]) -> BinderKindCount {
    let mut counts = BinderKindCount::default();
    for b in binders {
        match b.info {
            BinderKind::Default => counts.explicit += 1,
            BinderKind::Implicit => counts.implicit += 1,
            BinderKind::Instance => counts.instance += 1,
            BinderKind::StrictImplicit => counts.strict += 1,
        }
    }
    counts
}
/// Count elaborated binders by kind.
#[allow(dead_code)]
pub fn count_elab_binder_kinds(binders: &[BinderElabResult]) -> BinderKindCount {
    let mut counts = BinderKindCount::default();
    for b in binders {
        match b.info {
            BinderInfo::Default => counts.explicit += 1,
            BinderInfo::Implicit => counts.implicit += 1,
            BinderInfo::InstImplicit => counts.instance += 1,
            BinderInfo::StrictImplicit => counts.strict += 1,
        }
    }
    counts
}
/// Abstract an expression over a telescope, replacing FVar with BVar.
#[allow(dead_code)]
pub fn abstract_over_telescope(binders: &[BinderElabResult], body: Expr) -> Expr {
    let mut result = body;
    for (rev_i, binder) in binders.iter().rev().enumerate() {
        result = replace_fvar_with_bvar(result, binder.fvar, rev_i as u32);
    }
    for binder in binders.iter().rev() {
        result = Expr::Lam(
            binder.info,
            binder.name.clone(),
            Box::new(binder.ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Replace a specific FVar with a BVar at the given de Bruijn index.
#[allow(dead_code)]
fn replace_fvar_with_bvar(expr: Expr, fvar: FVarId, bvar_idx: u32) -> Expr {
    match expr {
        Expr::FVar(id) if id == fvar => Expr::BVar(bvar_idx),
        Expr::App(f, a) => Expr::App(
            Box::new(replace_fvar_with_bvar(*f, fvar, bvar_idx)),
            Box::new(replace_fvar_with_bvar(*a, fvar, bvar_idx)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            bi,
            n,
            Box::new(replace_fvar_with_bvar(*ty, fvar, bvar_idx + 1)),
            Box::new(replace_fvar_with_bvar(*body, fvar, bvar_idx + 1)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            bi,
            n,
            Box::new(replace_fvar_with_bvar(*ty, fvar, bvar_idx + 1)),
            Box::new(replace_fvar_with_bvar(*body, fvar, bvar_idx + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n,
            Box::new(replace_fvar_with_bvar(*ty, fvar, bvar_idx + 1)),
            Box::new(replace_fvar_with_bvar(*val, fvar, bvar_idx + 1)),
            Box::new(replace_fvar_with_bvar(*body, fvar, bvar_idx + 1)),
        ),
        Expr::Proj(name, idx, e) => Expr::Proj(
            name,
            idx,
            Box::new(replace_fvar_with_bvar(*e, fvar, bvar_idx)),
        ),
        _ => expr,
    }
}
/// Heuristically determine the universe of a binder's type.
#[allow(dead_code)]
pub fn classify_binder_universe(ty: &Expr) -> BinderUniverse {
    match ty {
        Expr::Sort(level) => {
            if *level == Level::zero() {
                BinderUniverse::Prop
            } else {
                BinderUniverse::Type
            }
        }
        Expr::Pi(_, _, _, cod) => classify_binder_universe(cod),
        _ => BinderUniverse::Unknown,
    }
}
/// Build the dependency graph for a slice of elaborated binders.
#[allow(dead_code)]
pub fn build_dependency_graph(binders: &[BinderElabResult]) -> Vec<BinderDep> {
    let mut deps = Vec::new();
    for i in 1..binders.len() {
        for j in 0..i {
            if expr_contains_fvar(&binders[i].ty, binders[j].fvar) {
                deps.push(BinderDep { from: i, to: j });
            }
        }
    }
    deps
}
/// Compute a topological ordering of binders respecting dependencies.
///
/// Returns `None` if there is a cycle.
#[allow(dead_code)]
pub fn topological_binder_order(n: usize, deps: &[BinderDep]) -> Option<Vec<usize>> {
    let mut in_degree = vec![0usize; n];
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for dep in deps {
        if dep.from < n && dep.to < n {
            adj[dep.to].push(dep.from);
            in_degree[dep.from] += 1;
        }
    }
    let mut queue: std::collections::VecDeque<usize> =
        (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &next in &adj[node] {
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                queue.push_back(next);
            }
        }
    }
    if order.len() == n {
        Some(order)
    } else {
        None
    }
}
/// Normalise binder names: replace empty names with `_`, trim whitespace.
#[allow(dead_code)]
pub fn normalise_binder_names(binders: &mut [Binder]) {
    for b in binders.iter_mut() {
        let trimmed = b.name.trim().to_string();
        b.name = if trimmed.is_empty() {
            "_".to_string()
        } else {
            trimmed
        };
    }
}
/// Check whether all binders in a slice have distinct names.
#[allow(dead_code)]
pub fn binders_have_distinct_names(binders: &[Binder]) -> bool {
    let mut seen = std::collections::HashSet::new();
    for b in binders {
        if b.name != "_" && !seen.insert(b.name.clone()) {
            return false;
        }
    }
    true
}
/// Return the indices of any duplicate-named binders.
#[allow(dead_code)]
pub fn duplicate_name_indices(binders: &[Binder]) -> Vec<usize> {
    let mut seen = std::collections::HashMap::new();
    let mut dupes = Vec::new();
    for (i, b) in binders.iter().enumerate() {
        if b.name == "_" {
            continue;
        }
        if seen.contains_key(&b.name) {
            dupes.push(i);
        } else {
            seen.insert(b.name.clone(), i);
        }
    }
    dupes
}
