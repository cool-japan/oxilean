//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::expr_util::lift_loose_bvars;
use crate::{Expr, FVarId, Level, Name};

use super::types::{
    ConfigNode, DecisionNode, Either2, Fixture, FlatSubstitution, FocusStack, LabelSet, MinHeap,
    NamedSubst, NonEmptyVec, PathBuf, PrefixCounter, RewriteRule, RewriteRuleSet, SimpleDag,
    SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch, StringPool, Telescope,
    TokenBucket, TransformStat, TransitiveClosure, VersionedRecord, WindowIterator, WriteOnce,
};

/// Instantiate bound variables in reverse order using a substitution array.
///
/// `instantiate_rev(body, [a0, a1, a2])` substitutes:
/// - BVar(0) with a2 (innermost)
/// - BVar(1) with a1
/// - BVar(2) with a0 (outermost)
///
/// This matches LEAN 4's convention where the substitution array is
/// ordered from outermost to innermost.
pub fn instantiate_rev(body: &Expr, subst: &[Expr]) -> Expr {
    if subst.is_empty() {
        return body.clone();
    }
    instantiate_rev_at(body, subst, 0)
}
fn instantiate_rev_at(expr: &Expr, subst: &[Expr], offset: u32) -> Expr {
    match expr {
        Expr::BVar(idx) => {
            if *idx >= offset {
                let vidx = (*idx - offset) as usize;
                if vidx < subst.len() {
                    let s_idx = subst.len() - vidx - 1;
                    lift_loose_bvars(&subst[s_idx], offset, 0)
                } else {
                    Expr::BVar(idx - subst.len() as u32)
                }
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => {
            let f_new = instantiate_rev_at(f, subst, offset);
            let a_new = instantiate_rev_at(a, subst, offset);
            Expr::App(Box::new(f_new), Box::new(a_new))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty_new = instantiate_rev_at(ty, subst, offset);
            let body_new = instantiate_rev_at(body, subst, offset + 1);
            Expr::Lam(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_new = instantiate_rev_at(ty, subst, offset);
            let body_new = instantiate_rev_at(body, subst, offset + 1);
            Expr::Pi(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Let(name, ty, val, body) => {
            let ty_new = instantiate_rev_at(ty, subst, offset);
            let val_new = instantiate_rev_at(val, subst, offset);
            let body_new = instantiate_rev_at(body, subst, offset + 1);
            Expr::Let(
                name.clone(),
                Box::new(ty_new),
                Box::new(val_new),
                Box::new(body_new),
            )
        }
        Expr::Proj(name, idx, e) => {
            let e_new = instantiate_rev_at(e, subst, offset);
            Expr::Proj(name.clone(), *idx, Box::new(e_new))
        }
    }
}
/// Instantiate bound variables in forward order.
///
/// `instantiate_many(body, [a0, a1, a2])` substitutes:
/// - BVar(0) with a0
/// - BVar(1) with a1
/// - BVar(2) with a2
pub fn instantiate_many(body: &Expr, subst: &[Expr]) -> Expr {
    if subst.is_empty() {
        return body.clone();
    }
    instantiate_many_at(body, subst, 0)
}
fn instantiate_many_at(expr: &Expr, subst: &[Expr], offset: u32) -> Expr {
    match expr {
        Expr::BVar(idx) => {
            if *idx >= offset {
                let vidx = (*idx - offset) as usize;
                if vidx < subst.len() {
                    lift_loose_bvars(&subst[vidx], offset, 0)
                } else {
                    Expr::BVar(idx - subst.len() as u32)
                }
            } else {
                expr.clone()
            }
        }
        Expr::Sort(_) | Expr::FVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => {
            let f_new = instantiate_many_at(f, subst, offset);
            let a_new = instantiate_many_at(a, subst, offset);
            Expr::App(Box::new(f_new), Box::new(a_new))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty_new = instantiate_many_at(ty, subst, offset);
            let body_new = instantiate_many_at(body, subst, offset + 1);
            Expr::Lam(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_new = instantiate_many_at(ty, subst, offset);
            let body_new = instantiate_many_at(body, subst, offset + 1);
            Expr::Pi(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Let(name, ty, val, body) => {
            let ty_new = instantiate_many_at(ty, subst, offset);
            let val_new = instantiate_many_at(val, subst, offset);
            let body_new = instantiate_many_at(body, subst, offset + 1);
            Expr::Let(
                name.clone(),
                Box::new(ty_new),
                Box::new(val_new),
                Box::new(body_new),
            )
        }
        Expr::Proj(name, idx, e) => {
            let e_new = instantiate_many_at(e, subst, offset);
            Expr::Proj(name.clone(), *idx, Box::new(e_new))
        }
    }
}
/// Instantiate universe level parameters in an expression's type.
///
/// Given a constant's type with universe parameters `[u, v]` and
/// concrete levels `[1, 2]`, replaces all occurrences of `Level::Param("u")`
/// with `Level::succ(Level::zero())` etc.
pub fn instantiate_type_lparams(ty: &Expr, param_names: &[Name], levels: &[Level]) -> Expr {
    if param_names.is_empty() || levels.is_empty() {
        return ty.clone();
    }
    instantiate_type_lparams_core(ty, param_names, levels)
}
fn instantiate_type_lparams_core(expr: &Expr, param_names: &[Name], levels: &[Level]) -> Expr {
    match expr {
        Expr::Sort(l) => {
            let new_l = instantiate_level_param(l, param_names, levels);
            Expr::Sort(new_l)
        }
        Expr::Const(name, ls) => {
            let new_ls: Vec<Level> = ls
                .iter()
                .map(|l| instantiate_level_param(l, param_names, levels))
                .collect();
            Expr::Const(name.clone(), new_ls)
        }
        Expr::App(f, a) => {
            let f_new = instantiate_type_lparams_core(f, param_names, levels);
            let a_new = instantiate_type_lparams_core(a, param_names, levels);
            Expr::App(Box::new(f_new), Box::new(a_new))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty_new = instantiate_type_lparams_core(ty, param_names, levels);
            let body_new = instantiate_type_lparams_core(body, param_names, levels);
            Expr::Lam(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_new = instantiate_type_lparams_core(ty, param_names, levels);
            let body_new = instantiate_type_lparams_core(body, param_names, levels);
            Expr::Pi(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Let(name, ty, val, body) => {
            let ty_new = instantiate_type_lparams_core(ty, param_names, levels);
            let val_new = instantiate_type_lparams_core(val, param_names, levels);
            let body_new = instantiate_type_lparams_core(body, param_names, levels);
            Expr::Let(
                name.clone(),
                Box::new(ty_new),
                Box::new(val_new),
                Box::new(body_new),
            )
        }
        Expr::Proj(name, idx, e) => {
            let e_new = instantiate_type_lparams_core(e, param_names, levels);
            Expr::Proj(name.clone(), *idx, Box::new(e_new))
        }
        Expr::BVar(_) | Expr::FVar(_) | Expr::Lit(_) => expr.clone(),
    }
}
fn instantiate_level_param(level: &Level, param_names: &[Name], levels: &[Level]) -> Level {
    match level {
        Level::Param(name) => {
            for (i, pn) in param_names.iter().enumerate() {
                if pn == name {
                    if let Some(l) = levels.get(i) {
                        return l.clone();
                    }
                }
            }
            level.clone()
        }
        Level::Succ(l) => Level::succ(instantiate_level_param(l, param_names, levels)),
        Level::Max(l1, l2) => Level::max(
            instantiate_level_param(l1, param_names, levels),
            instantiate_level_param(l2, param_names, levels),
        ),
        Level::IMax(l1, l2) => Level::imax(
            instantiate_level_param(l1, param_names, levels),
            instantiate_level_param(l2, param_names, levels),
        ),
        Level::Zero | Level::MVar(_) => level.clone(),
    }
}
/// Replace expression metavariables using a substitution function.
///
/// The function receives a free variable ID (acting as metavar id) and
/// returns `Some(expr)` if it should be substituted, `None` to keep it.
pub fn instantiate_expr_mvars(expr: &Expr, subst: &dyn Fn(FVarId) -> Option<Expr>) -> Expr {
    match expr {
        Expr::FVar(id) => {
            if let Some(replacement) = subst(*id) {
                instantiate_expr_mvars(&replacement, subst)
            } else {
                expr.clone()
            }
        }
        Expr::App(f, a) => {
            let f_new = instantiate_expr_mvars(f, subst);
            let a_new = instantiate_expr_mvars(a, subst);
            Expr::App(Box::new(f_new), Box::new(a_new))
        }
        Expr::Lam(bi, name, ty, body) => {
            let ty_new = instantiate_expr_mvars(ty, subst);
            let body_new = instantiate_expr_mvars(body, subst);
            Expr::Lam(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Pi(bi, name, ty, body) => {
            let ty_new = instantiate_expr_mvars(ty, subst);
            let body_new = instantiate_expr_mvars(body, subst);
            Expr::Pi(*bi, name.clone(), Box::new(ty_new), Box::new(body_new))
        }
        Expr::Let(name, ty, val, body) => {
            let ty_new = instantiate_expr_mvars(ty, subst);
            let val_new = instantiate_expr_mvars(val, subst);
            let body_new = instantiate_expr_mvars(body, subst);
            Expr::Let(
                name.clone(),
                Box::new(ty_new),
                Box::new(val_new),
                Box::new(body_new),
            )
        }
        Expr::Proj(name, idx, e) => {
            let e_new = instantiate_expr_mvars(e, subst);
            Expr::Proj(name.clone(), *idx, Box::new(e_new))
        }
        Expr::Sort(_) | Expr::BVar(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::BinderInfo;
    #[test]
    fn test_instantiate_rev_basic() {
        let body = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let a = Expr::FVar(FVarId(0));
        let b = Expr::FVar(FVarId(1));
        let result = instantiate_rev(&body, &[a.clone(), b.clone()]);
        match &result {
            Expr::App(f, arg) => {
                assert_eq!(**f, b);
                assert_eq!(**arg, a);
            }
            _ => panic!("Expected App, got {:?}", result),
        }
    }
    #[test]
    fn test_instantiate_rev_single() {
        let body = Expr::BVar(0);
        let arg = Expr::FVar(FVarId(42));
        let result = instantiate_rev(&body, std::slice::from_ref(&arg));
        assert_eq!(result, arg);
    }
    #[test]
    fn test_instantiate_rev_shift() {
        let body = Expr::BVar(2);
        let a = Expr::FVar(FVarId(0));
        let b = Expr::FVar(FVarId(1));
        let result = instantiate_rev(&body, &[a, b]);
        assert_eq!(result, Expr::BVar(0));
    }
    #[test]
    fn test_instantiate_rev_under_binder() {
        let body = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(1)),
        );
        let arg = Expr::FVar(FVarId(99));
        let result = instantiate_rev(&body, std::slice::from_ref(&arg));
        if let Expr::Lam(_, _, _, inner_body) = &result {
            assert_eq!(**inner_body, arg);
        } else {
            panic!("Expected Lam");
        }
    }
    #[test]
    fn test_instantiate_rev_empty() {
        let body = Expr::BVar(0);
        let result = instantiate_rev(&body, &[]);
        assert_eq!(result, body);
    }
    #[test]
    fn test_instantiate_many_basic() {
        let body = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(1)));
        let a = Expr::FVar(FVarId(0));
        let b = Expr::FVar(FVarId(1));
        let result = instantiate_many(&body, &[a.clone(), b.clone()]);
        match &result {
            Expr::App(f, arg) => {
                assert_eq!(**f, a);
                assert_eq!(**arg, b);
            }
            _ => panic!("Expected App"),
        }
    }
    #[test]
    fn test_instantiate_type_lparams_sort() {
        let ty = Expr::Sort(Level::param(Name::str("u")));
        let result =
            instantiate_type_lparams(&ty, &[Name::str("u")], &[Level::succ(Level::zero())]);
        assert_eq!(result, Expr::Sort(Level::succ(Level::zero())));
    }
    #[test]
    fn test_instantiate_type_lparams_const() {
        let ty = Expr::Const(Name::str("List"), vec![Level::param(Name::str("u"))]);
        let result = instantiate_type_lparams(&ty, &[Name::str("u")], &[Level::zero()]);
        assert_eq!(result, Expr::Const(Name::str("List"), vec![Level::zero()]));
    }
    #[test]
    fn test_instantiate_type_lparams_multi() {
        let ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::param(Name::str("u")))),
            Box::new(Expr::Sort(Level::param(Name::str("v")))),
        );
        let result = instantiate_type_lparams(
            &ty,
            &[Name::str("u"), Name::str("v")],
            &[Level::zero(), Level::succ(Level::zero())],
        );
        if let Expr::Pi(_, _, dom, cod) = &result {
            assert_eq!(**dom, Expr::Sort(Level::zero()));
            assert_eq!(**cod, Expr::Sort(Level::succ(Level::zero())));
        } else {
            panic!("Expected Pi");
        }
    }
    #[test]
    fn test_instantiate_expr_mvars() {
        let e = Expr::App(
            Box::new(Expr::FVar(FVarId(1))),
            Box::new(Expr::FVar(FVarId(2))),
        );
        let result = instantiate_expr_mvars(&e, &|id| {
            if id == FVarId(1) {
                Some(Expr::Const(Name::str("f"), vec![]))
            } else {
                None
            }
        });
        match &result {
            Expr::App(f, a) => {
                assert_eq!(**f, Expr::Const(Name::str("f"), vec![]));
                assert_eq!(**a, Expr::FVar(FVarId(2)));
            }
            _ => panic!("Expected App"),
        }
    }
}
/// Abstract a free variable `fvar` in `expr`, replacing it with `BVar(depth)`.
///
/// This is the inverse of instantiation: `instantiate(abstract_fvar(e, id), val) = e[id ↦ val]`.
#[allow(dead_code)]
pub fn abstract_fvar(expr: &Expr, fvar_id: FVarId, depth: u32) -> Expr {
    match expr {
        Expr::FVar(id) if *id == fvar_id => Expr::BVar(depth),
        Expr::FVar(_) | Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => {
            expr.clone()
        }
        Expr::App(f, a) => Expr::App(
            Box::new(abstract_fvar(f, fvar_id, depth)),
            Box::new(abstract_fvar(a, fvar_id, depth)),
        ),
        Expr::Lam(bi, n, dom, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(abstract_fvar(dom, fvar_id, depth)),
            Box::new(abstract_fvar(body, fvar_id, depth + 1)),
        ),
        Expr::Pi(bi, n, dom, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(abstract_fvar(dom, fvar_id, depth)),
            Box::new(abstract_fvar(body, fvar_id, depth + 1)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(abstract_fvar(ty, fvar_id, depth)),
            Box::new(abstract_fvar(val, fvar_id, depth)),
            Box::new(abstract_fvar(body, fvar_id, depth + 1)),
        ),
        Expr::Proj(n, i, e) => {
            Expr::Proj(n.clone(), *i, Box::new(abstract_fvar(e, fvar_id, depth)))
        }
    }
}
/// Abstract multiple free variables, each at consecutive depths starting from `base`.
///
/// `abstract_fvars(expr, [id0, id1, ...], base)` replaces:
/// - `FVar(id0)` → `BVar(base)`
/// - `FVar(id1)` → `BVar(base + 1)`
/// - …
#[allow(dead_code)]
pub fn abstract_fvars(expr: &Expr, fvar_ids: &[FVarId], base: u32) -> Expr {
    fvar_ids
        .iter()
        .enumerate()
        .fold(expr.clone(), |acc, (i, id)| {
            abstract_fvar(&acc, *id, base + i as u32)
        })
}
/// Check whether an expression has any loose `BVar` (index ≥ `depth`).
#[allow(dead_code)]
pub fn has_loose_bvars(expr: &Expr) -> bool {
    has_loose_bvars_at(expr, 0)
}
fn has_loose_bvars_at(expr: &Expr, depth: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i >= depth,
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => false,
        Expr::App(f, a) => has_loose_bvars_at(f, depth) || has_loose_bvars_at(a, depth),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            has_loose_bvars_at(dom, depth) || has_loose_bvars_at(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            has_loose_bvars_at(ty, depth)
                || has_loose_bvars_at(val, depth)
                || has_loose_bvars_at(body, depth + 1)
        }
        Expr::Proj(_, _, e) => has_loose_bvars_at(e, depth),
    }
}
/// Count the maximum loose `BVar` index (+1) or 0 if no loose bvars.
///
/// Useful to determine how many arguments an expression expects.
#[allow(dead_code)]
pub fn max_loose_bvar(expr: &Expr) -> u32 {
    max_loose_bvar_at(expr, 0)
}
fn max_loose_bvar_at(expr: &Expr, depth: u32) -> u32 {
    match expr {
        Expr::BVar(i) => {
            if *i >= depth {
                *i - depth + 1
            } else {
                0
            }
        }
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => 0,
        Expr::App(f, a) => max_loose_bvar_at(f, depth).max(max_loose_bvar_at(a, depth)),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            max_loose_bvar_at(dom, depth).max(max_loose_bvar_at(body, depth + 1))
        }
        Expr::Let(_, ty, val, body) => max_loose_bvar_at(ty, depth)
            .max(max_loose_bvar_at(val, depth))
            .max(max_loose_bvar_at(body, depth + 1)),
        Expr::Proj(_, _, e) => max_loose_bvar_at(e, depth),
    }
}
/// Lift all free `BVar` indices by `n` (shift up).
///
/// This is used when inserting a new binder around an expression.
#[allow(dead_code)]
pub fn lift_bvars(expr: &Expr, n: u32) -> Expr {
    lift_bvars_at(expr, n, 0)
}
fn lift_bvars_at(expr: &Expr, n: u32, depth: u32) -> Expr {
    match expr {
        Expr::BVar(i) => {
            if *i >= depth {
                Expr::BVar(*i + n)
            } else {
                expr.clone()
            }
        }
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => expr.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(lift_bvars_at(f, n, depth)),
            Box::new(lift_bvars_at(a, n, depth)),
        ),
        Expr::Lam(bi, name, dom, body) => Expr::Lam(
            *bi,
            name.clone(),
            Box::new(lift_bvars_at(dom, n, depth)),
            Box::new(lift_bvars_at(body, n, depth + 1)),
        ),
        Expr::Pi(bi, name, dom, body) => Expr::Pi(
            *bi,
            name.clone(),
            Box::new(lift_bvars_at(dom, n, depth)),
            Box::new(lift_bvars_at(body, n, depth + 1)),
        ),
        Expr::Let(name, ty, val, body) => Expr::Let(
            name.clone(),
            Box::new(lift_bvars_at(ty, n, depth)),
            Box::new(lift_bvars_at(val, n, depth)),
            Box::new(lift_bvars_at(body, n, depth + 1)),
        ),
        Expr::Proj(name, idx, e) => {
            Expr::Proj(name.clone(), *idx, Box::new(lift_bvars_at(e, n, depth)))
        }
    }
}
/// Instantiate `BVar(0)` with `subst` (simple single substitution).
///
/// Equivalent to `instantiate(expr, &[subst])`.
#[allow(dead_code)]
pub fn instantiate_one(expr: &Expr, subst: &Expr) -> Expr {
    instantiate_many(expr, std::slice::from_ref(subst))
}
/// Unfold a `Let` binding: replace `Let(n, ty, val, body)` with `body[0 := val]`.
#[allow(dead_code)]
pub fn unfold_let_one(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::Let(_, _, val, body) => Some(instantiate_one(body, val)),
        _ => None,
    }
}
/// Unfold all nested `Let` bindings from outermost to innermost.
#[allow(dead_code)]
pub fn unfold_all_lets(expr: &Expr) -> Expr {
    let mut cur = expr.clone();
    while let Some(next) = unfold_let_one(&cur) {
        cur = next;
    }
    cur
}
/// Beta-reduce the outermost beta-redex if one exists.
///
/// `(fun x => body) arg` → `body[0 := arg]`
#[allow(dead_code)]
pub fn beta_reduce_once(expr: &Expr) -> Option<Expr> {
    match expr {
        Expr::App(f, arg) => {
            if let Expr::Lam(_, _, _, body) = f.as_ref() {
                Some(instantiate_one(body, arg))
            } else {
                None
            }
        }
        _ => None,
    }
}
/// Repeat beta reduction until no more redexes at the top level.
#[allow(dead_code)]
pub fn beta_reduce_head(expr: &Expr) -> Expr {
    let mut cur = expr.clone();
    while let Some(next) = beta_reduce_once(&cur) {
        cur = next;
    }
    cur
}
/// Apply a list of arguments to an expression via beta reduction.
///
/// `beta_apply(f, [a, b]) = beta_reduce((f a) b)`
#[allow(dead_code)]
pub fn beta_apply(f: &Expr, args: &[Expr]) -> Expr {
    args.iter().fold(f.clone(), |acc, arg| {
        let applied = Expr::App(Box::new(acc), Box::new(arg.clone()));
        beta_reduce_head(&applied)
    })
}
/// Replace all occurrences of `FVar(id)` with `replacement` in `expr`.
#[allow(dead_code)]
pub fn subst_fvar(expr: &Expr, id: FVarId, replacement: &Expr) -> Expr {
    match expr {
        Expr::FVar(fid) if *fid == id => replacement.clone(),
        Expr::FVar(_) | Expr::BVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => {
            expr.clone()
        }
        Expr::App(f, a) => Expr::App(
            Box::new(subst_fvar(f, id, replacement)),
            Box::new(subst_fvar(a, id, replacement)),
        ),
        Expr::Lam(bi, n, dom, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(subst_fvar(dom, id, replacement)),
            Box::new(subst_fvar(body, id, replacement)),
        ),
        Expr::Pi(bi, n, dom, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(subst_fvar(dom, id, replacement)),
            Box::new(subst_fvar(body, id, replacement)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(subst_fvar(ty, id, replacement)),
            Box::new(subst_fvar(val, id, replacement)),
            Box::new(subst_fvar(body, id, replacement)),
        ),
        Expr::Proj(n, i, e) => Expr::Proj(n.clone(), *i, Box::new(subst_fvar(e, id, replacement))),
    }
}
/// Simultaneously substitute multiple `FVar` ids.
#[allow(dead_code)]
pub fn subst_fvars(expr: &Expr, subst: &[(FVarId, Expr)]) -> Expr {
    subst
        .iter()
        .fold(expr.clone(), |acc, (id, repl)| subst_fvar(&acc, *id, repl))
}
/// Check whether `expr` is in "weak head normal form" (no top-level beta/let redex).
#[allow(dead_code)]
pub fn is_whnf_simple(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, _) => !matches!(f.as_ref(), Expr::Lam(_, _, _, _)),
        Expr::Let(_, _, _, _) => false,
        _ => true,
    }
}
#[cfg(test)]
mod instantiate_extra_tests {
    use super::*;
    use crate::{BinderInfo, Expr, FVarId, Level, Name};
    #[test]
    fn test_abstract_fvar_hit() {
        let fvar = Expr::FVar(FVarId(5));
        let result = abstract_fvar(&fvar, FVarId(5), 0);
        assert_eq!(result, Expr::BVar(0));
    }
    #[test]
    fn test_abstract_fvar_miss() {
        let fvar = Expr::FVar(FVarId(3));
        let result = abstract_fvar(&fvar, FVarId(5), 0);
        assert_eq!(result, fvar);
    }
    #[test]
    fn test_has_loose_bvars_false() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(!has_loose_bvars(&e));
    }
    #[test]
    fn test_has_loose_bvars_true() {
        let e = Expr::BVar(0);
        assert!(has_loose_bvars(&e));
    }
    #[test]
    fn test_lift_bvars() {
        let e = Expr::BVar(0);
        let lifted = lift_bvars(&e, 2);
        assert_eq!(lifted, Expr::BVar(2));
    }
    #[test]
    fn test_lift_bvars_under_binder() {
        let inner = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(inner),
        );
        let lifted = lift_bvars(&lam, 1);
        if let Expr::Lam(_, _, _, body) = &lifted {
            assert_eq!(**body, Expr::BVar(0));
        } else {
            panic!("Expected Lam");
        }
    }
    #[test]
    fn test_instantiate_one() {
        let body = Expr::BVar(0);
        let a = Expr::Const(Name::str("a"), vec![]);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(body),
        );
        let app = Expr::App(Box::new(lam), Box::new(a.clone()));
        let result = beta_reduce_once(&app).expect("result should be present");
        assert_eq!(result, a);
    }
    #[test]
    fn test_unfold_let_one() {
        let a = Expr::Const(Name::str("a"), vec![]);
        let body = Expr::BVar(0);
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(a.clone()),
            Box::new(body),
        );
        let result = unfold_let_one(&let_expr).expect("result should be present");
        assert_eq!(result, a);
    }
    #[test]
    fn test_unfold_let_none() {
        let e = Expr::Const(Name::str("f"), vec![]);
        assert!(unfold_let_one(&e).is_none());
    }
    #[test]
    fn test_subst_fvar() {
        let e = Expr::FVar(FVarId(1));
        let repl = Expr::Const(Name::str("replacement"), vec![]);
        let result = subst_fvar(&e, FVarId(1), &repl);
        assert_eq!(result, repl);
    }
    #[test]
    fn test_subst_fvar_miss() {
        let e = Expr::FVar(FVarId(2));
        let repl = Expr::Const(Name::str("replacement"), vec![]);
        let result = subst_fvar(&e, FVarId(1), &repl);
        assert_eq!(result, e);
    }
    #[test]
    fn test_beta_apply_empty() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let result = beta_apply(&f, &[]);
        assert_eq!(result, f);
    }
    #[test]
    fn test_is_whnf_simple_const() {
        let e = Expr::Const(Name::str("f"), vec![]);
        assert!(is_whnf_simple(&e));
    }
    #[test]
    fn test_is_whnf_simple_let() {
        let e = Expr::Let(
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(!is_whnf_simple(&e));
    }
    #[test]
    fn test_max_loose_bvar_none() {
        let e = Expr::Const(Name::str("f"), vec![]);
        assert_eq!(max_loose_bvar(&e), 0);
    }
    #[test]
    fn test_max_loose_bvar_bvar0() {
        let e = Expr::BVar(0);
        assert_eq!(max_loose_bvar(&e), 1);
    }
}
pub(super) fn named_subst_apply(expr: &Expr, subst: &NamedSubst) -> Expr {
    match expr {
        Expr::Const(name, levels) if levels.is_empty() => {
            if let Some(repl) = subst.get(name) {
                return repl.clone();
            }
            expr.clone()
        }
        Expr::App(f, a) => Expr::App(
            Box::new(named_subst_apply(f, subst)),
            Box::new(named_subst_apply(a, subst)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(named_subst_apply(ty, subst)),
            Box::new(named_subst_apply(body, subst)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(named_subst_apply(ty, subst)),
            Box::new(named_subst_apply(body, subst)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(named_subst_apply(ty, subst)),
            Box::new(named_subst_apply(val, subst)),
            Box::new(named_subst_apply(body, subst)),
        ),
        Expr::Proj(n, i, e) => Expr::Proj(n.clone(), *i, Box::new(named_subst_apply(e, subst))),
        other => other.clone(),
    }
}
#[cfg(test)]
mod instantiate_new_tests {
    use super::*;
    use crate::{BinderInfo, Expr, Level, Name};
    fn mk_nat_type() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_telescope_empty() {
        let tel = Telescope::new();
        assert!(tel.is_empty());
        assert_eq!(tel.len(), 0);
    }
    #[test]
    fn test_telescope_push() {
        let mut tel = Telescope::new();
        tel.push(Name::str("x"), mk_nat_type());
        tel.push(Name::str("y"), mk_nat_type());
        assert_eq!(tel.len(), 2);
    }
    #[test]
    fn test_telescope_from_pi() {
        let nat = mk_nat_type();
        let pi = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("y"),
                Box::new(nat.clone()),
                Box::new(nat.clone()),
            )),
        );
        let (tel, body) = Telescope::from_pi(&pi);
        assert_eq!(tel.len(), 2);
        assert_eq!(body, nat);
    }
    #[test]
    fn test_telescope_to_pi_roundtrip() {
        let nat = mk_nat_type();
        let mut tel = Telescope::new();
        tel.push(Name::str("x"), nat.clone());
        tel.push(Name::str("y"), nat.clone());
        let pi = tel.to_pi(nat.clone());
        let (tel2, body) = Telescope::from_pi(&pi);
        assert_eq!(tel2.len(), 2);
        assert_eq!(body, nat);
    }
    #[test]
    fn test_telescope_to_lam() {
        let nat = mk_nat_type();
        let mut tel = Telescope::new();
        tel.push(Name::str("x"), nat.clone());
        let lam = tel.to_lam(Expr::BVar(0));
        assert!(matches!(lam, Expr::Lam(_, _, _, _)));
    }
    #[test]
    fn test_named_subst_basic() {
        let mut subst = NamedSubst::new();
        assert!(subst.is_empty());
        subst.insert(Name::str("x"), Expr::Lit(crate::Literal::Nat(42)));
        assert_eq!(subst.len(), 1);
        assert!(subst.get(&Name::str("x")).is_some());
        assert!(subst.get(&Name::str("y")).is_none());
    }
    #[test]
    fn test_named_subst_apply_hit() {
        let mut subst = NamedSubst::new();
        let repl = Expr::Lit(crate::Literal::Nat(99));
        subst.insert(Name::str("x"), repl.clone());
        let expr = Expr::Const(Name::str("x"), vec![]);
        let result = subst.apply(&expr);
        assert_eq!(result, repl);
    }
    #[test]
    fn test_named_subst_apply_miss() {
        let subst = NamedSubst::new();
        let expr = Expr::Const(Name::str("Nat"), vec![]);
        let result = subst.apply(&expr);
        assert_eq!(result, expr);
    }
    #[test]
    fn test_named_subst_apply_in_app() {
        let mut subst = NamedSubst::new();
        subst.insert(Name::str("f"), Expr::Const(Name::str("g"), vec![]));
        let expr = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Lit(crate::Literal::Nat(1))),
        );
        let result = subst.apply(&expr);
        if let Expr::App(head, _) = &result {
            assert_eq!(head.as_ref(), &Expr::Const(Name::str("g"), vec![]));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_telescope_instantiate_empty_args() {
        let nat = mk_nat_type();
        let mut tel = Telescope::new();
        tel.push(Name::str("x"), nat.clone());
        let types = tel.instantiate(&[]);
        assert_eq!(types.len(), 1);
    }
    #[test]
    fn test_abstract_fvars_multi() {
        use crate::FVarId;
        let ids = vec![FVarId(0), FVarId(1)];
        let expr = Expr::App(
            Box::new(Expr::FVar(FVarId(0))),
            Box::new(Expr::FVar(FVarId(1))),
        );
        let result = abstract_fvars(&expr, &ids, 0);
        if let Expr::App(f, a) = &result {
            assert_eq!(f.as_ref(), &Expr::BVar(0));
            assert_eq!(a.as_ref(), &Expr::BVar(1));
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_named_subst_const_with_levels_not_replaced() {
        let mut subst = NamedSubst::new();
        subst.insert(Name::str("f"), Expr::Lit(crate::Literal::Nat(0)));
        let expr = Expr::Const(Name::str("f"), vec![Level::zero()]);
        let result = subst.apply(&expr);
        assert_eq!(result, expr);
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
