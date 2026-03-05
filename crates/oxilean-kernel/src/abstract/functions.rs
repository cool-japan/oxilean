//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::expr_util::has_any_fvar;
use crate::subst::instantiate;
use crate::{Expr, FVarId, Name};

use super::types::{
    ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet, MinHeap,
    NonEmptyVec, PathBuf, PrefixCounter, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum,
    SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat,
    TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Abstract multiple free variables simultaneously.
///
/// `abstract_fvars(e, [x, y, z])` replaces:
/// - `x` (at subst\[0\]) -> BVar(2) (outermost)
/// - `y` (at subst\[1\]) -> BVar(1) (middle)
/// - `z` (at subst\[2\]) -> BVar(0) (innermost)
///
/// This follows LEAN 4's convention where elements at the end of the
/// array become innermost binders.
pub fn abstract_fvars(expr: &Expr, fvars: &[FVarId]) -> Expr {
    if fvars.is_empty() || !has_any_fvar(expr) {
        return expr.clone();
    }
    abstract_fvars_at(expr, fvars, 0)
}
fn abstract_fvars_at(expr: &Expr, fvars: &[FVarId], offset: u32) -> Expr {
    match expr {
        Expr::FVar(id) => {
            let n = fvars.len();
            for (i, fv) in fvars.iter().enumerate().rev() {
                if id == fv {
                    return Expr::BVar(offset + (n - i - 1) as u32);
                }
            }
            for (i, fv) in fvars.iter().enumerate() {
                if id == fv {
                    return Expr::BVar(offset + (n - i - 1) as u32);
                }
            }
            expr.clone()
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => {
            let f_new = abstract_fvars_at(f, fvars, offset);
            let a_new = abstract_fvars_at(a, fvars, offset);
            Expr::App(Box::new(f_new), Box::new(a_new))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty_new = abstract_fvars_at(ty, fvars, offset);
            let body_new = abstract_fvars_at(body, fvars, offset + 1);
            Expr::Lam(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_new = abstract_fvars_at(ty, fvars, offset);
            let body_new = abstract_fvars_at(body, fvars, offset + 1);
            Expr::Pi(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Let(name, ty, val, body) => {
            let ty_new = abstract_fvars_at(ty, fvars, offset);
            let val_new = abstract_fvars_at(val, fvars, offset);
            let body_new = abstract_fvars_at(body, fvars, offset + 1);
            Expr::Let(
                name.clone(),
                Box::new(ty_new),
                Box::new(val_new),
                Box::new(body_new),
            )
        }
        Expr::Proj(name, idx, e) => {
            let e_new = abstract_fvars_at(e, fvars, offset);
            Expr::Proj(name.clone(), *idx, Box::new(e_new))
        }
    }
}
/// Abstract a single free variable, converting it to BVar(0).
///
/// Equivalent to `abstract_fvars(expr, [fvar])` but more efficient.
pub fn abstract_single(expr: &Expr, fvar: FVarId) -> Expr {
    crate::subst::abstract_expr(expr, fvar)
}
/// Cheap beta reduction: reduce `(λ x. body) arg` to `body[arg/x]`.
///
/// This only reduces the outermost beta-redex. Does not perform
/// WHNF or any other reduction.
pub fn cheap_beta_reduce(expr: &Expr) -> Expr {
    match expr {
        Expr::App(f, a) => {
            if let Expr::Lam(_, _, _, body) = f.as_ref() {
                let result = instantiate(body, a);
                cheap_beta_reduce(&result)
            } else {
                expr.clone()
            }
        }
        _ => expr.clone(),
    }
}
/// Apply multiple arguments to a lambda, performing beta reduction.
///
/// `apply_beta(λ x y. body, [a, b])` reduces to `body[a/x, b/y]`.
pub fn apply_beta(mut expr: Expr, args: &[Expr]) -> Expr {
    for arg in args {
        match expr {
            Expr::Lam(_, _, _, body) => {
                expr = instantiate(&body, arg);
            }
            _ => {
                expr = Expr::App(Box::new(expr), Box::new(arg.clone()));
            }
        }
    }
    expr
}
/// Build a lambda abstraction from free variables.
///
/// Given `body` and free variables with their types `[(x, A), (y, B)]`,
/// constructs `λ (x : A) (y : B). body[x→#1, y→#0]`.
pub fn mk_lambda(fvars: &[(FVarId, Name, Expr)], body: &Expr) -> Expr {
    let fvar_ids: Vec<FVarId> = fvars.iter().map(|(id, _, _)| *id).collect();
    let mut result = abstract_fvars(body, &fvar_ids);
    for (_, name, ty) in fvars.iter().rev() {
        let ty_abs = abstract_fvars(ty, &fvar_ids);
        result = Expr::Lam(
            crate::BinderInfo::Default,
            name.clone(),
            Box::new(ty_abs),
            Box::new(result),
        );
    }
    result
}
/// Build a Pi type from free variables.
///
/// Given `body` and free variables with their types `[(x, A), (y, B)]`,
/// constructs `Π (x : A) (y : B). body[x→#1, y→#0]`.
pub fn mk_forall(fvars: &[(FVarId, Name, Expr)], body: &Expr) -> Expr {
    let fvar_ids: Vec<FVarId> = fvars.iter().map(|(id, _, _)| *id).collect();
    let mut result = abstract_fvars(body, &fvar_ids);
    for (_, name, ty) in fvars.iter().rev() {
        let ty_abs = abstract_fvars(ty, &fvar_ids);
        result = Expr::Pi(
            crate::BinderInfo::Default,
            name.clone(),
            Box::new(ty_abs),
            Box::new(result),
        );
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, Level};
    #[test]
    fn test_abstract_fvars_single() {
        let fvar = FVarId(42);
        let e = Expr::FVar(fvar);
        let result = abstract_fvars(&e, &[fvar]);
        assert_eq!(result, Expr::BVar(0));
    }
    #[test]
    fn test_abstract_fvars_multiple() {
        let x = FVarId(1);
        let y = FVarId(2);
        let z = FVarId(3);
        let e = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("f"), vec![])),
                    Box::new(Expr::FVar(x)),
                )),
                Box::new(Expr::FVar(y)),
            )),
            Box::new(Expr::FVar(z)),
        );
        let result = abstract_fvars(&e, &[x, y, z]);
        let expected = Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("f"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
                Box::new(Expr::BVar(1)),
            )),
            Box::new(Expr::BVar(0)),
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn test_abstract_fvars_under_binder() {
        let x = FVarId(1);
        let e = Expr::Lam(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("f"), vec![])),
                Box::new(Expr::FVar(x)),
            )),
        );
        let result = abstract_fvars(&e, &[x]);
        if let Expr::Lam(_, _, _, body) = &result {
            if let Expr::App(_, arg) = body.as_ref() {
                assert_eq!(**arg, Expr::BVar(1));
            } else {
                panic!("Expected App");
            }
        } else {
            panic!("Expected Lam");
        }
    }
    #[test]
    fn test_abstract_no_fvars() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let result = abstract_fvars(&e, &[FVarId(1)]);
        assert_eq!(result, e);
    }
    #[test]
    fn test_cheap_beta_reduce() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let a = Expr::FVar(FVarId(99));
        let app = Expr::App(Box::new(lam), Box::new(a.clone()));
        let result = cheap_beta_reduce(&app);
        assert_eq!(result, a);
    }
    #[test]
    fn test_cheap_beta_reduce_non_redex() {
        let app = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        let result = cheap_beta_reduce(&app);
        assert_eq!(result, app);
    }
    #[test]
    fn test_apply_beta() {
        let inner = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(1)),
        );
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(inner),
        );
        let a = Expr::FVar(FVarId(1));
        let b = Expr::FVar(FVarId(2));
        let result = apply_beta(lam, &[a.clone(), b]);
        assert_eq!(result, a);
    }
    #[test]
    fn test_abstract_roundtrip() {
        let x = FVarId(1);
        let y = FVarId(2);
        let e = Expr::App(Box::new(Expr::FVar(x)), Box::new(Expr::FVar(y)));
        let abstracted = abstract_fvars(&e, &[x, y]);
        let back =
            crate::instantiate::instantiate_rev(&abstracted, &[Expr::FVar(x), Expr::FVar(y)]);
        assert_eq!(back, e);
    }
}
/// Abstract a single free variable and simultaneously replace it with a let-binding.
pub fn let_abstract(expr: &Expr, fvar: FVarId, ty: &Expr, val: &Expr) -> Expr {
    let body = abstract_fvars(expr, &[fvar]);
    Expr::Let(
        Name::str("_let"),
        Box::new(ty.clone()),
        Box::new(val.clone()),
        Box::new(body),
    )
}
/// Count how many times a free variable appears in an expression.
pub fn count_fvar_occurrences(expr: &Expr, fvar: FVarId) -> usize {
    match expr {
        Expr::FVar(id) => {
            if *id == fvar {
                1
            } else {
                0
            }
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::App(f, a) => count_fvar_occurrences(f, fvar) + count_fvar_occurrences(a, fvar),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_fvar_occurrences(ty, fvar) + count_fvar_occurrences(body, fvar)
        }
        Expr::Let(_, ty, val, body) => {
            count_fvar_occurrences(ty, fvar)
                + count_fvar_occurrences(val, fvar)
                + count_fvar_occurrences(body, fvar)
        }
        Expr::Proj(_, _, e) => count_fvar_occurrences(e, fvar),
    }
}
/// Collect all free variable IDs occurring in an expression.
pub fn collect_fvars(expr: &Expr) -> std::collections::HashSet<FVarId> {
    let mut result = std::collections::HashSet::new();
    collect_fvars_impl(expr, &mut result);
    result
}
fn collect_fvars_impl(expr: &Expr, result: &mut std::collections::HashSet<FVarId>) {
    match expr {
        Expr::FVar(id) => {
            result.insert(*id);
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => {}
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
        Expr::Proj(_, _, e) => collect_fvars_impl(e, result),
    }
}
/// Check if an expression is closed under a given set of free variables.
pub fn is_closed_under(expr: &Expr, allowed_fvars: &[FVarId]) -> bool {
    let free = collect_fvars(expr);
    free.iter().all(|fv| allowed_fvars.contains(fv))
}
/// Replace a free variable with a given expression.
pub fn subst_fvar(expr: &Expr, fvar: FVarId, replacement: &Expr) -> Expr {
    match expr {
        Expr::FVar(id) => {
            if *id == fvar {
                replacement.clone()
            } else {
                expr.clone()
            }
        }
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(subst_fvar(f, fvar, replacement)),
            Box::new(subst_fvar(a, fvar, replacement)),
        ),
        Expr::Lam(bi, name, ty, body) => {
            let ty_new = subst_fvar(ty, fvar, replacement);
            let shifted = shift_bvars(replacement, 1, 0);
            Expr::Lam(
                *bi,
                name.clone(),
                Box::new(ty_new),
                Box::new(subst_fvar(body, fvar, &shifted)),
            )
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_new = subst_fvar(ty, fvar, replacement);
            let shifted = shift_bvars(replacement, 1, 0);
            Expr::Pi(
                *bi,
                name.clone(),
                Box::new(ty_new),
                Box::new(subst_fvar(body, fvar, &shifted)),
            )
        }
        Expr::Let(name, ty, val, body) => {
            let ty_new = subst_fvar(ty, fvar, replacement);
            let val_new = subst_fvar(val, fvar, replacement);
            let shifted = shift_bvars(replacement, 1, 0);
            Expr::Let(
                name.clone(),
                Box::new(ty_new),
                Box::new(val_new),
                Box::new(subst_fvar(body, fvar, &shifted)),
            )
        }
        Expr::Proj(name, idx, e) => Expr::Proj(
            name.clone(),
            *idx,
            Box::new(subst_fvar(e, fvar, replacement)),
        ),
    }
}
/// Shift all bound variable indices >= cutoff by amount.
pub fn shift_bvars(expr: &Expr, amount: u32, cutoff: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= cutoff {
                Expr::BVar(*i + amount)
            } else {
                expr.clone()
            }
        }
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(shift_bvars(f, amount, cutoff)),
            Box::new(shift_bvars(a, amount, cutoff)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(shift_bvars(ty, amount, cutoff)),
            Box::new(shift_bvars(body, amount, cutoff + 1)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(shift_bvars(ty, amount, cutoff)),
            Box::new(shift_bvars(body, amount, cutoff + 1)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(shift_bvars(ty, amount, cutoff)),
            Box::new(shift_bvars(val, amount, cutoff)),
            Box::new(shift_bvars(body, amount, cutoff + 1)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Box::new(shift_bvars(e, amount, cutoff)))
        }
    }
}
/// Split a pi type: extract the domain and codomain.
pub fn split_pi(expr: &Expr) -> Option<(&Expr, &Expr)> {
    if let Expr::Pi(_, _, ty, body) = expr {
        Some((ty, body))
    } else {
        None
    }
}
/// Split a lambda: extract the type and body.
pub fn split_lam(expr: &Expr) -> Option<(&Expr, &Expr)> {
    if let Expr::Lam(_, _, ty, body) = expr {
        Some((ty, body))
    } else {
        None
    }
}
/// Build a nested application from a function and a list of arguments.
pub fn mk_app(f: Expr, args: &[Expr]) -> Expr {
    let mut result = f;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg.clone()));
    }
    result
}
/// Collect function and all arguments from a nested application.
pub fn collect_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}
/// Count the number of leading pi binders.
pub fn pi_arity(expr: &Expr) -> usize {
    let mut n = 0;
    let mut e = expr;
    while let Expr::Pi(_, _, _, body) = e {
        n += 1;
        e = body;
    }
    n
}
/// Count the number of leading lambda binders.
pub fn lam_arity(expr: &Expr) -> usize {
    let mut n = 0;
    let mut e = expr;
    while let Expr::Lam(_, _, _, body) = e {
        n += 1;
        e = body;
    }
    n
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::{BinderInfo, Level};
    #[test]
    fn test_count_fvar_occurrences_zero() {
        assert_eq!(count_fvar_occurrences(&Expr::BVar(0), FVarId(1)), 0);
    }
    #[test]
    fn test_count_fvar_occurrences_one() {
        let fv = FVarId(5);
        assert_eq!(count_fvar_occurrences(&Expr::FVar(fv), fv), 1);
    }
    #[test]
    fn test_count_fvar_occurrences_multiple() {
        let fv = FVarId(5);
        let e = Expr::App(Box::new(Expr::FVar(fv)), Box::new(Expr::FVar(fv)));
        assert_eq!(count_fvar_occurrences(&e, fv), 2);
    }
    #[test]
    fn test_collect_fvars_empty() {
        assert!(collect_fvars(&Expr::BVar(0)).is_empty());
    }
    #[test]
    fn test_collect_fvars_multi() {
        let x = FVarId(1);
        let y = FVarId(2);
        let e = Expr::App(Box::new(Expr::FVar(x)), Box::new(Expr::FVar(y)));
        let fvars = collect_fvars(&e);
        assert!(fvars.contains(&x) && fvars.contains(&y));
    }
    #[test]
    fn test_subst_fvar_replacement() {
        let x = FVarId(1);
        let replacement = Expr::Const(Name::str("Nat"), vec![]);
        let result = subst_fvar(&Expr::FVar(x), x, &replacement);
        assert_eq!(result, replacement);
    }
    #[test]
    fn test_shift_bvars_above_cutoff() {
        let shifted = shift_bvars(&Expr::BVar(2), 1, 0);
        assert_eq!(shifted, Expr::BVar(3));
    }
    #[test]
    fn test_shift_bvars_below_cutoff() {
        let shifted = shift_bvars(&Expr::BVar(0), 5, 2);
        assert_eq!(shifted, Expr::BVar(0));
    }
    #[test]
    fn test_is_closed_under() {
        let x = FVarId(1);
        assert!(is_closed_under(&Expr::FVar(x), &[x]));
        assert!(!is_closed_under(&Expr::FVar(x), &[]));
    }
    #[test]
    fn test_split_pi() {
        let ty = Expr::Sort(Level::zero());
        let body = Expr::BVar(0);
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty.clone()),
            Box::new(body.clone()),
        );
        let (d, c) = split_pi(&pi).expect("value should be present");
        assert_eq!(d, &ty);
        assert_eq!(c, &body);
    }
    #[test]
    fn test_mk_app_multiple() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let result = mk_app(f.clone(), &[a.clone(), b.clone()]);
        let expected = Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_collect_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = collect_app(&app);
        assert_eq!(head, &f);
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_pi_arity() {
        let base = Expr::Sort(Level::zero());
        let pi1 = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(base.clone()),
            Box::new(base.clone()),
        );
        let pi2 = Expr::Pi(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(base.clone()),
            Box::new(pi1),
        );
        assert_eq!(pi_arity(&pi2), 2);
    }
    #[test]
    fn test_let_abstract() {
        let x = FVarId(10);
        let ty = Expr::Const(Name::str("Nat"), vec![]);
        let val = Expr::Lit(crate::Literal::Nat(42));
        let result = let_abstract(&Expr::FVar(x), x, &ty, &val);
        assert!(matches!(result, Expr::Let(_, _, _, _)));
    }
}
/// Topological sort of free variables by dependency.
pub fn topo_sort_fvars(fvars: &[(FVarId, Expr)]) -> Result<Vec<FVarId>, String> {
    let n = fvars.len();
    let mut result = Vec::with_capacity(n);
    let mut visited = vec![false; n];
    let id_to_idx: std::collections::HashMap<FVarId, usize> = fvars
        .iter()
        .enumerate()
        .map(|(i, (id, _))| (*id, i))
        .collect();
    fn visit(
        i: usize,
        fvars: &[(FVarId, Expr)],
        id_to_idx: &std::collections::HashMap<FVarId, usize>,
        visited: &mut Vec<bool>,
        in_stack: &mut Vec<bool>,
        result: &mut Vec<FVarId>,
    ) -> Result<(), String> {
        if in_stack[i] {
            return Err(format!("cycle involving {:?}", fvars[i].0));
        }
        if visited[i] {
            return Ok(());
        }
        in_stack[i] = true;
        let deps = {
            let mut set = std::collections::HashSet::new();
            collect_fvars_impl(&fvars[i].1, &mut set);
            set
        };
        for dep in deps {
            if let Some(&j) = id_to_idx.get(&dep) {
                visit(j, fvars, id_to_idx, visited, in_stack, result)?;
            }
        }
        in_stack[i] = false;
        visited[i] = true;
        result.push(fvars[i].0);
        Ok(())
    }
    let mut in_stack = vec![false; n];
    for i in 0..n {
        visit(
            i,
            fvars,
            &id_to_idx,
            &mut visited,
            &mut in_stack,
            &mut result,
        )?;
    }
    Ok(result)
}
/// Abstract over free variables in dependency order.
pub fn abstract_fvars_ordered(
    expr: &Expr,
    fvars: &[(FVarId, Expr)],
) -> Result<(Expr, Vec<FVarId>), String> {
    let sorted = topo_sort_fvars(fvars)?;
    let abstracted = abstract_fvars(expr, &sorted);
    Ok((abstracted, sorted))
}
/// Let-abstract multiple bindings in sequence.
pub fn let_abstract_many(expr: &Expr, bindings: &[(FVarId, Name, Expr, Expr)]) -> Expr {
    let fvar_ids: Vec<FVarId> = bindings.iter().map(|(id, _, _, _)| *id).collect();
    let mut result = abstract_fvars(expr, &fvar_ids);
    for (_, name, ty, val) in bindings.iter().rev() {
        result = Expr::Let(
            name.clone(),
            Box::new(ty.clone()),
            Box::new(val.clone()),
            Box::new(result),
        );
    }
    result
}
#[cfg(test)]
mod topo_tests {
    use super::*;
    use crate::{BinderInfo, Level};
    #[test]
    fn test_topo_sort_no_deps() {
        let x = FVarId(1);
        let y = FVarId(2);
        let fvars = vec![(x, Expr::BVar(0)), (y, Expr::Sort(Level::zero()))];
        let sorted = topo_sort_fvars(&fvars).expect("sorted should be present");
        assert_eq!(sorted.len(), 2);
    }
    #[test]
    fn test_abstract_fvars_ordered_simple() {
        let x = FVarId(1);
        let fvars = vec![(x, Expr::Sort(Level::zero()))];
        let (abstracted, order) =
            abstract_fvars_ordered(&Expr::FVar(x), &fvars).expect("value should be present");
        assert_eq!(abstracted, Expr::BVar(0));
        assert_eq!(order, vec![x]);
    }
    #[test]
    fn test_let_abstract_many_empty() {
        let body = Expr::BVar(0);
        let result = let_abstract_many(&body, &[]);
        assert_eq!(result, body);
    }
    #[test]
    fn test_lam_arity() {
        let base = Expr::Sort(Level::zero());
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(base.clone()),
            Box::new(base.clone()),
        );
        assert_eq!(lam_arity(&lam), 1);
        assert_eq!(lam_arity(&base), 0);
    }
    #[test]
    fn test_split_lam() {
        let ty = Expr::Sort(Level::zero());
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty.clone()),
            Box::new(body.clone()),
        );
        let (d, b) = split_lam(&lam).expect("value should be present");
        assert_eq!(d, &ty);
        assert_eq!(b, &body);
    }
    #[test]
    fn test_split_pi_not_pi() {
        assert!(split_pi(&Expr::BVar(0)).is_none());
    }
    #[test]
    fn test_mk_app_empty() {
        let f = Expr::Const(Name::str("f"), vec![]);
        assert_eq!(mk_app(f.clone(), &[]), f);
    }
    #[test]
    fn test_mk_forall_empty() {
        let body = Expr::Sort(Level::zero());
        assert_eq!(mk_forall(&[], &body), body);
    }
    #[test]
    fn test_mk_lambda_empty() {
        let body = Expr::BVar(0);
        assert_eq!(mk_lambda(&[], &body), body);
    }
    #[test]
    fn test_subst_fvar_identity() {
        let x = FVarId(1);
        let y = FVarId(2);
        let e = Expr::FVar(x);
        let result = subst_fvar(&e, y, &Expr::BVar(0));
        assert_eq!(result, e);
    }
}
/// Check if an expression is "ground" — no free or bound variables.
///
/// A ground expression is completely closed and self-contained.
pub fn is_ground(expr: &Expr) -> bool {
    collect_fvars(expr).is_empty() && !has_any_bvar(expr)
}
fn has_any_bvar(expr: &Expr) -> bool {
    match expr {
        Expr::BVar(_) => true,
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_any_bvar(f) || has_any_bvar(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_any_bvar(ty) || has_any_bvar(body)
        }
        Expr::Let(_, ty, val, body) => has_any_bvar(ty) || has_any_bvar(val) || has_any_bvar(body),
        Expr::Proj(_, _, e) => has_any_bvar(e),
    }
}
/// Compute the maximum bound variable index in an expression.
///
/// Returns None if there are no bound variables.
pub fn max_bvar(expr: &Expr) -> Option<u32> {
    match expr {
        Expr::BVar(i) => Some(*i),
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => None,
        Expr::App(f, a) => match (max_bvar(f), max_bvar(a)) {
            (Some(x), Some(y)) => Some(x.max(y)),
            (Some(x), None) | (None, Some(x)) => Some(x),
            (None, None) => None,
        },
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            match (max_bvar(ty), max_bvar(body)) {
                (Some(x), Some(y)) => Some(x.max(y)),
                (Some(x), None) | (None, Some(x)) => Some(x),
                (None, None) => None,
            }
        }
        Expr::Let(_, ty, val, body) => {
            let m1 = max_bvar(ty);
            let m2 = max_bvar(val);
            let m3 = max_bvar(body);
            [m1, m2, m3].iter().filter_map(|&x| x).max()
        }
        Expr::Proj(_, _, e) => max_bvar(e),
    }
}
#[cfg(test)]
mod ground_tests {
    use super::*;
    use crate::Literal;
    #[test]
    fn test_is_ground_lit() {
        assert!(is_ground(&Expr::Lit(Literal::Nat(42))));
    }
    #[test]
    fn test_is_ground_bvar() {
        assert!(!is_ground(&Expr::BVar(0)));
    }
    #[test]
    fn test_is_ground_fvar() {
        assert!(!is_ground(&Expr::FVar(FVarId(1))));
    }
    #[test]
    fn test_max_bvar_none() {
        assert_eq!(max_bvar(&Expr::Lit(Literal::Nat(0))), None);
    }
    #[test]
    fn test_max_bvar_some() {
        assert_eq!(max_bvar(&Expr::BVar(3)), Some(3));
    }
    #[test]
    fn test_max_bvar_app() {
        let e = Expr::App(Box::new(Expr::BVar(2)), Box::new(Expr::BVar(5)));
        assert_eq!(max_bvar(&e), Some(5));
    }
}
/// Compute the number of distinct names (constants) in an expression.
#[allow(dead_code)]
pub fn distinct_const_count(expr: &Expr) -> usize {
    let mut seen = std::collections::HashSet::new();
    collect_const_names(expr, &mut seen);
    seen.len()
}
fn collect_const_names(expr: &Expr, seen: &mut std::collections::HashSet<String>) {
    match expr {
        Expr::Const(name, _) => {
            seen.insert(name.to_string());
        }
        Expr::App(f, a) => {
            collect_const_names(f, seen);
            collect_const_names(a, seen);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_const_names(ty, seen);
            collect_const_names(body, seen);
        }
        Expr::Let(_, ty, val, body) => {
            collect_const_names(ty, seen);
            collect_const_names(val, seen);
            collect_const_names(body, seen);
        }
        Expr::Proj(_, _, e) => collect_const_names(e, seen),
        _ => {}
    }
}
/// Check whether all bound variable indices are within valid range for a
/// well-formed expression.
///
/// An expression with `k` open binders is valid if no `BVar(i)` appears
/// with `i >= k + depth`.
#[allow(dead_code)]
pub fn bvars_in_range(expr: &Expr, open_binders: u32) -> bool {
    check_bvars(expr, open_binders)
}
fn check_bvars(expr: &Expr, depth: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i < depth,
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => true,
        Expr::App(f, a) => check_bvars(f, depth) && check_bvars(a, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            check_bvars(ty, depth) && check_bvars(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            check_bvars(ty, depth) && check_bvars(val, depth) && check_bvars(body, depth + 1)
        }
        Expr::Proj(_, _, e) => check_bvars(e, depth),
    }
}
/// Rename all occurrences of `old_name` constant to `new_name`.
#[allow(dead_code)]
pub fn rename_const(expr: &Expr, old_name: &Name, new_name: &Name) -> Expr {
    match expr {
        Expr::Const(n, ls) => {
            if n == old_name {
                Expr::Const(new_name.clone(), ls.clone())
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(rename_const(f, old_name, new_name)),
            Box::new(rename_const(a, old_name, new_name)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(rename_const(ty, old_name, new_name)),
            Box::new(rename_const(body, old_name, new_name)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(rename_const(ty, old_name, new_name)),
            Box::new(rename_const(body, old_name, new_name)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(rename_const(ty, old_name, new_name)),
            Box::new(rename_const(val, old_name, new_name)),
            Box::new(rename_const(body, old_name, new_name)),
        ),
        Expr::Proj(n, i, e) => {
            Expr::Proj(n.clone(), *i, Box::new(rename_const(e, old_name, new_name)))
        }
        _ => expr.clone(),
    }
}
/// Replace all literal naturals with a given value.
#[allow(dead_code)]
pub fn replace_nat_lit(expr: &Expr, old: u64, new: u64) -> Expr {
    match expr {
        Expr::Lit(crate::Literal::Nat(n)) => {
            if *n == old {
                Expr::Lit(crate::Literal::Nat(new))
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => Expr::App(
            Box::new(replace_nat_lit(f, old, new)),
            Box::new(replace_nat_lit(a, old, new)),
        ),
        Expr::Lam(bi, name, ty, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(replace_nat_lit(ty, old, new)),
            Box::new(replace_nat_lit(body, old, new)),
        ),
        Expr::Pi(bi, name, ty, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(replace_nat_lit(ty, old, new)),
            Box::new(replace_nat_lit(body, old, new)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(replace_nat_lit(ty, old, new)),
            Box::new(replace_nat_lit(val, old, new)),
            Box::new(replace_nat_lit(body, old, new)),
        ),
        Expr::Proj(n, i, e) => Expr::Proj(n.clone(), *i, Box::new(replace_nat_lit(e, old, new))),
        _ => expr.clone(),
    }
}
#[cfg(test)]
mod extra_abstract_tests {
    use super::*;
    use crate::{BinderInfo, Level, Literal};
    #[test]
    fn test_distinct_const_count_zero() {
        assert_eq!(distinct_const_count(&Expr::BVar(0)), 0);
    }
    #[test]
    fn test_distinct_const_count_one() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(distinct_const_count(&e), 1);
    }
    #[test]
    fn test_distinct_const_count_dedup() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let e = Expr::App(Box::new(nat.clone()), Box::new(nat));
        assert_eq!(distinct_const_count(&e), 1);
    }
    #[test]
    fn test_bvars_in_range_closed() {
        let e = Expr::Lit(Literal::Nat(42));
        assert!(bvars_in_range(&e, 0));
    }
    #[test]
    fn test_bvars_in_range_bvar_ok() {
        let e = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(bvars_in_range(&e, 0));
    }
    #[test]
    fn test_bvars_in_range_bvar_out_of_range() {
        assert!(!bvars_in_range(&Expr::BVar(0), 0));
    }
    #[test]
    fn test_rename_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        let renamed = rename_const(&e, &Name::str("Nat"), &Name::str("Int"));
        assert!(matches!(renamed, Expr::Const(n, _) if n == Name::str("Int")));
    }
    #[test]
    fn test_rename_const_no_match() {
        let e = Expr::Const(Name::str("Bool"), vec![]);
        let renamed = rename_const(&e, &Name::str("Nat"), &Name::str("Int"));
        assert_eq!(renamed, e);
    }
    #[test]
    fn test_replace_nat_lit() {
        let e = Expr::Lit(Literal::Nat(42));
        let replaced = replace_nat_lit(&e, 42, 0);
        assert_eq!(replaced, Expr::Lit(Literal::Nat(0)));
    }
    #[test]
    fn test_replace_nat_lit_no_match() {
        let e = Expr::Lit(Literal::Nat(1));
        let replaced = replace_nat_lit(&e, 42, 0);
        assert_eq!(replaced, e);
    }
    #[test]
    fn test_has_any_bvar_const() {
        assert!(!has_any_bvar(&Expr::Const(Name::str("Nat"), vec![])));
    }
    #[test]
    fn test_is_ground_sort() {
        assert!(is_ground(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_max_bvar_app() {
        let e = Expr::App(Box::new(Expr::BVar(5)), Box::new(Expr::BVar(3)));
        assert_eq!(max_bvar(&e), Some(5));
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
