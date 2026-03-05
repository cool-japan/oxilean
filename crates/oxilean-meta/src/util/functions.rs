//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ExprSummary, FunInfo, MetaUtilCache, MetaUtilLogger, MetaUtilPriorityQueue, MetaUtilRegistry,
    MetaUtilStats, MetaUtilUtil0, UtilAnalysisPass, UtilConfig, UtilConfigValue, UtilDiagnostics,
    UtilDiff, UtilExtConfig3000, UtilExtConfigVal3000, UtilExtDiag3000, UtilExtDiff3000,
    UtilExtPass3000, UtilExtPipeline3000, UtilExtResult3000, UtilPipeline, UtilResult,
};
use crate::basic::{MVarId, MetaContext, MVAR_FVAR_OFFSET};
use oxilean_kernel::{BinderInfo, Expr, FVarId, Level};

/// Collect all free variable IDs in an expression.
pub fn collect_fvars(expr: &Expr) -> Vec<FVarId> {
    let mut result = Vec::new();
    collect_fvars_impl(expr, &mut result);
    result
}
pub(super) fn collect_fvars_impl(expr: &Expr, result: &mut Vec<FVarId>) {
    match expr {
        Expr::FVar(id) => {
            if id.0 < MVAR_FVAR_OFFSET && !result.contains(id) {
                result.push(*id);
            }
        }
        Expr::App(f, a) => {
            collect_fvars_impl(f, result);
            collect_fvars_impl(a, result);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars_impl(ty, result);
            collect_fvars_impl(body, result);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_impl(ty, result);
            collect_fvars_impl(val, result);
            collect_fvars_impl(body, result);
        }
        Expr::Proj(_, _, e) => {
            collect_fvars_impl(e, result);
        }
        _ => {}
    }
}
/// Collect all metavariable IDs in an expression.
pub fn collect_mvars(expr: &Expr) -> Vec<MVarId> {
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
/// Abstract a free variable in an expression, converting it to BVar(0).
///
/// Shifts existing bound variables as needed.
pub fn abstract_fvar(expr: &Expr, fvar_id: FVarId) -> Expr {
    abstract_fvar_impl(expr, fvar_id, 0)
}
pub(super) fn abstract_fvar_impl(expr: &Expr, fvar_id: FVarId, depth: u32) -> Expr {
    match expr {
        Expr::FVar(id) if *id == fvar_id => Expr::BVar(depth),
        Expr::BVar(idx) if *idx >= depth => Expr::BVar(idx + 1),
        Expr::App(f, a) => {
            let f2 = abstract_fvar_impl(f, fvar_id, depth);
            let a2 = abstract_fvar_impl(a, fvar_id, depth);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = abstract_fvar_impl(ty, fvar_id, depth);
            let body2 = abstract_fvar_impl(body, fvar_id, depth + 1);
            Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = abstract_fvar_impl(ty, fvar_id, depth);
            let body2 = abstract_fvar_impl(body, fvar_id, depth + 1);
            Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = abstract_fvar_impl(ty, fvar_id, depth);
            let val2 = abstract_fvar_impl(val, fvar_id, depth);
            let body2 = abstract_fvar_impl(body, fvar_id, depth + 1);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = abstract_fvar_impl(e, fvar_id, depth);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Abstract multiple free variables at once.
///
/// `fvar_ids[0]` becomes `BVar(n-1)`, ..., `fvar_ids[n-1]` becomes `BVar(0)`.
pub fn abstract_fvars(expr: &Expr, fvar_ids: &[FVarId]) -> Expr {
    let mut result = expr.clone();
    for (i, fvar_id) in fvar_ids.iter().enumerate().rev() {
        result = abstract_fvar(&result, *fvar_id);
        let _ = i;
    }
    result
}
/// Instantiate BVar(0) in an expression with a replacement.
pub fn instantiate(expr: &Expr, replacement: &Expr) -> Expr {
    instantiate_at(expr, 0, replacement)
}
/// Instantiate BVar(idx) in an expression.
pub fn instantiate_at(expr: &Expr, idx: u32, replacement: &Expr) -> Expr {
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
            let f2 = instantiate_at(f, idx, replacement);
            let a2 = instantiate_at(a, idx, replacement);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = instantiate_at(ty, idx, replacement);
            let body2 = instantiate_at(body, idx + 1, replacement);
            Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = instantiate_at(ty, idx, replacement);
            let body2 = instantiate_at(body, idx + 1, replacement);
            Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = instantiate_at(ty, idx, replacement);
            let val2 = instantiate_at(val, idx, replacement);
            let body2 = instantiate_at(body, idx + 1, replacement);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = instantiate_at(e, idx, replacement);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Check if an expression has loose bound variables.
pub fn has_loose_bvars(expr: &Expr) -> bool {
    has_loose_bvar_above(expr, 0)
}
pub(super) fn has_loose_bvar_above(expr: &Expr, depth: u32) -> bool {
    match expr {
        Expr::BVar(idx) => *idx >= depth,
        Expr::App(f, a) => has_loose_bvar_above(f, depth) || has_loose_bvar_above(a, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_loose_bvar_above(ty, depth) || has_loose_bvar_above(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            has_loose_bvar_above(ty, depth)
                || has_loose_bvar_above(val, depth)
                || has_loose_bvar_above(body, depth + 1)
        }
        Expr::Proj(_, _, e) => has_loose_bvar_above(e, depth),
        _ => false,
    }
}
/// Analyze a function type to extract argument information.
pub fn get_fun_info(ty: &Expr) -> FunInfo {
    let mut arg_types = Vec::new();
    let mut arg_implicit = Vec::new();
    let mut arg_inst_implicit = Vec::new();
    let mut current = ty.clone();
    while let Expr::Pi(bi, _name, domain, body) = &current {
        arg_types.push(*domain.clone());
        arg_implicit.push(matches!(
            bi,
            BinderInfo::Implicit | BinderInfo::StrictImplicit
        ));
        arg_inst_implicit.push(matches!(bi, BinderInfo::InstImplicit));
        current = *body.clone();
    }
    FunInfo {
        arg_types,
        arg_implicit,
        arg_inst_implicit,
        result_type: current,
    }
}
/// Get the head function and arguments of an application.
///
/// `f a₁ a₂ a₃` → `(f, [a₁, a₂, a₃])`
pub fn get_app_fn_args(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut current = expr;
    while let Expr::App(f, a) = current {
        args.push(a.as_ref());
        current = f;
    }
    args.reverse();
    (current, args)
}
/// Reconstruct an application from a function and arguments.
pub fn mk_app_n(f: Expr, args: &[Expr]) -> Expr {
    let mut result = f;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg.clone()));
    }
    result
}
/// Count the number of leading Pi binders.
pub fn count_pis(expr: &Expr) -> usize {
    let mut count = 0;
    let mut current = expr;
    while let Expr::Pi(_, _, _, body) = current {
        count += 1;
        current = body;
    }
    count
}
/// Count the number of leading Lambda binders.
pub fn count_lambdas(expr: &Expr) -> usize {
    let mut count = 0;
    let mut current = expr;
    while let Expr::Lam(_, _, _, body) = current {
        count += 1;
        current = body;
    }
    count
}
/// Check if an expression is a proposition (simplified).
pub fn is_prop_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(Level::Zero))
}
/// Apply `for_each` to every subexpression.
pub fn for_each_expr<F>(expr: &Expr, f: &mut F)
where
    F: FnMut(&Expr),
{
    f(expr);
    match expr {
        Expr::App(fun, arg) => {
            for_each_expr(fun, f);
            for_each_expr(arg, f);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            for_each_expr(ty, f);
            for_each_expr(body, f);
        }
        Expr::Let(_, ty, val, body) => {
            for_each_expr(ty, f);
            for_each_expr(val, f);
            for_each_expr(body, f);
        }
        Expr::Proj(_, _, e) => {
            for_each_expr(e, f);
        }
        _ => {}
    }
}
/// Replace subexpressions matching a predicate.
pub fn replace_expr<F>(expr: &Expr, f: &F) -> Expr
where
    F: Fn(&Expr) -> Option<Expr>,
{
    if let Some(replacement) = f(expr) {
        return replacement;
    }
    match expr {
        Expr::App(fun, arg) => {
            let fun2 = replace_expr(fun, f);
            let arg2 = replace_expr(arg, f);
            Expr::App(Box::new(fun2), Box::new(arg2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = replace_expr(ty, f);
            let body2 = replace_expr(body, f);
            Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = replace_expr(ty, f);
            let body2 = replace_expr(body, f);
            Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = replace_expr(ty, f);
            let val2 = replace_expr(val, f);
            let body2 = replace_expr(body, f);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = replace_expr(e, f);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;
    use oxilean_kernel::Name;
    fn fv(n: u64) -> FVarId {
        FVarId::new(n)
    }
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_collect_fvars() {
        let expr = Expr::App(Box::new(Expr::FVar(fv(1))), Box::new(Expr::FVar(fv(2))));
        let fvars = collect_fvars(&expr);
        assert_eq!(fvars.len(), 2);
        assert!(fvars.contains(&fv(1)));
        assert!(fvars.contains(&fv(2)));
    }
    #[test]
    fn test_collect_fvars_no_mvars() {
        let expr = Expr::FVar(FVarId::new(MVAR_FVAR_OFFSET + 5));
        let fvars = collect_fvars(&expr);
        assert!(fvars.is_empty());
    }
    #[test]
    fn test_collect_mvars() {
        let expr = Expr::FVar(FVarId::new(MVAR_FVAR_OFFSET + 5));
        let mvars = collect_mvars(&expr);
        assert_eq!(mvars.len(), 1);
        assert_eq!(mvars[0], MVarId(5));
    }
    #[test]
    fn test_abstract_fvar() {
        let fid = fv(42);
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::FVar(fid)),
        );
        let result = abstract_fvar(&expr, fid);
        let expected = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_instantiate() {
        let body = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        let replacement = Expr::Const(Name::str("x"), vec![]);
        let result = instantiate(&body, &replacement);
        let expected = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(replacement),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_has_loose_bvars() {
        assert!(has_loose_bvars(&Expr::BVar(0)));
        assert!(!has_loose_bvars(&Expr::Const(Name::str("x"), vec![])));
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty()),
            Box::new(Expr::BVar(0)),
        );
        assert!(!has_loose_bvars(&lam));
        let lam2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty()),
            Box::new(Expr::BVar(1)),
        );
        assert!(has_loose_bvars(&lam2));
    }
    #[test]
    fn test_get_fun_info() {
        let ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(nat_ty()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(nat_ty()),
                Box::new(Expr::Const(Name::str("Bool"), vec![])),
            )),
        );
        let info = get_fun_info(&ty);
        assert_eq!(info.arg_types.len(), 2);
        assert!(!info.arg_implicit[0]);
        assert_eq!(info.result_type, Expr::Const(Name::str("Bool"), vec![]));
    }
    #[test]
    fn test_get_fun_info_implicit() {
        let ty = Expr::Pi(
            BinderInfo::Implicit,
            Name::str("α"),
            Box::new(Expr::Sort(Level::Param(Name::str("u")))),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::BVar(1)),
            )),
        );
        let info = get_fun_info(&ty);
        assert_eq!(info.arg_types.len(), 2);
        assert!(info.arg_implicit[0]);
        assert!(!info.arg_implicit[1]);
    }
    #[test]
    fn test_get_app_fn_args() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = get_app_fn_args(&app);
        assert_eq!(head, &f);
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], &a);
        assert_eq!(args[1], &b);
    }
    #[test]
    fn test_mk_app_n() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let result = mk_app_n(f.clone(), std::slice::from_ref(&a));
        assert_eq!(result, Expr::App(Box::new(f), Box::new(a)));
    }
    #[test]
    fn test_count_pis() {
        let ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(nat_ty()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(nat_ty()),
                Box::new(nat_ty()),
            )),
        );
        assert_eq!(count_pis(&ty), 2);
        assert_eq!(count_pis(&nat_ty()), 0);
    }
    #[test]
    fn test_count_lambdas() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty()),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(count_lambdas(&lam), 1);
    }
    #[test]
    fn test_for_each_expr() {
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        let mut count = 0;
        for_each_expr(&expr, &mut |_| count += 1);
        assert_eq!(count, 3);
    }
    #[test]
    fn test_replace_expr() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let f_a = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(a.clone()),
        );
        let result = replace_expr(&f_a, &|e| {
            if e == &a {
                Some(b.clone())
            } else {
                None
            }
        });
        let expected = Expr::App(Box::new(Expr::Const(Name::str("f"), vec![])), Box::new(b));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_is_prop_expr() {
        assert!(is_prop_expr(&Expr::Sort(Level::Zero)));
        assert!(!is_prop_expr(&Expr::Sort(Level::succ(Level::Zero))));
    }
}
/// Structural equality check (syntactic, no definitional equality).
pub fn syntactic_eq(a: &Expr, b: &Expr) -> bool {
    a == b
}
/// Check if an expression contains metavariables.
pub fn has_metavar(expr: &Expr) -> bool {
    !collect_mvars(expr).is_empty()
}
/// Check if an expression is "closed" (no free variables and no loose bvars).
pub fn is_closed(expr: &Expr) -> bool {
    collect_fvars(expr).is_empty() && !has_loose_bvars(expr)
}
/// Count the total number of nodes in an expression tree.
pub fn expr_size(expr: &Expr) -> usize {
    let mut count = 0;
    for_each_expr(expr, &mut |_| {
        count += 1;
    });
    count
}
/// Collect all constant names referenced in an expression.
pub fn collect_const_names(expr: &Expr) -> Vec<oxilean_kernel::Name> {
    let mut names = Vec::new();
    for_each_expr(expr, &mut |sub| {
        if let Expr::Const(name, _) = sub {
            if !names.contains(name) {
                names.push(name.clone());
            }
        }
    });
    names
}
/// Flatten nested applications into a head and argument list.
pub fn flatten_app(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut curr = expr.clone();
    loop {
        match curr {
            Expr::App(f, a) => {
                args.push(*a);
                curr = *f;
            }
            _ => {
                args.reverse();
                return (curr, args);
            }
        }
    }
}
/// Rebuild an application from a head and argument list.
pub fn build_app(head: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter()
        .fold(head, |acc, arg| Expr::App(Box::new(acc), Box::new(arg)))
}
/// Check if an expression is a "neutral" term.
pub fn is_neutral(expr: &Expr) -> bool {
    match expr {
        Expr::FVar(_) | Expr::Const(_, _) | Expr::BVar(_) => true,
        Expr::App(f, _) => is_neutral(f),
        _ => false,
    }
}
/// Shift all BVar indices up by `amount`.
pub(super) fn shift_bvars_up(expr: &Expr, amount: u32) -> Expr {
    if amount == 0 {
        return expr.clone();
    }
    shift_bvars_up_at(expr, amount, 0)
}
pub(super) fn shift_bvars_up_at(expr: &Expr, amount: u32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(n) => {
            if *n >= cutoff {
                Expr::BVar(*n + amount)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f2 = shift_bvars_up_at(f, amount, cutoff);
            let a2 = shift_bvars_up_at(a, amount, cutoff);
            Expr::App(Box::new(f2), Box::new(a2))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty2 = shift_bvars_up_at(ty, amount, cutoff);
            let body2 = shift_bvars_up_at(body, amount, cutoff + 1);
            Expr::Lam(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty2 = shift_bvars_up_at(ty, amount, cutoff);
            let body2 = shift_bvars_up_at(body, amount, cutoff + 1);
            Expr::Pi(*bi, name.clone(), Box::new(ty2), Box::new(body2))
        }
        Expr::Let(name, ty, val, body) => {
            let ty2 = shift_bvars_up_at(ty, amount, cutoff);
            let val2 = shift_bvars_up_at(val, amount, cutoff);
            let body2 = shift_bvars_up_at(body, amount, cutoff + 1);
            Expr::Let(name.clone(), Box::new(ty2), Box::new(val2), Box::new(body2))
        }
        Expr::Proj(name, i, e) => {
            let e2 = shift_bvars_up_at(e, amount, cutoff);
            Expr::Proj(name.clone(), *i, Box::new(e2))
        }
        _ => expr.clone(),
    }
}
/// Eta-expand a term to have exactly `n` more arguments.
pub fn eta_expand(expr: &Expr, n: usize) -> Expr {
    if n == 0 {
        return expr.clone();
    }
    let dummy_ty = Expr::Sort(Level::Zero);
    let mut result = shift_bvars_up(expr, n as u32);
    for i in 0..n {
        result = Expr::App(Box::new(result), Box::new(Expr::BVar((n - 1 - i) as u32)));
    }
    for i in 0..n {
        result = Expr::Lam(
            BinderInfo::Default,
            oxilean_kernel::Name::str(format!("_eta{}", i)),
            Box::new(dummy_ty.clone()),
            Box::new(result),
        );
    }
    result
}
/// Collect the set of all universe level parameters in an expression.
pub fn collect_level_params(expr: &Expr) -> Vec<oxilean_kernel::Name> {
    let mut params = Vec::new();
    collect_level_params_impl(expr, &mut params);
    params
}
pub(super) fn collect_level_params_impl(expr: &Expr, params: &mut Vec<oxilean_kernel::Name>) {
    match expr {
        Expr::Sort(l) => collect_level_param_names(l, params),
        Expr::Const(_, ls) => {
            for l in ls {
                collect_level_param_names(l, params);
            }
        }
        Expr::App(f, a) => {
            collect_level_params_impl(f, params);
            collect_level_params_impl(a, params);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_level_params_impl(ty, params);
            collect_level_params_impl(body, params);
        }
        Expr::Let(_, ty, val, body) => {
            collect_level_params_impl(ty, params);
            collect_level_params_impl(val, params);
            collect_level_params_impl(body, params);
        }
        Expr::Proj(_, _, e) => collect_level_params_impl(e, params),
        _ => {}
    }
}
pub(super) fn collect_level_param_names(level: &Level, params: &mut Vec<oxilean_kernel::Name>) {
    match level {
        Level::Param(name) => {
            if !params.contains(name) {
                params.push(name.clone());
            }
        }
        Level::Succ(l) => collect_level_param_names(l, params),
        Level::Max(l1, l2) | Level::IMax(l1, l2) => {
            collect_level_param_names(l1, params);
            collect_level_param_names(l2, params);
        }
        _ => {}
    }
}
/// Depth of nesting for binders.
pub fn max_binder_depth(expr: &Expr) -> usize {
    max_binder_depth_impl(expr, 0)
}
pub(super) fn max_binder_depth_impl(expr: &Expr, depth: usize) -> usize {
    match expr {
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            let ty_depth = max_binder_depth_impl(ty, depth);
            let body_depth = max_binder_depth_impl(body, depth + 1);
            ty_depth.max(body_depth).max(depth)
        }
        Expr::Let(_, ty, val, body) => {
            let ty_depth = max_binder_depth_impl(ty, depth);
            let val_depth = max_binder_depth_impl(val, depth);
            let body_depth = max_binder_depth_impl(body, depth + 1);
            ty_depth.max(val_depth).max(body_depth).max(depth)
        }
        Expr::App(f, a) => max_binder_depth_impl(f, depth).max(max_binder_depth_impl(a, depth)),
        Expr::Proj(_, _, e) => max_binder_depth_impl(e, depth),
        _ => depth,
    }
}
/// Compute a summary of an expression's structure.
pub fn summarize_expr(expr: &Expr) -> ExprSummary {
    let mut summary = ExprSummary {
        num_lambdas: count_lambdas(expr),
        num_pis: count_pis(expr),
        total_nodes: expr_size(expr),
        num_fvars: collect_fvars(expr).len(),
        num_mvars: collect_mvars(expr).len(),
        ..Default::default()
    };
    let mut curr = expr;
    while let Expr::App(f, _) = curr {
        summary.num_apps += 1;
        curr = f;
    }
    summary
}
/// Substitute all metavariables using the given assignment map.
pub fn subst_metavars(expr: &Expr, assignments: &std::collections::HashMap<MVarId, Expr>) -> Expr {
    use crate::basic::MetaContext;
    if assignments.is_empty() {
        return expr.clone();
    }
    replace_expr(expr, &|sub| {
        if let Some(id) = MetaContext::is_mvar_expr(sub) {
            assignments.get(&id).cloned()
        } else {
            None
        }
    })
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::util::*;
    use oxilean_kernel::Name;
    fn nat_ty() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn fv(n: u64) -> FVarId {
        FVarId::new(n)
    }
    #[test]
    fn test_syntactic_eq() {
        let a = nat_ty();
        let b = nat_ty();
        assert!(syntactic_eq(&a, &b));
        assert!(!syntactic_eq(&a, &Expr::BVar(0)));
    }
    #[test]
    fn test_is_closed_bvar() {
        assert!(!is_closed(&Expr::BVar(0)));
    }
    #[test]
    fn test_is_closed_const() {
        assert!(is_closed(&nat_ty()));
    }
    #[test]
    fn test_is_closed_fvar() {
        assert!(!is_closed(&Expr::FVar(fv(1))));
    }
    #[test]
    fn test_expr_size_const() {
        assert_eq!(expr_size(&nat_ty()), 1);
    }
    #[test]
    fn test_expr_size_app() {
        let app = Expr::App(Box::new(nat_ty()), Box::new(nat_ty()));
        assert_eq!(expr_size(&app), 3);
    }
    #[test]
    fn test_collect_const_names() {
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        let names = collect_const_names(&app);
        assert_eq!(names.len(), 2);
        assert!(names.contains(&Name::str("f")));
    }
    #[test]
    fn test_flatten_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = flatten_app(&app);
        assert_eq!(head, f);
        assert_eq!(args, vec![a, b]);
    }
    #[test]
    fn test_build_app_roundtrip() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let result = build_app(f.clone(), vec![a.clone()]);
        assert_eq!(result, Expr::App(Box::new(f), Box::new(a)));
    }
    #[test]
    fn test_is_neutral_fvar() {
        assert!(is_neutral(&Expr::FVar(fv(1))));
    }
    #[test]
    fn test_is_neutral_lam() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty()),
            Box::new(Expr::BVar(0)),
        );
        assert!(!is_neutral(&lam));
    }
    #[test]
    fn test_max_binder_depth_flat() {
        assert_eq!(max_binder_depth(&nat_ty()), 0);
    }
    #[test]
    fn test_max_binder_depth_lam() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("y"),
                Box::new(nat_ty()),
                Box::new(Expr::BVar(0)),
            )),
        );
        assert!(max_binder_depth(&lam) >= 1);
    }
    #[test]
    fn test_summarize_expr_lam() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty()),
            Box::new(Expr::BVar(0)),
        );
        let summary = summarize_expr(&lam);
        assert_eq!(summary.num_lambdas, 1);
        assert_eq!(summary.num_pis, 0);
    }
    #[test]
    fn test_collect_level_params_empty() {
        let params = collect_level_params(&nat_ty());
        assert!(params.is_empty());
    }
    #[test]
    fn test_collect_level_params_param() {
        let expr = Expr::Sort(Level::Param(Name::str("u")));
        let params = collect_level_params(&expr);
        assert_eq!(params.len(), 1);
        assert_eq!(params[0], Name::str("u"));
    }
    #[test]
    fn test_subst_metavars_empty() {
        let expr = nat_ty();
        let assignments = std::collections::HashMap::new();
        let result = subst_metavars(&expr, &assignments);
        assert_eq!(result, expr);
    }
}
/// Compute a simple hash of a MetaUtil name.
#[allow(dead_code)]
pub fn metautil_hash(name: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in name.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}
/// Check if a MetaUtil name is valid.
#[allow(dead_code)]
pub fn metautil_is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
/// Count the occurrences of a character in a MetaUtil string.
#[allow(dead_code)]
pub fn metautil_count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}
/// Truncate a MetaUtil string to a maximum length.
#[allow(dead_code)]
pub fn metautil_truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}
/// Join MetaUtil strings with a separator.
#[allow(dead_code)]
pub fn metautil_join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}
#[cfg(test)]
mod metautil_ext_tests {
    use super::*;
    use crate::util::*;
    #[test]
    fn test_metautil_util_new() {
        let u = MetaUtilUtil0::new(1, "test", 42);
        assert_eq!(u.id, 1);
        assert_eq!(u.name, "test");
        assert_eq!(u.value, 42);
        assert!(u.is_active());
    }
    #[test]
    fn test_metautil_util_tag() {
        let u = MetaUtilUtil0::new(2, "tagged", 10).with_tag("important");
        assert!(u.has_tag("important"));
        assert_eq!(u.tag_count(), 1);
    }
    #[test]
    fn test_metautil_util_disable() {
        let u = MetaUtilUtil0::new(3, "disabled", 100).disable();
        assert!(!u.is_active());
        assert_eq!(u.score(), 0);
    }
    #[test]
    fn test_metautil_registry_register() {
        let mut reg = MetaUtilRegistry::new(10);
        let u = MetaUtilUtil0::new(1, "a", 1);
        assert!(reg.register(u));
        assert_eq!(reg.count(), 1);
    }
    #[test]
    fn test_metautil_registry_lookup() {
        let mut reg = MetaUtilRegistry::new(10);
        reg.register(MetaUtilUtil0::new(5, "five", 5));
        assert!(reg.lookup(5).is_some());
        assert!(reg.lookup(99).is_none());
    }
    #[test]
    fn test_metautil_registry_capacity() {
        let mut reg = MetaUtilRegistry::new(2);
        reg.register(MetaUtilUtil0::new(1, "a", 1));
        reg.register(MetaUtilUtil0::new(2, "b", 2));
        assert!(reg.is_full());
        assert!(!reg.register(MetaUtilUtil0::new(3, "c", 3)));
    }
    #[test]
    fn test_metautil_registry_score() {
        let mut reg = MetaUtilRegistry::new(10);
        reg.register(MetaUtilUtil0::new(1, "a", 10));
        reg.register(MetaUtilUtil0::new(2, "b", 20));
        assert_eq!(reg.total_score(), 30);
    }
    #[test]
    fn test_metautil_cache_hit_miss() {
        let mut cache = MetaUtilCache::new();
        cache.insert("key1", 42);
        assert_eq!(cache.get("key1"), Some(42));
        assert_eq!(cache.get("key2"), None);
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_metautil_cache_hit_rate() {
        let mut cache = MetaUtilCache::new();
        cache.insert("k", 1);
        cache.get("k");
        cache.get("k");
        cache.get("nope");
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_metautil_cache_clear() {
        let mut cache = MetaUtilCache::new();
        cache.insert("k", 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hits, 0);
    }
    #[test]
    fn test_metautil_logger_basic() {
        let mut logger = MetaUtilLogger::new(100);
        logger.log("msg1");
        logger.log("msg2");
        assert_eq!(logger.count(), 2);
        assert_eq!(logger.last(), Some("msg2"));
    }
    #[test]
    fn test_metautil_logger_capacity() {
        let mut logger = MetaUtilLogger::new(2);
        logger.log("a");
        logger.log("b");
        logger.log("c");
        assert_eq!(logger.count(), 2);
    }
    #[test]
    fn test_metautil_stats_success() {
        let mut stats = MetaUtilStats::new();
        stats.record_success(100);
        stats.record_success(200);
        assert_eq!(stats.total_ops, 2);
        assert_eq!(stats.successful_ops, 2);
        assert!((stats.success_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_metautil_stats_failure() {
        let mut stats = MetaUtilStats::new();
        stats.record_success(100);
        stats.record_failure();
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_metautil_stats_merge() {
        let mut a = MetaUtilStats::new();
        let mut b = MetaUtilStats::new();
        a.record_success(100);
        b.record_failure();
        a.merge(&b);
        assert_eq!(a.total_ops, 2);
    }
    #[test]
    fn test_metautil_priority_queue() {
        let mut pq = MetaUtilPriorityQueue::new();
        pq.push(MetaUtilUtil0::new(1, "low", 1), 1);
        pq.push(MetaUtilUtil0::new(2, "high", 2), 100);
        let (_, p) = pq.pop().expect("collection should not be empty");
        assert_eq!(p, 100);
    }
    #[test]
    fn test_metautil_hash() {
        let h1 = metautil_hash("foo");
        let h2 = metautil_hash("foo");
        assert_eq!(h1, h2);
        let h3 = metautil_hash("bar");
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_metautil_valid_name() {
        assert!(metautil_is_valid_name("foo_bar"));
        assert!(!metautil_is_valid_name("foo-bar"));
        assert!(!metautil_is_valid_name(""));
    }
    #[test]
    fn test_metautil_join() {
        let parts = ["a", "b", "c"];
        assert_eq!(metautil_join(&parts, ", "), "a, b, c");
    }
}
#[cfg(test)]
mod util_analysis_tests {
    use super::*;
    use crate::util::*;
    #[test]
    fn test_util_result_ok() {
        let r = UtilResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_util_result_err() {
        let r = UtilResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_util_result_partial() {
        let r = UtilResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_util_result_skipped() {
        let r = UtilResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_util_analysis_pass_run() {
        let mut p = UtilAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_util_analysis_pass_empty_input() {
        let mut p = UtilAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_util_analysis_pass_success_rate() {
        let mut p = UtilAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_util_analysis_pass_disable() {
        let mut p = UtilAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_util_pipeline_basic() {
        let mut pipeline = UtilPipeline::new("main_pipeline");
        pipeline.add_pass(UtilAnalysisPass::new("pass1"));
        pipeline.add_pass(UtilAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_util_pipeline_disabled_pass() {
        let mut pipeline = UtilPipeline::new("partial");
        let mut p = UtilAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(UtilAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_util_diff_basic() {
        let mut d = UtilDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_util_diff_summary() {
        let mut d = UtilDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_util_config_set_get() {
        let mut cfg = UtilConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_util_config_read_only() {
        let mut cfg = UtilConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_util_config_remove() {
        let mut cfg = UtilConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_util_diagnostics_basic() {
        let mut diag = UtilDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_util_diagnostics_max_errors() {
        let mut diag = UtilDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_util_diagnostics_clear() {
        let mut diag = UtilDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_util_config_value_types() {
        let b = UtilConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = UtilConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = UtilConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = UtilConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = UtilConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod util_ext_tests_3000 {
    use super::*;
    use crate::util::*;
    #[test]
    fn test_util_ext_result_ok_3000() {
        let r = UtilExtResult3000::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_util_ext_result_err_3000() {
        let r = UtilExtResult3000::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_util_ext_result_partial_3000() {
        let r = UtilExtResult3000::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_util_ext_result_skipped_3000() {
        let r = UtilExtResult3000::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_util_ext_pass_run_3000() {
        let mut p = UtilExtPass3000::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_util_ext_pass_empty_3000() {
        let mut p = UtilExtPass3000::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_util_ext_pass_rate_3000() {
        let mut p = UtilExtPass3000::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_util_ext_pass_disable_3000() {
        let mut p = UtilExtPass3000::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_util_ext_pipeline_basic_3000() {
        let mut pipeline = UtilExtPipeline3000::new("main_pipeline");
        pipeline.add_pass(UtilExtPass3000::new("pass1"));
        pipeline.add_pass(UtilExtPass3000::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_util_ext_pipeline_disabled_3000() {
        let mut pipeline = UtilExtPipeline3000::new("partial");
        let mut p = UtilExtPass3000::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(UtilExtPass3000::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_util_ext_diff_basic_3000() {
        let mut d = UtilExtDiff3000::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_util_ext_config_set_get_3000() {
        let mut cfg = UtilExtConfig3000::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_util_ext_config_read_only_3000() {
        let mut cfg = UtilExtConfig3000::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_util_ext_config_remove_3000() {
        let mut cfg = UtilExtConfig3000::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_util_ext_diagnostics_basic_3000() {
        let mut diag = UtilExtDiag3000::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_util_ext_diagnostics_max_errors_3000() {
        let mut diag = UtilExtDiag3000::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_util_ext_diagnostics_clear_3000() {
        let mut diag = UtilExtDiag3000::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_util_ext_config_value_types_3000() {
        let b = UtilExtConfigVal3000::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = UtilExtConfigVal3000::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = UtilExtConfigVal3000::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = UtilExtConfigVal3000::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = UtilExtConfigVal3000::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
