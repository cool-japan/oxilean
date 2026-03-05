//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::{Level, Name};

use super::types::{
    BinderInfo, ConfigNode, DecisionNode, Either2, Expr, FVarId, FVarIdGen, Fixture,
    FlatSubstitution, FocusStack, LabelSet, Literal, MinHeap, NonEmptyVec, PathBuf, PrefixCounter,
    RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc,
    StatSummary, Stopwatch, StringPool, TokenBucket, TransformStat, TransitiveClosure,
    VersionedRecord, WindowIterator, WriteOnce,
};

/// Check if an expression has a loose bound variable at the given level.
///
/// This is a helper for pretty-printing Pi types to determine if they're
/// non-dependent arrows.
pub(super) fn has_loose_bvar(e: &Expr, level: u32) -> bool {
    match e {
        Expr::BVar(n) => *n == level,
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
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expr_predicates() {
        let prop = Expr::Sort(Level::zero());
        let bvar = Expr::BVar(0);
        let fvar = Expr::FVar(FVarId::new(1));
        assert!(prop.is_sort());
        assert!(prop.is_prop());
        assert!(bvar.is_bvar());
        assert!(fvar.is_fvar());
    }
    #[test]
    fn test_expr_display() {
        let prop = Expr::Sort(Level::zero());
        let nat_const = Expr::Const(Name::str("Nat"), vec![]);
        let app = Expr::App(
            Box::new(nat_const.clone()),
            Box::new(Expr::Lit(Literal::Nat(42))),
        );
        assert_eq!(prop.to_string(), "Prop");
        assert_eq!(nat_const.to_string(), "Nat");
        assert!(app.to_string().contains("Nat"));
    }
}
pub(super) fn count_bvar_occ(e: &Expr, idx: u32) -> usize {
    match e {
        Expr::BVar(n) => {
            if *n == idx {
                1
            } else {
                0
            }
        }
        Expr::App(f, a) => count_bvar_occ(f, idx) + count_bvar_occ(a, idx),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_bvar_occ(ty, idx) + count_bvar_occ(body, idx + 1)
        }
        Expr::Let(_, ty, val, body) => {
            count_bvar_occ(ty, idx) + count_bvar_occ(val, idx) + count_bvar_occ(body, idx + 1)
        }
        Expr::Proj(_, _, s) => count_bvar_occ(s, idx),
        _ => 0,
    }
}
pub(super) fn max_loose_bvar_impl(e: &Expr, depth: u32) -> Option<u32> {
    match e {
        Expr::BVar(n) if *n >= depth => Some(*n - depth),
        Expr::App(f, a) => {
            let mf = max_loose_bvar_impl(f, depth);
            let ma = max_loose_bvar_impl(a, depth);
            match (mf, ma) {
                (Some(x), Some(y)) => Some(x.max(y)),
                (Some(x), None) | (None, Some(x)) => Some(x),
                (None, None) => None,
            }
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            let mt = max_loose_bvar_impl(ty, depth);
            let mb = max_loose_bvar_impl(body, depth + 1);
            match (mt, mb) {
                (Some(x), Some(y)) => Some(x.max(y)),
                (Some(x), None) | (None, Some(x)) => Some(x),
                (None, None) => None,
            }
        }
        Expr::Let(_, ty, val, body) => {
            let mt = max_loose_bvar_impl(ty, depth);
            let mv = max_loose_bvar_impl(val, depth);
            let mb = max_loose_bvar_impl(body, depth + 1);
            [mt, mv, mb].into_iter().flatten().max()
        }
        Expr::Proj(_, _, s) => max_loose_bvar_impl(s, depth),
        _ => None,
    }
}
pub(super) fn collect_fvars(e: &Expr, out: &mut std::collections::HashSet<FVarId>) {
    match e {
        Expr::FVar(id) => {
            out.insert(*id);
        }
        Expr::App(f, a) => {
            collect_fvars(f, out);
            collect_fvars(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_fvars(ty, out);
            collect_fvars(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars(ty, out);
            collect_fvars(val, out);
            collect_fvars(body, out);
        }
        Expr::Proj(_, _, s) => collect_fvars(s, out),
        _ => {}
    }
}
pub(super) fn collect_consts(e: &Expr, out: &mut std::collections::HashSet<Name>) {
    match e {
        Expr::Const(n, _) => {
            out.insert(n.clone());
        }
        Expr::App(f, a) => {
            collect_consts(f, out);
            collect_consts(a, out);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_consts(ty, out);
            collect_consts(body, out);
        }
        Expr::Let(_, ty, val, body) => {
            collect_consts(ty, out);
            collect_consts(val, out);
            collect_consts(body, out);
        }
        Expr::Proj(_, _, s) => collect_consts(s, out),
        _ => {}
    }
}
/// Convenience constructor: `Prop` (Sort 0).
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Convenience constructor: `Type 0` (Sort 1).
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Convenience constructor: `Type 1` (Sort 2).
pub fn type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// Build a named constant with no universe parameters.
pub fn mk_const(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
/// Build a non-dependent arrow type: `a → b`.
pub fn mk_arrow(a: Expr, b: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(a),
        Box::new(b),
    )
}
/// Build a chain of non-dependent arrow types.
///
/// `mk_arrows(&[A, B, C])` produces `A → B → C`.
pub fn mk_arrows(tys: &[Expr]) -> Expr {
    assert!(!tys.is_empty(), "mk_arrows requires at least one type");
    if tys.len() == 1 {
        return tys[0].clone();
    }
    mk_arrow(tys[0].clone(), mk_arrows(&tys[1..]))
}
#[cfg(test)]
mod extended_expr_tests {
    use super::*;
    #[test]
    fn test_fvar_id_gen_fresh() {
        let mut gen = FVarIdGen::new();
        let id0 = gen.fresh();
        let id1 = gen.fresh();
        assert_ne!(id0, id1);
        assert_eq!(id0.raw(), 0);
        assert_eq!(id1.raw(), 1);
    }
    #[test]
    fn test_fvar_id_gen_reset() {
        let mut gen = FVarIdGen::new();
        let _ = gen.fresh();
        gen.reset();
        assert_eq!(gen.current(), 0);
    }
    #[test]
    fn test_binder_info_predicates() {
        assert!(BinderInfo::Default.is_explicit());
        assert!(!BinderInfo::Implicit.is_explicit());
        assert!(BinderInfo::Implicit.is_implicit());
        assert!(BinderInfo::StrictImplicit.is_implicit());
        assert!(BinderInfo::InstImplicit.is_inst_implicit());
    }
    #[test]
    fn test_binder_info_delimiters() {
        assert_eq!(BinderInfo::Default.open_delim(), "(");
        assert_eq!(BinderInfo::Implicit.open_delim(), "{");
        assert_eq!(BinderInfo::InstImplicit.open_delim(), "[");
    }
    #[test]
    fn test_literal_predicates() {
        let n = Literal::Nat(42);
        let s = Literal::Str("hello".to_string());
        assert!(n.is_nat());
        assert!(!n.is_str());
        assert!(s.is_str());
        assert_eq!(n.as_nat(), Some(42));
        assert_eq!(s.as_str(), Some("hello"));
        assert_eq!(n.type_name(), "Nat");
        assert_eq!(s.type_name(), "String");
    }
    #[test]
    fn test_expr_is_atom() {
        assert!(Expr::Sort(Level::zero()).is_atom());
        assert!(Expr::BVar(0).is_atom());
        assert!(Expr::Const(Name::str("Nat"), vec![]).is_atom());
        assert!(!Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::BVar(0))
        )
        .is_atom());
    }
    #[test]
    fn test_expr_as_bvar() {
        assert_eq!(Expr::BVar(3).as_bvar(), Some(3));
        assert_eq!(Expr::Sort(Level::zero()).as_bvar(), None);
    }
    #[test]
    fn test_expr_as_const_name() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(e.as_const_name(), Some(&Name::str("Nat")));
        assert_eq!(Expr::BVar(0).as_const_name(), None);
    }
    #[test]
    fn test_expr_app_head_args() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = app.app_head_args();
        assert_eq!(head, &f);
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_expr_app_arity() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let app1 = Expr::App(Box::new(f.clone()), Box::new(Expr::BVar(0)));
        let app2 = Expr::App(Box::new(app1), Box::new(Expr::BVar(1)));
        assert_eq!(app2.app_arity(), 2);
        assert_eq!(f.app_arity(), 0);
    }
    #[test]
    fn test_expr_pi_arity() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let pi1 = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat.clone()),
            Box::new(nat.clone()),
        );
        let pi2 = Expr::Pi(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(nat.clone()),
            Box::new(pi1),
        );
        assert_eq!(pi2.pi_arity(), 2);
    }
    #[test]
    fn test_expr_size() {
        let e = Expr::BVar(0);
        assert_eq!(e.size(), 1);
        let app = Expr::App(Box::new(e.clone()), Box::new(e.clone()));
        assert_eq!(app.size(), 3);
    }
    #[test]
    fn test_expr_ast_depth() {
        let e = Expr::BVar(0);
        assert_eq!(e.ast_depth(), 0);
        let app = Expr::App(Box::new(e.clone()), Box::new(e.clone()));
        assert_eq!(app.ast_depth(), 1);
    }
    #[test]
    fn test_mk_app_many() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let app = f.mk_app_many(&[a, b]);
        assert_eq!(app.app_arity(), 2);
    }
    #[test]
    fn test_count_bvar_occurrences() {
        let e = Expr::App(Box::new(Expr::BVar(0)), Box::new(Expr::BVar(0)));
        assert_eq!(e.count_bvar_occurrences(0), 2);
        assert_eq!(e.count_bvar_occurrences(1), 0);
    }
    #[test]
    fn test_free_vars_empty() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(e.free_vars().is_empty());
    }
    #[test]
    fn test_free_vars_with_fvar() {
        let id = FVarId::new(99);
        let e = Expr::FVar(id);
        let fvars = e.free_vars();
        assert_eq!(fvars.len(), 1);
        assert!(fvars.contains(&id));
    }
    #[test]
    fn test_constants_collection() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let int = Expr::Const(Name::str("Int"), vec![]);
        let app = Expr::App(Box::new(nat), Box::new(int));
        let consts = app.constants();
        assert_eq!(consts.len(), 2);
    }
    #[test]
    fn test_mk_arrow() {
        let nat = mk_const("Nat");
        let arr = mk_arrow(nat.clone(), nat.clone());
        assert!(arr.is_pi());
        assert_eq!(arr.pi_arity(), 1);
    }
    #[test]
    fn test_mk_arrows_chain() {
        let nat = mk_const("Nat");
        let chain = mk_arrows(&[nat.clone(), nat.clone(), nat.clone()]);
        assert_eq!(chain.pi_arity(), 2);
    }
    #[test]
    fn test_prop_type0_type1() {
        assert!(prop().is_prop());
        assert!(type0().is_sort());
        assert!(type1().is_sort());
    }
}
/// Build a lambda over a list of binders, innermost last.
///
/// `mk_lam_many(&[(name, ty), ...], body)` wraps `body` in lambdas.
pub fn mk_lam_many(binders: &[(Name, Expr)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (name, ty)| {
        Expr::Lam(
            BinderInfo::Default,
            name.clone(),
            Box::new(ty.clone()),
            Box::new(acc),
        )
    })
}
/// Build a Pi type over a list of binders.
///
/// `mk_pi_many(&[(name, ty), ...], body)` wraps `body` in Pis.
pub fn mk_pi_many(binders: &[(Name, Expr)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (name, ty)| {
        Expr::Pi(
            BinderInfo::Default,
            name.clone(),
            Box::new(ty.clone()),
            Box::new(acc),
        )
    })
}
/// Build `Eq α a b`: the propositional equality type.
pub fn mk_eq(alpha: Expr, a: Expr, b: Expr) -> Expr {
    let eq_const = Expr::Const(Name::str("Eq"), vec![]);
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::App(Box::new(eq_const), Box::new(alpha))),
            Box::new(a),
        )),
        Box::new(b),
    )
}
/// Build `Eq.refl α a`.
pub fn mk_refl(alpha: Expr, a: Expr) -> Expr {
    let refl = Expr::Const(Name::str("Eq.refl"), vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(refl), Box::new(alpha))),
        Box::new(a),
    )
}
/// Build `And a b`: logical conjunction.
pub fn mk_and(a: Expr, b: Expr) -> Expr {
    let and_const = Expr::Const(Name::str("And"), vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(and_const), Box::new(a))),
        Box::new(b),
    )
}
/// Build `Or a b`: logical disjunction.
pub fn mk_or(a: Expr, b: Expr) -> Expr {
    let or_const = Expr::Const(Name::str("Or"), vec![]);
    Expr::App(
        Box::new(Expr::App(Box::new(or_const), Box::new(a))),
        Box::new(b),
    )
}
/// Build `Not p = p → False`.
pub fn mk_not(p: Expr) -> Expr {
    let false_expr = Expr::Const(Name::str("False"), vec![]);
    mk_arrow(p, false_expr)
}
/// Build a let-binding `let x : ty := val in body`.
pub fn mk_let(name: Name, ty: Expr, val: Expr, body: Expr) -> Expr {
    Expr::Let(name, Box::new(ty), Box::new(val), Box::new(body))
}
/// Check if two expressions share the same head constant.
pub fn same_head(e1: &Expr, e2: &Expr) -> bool {
    let h1 = app_head(e1);
    let h2 = app_head(e2);
    match (h1, h2) {
        (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
        (Expr::FVar(f1), Expr::FVar(f2)) => f1 == f2,
        (Expr::BVar(b1), Expr::BVar(b2)) => b1 == b2,
        _ => false,
    }
}
/// Return the head of an application spine (strips all Apps).
pub fn app_head(e: &Expr) -> &Expr {
    let mut cur = e;
    while let Expr::App(f, _) = cur {
        cur = f;
    }
    cur
}
/// Collect the arguments from an application spine.
pub fn app_args(e: &Expr) -> Vec<&Expr> {
    let mut args = Vec::new();
    let mut cur = e;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f;
    }
    args.reverse();
    args
}
/// Substitute `old` with `new_expr` everywhere it appears in `expr`.
///
/// Only performs exact structural matching (not modulo alpha-equivalence).
pub fn subst_expr(expr: &Expr, old: &Expr, new_expr: &Expr) -> Expr {
    if expr == old {
        return new_expr.clone();
    }
    match expr {
        Expr::App(f, a) => Expr::App(
            Box::new(subst_expr(f, old, new_expr)),
            Box::new(subst_expr(a, old, new_expr)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(subst_expr(ty, old, new_expr)),
            Box::new(subst_expr(body, old, new_expr)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(subst_expr(ty, old, new_expr)),
            Box::new(subst_expr(body, old, new_expr)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(subst_expr(ty, old, new_expr)),
            Box::new(subst_expr(val, old, new_expr)),
            Box::new(subst_expr(body, old, new_expr)),
        ),
        Expr::Proj(n, i, e) => Expr::Proj(n.clone(), *i, Box::new(subst_expr(e, old, new_expr))),
        other => other.clone(),
    }
}
/// Count the total number of occurrences of a sub-expression in another.
pub fn count_occurrences(expr: &Expr, target: &Expr) -> usize {
    if expr == target {
        return 1;
    }
    match expr {
        Expr::App(f, a) => count_occurrences(f, target) + count_occurrences(a, target),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            count_occurrences(ty, target) + count_occurrences(body, target)
        }
        Expr::Let(_, ty, val, body) => {
            count_occurrences(ty, target)
                + count_occurrences(val, target)
                + count_occurrences(body, target)
        }
        Expr::Proj(_, _, e) => count_occurrences(e, target),
        _ => 0,
    }
}
#[cfg(test)]
mod expr_new_tests {
    use super::*;
    #[test]
    fn test_mk_lam_many() {
        let nat = mk_const("Nat");
        let binders = vec![(Name::str("x"), nat.clone()), (Name::str("y"), nat.clone())];
        let body = Expr::BVar(0);
        let result = mk_lam_many(&binders, body);
        assert_eq!(result.lam_arity(), 2);
    }
    #[test]
    fn test_mk_pi_many() {
        let nat = mk_const("Nat");
        let binders = vec![(Name::str("x"), nat.clone())];
        let body = nat.clone();
        let result = mk_pi_many(&binders, body);
        assert_eq!(result.pi_arity(), 1);
    }
    #[test]
    fn test_mk_eq() {
        let nat = mk_const("Nat");
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let eq = mk_eq(nat, a, b);
        assert_eq!(eq.app_arity(), 3);
    }
    #[test]
    fn test_mk_refl() {
        let nat = mk_const("Nat");
        let a = Expr::BVar(0);
        let refl = mk_refl(nat, a);
        assert_eq!(refl.app_arity(), 2);
    }
    #[test]
    fn test_mk_and() {
        let p = mk_const("P");
        let q = mk_const("Q");
        let conj = mk_and(p, q);
        assert_eq!(conj.app_arity(), 2);
    }
    #[test]
    fn test_mk_or() {
        let p = mk_const("P");
        let q = mk_const("Q");
        let disj = mk_or(p, q);
        assert_eq!(disj.app_arity(), 2);
    }
    #[test]
    fn test_mk_not() {
        let p = mk_const("P");
        let neg = mk_not(p);
        assert!(neg.is_pi());
    }
    #[test]
    fn test_mk_let() {
        let nat = mk_const("Nat");
        let val = Expr::Lit(Literal::Nat(42));
        let body = Expr::BVar(0);
        let let_expr = mk_let(Name::str("x"), nat, val, body);
        assert!(let_expr.is_let());
    }
    #[test]
    fn test_app_head() {
        let f = mk_const("f");
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a))),
            Box::new(b),
        );
        assert_eq!(app_head(&app), &f);
    }
    #[test]
    fn test_app_args() {
        let f = mk_const("f");
        let a = Expr::BVar(0);
        let b = Expr::BVar(1);
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let args = app_args(&app);
        assert_eq!(args.len(), 2);
    }
    #[test]
    fn test_same_head_true() {
        let f = mk_const("f");
        let e1 = Expr::App(Box::new(f.clone()), Box::new(Expr::BVar(0)));
        let e2 = Expr::App(Box::new(f.clone()), Box::new(Expr::BVar(1)));
        assert!(same_head(&e1, &e2));
    }
    #[test]
    fn test_same_head_false() {
        let f = mk_const("f");
        let g = mk_const("g");
        let e1 = Expr::App(Box::new(f), Box::new(Expr::BVar(0)));
        let e2 = Expr::App(Box::new(g), Box::new(Expr::BVar(0)));
        assert!(!same_head(&e1, &e2));
    }
    #[test]
    fn test_subst_expr() {
        let target = Expr::Lit(Literal::Nat(1));
        let replacement = Expr::Lit(Literal::Nat(99));
        let expr = Expr::App(Box::new(target.clone()), Box::new(target.clone()));
        let result = subst_expr(&expr, &target, &replacement);
        if let Expr::App(f, a) = &result {
            assert_eq!(f.as_ref(), &replacement);
            assert_eq!(a.as_ref(), &replacement);
        } else {
            panic!("expected App");
        }
    }
    #[test]
    fn test_count_occurrences() {
        let target = Expr::Lit(Literal::Nat(42));
        let expr = Expr::App(
            Box::new(Expr::App(
                Box::new(target.clone()),
                Box::new(target.clone()),
            )),
            Box::new(target.clone()),
        );
        assert_eq!(count_occurrences(&expr, &target), 3);
    }
    #[test]
    fn test_count_occurrences_not_found() {
        let target = Expr::Lit(Literal::Nat(0));
        let expr = Expr::Lit(Literal::Nat(1));
        assert_eq!(count_occurrences(&expr, &target), 0);
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
