//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Expr, FVarId, Level, Name};

use super::types::{
    ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet, MinHeap,
    NonEmptyVec, PathBuf, PrefixCounter, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum,
    SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Extract the function head from a chain of applications.
///
/// `get_app_fn(f a1 a2 a3)` returns `f`.
pub fn get_app_fn(e: &Expr) -> &Expr {
    match e {
        Expr::App(f, _) => get_app_fn(f),
        _ => e,
    }
}
/// Extract all arguments from a chain of applications.
///
/// `get_app_args(f a1 a2 a3)` returns `[a1, a2, a3]`.
pub fn get_app_args(e: &Expr) -> Vec<&Expr> {
    let mut args = Vec::new();
    get_app_args_aux(e, &mut args);
    args
}
fn get_app_args_aux<'a>(e: &'a Expr, args: &mut Vec<&'a Expr>) {
    if let Expr::App(f, a) = e {
        get_app_args_aux(f, args);
        args.push(a);
    }
}
/// Decompose an expression into its head function and arguments.
///
/// `get_app_fn_args(f a1 a2)` returns `(f, [a1, a2])`.
pub fn get_app_fn_args(e: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let f = get_app_fn_args_aux(e, &mut args);
    (f, args)
}
fn get_app_fn_args_aux<'a>(e: &'a Expr, args: &mut Vec<&'a Expr>) -> &'a Expr {
    match e {
        Expr::App(f, a) => {
            let head = get_app_fn_args_aux(f, args);
            args.push(a);
            head
        }
        _ => e,
    }
}
/// Get the number of arguments in an application chain.
pub fn get_app_num_args(e: &Expr) -> usize {
    match e {
        Expr::App(f, _) => 1 + get_app_num_args(f),
        _ => 0,
    }
}
/// Construct an application from a function and a list of arguments.
///
/// `mk_app(f, [a1, a2, a3])` returns `f a1 a2 a3`.
pub fn mk_app(f: Expr, args: &[Expr]) -> Expr {
    args.iter().fold(f, |acc, arg| {
        Expr::App(Box::new(acc), Box::new(arg.clone()))
    })
}
/// Construct an application from a function and a range of arguments.
///
/// `mk_app_range(f, args, 1, 3)` returns `f args[1] args[2]`.
pub fn mk_app_range(f: Expr, args: &[Expr], begin: usize, end: usize) -> Expr {
    let end = end.min(args.len());
    let begin = begin.min(end);
    mk_app(f, &args[begin..end])
}
/// Check if an expression has any loose bound variables.
///
/// A bound variable is "loose" if its de Bruijn index refers to a binder
/// that is not part of the expression.
pub fn has_loose_bvars(e: &Expr) -> bool {
    has_loose_bvar_ge(e, 0)
}
/// Check if an expression has a loose bound variable with index >= `depth`.
pub fn has_loose_bvar_ge(e: &Expr, depth: u32) -> bool {
    match e {
        Expr::BVar(n) => *n >= depth,
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_loose_bvar_ge(f, depth) || has_loose_bvar_ge(a, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_loose_bvar_ge(ty, depth) || has_loose_bvar_ge(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            has_loose_bvar_ge(ty, depth)
                || has_loose_bvar_ge(val, depth)
                || has_loose_bvar_ge(body, depth + 1)
        }
        Expr::Proj(_, _, e) => has_loose_bvar_ge(e, depth),
    }
}
/// Check if an expression has a specific loose bound variable at `level`.
pub fn has_loose_bvar(e: &Expr, level: u32) -> bool {
    match e {
        Expr::BVar(n) => *n == level,
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_loose_bvar(f, level) || has_loose_bvar(a, level),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_loose_bvar(ty, level) || has_loose_bvar(body, level + 1)
        }
        Expr::Let(_, ty, val, body) => {
            has_loose_bvar(ty, level)
                || has_loose_bvar(val, level)
                || has_loose_bvar(body, level + 1)
        }
        Expr::Proj(_, _, e) => has_loose_bvar(e, level),
    }
}
/// Check if an expression contains a specific free variable.
pub fn has_fvar(e: &Expr, fvar: FVarId) -> bool {
    match e {
        Expr::FVar(id) => *id == fvar,
        Expr::Sort(_) | Expr::BVar(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_fvar(f, fvar) || has_fvar(a, fvar),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_fvar(ty, fvar) || has_fvar(body, fvar)
        }
        Expr::Let(_, ty, val, body) => {
            has_fvar(ty, fvar) || has_fvar(val, fvar) || has_fvar(body, fvar)
        }
        Expr::Proj(_, _, e) => has_fvar(e, fvar),
    }
}
/// Check if an expression contains any free variables.
pub fn has_any_fvar(e: &Expr) -> bool {
    match e {
        Expr::FVar(_) => true,
        Expr::Sort(_) | Expr::BVar(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_any_fvar(f) || has_any_fvar(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_any_fvar(ty) || has_any_fvar(body)
        }
        Expr::Let(_, ty, val, body) => has_any_fvar(ty) || has_any_fvar(val) || has_any_fvar(body),
        Expr::Proj(_, _, e) => has_any_fvar(e),
    }
}
/// Traverse every sub-expression, calling `f` on each.
///
/// The callback receives the expression and the current binding depth.
/// Return `false` from `f` to stop traversal early.
pub fn for_each_expr(e: &Expr, f: &mut dyn FnMut(&Expr, u32) -> bool) {
    for_each_expr_aux(e, f, 0);
}
fn for_each_expr_aux(e: &Expr, f: &mut dyn FnMut(&Expr, u32) -> bool, depth: u32) {
    if !f(e, depth) {
        return;
    }
    match e {
        Expr::App(fun, arg) => {
            for_each_expr_aux(fun, f, depth);
            for_each_expr_aux(arg, f, depth);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            for_each_expr_aux(ty, f, depth);
            for_each_expr_aux(body, f, depth + 1);
        }
        Expr::Let(_, ty, val, body) => {
            for_each_expr_aux(ty, f, depth);
            for_each_expr_aux(val, f, depth);
            for_each_expr_aux(body, f, depth + 1);
        }
        Expr::Proj(_, _, e) => {
            for_each_expr_aux(e, f, depth);
        }
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => {}
    }
}
/// Replace sub-expressions using a callback.
///
/// The callback receives the expression and binding depth.
/// Return `Some(replacement)` to replace, or `None` to recurse.
pub fn replace_expr(e: &Expr, f: &mut dyn FnMut(&Expr, u32) -> Option<Expr>) -> Expr {
    replace_expr_aux(e, f, 0)
}
fn replace_expr_aux(e: &Expr, f: &mut dyn FnMut(&Expr, u32) -> Option<Expr>, depth: u32) -> Expr {
    if let Some(replacement) = f(e, depth) {
        return replacement;
    }
    match e {
        Expr::App(fun, arg) => {
            let new_fun = replace_expr_aux(fun, f, depth);
            let new_arg = replace_expr_aux(arg, f, depth);
            Expr::App(Box::new(new_fun), Box::new(new_arg))
        }
        Expr::Lam(bi, name, ty, body) => {
            let new_ty = replace_expr_aux(ty, f, depth);
            let new_body = replace_expr_aux(body, f, depth + 1);
            Expr::Lam(*bi, name.clone(), Box::new(new_ty), Box::new(new_body))
        }
        Expr::Pi(bi, name, ty, body) => {
            let new_ty = replace_expr_aux(ty, f, depth);
            let new_body = replace_expr_aux(body, f, depth + 1);
            Expr::Pi(*bi, name.clone(), Box::new(new_ty), Box::new(new_body))
        }
        Expr::Let(name, ty, val, body) => {
            let new_ty = replace_expr_aux(ty, f, depth);
            let new_val = replace_expr_aux(val, f, depth);
            let new_body = replace_expr_aux(body, f, depth + 1);
            Expr::Let(
                name.clone(),
                Box::new(new_ty),
                Box::new(new_val),
                Box::new(new_body),
            )
        }
        Expr::Proj(name, idx, e_inner) => {
            let new_e = replace_expr_aux(e_inner, f, depth);
            Expr::Proj(name.clone(), *idx, Box::new(new_e))
        }
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => {
            e.clone()
        }
    }
}
/// Collect all free variables in an expression.
pub fn collect_fvars(e: &Expr) -> Vec<FVarId> {
    let mut fvars = Vec::new();
    for_each_expr(e, &mut |sub, _depth| {
        if let Expr::FVar(id) = sub {
            if !fvars.contains(id) {
                fvars.push(*id);
            }
        }
        true
    });
    fvars
}
/// Collect all constant names referenced in an expression.
pub fn collect_consts(e: &Expr) -> Vec<Name> {
    let mut consts = Vec::new();
    for_each_expr(e, &mut |sub, _depth| {
        if let Expr::Const(name, _) = sub {
            if !consts.contains(name) {
                consts.push(name.clone());
            }
        }
        true
    });
    consts
}
/// Lift (shift) loose bound variables by `n`.
///
/// Increments the index of all loose bound variables (with index >= `offset`)
/// by `n`. Used when inserting an expression under additional binders.
pub fn lift_loose_bvars(e: &Expr, n: u32, offset: u32) -> Expr {
    if n == 0 {
        return e.clone();
    }
    lift_loose_bvars_aux(e, n, offset)
}
fn lift_loose_bvars_aux(e: &Expr, n: u32, depth: u32) -> Expr {
    match e {
        Expr::BVar(idx) => {
            if *idx >= depth {
                Expr::BVar(idx + n)
            } else {
                e.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => e.clone(),
        Expr::App(f, a) => {
            let f_new = lift_loose_bvars_aux(f, n, depth);
            let a_new = lift_loose_bvars_aux(a, n, depth);
            Expr::App(Box::new(f_new), Box::new(a_new))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty_new = lift_loose_bvars_aux(ty, n, depth);
            let body_new = lift_loose_bvars_aux(body, n, depth + 1);
            Expr::Lam(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_new = lift_loose_bvars_aux(ty, n, depth);
            let body_new = lift_loose_bvars_aux(body, n, depth + 1);
            Expr::Pi(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Let(name, ty, val, body) => {
            let ty_new = lift_loose_bvars_aux(ty, n, depth);
            let val_new = lift_loose_bvars_aux(val, n, depth);
            let body_new = lift_loose_bvars_aux(body, n, depth + 1);
            Expr::Let(
                name.clone(),
                Box::new(ty_new),
                Box::new(val_new),
                Box::new(body_new),
            )
        }
        Expr::Proj(name, idx, inner) => {
            let inner_new = lift_loose_bvars_aux(inner, n, depth);
            Expr::Proj(name.clone(), *idx, Box::new(inner_new))
        }
    }
}
/// Check if a constant name occurs anywhere in an expression.
pub fn occurs_const(e: &Expr, name: &Name) -> bool {
    match e {
        Expr::Const(n, _) => n == name,
        Expr::App(f, a) => occurs_const(f, name) || occurs_const(a, name),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            occurs_const(ty, name) || occurs_const(body, name)
        }
        Expr::Let(_, ty, val, body) => {
            occurs_const(ty, name) || occurs_const(val, name) || occurs_const(body, name)
        }
        Expr::Proj(_, _, e) => occurs_const(e, name),
        _ => false,
    }
}
/// Count the number of leading lambdas.
pub fn count_lambdas(e: &Expr) -> u32 {
    match e {
        Expr::Lam(_, _, _, body) => 1 + count_lambdas(body),
        _ => 0,
    }
}
/// Count the number of leading Pi binders.
pub fn count_pis(e: &Expr) -> u32 {
    match e {
        Expr::Pi(_, _, _, body) => 1 + count_pis(body),
        _ => 0,
    }
}
/// Get the body after stripping `n` leading lambdas.
pub fn strip_lambdas(e: &Expr, n: u32) -> &Expr {
    if n == 0 {
        return e;
    }
    match e {
        Expr::Lam(_, _, _, body) => strip_lambdas(body, n - 1),
        _ => e,
    }
}
/// Get the body after stripping `n` leading Pis.
pub fn strip_pis(e: &Expr, n: u32) -> &Expr {
    if n == 0 {
        return e;
    }
    match e {
        Expr::Pi(_, _, _, body) => strip_pis(body, n - 1),
        _ => e,
    }
}
/// Check if a level has metavariables.
pub fn level_has_mvar(l: &Level) -> bool {
    match l {
        Level::MVar(_) => true,
        Level::Succ(l) => level_has_mvar(l),
        Level::Max(l1, l2) | Level::IMax(l1, l2) => level_has_mvar(l1) || level_has_mvar(l2),
        Level::Zero | Level::Param(_) => false,
    }
}
/// Check if an expression has universe level metavariables.
pub fn has_level_mvar(e: &Expr) -> bool {
    match e {
        Expr::Sort(l) => level_has_mvar(l),
        Expr::Const(_, ls) => ls.iter().any(level_has_mvar),
        Expr::App(f, a) => has_level_mvar(f) || has_level_mvar(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_level_mvar(ty) || has_level_mvar(body)
        }
        Expr::Let(_, ty, val, body) => {
            has_level_mvar(ty) || has_level_mvar(val) || has_level_mvar(body)
        }
        Expr::Proj(_, _, e) => has_level_mvar(e),
        Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => false,
    }
}
/// Check if an expression contains any level parameters.
pub fn has_level_param(e: &Expr) -> bool {
    match e {
        Expr::Sort(l) => level_has_param(l),
        Expr::Const(_, ls) => ls.iter().any(level_has_param),
        Expr::App(f, a) => has_level_param(f) || has_level_param(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_level_param(ty) || has_level_param(body)
        }
        Expr::Let(_, ty, val, body) => {
            has_level_param(ty) || has_level_param(val) || has_level_param(body)
        }
        Expr::Proj(_, _, e) => has_level_param(e),
        Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => false,
    }
}
fn level_has_param(l: &Level) -> bool {
    match l {
        Level::Param(_) => true,
        Level::Succ(l) => level_has_param(l),
        Level::Max(l1, l2) | Level::IMax(l1, l2) => level_has_param(l1) || level_has_param(l2),
        Level::Zero | Level::MVar(_) => false,
    }
}
/// Compute the "weight" of an expression (rough size metric).
pub fn expr_weight(e: &Expr) -> usize {
    match e {
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => 1,
        Expr::Const(_, _) => 1,
        Expr::App(f, a) => 1 + expr_weight(f) + expr_weight(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            1 + expr_weight(ty) + expr_weight(body)
        }
        Expr::Let(_, ty, val, body) => 1 + expr_weight(ty) + expr_weight(val) + expr_weight(body),
        Expr::Proj(_, _, e) => 1 + expr_weight(e),
    }
}
/// Check if the head of an application is a constant with a specific name.
pub fn is_app_of(e: &Expr, name: &Name) -> bool {
    matches!(get_app_fn(e), Expr::Const(n, _) if n == name)
}
/// Build a non-dependent arrow type: `a → b`.
pub fn mk_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        crate::BinderInfo::Default,
        Name::Anonymous,
        Box::new(a),
        Box::new(lift_loose_bvars(&b, 1, 0)),
    )
}
/// Build `Sort 0` (Prop).
pub fn mk_prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Build `Sort (u + 1)` (Type u).
pub fn mk_type(u: Level) -> Expr {
    Expr::Sort(Level::succ(u))
}
/// Build `Sort 1` (Type 0).
pub fn mk_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Shorthand: create a constant expression.
pub fn var(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
/// Shorthand: create a bound variable.
pub fn bvar(idx: u32) -> Expr {
    Expr::BVar(idx)
}
/// Shorthand: create an application.
pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Shorthand: create a lambda.
pub fn lam(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Lam(
        crate::BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Shorthand: create a pi type.
pub fn pi(name: &str, ty: Expr, body: Expr) -> Expr {
    Expr::Pi(
        crate::BinderInfo::Default,
        Name::str(name),
        Box::new(ty),
        Box::new(body),
    )
}
/// Shorthand: `Prop` = `Sort 0`.
pub fn prop() -> Expr {
    Expr::Sort(crate::Level::zero())
}
/// Shorthand: `Sort(Level::zero())`.
pub fn sort() -> Expr {
    Expr::Sort(crate::Level::zero())
}
/// Shorthand: `Type 0` = `Sort 1`.
pub fn type0() -> Expr {
    Expr::Sort(crate::Level::succ(crate::Level::zero()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::BinderInfo;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn bool_ty() -> Expr {
        Expr::Const(Name::str("Bool"), vec![])
    }
    #[test]
    fn test_get_app_fn() {
        let f = nat();
        let e = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(Expr::BVar(0)))),
            Box::new(Expr::BVar(1)),
        );
        assert_eq!(get_app_fn(&e), &f);
    }
    #[test]
    fn test_get_app_args() {
        let f = nat();
        let a1 = Expr::BVar(0);
        let a2 = Expr::BVar(1);
        let e = Expr::App(
            Box::new(Expr::App(Box::new(f), Box::new(a1.clone()))),
            Box::new(a2.clone()),
        );
        let args = get_app_args(&e);
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], &a1);
        assert_eq!(args[1], &a2);
    }
    #[test]
    fn test_get_app_fn_args() {
        let f = nat();
        let a = Expr::BVar(0);
        let e = Expr::App(Box::new(f.clone()), Box::new(a.clone()));
        let (head, args) = get_app_fn_args(&e);
        assert_eq!(head, &f);
        assert_eq!(args, vec![&a]);
    }
    #[test]
    fn test_mk_app() {
        let f = nat();
        let a1 = Expr::BVar(0);
        let a2 = Expr::BVar(1);
        let result = mk_app(f.clone(), &[a1.clone(), a2.clone()]);
        let (head, args) = get_app_fn_args(&result);
        assert_eq!(head, &f);
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_has_loose_bvars() {
        assert!(has_loose_bvars(&Expr::BVar(0)));
        assert!(!has_loose_bvars(&nat()));
        assert!(!has_loose_bvars(&Expr::FVar(FVarId(1))));
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        assert!(!has_loose_bvars(&lam));
        let lam2 = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(1)),
        );
        assert!(has_loose_bvars(&lam2));
    }
    #[test]
    fn test_has_fvar() {
        let id = FVarId(42);
        let e = Expr::App(Box::new(Expr::FVar(id)), Box::new(Expr::FVar(FVarId(99))));
        assert!(has_fvar(&e, id));
        assert!(has_fvar(&e, FVarId(99)));
        assert!(!has_fvar(&e, FVarId(1)));
    }
    #[test]
    fn test_collect_fvars() {
        let e = Expr::App(
            Box::new(Expr::FVar(FVarId(1))),
            Box::new(Expr::App(
                Box::new(Expr::FVar(FVarId(2))),
                Box::new(Expr::FVar(FVarId(1))),
            )),
        );
        let fvars = collect_fvars(&e);
        assert_eq!(fvars.len(), 2);
        assert!(fvars.contains(&FVarId(1)));
        assert!(fvars.contains(&FVarId(2)));
    }
    #[test]
    fn test_for_each_expr() {
        let e = Expr::App(Box::new(nat()), Box::new(Expr::BVar(0)));
        let mut count = 0;
        for_each_expr(&e, &mut |_, _| {
            count += 1;
            true
        });
        assert_eq!(count, 3);
    }
    #[test]
    fn test_replace_expr() {
        let e = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let result = replace_expr(&e, &mut |sub, _depth| {
            if let Expr::BVar(0) = sub {
                Some(nat())
            } else {
                None
            }
        });
        match &result {
            Expr::App(f, _) => assert_eq!(**f, nat()),
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_lift_loose_bvars() {
        let e = Expr::BVar(0);
        let lifted = lift_loose_bvars(&e, 2, 0);
        assert_eq!(lifted, Expr::BVar(2));
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::BVar(0)),
        );
        let lifted_lam = lift_loose_bvars(&lam, 1, 0);
        if let Expr::Lam(_, _, _, body) = &lifted_lam {
            assert_eq!(**body, Expr::BVar(0));
        } else {
            panic!("Expected Lam");
        }
    }
    #[test]
    fn test_count_lambdas_pis() {
        let e = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(Expr::Lam(
                BinderInfo::Default,
                Name::str("y"),
                Box::new(nat()),
                Box::new(Expr::BVar(0)),
            )),
        );
        assert_eq!(count_lambdas(&e), 2);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat()),
            Box::new(nat()),
        );
        assert_eq!(count_pis(&pi), 1);
    }
    #[test]
    fn test_is_app_of() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert!(is_app_of(&e, &Name::str("f")));
        assert!(!is_app_of(&e, &Name::str("g")));
    }
    #[test]
    fn test_mk_arrow() {
        let arrow = mk_arrow(nat(), bool_ty());
        assert!(arrow.is_pi());
    }
    #[test]
    fn test_expr_weight() {
        assert_eq!(expr_weight(&nat()), 1);
        let app = Expr::App(Box::new(nat()), Box::new(Expr::BVar(0)));
        assert_eq!(expr_weight(&app), 3);
    }
    #[test]
    fn test_mk_app_range() {
        let f = nat();
        let args = vec![Expr::BVar(0), Expr::BVar(1), Expr::BVar(2)];
        let result = mk_app_range(f.clone(), &args, 1, 3);
        assert_eq!(get_app_num_args(&result), 2);
    }
    #[test]
    fn test_occurs_const() {
        let e = Expr::App(Box::new(nat()), Box::new(bool_ty()));
        assert!(occurs_const(&e, &Name::str("Nat")));
        assert!(occurs_const(&e, &Name::str("Bool")));
        assert!(!occurs_const(&e, &Name::str("Int")));
    }
}
/// Check if the expression is a `Sort`.
#[inline]
pub fn is_sort(e: &Expr) -> bool {
    matches!(e, Expr::Sort(_))
}
/// Check if the expression is `Sort(Level::Zero)` (i.e. `Prop`).
#[inline]
pub fn is_prop(e: &Expr) -> bool {
    matches!(e, Expr::Sort(l) if matches!(l, Level::Zero))
}
/// Check if the expression is `Sort(Level::Succ(Level::Zero))` (i.e. `Type 0`).
#[inline]
pub fn is_type0(e: &Expr) -> bool {
    matches!(e, Expr::Sort(Level::Succ(inner)) if matches!(inner.as_ref(), Level::Zero))
}
/// Check if the expression is a lambda abstraction.
#[inline]
pub fn is_lambda(e: &Expr) -> bool {
    matches!(e, Expr::Lam(_, _, _, _))
}
/// Check if the expression is a Pi type.
#[inline]
pub fn is_pi(e: &Expr) -> bool {
    matches!(e, Expr::Pi(_, _, _, _))
}
/// Check if the expression is an FVar.
#[inline]
pub fn is_fvar(e: &Expr) -> bool {
    matches!(e, Expr::FVar(_))
}
/// Check if the expression is a BVar.
#[inline]
pub fn is_bvar(e: &Expr) -> bool {
    matches!(e, Expr::BVar(_))
}
/// Check if the expression is a Literal.
#[inline]
pub fn is_literal(e: &Expr) -> bool {
    matches!(e, Expr::Lit(_))
}
/// Check if the expression is a Const.
#[inline]
pub fn is_const(e: &Expr) -> bool {
    matches!(e, Expr::Const(_, _))
}
/// Check if the expression is an App.
#[inline]
pub fn is_app(e: &Expr) -> bool {
    matches!(e, Expr::App(_, _))
}
/// Check if the expression is a Let.
#[inline]
pub fn is_let(e: &Expr) -> bool {
    matches!(e, Expr::Let(_, _, _, _))
}
/// Check if the expression is a Proj.
#[inline]
pub fn is_proj(e: &Expr) -> bool {
    matches!(e, Expr::Proj(_, _, _))
}
/// Extract the level from a Sort expression.
#[inline]
pub fn get_sort_level(e: &Expr) -> Option<&Level> {
    if let Expr::Sort(l) = e {
        Some(l)
    } else {
        None
    }
}
/// Extract the name from a Const expression.
#[inline]
pub fn get_const_name(e: &Expr) -> Option<&Name> {
    if let Expr::Const(n, _) = e {
        Some(n)
    } else {
        None
    }
}
/// Extract the universe levels from a Const expression.
#[inline]
pub fn get_const_levels(e: &Expr) -> Option<&[Level]> {
    if let Expr::Const(_, ls) = e {
        Some(ls)
    } else {
        None
    }
}
/// Extract the FVarId from an FVar expression.
#[inline]
pub fn get_fvar_id(e: &Expr) -> Option<FVarId> {
    if let Expr::FVar(id) = e {
        Some(*id)
    } else {
        None
    }
}
/// Extract the BVar index from a BVar expression.
#[inline]
pub fn get_bvar_idx(e: &Expr) -> Option<u32> {
    if let Expr::BVar(i) = e {
        Some(*i)
    } else {
        None
    }
}
/// Decompose a Let expression into `(type, value, body)`.
#[inline]
pub fn decompose_let(e: &Expr) -> Option<(&Expr, &Expr, &Expr)> {
    if let Expr::Let(_, ty, val, body) = e {
        Some((ty, val, body))
    } else {
        None
    }
}
/// Decompose a Pi expression into `(binder_info, domain, codomain)`.
#[inline]
pub fn decompose_pi(e: &Expr) -> Option<(crate::BinderInfo, &Expr, &Expr)> {
    if let Expr::Pi(bi, _, dom, cod) = e {
        Some((*bi, dom, cod))
    } else {
        None
    }
}
/// Decompose a Lam expression into `(binder_info, domain, body)`.
#[inline]
pub fn decompose_lam(e: &Expr) -> Option<(crate::BinderInfo, &Expr, &Expr)> {
    if let Expr::Lam(bi, _, dom, body) = e {
        Some((*bi, dom, body))
    } else {
        None
    }
}
/// Decompose a Proj expression into `(struct_name, field_index, value)`.
#[inline]
pub fn decompose_proj(e: &Expr) -> Option<(&Name, usize, &Expr)> {
    if let Expr::Proj(n, idx, val) = e {
        Some((n, *idx as usize, val))
    } else {
        None
    }
}
/// Build a nested Pi type from a list of binders and a return type.
///
/// `mk_pi_n([(bi1, ty1), ..., (bin, tyn)], ret)` produces
/// `Pi bi1 ty1 (Pi bi2 ty2 (... (Pi bin tyn ret) ...))`.
pub fn mk_pi_n(binders: &[(crate::BinderInfo, Expr)], ret: Expr) -> Expr {
    binders.iter().rev().fold(ret, |acc, (bi, ty)| {
        Expr::Pi(
            *bi,
            crate::Name::Anonymous,
            Box::new(ty.clone()),
            Box::new(acc),
        )
    })
}
/// Build a nested Lam abstraction from a list of binders and a body.
pub fn mk_lam_n(binders: &[(crate::BinderInfo, Expr)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (bi, ty)| {
        Expr::Lam(
            *bi,
            crate::Name::Anonymous,
            Box::new(ty.clone()),
            Box::new(acc),
        )
    })
}
/// Check if `e` is an application of FVar `id`.
pub fn is_app_of_fvar(e: &Expr, id: FVarId) -> bool {
    get_app_fn(e) == &Expr::FVar(id)
}
/// Get the `n`-th argument in an application chain (0-indexed).
pub fn get_nth_arg(e: &Expr, n: usize) -> Option<&Expr> {
    let args = get_app_args(e);
    args.get(n).copied()
}
/// Check if an expression is "simple" (atomic: no subexpressions).
pub fn is_simple(e: &Expr) -> bool {
    matches!(
        e,
        Expr::Sort(_) | Expr::BVar(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_)
    )
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::{BinderInfo, Level};
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn prop() -> Expr {
        Expr::Sort(Level::Zero)
    }
    fn bv(i: u32) -> Expr {
        Expr::BVar(i)
    }
    #[test]
    fn test_is_prop() {
        assert!(is_prop(&prop()));
        assert!(!is_prop(&nat()));
    }
    #[test]
    fn test_is_sort() {
        assert!(is_sort(&prop()));
        assert!(is_sort(&Expr::Sort(Level::Succ(Box::new(Level::Zero)))));
        assert!(!is_sort(&nat()));
    }
    #[test]
    fn test_is_lambda_pi() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            crate::Name::Anonymous,
            Box::new(prop()),
            Box::new(bv(0)),
        );
        let pi = Expr::Pi(
            BinderInfo::Default,
            crate::Name::Anonymous,
            Box::new(prop()),
            Box::new(bv(0)),
        );
        assert!(is_lambda(&lam));
        assert!(!is_lambda(&pi));
        assert!(is_pi(&pi));
        assert!(!is_pi(&lam));
    }
    #[test]
    fn test_is_fvar_bvar() {
        use crate::FVarId;
        assert!(is_fvar(&Expr::FVar(FVarId(0))));
        assert!(is_bvar(&bv(0)));
        assert!(!is_fvar(&nat()));
    }
    #[test]
    fn test_get_sort_level() {
        let s = Expr::Sort(Level::Zero);
        assert_eq!(get_sort_level(&s), Some(&Level::Zero));
        assert_eq!(get_sort_level(&nat()), None);
    }
    #[test]
    fn test_get_const_name() {
        assert_eq!(get_const_name(&nat()), Some(&Name::str("Nat")));
        assert_eq!(get_const_name(&prop()), None);
    }
    #[test]
    fn test_get_bvar_idx() {
        assert_eq!(get_bvar_idx(&bv(3)), Some(3));
        assert_eq!(get_bvar_idx(&nat()), None);
    }
    #[test]
    fn test_decompose_let() {
        let e = Expr::Let(
            crate::Name::Anonymous,
            Box::new(nat()),
            Box::new(bv(0)),
            Box::new(bv(0)),
        );
        let (ty, val, body) = decompose_let(&e).expect("value should be present");
        assert_eq!(ty, &nat());
        assert_eq!(val, &bv(0));
        assert_eq!(body, &bv(0));
    }
    #[test]
    fn test_decompose_pi() {
        let e = Expr::Pi(
            BinderInfo::Default,
            crate::Name::Anonymous,
            Box::new(nat()),
            Box::new(prop()),
        );
        let (bi, dom, cod) = decompose_pi(&e).expect("value should be present");
        assert_eq!(bi, BinderInfo::Default);
        assert_eq!(dom, &nat());
        assert_eq!(cod, &prop());
    }
    #[test]
    fn test_decompose_lam() {
        let e = Expr::Lam(
            BinderInfo::Implicit,
            crate::Name::Anonymous,
            Box::new(nat()),
            Box::new(bv(0)),
        );
        let (bi, dom, body) = decompose_lam(&e).expect("value should be present");
        assert_eq!(bi, BinderInfo::Implicit);
        assert_eq!(dom, &nat());
        assert_eq!(body, &bv(0));
    }
    #[test]
    fn test_mk_pi_n_empty() {
        let ret = nat();
        let result = mk_pi_n(&[], ret.clone());
        assert_eq!(result, ret);
    }
    #[test]
    fn test_mk_pi_n_two() {
        let binders = vec![(BinderInfo::Default, nat()), (BinderInfo::Default, prop())];
        let ret = bv(0);
        let result = mk_pi_n(&binders, ret);
        assert!(is_pi(&result));
        let (_, _, inner) = decompose_pi(&result).expect("value should be present");
        assert!(is_pi(inner));
    }
    #[test]
    fn test_mk_lam_n_two() {
        let binders = vec![(BinderInfo::Default, nat()), (BinderInfo::Implicit, prop())];
        let result = mk_lam_n(&binders, bv(0));
        assert!(is_lambda(&result));
    }
    #[test]
    fn test_is_simple() {
        assert!(is_simple(&nat()));
        assert!(is_simple(&bv(0)));
        assert!(is_simple(&prop()));
        let app = Expr::App(Box::new(nat()), Box::new(bv(0)));
        assert!(!is_simple(&app));
    }
    #[test]
    fn test_get_nth_arg() {
        let f = nat();
        let arg0 = bv(0);
        let arg1 = bv(1);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f), Box::new(arg0.clone()))),
            Box::new(arg1.clone()),
        );
        assert_eq!(get_nth_arg(&app, 0), Some(&arg0));
        assert_eq!(get_nth_arg(&app, 1), Some(&arg1));
        assert_eq!(get_nth_arg(&app, 2), None);
    }
    #[test]
    fn test_is_literal() {
        use crate::Literal;
        let lit = Expr::Lit(Literal::Nat(42));
        assert!(is_literal(&lit));
        assert!(!is_literal(&nat()));
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
#[cfg(test)]
mod tests_final_padding {
    use super::*;
    #[test]
    fn test_min_heap() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1u32);
        h.push(3u32);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert!(h.is_empty());
    }
    #[test]
    fn test_prefix_counter() {
        let mut pc = PrefixCounter::new();
        pc.record("hello");
        pc.record("help");
        pc.record("world");
        assert_eq!(pc.count_with_prefix("hel"), 2);
        assert_eq!(pc.count_with_prefix("wor"), 1);
        assert_eq!(pc.count_with_prefix("xyz"), 0);
    }
    #[test]
    fn test_fixture() {
        let mut f = Fixture::new();
        f.set("key1", "val1");
        f.set("key2", "val2");
        assert_eq!(f.get("key1"), Some("val1"));
        assert_eq!(f.get("key3"), None);
        assert_eq!(f.len(), 2);
    }
}
