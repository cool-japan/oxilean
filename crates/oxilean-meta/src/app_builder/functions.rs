//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AppBuildCache, AppBuildLogger, AppBuildPriorityQueue, AppBuildRegistry, AppBuildStats,
    AppBuildUtil0, AppBuilderAnalysisPass, AppBuilderConfig, AppBuilderConfigValue,
    AppBuilderDiagnostics, AppBuilderDiff, AppBuilderExtConfig3700, AppBuilderExtConfigVal3700,
    AppBuilderExtDiag3700, AppBuilderExtDiff3700, AppBuilderExtPass3700, AppBuilderExtPipeline3700,
    AppBuilderExtResult3700, AppBuilderPipeline, AppBuilderResult,
};
use oxilean_kernel::{BinderInfo, Expr, Level, Name};

/// Build `@Eq α a b` (equality type).
pub fn mk_eq(ty: Expr, lhs: Expr, rhs: Expr) -> Expr {
    mk_app3(
        Expr::Const(Name::str("Eq"), vec![Level::zero()]),
        ty,
        lhs,
        rhs,
    )
}
/// Build `@Eq.refl α a` (reflexivity proof).
pub fn mk_eq_refl(ty: Expr, val: Expr) -> Expr {
    mk_app2(
        Expr::Const(Name::str("Eq.refl"), vec![Level::zero()]),
        ty,
        val,
    )
}
/// Build `@Eq.symm α a b h` (symmetry proof).
pub fn mk_eq_symm(ty: Expr, lhs: Expr, rhs: Expr, proof: Expr) -> Expr {
    mk_app4(
        Expr::Const(Name::str("Eq.symm"), vec![Level::zero()]),
        ty,
        lhs,
        rhs,
        proof,
    )
}
/// Build `@Eq.trans α a b c h₁ h₂` (transitivity proof).
#[allow(clippy::too_many_arguments)]
pub fn mk_eq_trans(ty: Expr, a: Expr, b: Expr, c: Expr, proof1: Expr, proof2: Expr) -> Expr {
    mk_app6(
        Expr::Const(Name::str("Eq.trans"), vec![Level::zero()]),
        ty,
        a,
        b,
        c,
        proof1,
        proof2,
    )
}
/// Build `@congrArg α β f a b h` (congruence of function application).
#[allow(clippy::too_many_arguments)]
pub fn mk_congr_arg(alpha: Expr, beta: Expr, f: Expr, a: Expr, b: Expr, proof: Expr) -> Expr {
    mk_app6(
        Expr::Const(Name::str("congrArg"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
        f,
        a,
        b,
        proof,
    )
}
/// Build `@congrFun α β f g a h` (congruence of function equality applied to arg).
#[allow(clippy::too_many_arguments)]
pub fn mk_congr_fun(alpha: Expr, beta: Expr, f: Expr, g: Expr, a: Expr, proof: Expr) -> Expr {
    mk_app6(
        Expr::Const(Name::str("congrFun"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
        f,
        g,
        a,
        proof,
    )
}
/// Build `@congr α β f g a b h₁ h₂` (full congruence).
#[allow(clippy::too_many_arguments)]
pub fn mk_congr(
    alpha: Expr,
    beta: Expr,
    f: Expr,
    g: Expr,
    a: Expr,
    b: Expr,
    proof_f: Expr,
    proof_a: Expr,
) -> Expr {
    mk_apps(
        Expr::Const(Name::str("congr"), vec![Level::zero(), Level::zero()]),
        &[alpha, beta, f, g, a, b, proof_f, proof_a],
    )
}
/// Build `@funext α β f g h` (function extensionality).
pub fn mk_funext(alpha: Expr, beta: Expr, f: Expr, g: Expr, proof: Expr) -> Expr {
    mk_app5(
        Expr::Const(Name::str("funext"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
        f,
        g,
        proof,
    )
}
/// Build `@propext a b h` (propositional extensionality).
pub fn mk_propext(a: Expr, b: Expr, proof: Expr) -> Expr {
    mk_app3(Expr::Const(Name::str("propext"), vec![]), a, b, proof)
}
/// Build `@absurd a b h₁ h₂` (derive anything from contradiction).
pub fn mk_absurd(a: Expr, b: Expr, proof: Expr, neg_proof: Expr) -> Expr {
    mk_app4(
        Expr::Const(Name::str("absurd"), vec![Level::zero()]),
        a,
        b,
        proof,
        neg_proof,
    )
}
/// Build `Not p` (negation).
pub fn mk_not(p: Expr) -> Expr {
    mk_app1(Expr::Const(Name::str("Not"), vec![]), p)
}
/// Build `And p q`.
pub fn mk_and(p: Expr, q: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("And"), vec![]), p, q)
}
/// Build `Or p q`.
pub fn mk_or(p: Expr, q: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("Or"), vec![]), p, q)
}
/// Build `Iff p q`.
pub fn mk_iff(p: Expr, q: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("Iff"), vec![]), p, q)
}
/// Build `True`.
pub fn mk_true() -> Expr {
    Expr::Const(Name::str("True"), vec![])
}
/// Build `False`.
pub fn mk_false() -> Expr {
    Expr::Const(Name::str("False"), vec![])
}
/// Build `@HEq α a β b` (heterogeneous equality).
pub fn mk_heq(alpha: Expr, a: Expr, beta: Expr, b: Expr) -> Expr {
    mk_app4(
        Expr::Const(Name::str("HEq"), vec![Level::zero(), Level::zero()]),
        alpha,
        a,
        beta,
        b,
    )
}
/// Build a non-dependent arrow type: `α → β`.
pub fn mk_arrow(domain: Expr, codomain: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(domain),
        Box::new(codomain),
    )
}
/// Build `@Prod α β`.
pub fn mk_prod(alpha: Expr, beta: Expr) -> Expr {
    mk_app2(
        Expr::Const(Name::str("Prod"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
    )
}
/// Build `@Sigma α β`.
pub fn mk_sigma(alpha: Expr, beta: Expr) -> Expr {
    mk_app2(
        Expr::Const(Name::str("Sigma"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
    )
}
/// Build `f a`.
pub(super) fn mk_app1(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
/// Build `f a b`.
pub(super) fn mk_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    Expr::App(Box::new(mk_app1(f, a)), Box::new(b))
}
/// Build `f a b c`.
pub(super) fn mk_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    Expr::App(Box::new(mk_app2(f, a, b)), Box::new(c))
}
/// Build `f a b c d`.
pub(super) fn mk_app4(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr) -> Expr {
    Expr::App(Box::new(mk_app3(f, a, b, c)), Box::new(d))
}
/// Build `f a b c d e`.
pub(super) fn mk_app5(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr, e: Expr) -> Expr {
    Expr::App(Box::new(mk_app4(f, a, b, c, d)), Box::new(e))
}
/// Build `f a b c d e g`.
pub(super) fn mk_app6(f: Expr, a: Expr, b: Expr, c: Expr, d: Expr, e: Expr, g: Expr) -> Expr {
    Expr::App(Box::new(mk_app5(f, a, b, c, d, e)), Box::new(g))
}
/// Build `f args[0] args[1] ... args[n-1]`.
pub(super) fn mk_apps(f: Expr, args: &[Expr]) -> Expr {
    let mut result = f;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg.clone()));
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_builder::*;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn zero() -> Expr {
        Expr::Const(Name::str("Nat.zero"), vec![])
    }
    #[test]
    fn test_mk_eq() {
        let eq = mk_eq(nat(), zero(), zero());
        assert!(matches!(eq, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_eq_refl() {
        let refl = mk_eq_refl(nat(), zero());
        assert!(matches!(refl, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_not() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let not_p = mk_not(p);
        assert!(matches!(not_p, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_and() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let and_pq = mk_and(p, q);
        assert!(matches!(and_pq, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_or() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let or_pq = mk_or(p, q);
        assert!(matches!(or_pq, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_true() {
        let t = mk_true();
        assert_eq!(t, Expr::Const(Name::str("True"), vec![]));
    }
    #[test]
    fn test_mk_false() {
        let f = mk_false();
        assert_eq!(f, Expr::Const(Name::str("False"), vec![]));
    }
    #[test]
    fn test_mk_arrow() {
        let arr = mk_arrow(nat(), nat());
        assert!(matches!(arr, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_iff() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let iff = mk_iff(p, q);
        assert!(matches!(iff, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_prod() {
        let prod = mk_prod(nat(), nat());
        assert!(matches!(prod, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_apps() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let result = mk_apps(f, &[a, b]);
        assert!(matches!(result, Expr::App(_, _)));
    }
}
/// Build `Nat.zero`.
pub fn mk_nat_zero() -> Expr {
    Expr::Const(Name::str("Nat.zero"), vec![])
}
/// Build `Nat.succ n`.
pub fn mk_nat_succ(n: Expr) -> Expr {
    mk_app1(Expr::Const(Name::str("Nat.succ"), vec![]), n)
}
/// Build a numeral expression from a `u64`.
///
/// Constructs the corresponding chain of `Nat.succ` applications.
/// Uses `Nat.ofNat` for large values to avoid deep nesting.
pub fn mk_nat_lit(n: u64) -> Expr {
    Expr::Lit(oxilean_kernel::Literal::Nat(n))
}
/// Build `@List.nil α`.
pub fn mk_list_nil(alpha: Expr) -> Expr {
    mk_app1(
        Expr::Const(Name::str("List.nil"), vec![Level::zero()]),
        alpha,
    )
}
/// Build `@List.cons α head tail`.
pub fn mk_list_cons(alpha: Expr, head: Expr, tail: Expr) -> Expr {
    mk_app3(
        Expr::Const(Name::str("List.cons"), vec![Level::zero()]),
        alpha,
        head,
        tail,
    )
}
/// Build `@Option.none α`.
pub fn mk_option_none(alpha: Expr) -> Expr {
    mk_app1(
        Expr::Const(Name::str("Option.none"), vec![Level::zero()]),
        alpha,
    )
}
/// Build `@Option.some α a`.
pub fn mk_option_some(alpha: Expr, val: Expr) -> Expr {
    mk_app2(
        Expr::Const(Name::str("Option.some"), vec![Level::zero()]),
        alpha,
        val,
    )
}
/// Build `@Prod.mk α β a b`.
pub fn mk_prod_mk(alpha: Expr, beta: Expr, a: Expr, b: Expr) -> Expr {
    mk_app4(
        Expr::Const(Name::str("Prod.mk"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
        a,
        b,
    )
}
/// Build `@Prod.fst α β pair`.
pub fn mk_prod_fst(alpha: Expr, beta: Expr, pair: Expr) -> Expr {
    mk_app3(
        Expr::Const(Name::str("Prod.fst"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
        pair,
    )
}
/// Build `@Prod.snd α β pair`.
pub fn mk_prod_snd(alpha: Expr, beta: Expr, pair: Expr) -> Expr {
    mk_app3(
        Expr::Const(Name::str("Prod.snd"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
        pair,
    )
}
/// Build `@Sum.inl α β val`.
pub fn mk_sum_inl(alpha: Expr, beta: Expr, val: Expr) -> Expr {
    mk_app3(
        Expr::Const(Name::str("Sum.inl"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
        val,
    )
}
/// Build `@Sum.inr α β val`.
pub fn mk_sum_inr(alpha: Expr, beta: Expr, val: Expr) -> Expr {
    mk_app3(
        Expr::Const(Name::str("Sum.inr"), vec![Level::zero(), Level::zero()]),
        alpha,
        beta,
        val,
    )
}
/// Build `@Subtype.mk α p val proof`.
pub fn mk_subtype_mk(alpha: Expr, p: Expr, val: Expr, proof: Expr) -> Expr {
    mk_app4(
        Expr::Const(Name::str("Subtype.mk"), vec![Level::zero()]),
        alpha,
        p,
        val,
        proof,
    )
}
/// Build `Nat → Nat` (a simple function type between naturals).
pub fn mk_nat_to_nat() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::Anonymous,
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
        Box::new(Expr::Const(Name::str("Nat"), vec![])),
    )
}
/// Build `∀ (x : α), P x` from a name, type, and body.
pub fn mk_forall(name: Name, alpha: Expr, body: Expr) -> Expr {
    Expr::Pi(BinderInfo::Default, name, Box::new(alpha), Box::new(body))
}
/// Build `∀ (x : α), P x` with implicit binder.
pub fn mk_forall_implicit(name: Name, alpha: Expr, body: Expr) -> Expr {
    Expr::Pi(BinderInfo::Implicit, name, Box::new(alpha), Box::new(body))
}
/// Build a lambda abstraction.
pub fn mk_lam(name: Name, alpha: Expr, body: Expr) -> Expr {
    Expr::Lam(BinderInfo::Default, name, Box::new(alpha), Box::new(body))
}
/// Build a let expression.
pub fn mk_let(name: Name, ty: Expr, val: Expr, body: Expr) -> Expr {
    Expr::Let(name, Box::new(ty), Box::new(val), Box::new(body))
}
/// Build `Prop` (Sort 0).
pub fn mk_prop() -> Expr {
    Expr::Sort(Level::zero())
}
/// Build `Type` (Sort 1).
pub fn mk_type() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// Build `Type 2` (Sort 2).
pub fn mk_type2() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}
/// Build `@And.intro p q h1 h2`.
pub fn mk_and_intro(p: Expr, q: Expr, h1: Expr, h2: Expr) -> Expr {
    mk_app4(Expr::Const(Name::str("And.intro"), vec![]), p, q, h1, h2)
}
/// Build `@And.left p q h`.
pub fn mk_and_left(p: Expr, q: Expr, h: Expr) -> Expr {
    mk_app3(Expr::Const(Name::str("And.left"), vec![]), p, q, h)
}
/// Build `@And.right p q h`.
pub fn mk_and_right(p: Expr, q: Expr, h: Expr) -> Expr {
    mk_app3(Expr::Const(Name::str("And.right"), vec![]), p, q, h)
}
/// Build `@Or.inl p q h`.
pub fn mk_or_inl(p: Expr, q: Expr, h: Expr) -> Expr {
    mk_app3(Expr::Const(Name::str("Or.inl"), vec![]), p, q, h)
}
/// Build `@Or.inr p q h`.
pub fn mk_or_inr(p: Expr, q: Expr, h: Expr) -> Expr {
    mk_app3(Expr::Const(Name::str("Or.inr"), vec![]), p, q, h)
}
/// Build `@Iff.intro p q h1 h2`.
pub fn mk_iff_intro(p: Expr, q: Expr, h1: Expr, h2: Expr) -> Expr {
    mk_app4(Expr::Const(Name::str("Iff.intro"), vec![]), p, q, h1, h2)
}
/// Build `@Iff.mp p q iff_pq`.
pub fn mk_iff_mp(p: Expr, q: Expr, iff: Expr) -> Expr {
    mk_app3(Expr::Const(Name::str("Iff.mp"), vec![]), p, q, iff)
}
/// Build `@Iff.mpr p q iff_pq`.
pub fn mk_iff_mpr(p: Expr, q: Expr, iff: Expr) -> Expr {
    mk_app3(Expr::Const(Name::str("Iff.mpr"), vec![]), p, q, iff)
}
/// Build `@Classical.em p`.
pub fn mk_classical_em(p: Expr) -> Expr {
    mk_app1(Expr::Const(Name::str("Classical.em"), vec![]), p)
}
/// Build `@Classical.byContradiction p h`.
pub fn mk_by_contradiction(p: Expr, h: Expr) -> Expr {
    mk_app2(
        Expr::Const(Name::str("Classical.byContradiction"), vec![]),
        p,
        h,
    )
}
/// Build `@id α a`.
pub fn mk_id(alpha: Expr, a: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("id"), vec![Level::zero()]), alpha, a)
}
/// Build `@Function.comp α β γ f g`.
pub fn mk_function_comp(alpha: Expr, beta: Expr, gamma: Expr, f: Expr, g: Expr) -> Expr {
    mk_app5(
        Expr::Const(
            Name::str("Function.comp"),
            vec![Level::zero(), Level::zero(), Level::zero()],
        ),
        alpha,
        beta,
        gamma,
        f,
        g,
    )
}
#[cfg(test)]
mod tests_extra {
    use super::*;
    use crate::app_builder::*;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_mk_nat_zero() {
        let z = mk_nat_zero();
        assert_eq!(z, Expr::Const(Name::str("Nat.zero"), vec![]));
    }
    #[test]
    fn test_mk_nat_succ() {
        let z = mk_nat_zero();
        let one = mk_nat_succ(z);
        assert!(matches!(one, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_nat_lit() {
        let lit = mk_nat_lit(42);
        assert!(matches!(lit, Expr::Lit(_)));
    }
    #[test]
    fn test_mk_list_nil() {
        let nil = mk_list_nil(nat());
        assert!(matches!(nil, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_option_some_none() {
        let none = mk_option_none(nat());
        let some_z = mk_option_some(nat(), mk_nat_zero());
        assert!(matches!(none, Expr::App(_, _)));
        assert!(matches!(some_z, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_prod_mk() {
        let pair = mk_prod_mk(nat(), nat(), mk_nat_zero(), mk_nat_zero());
        assert!(matches!(pair, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_forall() {
        let f = mk_forall(Name::str("x"), nat(), Expr::BVar(0));
        assert!(matches!(f, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_forall_implicit() {
        let f = mk_forall_implicit(Name::str("α"), mk_type(), Expr::BVar(0));
        assert!(matches!(f, Expr::Pi(BinderInfo::Implicit, _, _, _)));
    }
    #[test]
    fn test_mk_lam() {
        let l = mk_lam(Name::str("x"), nat(), Expr::BVar(0));
        assert!(matches!(l, Expr::Lam(BinderInfo::Default, _, _, _)));
    }
    #[test]
    fn test_mk_let() {
        let let_expr = mk_let(Name::str("x"), nat(), mk_nat_zero(), Expr::BVar(0));
        assert!(matches!(let_expr, Expr::Let(_, _, _, _)));
    }
    #[test]
    fn test_mk_prop_type() {
        assert_eq!(mk_prop(), Expr::Sort(Level::zero()));
        assert_eq!(mk_type(), Expr::Sort(Level::succ(Level::zero())));
    }
    #[test]
    fn test_mk_and_intro() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let hp = Expr::Const(Name::str("hp"), vec![]);
        let hq = Expr::Const(Name::str("hq"), vec![]);
        let intro = mk_and_intro(p, q, hp, hq);
        assert!(matches!(intro, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_or_inl() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let hp = Expr::Const(Name::str("hp"), vec![]);
        let inl = mk_or_inl(p, q, hp);
        assert!(matches!(inl, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_iff_mp() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let iff = Expr::Const(Name::str("iff_pq"), vec![]);
        let mp = mk_iff_mp(p, q, iff);
        assert!(matches!(mp, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_nat_to_nat() {
        let fn_ty = mk_nat_to_nat();
        assert!(matches!(fn_ty, Expr::Pi(BinderInfo::Default, _, _, _)));
    }
}
/// Collect the head function and argument list of a nested application.
pub fn collect_app(expr: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut e = expr.clone();
    while let Expr::App(f, a) = e {
        args.push(*a);
        e = *f;
    }
    args.reverse();
    (e, args)
}
/// Rebuild a nested application from head and arguments.
pub fn rebuild_app(head: Expr, args: Vec<Expr>) -> Expr {
    let mut result = head;
    for arg in args {
        result = Expr::App(Box::new(result), Box::new(arg));
    }
    result
}
/// Get the number of arguments applied to a head in a nested application.
pub fn app_arity(expr: &Expr) -> usize {
    let mut count = 0;
    let mut e = expr;
    while let Expr::App(f, _) = e {
        count += 1;
        e = f;
    }
    count
}
/// Get just the head function (removing all arguments).
pub fn app_head(expr: &Expr) -> &Expr {
    let mut e = expr;
    while let Expr::App(f, _) = e {
        e = f;
    }
    e
}
/// Check if `expr` is an application of `const_name` to some arguments.
pub fn is_app_of(expr: &Expr, const_name: &Name) -> bool {
    match app_head(expr) {
        Expr::Const(n, _) => n == const_name,
        _ => false,
    }
}
/// Build `@Exists α p` (the existential quantifier applied to type and predicate).
pub fn mk_exists(alpha: Expr, predicate: Expr) -> Expr {
    mk_app2(
        Expr::Const(Name::str("Exists"), vec![Level::zero()]),
        alpha,
        predicate,
    )
}
/// Build `@ExistsUnique α p`.
pub fn mk_exists_unique(alpha: Expr, predicate: Expr) -> Expr {
    mk_app2(
        Expr::Const(Name::str("ExistsUnique"), vec![Level::zero()]),
        alpha,
        predicate,
    )
}
/// Build a Prop-valued conjunction chain `p1 ∧ p2 ∧ ... ∧ pn`.
///
/// Returns `True` if the list is empty.
pub fn mk_and_chain(props: &[Expr]) -> Expr {
    match props {
        [] => mk_true(),
        [p] => p.clone(),
        [p, rest @ ..] => mk_and(p.clone(), mk_and_chain(rest)),
    }
}
/// Build a Prop-valued disjunction chain `p1 ∨ p2 ∨ ... ∨ pn`.
///
/// Returns `False` if the list is empty.
pub fn mk_or_chain(props: &[Expr]) -> Expr {
    match props {
        [] => mk_false(),
        [p] => p.clone(),
        [p, rest @ ..] => mk_or(p.clone(), mk_or_chain(rest)),
    }
}
#[cfg(test)]
mod tests_util {
    use super::*;
    use crate::app_builder::*;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    #[test]
    fn test_collect_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Lit(oxilean_kernel::Literal::Nat(1));
        let b = Expr::Lit(oxilean_kernel::Literal::Nat(2));
        let app = Expr::App(
            Box::new(Expr::App(Box::new(f.clone()), Box::new(a.clone()))),
            Box::new(b.clone()),
        );
        let (head, args) = collect_app(&app);
        assert_eq!(head, f);
        assert_eq!(args, vec![a, b]);
    }
    #[test]
    fn test_rebuild_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = nat();
        let rebuilt = rebuild_app(f.clone(), vec![a.clone()]);
        assert_eq!(rebuilt, Expr::App(Box::new(f), Box::new(a)));
    }
    #[test]
    fn test_app_arity() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = nat();
        let b = nat();
        let app = Expr::App(Box::new(Expr::App(Box::new(f), Box::new(a))), Box::new(b));
        assert_eq!(app_arity(&app), 2);
        assert_eq!(app_arity(&nat()), 0);
    }
    #[test]
    fn test_app_head() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = nat();
        let app = Expr::App(Box::new(f.clone()), Box::new(a));
        assert_eq!(app_head(&app), &f);
    }
    #[test]
    fn test_is_app_of() {
        let f = Expr::Const(Name::str("Nat.succ"), vec![]);
        let z = Expr::Const(Name::str("Nat.zero"), vec![]);
        let one = Expr::App(Box::new(f), Box::new(z));
        assert!(is_app_of(&one, &Name::str("Nat.succ")));
        assert!(!is_app_of(&one, &Name::str("Nat.zero")));
    }
    #[test]
    fn test_mk_and_chain_empty() {
        assert_eq!(mk_and_chain(&[]), mk_true());
    }
    #[test]
    fn test_mk_and_chain_one() {
        let p = Expr::Const(Name::str("P"), vec![]);
        assert_eq!(mk_and_chain(std::slice::from_ref(&p)), p);
    }
    #[test]
    fn test_mk_and_chain_two() {
        let p = Expr::Const(Name::str("P"), vec![]);
        let q = Expr::Const(Name::str("Q"), vec![]);
        let chain = mk_and_chain(&[p.clone(), q.clone()]);
        assert_eq!(chain, mk_and(p, q));
    }
    #[test]
    fn test_mk_or_chain_empty() {
        assert_eq!(mk_or_chain(&[]), mk_false());
    }
    #[test]
    fn test_mk_exists() {
        let e = mk_exists(nat(), Expr::Const(Name::str("P"), vec![]));
        assert!(matches!(e, Expr::App(_, _)));
    }
}
/// Return a compact one-line string representation of an expression.
///
/// Used for debugging and error messages. Not a full pretty-printer.
#[allow(dead_code)]
pub fn expr_to_debug_str(expr: &Expr) -> String {
    match expr {
        Expr::BVar(i) => format!("#{}", i),
        Expr::FVar(id) => format!("fv({})", id.0),
        Expr::Const(n, _) => n.to_string(),
        Expr::Sort(l) => format!("Sort({})", l),
        Expr::Lit(oxilean_kernel::Literal::Nat(n)) => format!("{}", n),
        Expr::Lit(oxilean_kernel::Literal::Str(s)) => format!("{:?}", s),
        Expr::App(f, a) => format!("({} {})", expr_to_debug_str(f), expr_to_debug_str(a)),
        Expr::Lam(_, n, _, b) => format!("(fun {} => {})", n, expr_to_debug_str(b)),
        Expr::Pi(_, n, _, b) => format!("(forall {} => {})", n, expr_to_debug_str(b)),
        Expr::Let(n, _, v, b) => {
            format!(
                "(let {} := {} in {})",
                n,
                expr_to_debug_str(v),
                expr_to_debug_str(b)
            )
        }
        Expr::Proj(_, idx, e) => format!("{}.{}", expr_to_debug_str(e), idx),
    }
}
/// Check if two expressions are structurally identical (pointer-equal types).
#[allow(dead_code)]
pub fn exprs_structurally_eq(e1: &Expr, e2: &Expr) -> bool {
    e1 == e2
}
/// Return the number of `App` nodes at the outermost spine.
#[allow(dead_code)]
pub fn spine_length(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, _) => 1 + spine_length(f),
        _ => 0,
    }
}
/// Return `true` if the expression has a `Const` node as its outermost head.
#[allow(dead_code)]
pub fn has_const_head(expr: &Expr) -> bool {
    matches!(app_head(expr), Expr::Const(_, _))
}
/// Build `@Eq.subst α motive a b h_eq h_a`.
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
pub fn mk_eq_subst(alpha: Expr, motive: Expr, a: Expr, b: Expr, h_eq: Expr, h_a: Expr) -> Expr {
    mk_apps(
        Expr::Const(Name::str("Eq.subst"), vec![Level::zero(), Level::zero()]),
        &[alpha, motive, a, b, h_eq, h_a],
    )
}
/// Build `@Nat.rec motive hz hs n` (primitive Nat recursion).
#[allow(dead_code)]
pub fn mk_nat_rec(motive: Expr, hz: Expr, hs: Expr, n: Expr) -> Expr {
    let f = Expr::Const(Name::str("Nat.rec"), vec![Level::zero()]);
    mk_apps(f, &[motive, hz, hs, n])
}
/// Build `Bool.true`.
#[allow(dead_code)]
pub fn mk_bool_true() -> Expr {
    Expr::Const(Name::str("Bool.true"), vec![])
}
/// Build `Bool.false`.
#[allow(dead_code)]
pub fn mk_bool_false() -> Expr {
    Expr::Const(Name::str("Bool.false"), vec![])
}
/// Build `@Bool.rec motive ht hf b` (Bool elimination).
#[allow(dead_code)]
pub fn mk_bool_rec(motive: Expr, ht: Expr, hf: Expr, b: Expr) -> Expr {
    let f = Expr::Const(Name::str("Bool.rec"), vec![Level::zero()]);
    mk_apps(f, &[motive, ht, hf, b])
}
/// Build `@Nat.add a b`.
#[allow(dead_code)]
pub fn mk_nat_add(a: Expr, b: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("Nat.add"), vec![]), a, b)
}
/// Build `@Nat.mul a b`.
#[allow(dead_code)]
pub fn mk_nat_mul(a: Expr, b: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("Nat.mul"), vec![]), a, b)
}
/// Build `@Nat.le a b` (a ≤ b).
#[allow(dead_code)]
pub fn mk_nat_le(a: Expr, b: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("Nat.le"), vec![]), a, b)
}
/// Build `@Nat.lt a b` (a < b).
#[allow(dead_code)]
pub fn mk_nat_lt(a: Expr, b: Expr) -> Expr {
    mk_app2(Expr::Const(Name::str("Nat.lt"), vec![]), a, b)
}
/// Build `@List.length α l`.
#[allow(dead_code)]
pub fn mk_list_length(alpha: Expr, l: Expr) -> Expr {
    mk_app2(
        Expr::Const(Name::str("List.length"), vec![Level::zero()]),
        alpha,
        l,
    )
}
/// Build `@List.append α l1 l2`.
#[allow(dead_code)]
pub fn mk_list_append(alpha: Expr, l1: Expr, l2: Expr) -> Expr {
    mk_app3(
        Expr::Const(Name::str("List.append"), vec![Level::zero()]),
        alpha,
        l1,
        l2,
    )
}
#[cfg(test)]
mod expr_helpers_tests {
    use super::*;
    use crate::app_builder::*;
    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn zero() -> Expr {
        Expr::Const(Name::str("Nat.zero"), vec![])
    }
    #[test]
    fn test_expr_to_debug_str_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert_eq!(expr_to_debug_str(&e), "Nat");
    }
    #[test]
    fn test_expr_to_debug_str_lit() {
        let e = Expr::Lit(oxilean_kernel::Literal::Nat(42));
        assert_eq!(expr_to_debug_str(&e), "42");
    }
    #[test]
    fn test_expr_to_debug_str_app() {
        let e = Expr::App(
            Box::new(Expr::Const(Name::str("f"), vec![])),
            Box::new(Expr::Const(Name::str("a"), vec![])),
        );
        let s = expr_to_debug_str(&e);
        assert!(s.contains('f'));
        assert!(s.contains('a'));
    }
    #[test]
    fn test_spine_length_none() {
        assert_eq!(spine_length(&zero()), 0);
    }
    #[test]
    fn test_spine_length_one() {
        let e = Expr::App(Box::new(zero()), Box::new(zero()));
        assert_eq!(spine_length(&e), 1);
    }
    #[test]
    fn test_has_const_head_const() {
        assert!(has_const_head(&zero()));
    }
    #[test]
    fn test_has_const_head_app() {
        let e = Expr::App(Box::new(zero()), Box::new(zero()));
        assert!(has_const_head(&e));
    }
    #[test]
    fn test_has_const_head_bvar() {
        assert!(!has_const_head(&Expr::BVar(0)));
    }
    #[test]
    fn test_mk_bool_true_false() {
        let t = mk_bool_true();
        let f = mk_bool_false();
        assert!(matches!(t, Expr::Const(n, _) if n == Name::str("Bool.true")));
        assert!(matches!(f, Expr::Const(n, _) if n == Name::str("Bool.false")));
    }
    #[test]
    fn test_mk_nat_add() {
        let e = mk_nat_add(zero(), zero());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_nat_mul() {
        let e = mk_nat_mul(zero(), zero());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_nat_le() {
        let e = mk_nat_le(zero(), zero());
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_mk_list_length() {
        let e = mk_list_length(nat(), mk_list_nil(nat()));
        assert!(matches!(e, Expr::App(_, _)));
    }
    #[test]
    fn test_exprs_structurally_eq() {
        let e = Expr::Const(Name::str("x"), vec![]);
        assert!(exprs_structurally_eq(&e, &e.clone()));
    }
}
/// Compute a simple hash of a AppBuild name.
#[allow(dead_code)]
pub fn appbuild_hash(name: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in name.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}
/// Check if a AppBuild name is valid.
#[allow(dead_code)]
pub fn appbuild_is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
/// Count the occurrences of a character in a AppBuild string.
#[allow(dead_code)]
pub fn appbuild_count_char(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}
/// Truncate a AppBuild string to a maximum length.
#[allow(dead_code)]
pub fn appbuild_truncate(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}
/// Join AppBuild strings with a separator.
#[allow(dead_code)]
pub fn appbuild_join(parts: &[&str], sep: &str) -> String {
    parts.join(sep)
}
#[cfg(test)]
mod appbuild_ext_tests {
    use super::*;
    use crate::app_builder::*;
    #[test]
    fn test_appbuild_util_new() {
        let u = AppBuildUtil0::new(1, "test", 42);
        assert_eq!(u.id, 1);
        assert_eq!(u.name, "test");
        assert_eq!(u.value, 42);
        assert!(u.is_active());
    }
    #[test]
    fn test_appbuild_util_tag() {
        let u = AppBuildUtil0::new(2, "tagged", 10).with_tag("important");
        assert!(u.has_tag("important"));
        assert_eq!(u.tag_count(), 1);
    }
    #[test]
    fn test_appbuild_util_disable() {
        let u = AppBuildUtil0::new(3, "disabled", 100).disable();
        assert!(!u.is_active());
        assert_eq!(u.score(), 0);
    }
    #[test]
    fn test_appbuild_registry_register() {
        let mut reg = AppBuildRegistry::new(10);
        let u = AppBuildUtil0::new(1, "a", 1);
        assert!(reg.register(u));
        assert_eq!(reg.count(), 1);
    }
    #[test]
    fn test_appbuild_registry_lookup() {
        let mut reg = AppBuildRegistry::new(10);
        reg.register(AppBuildUtil0::new(5, "five", 5));
        assert!(reg.lookup(5).is_some());
        assert!(reg.lookup(99).is_none());
    }
    #[test]
    fn test_appbuild_registry_capacity() {
        let mut reg = AppBuildRegistry::new(2);
        reg.register(AppBuildUtil0::new(1, "a", 1));
        reg.register(AppBuildUtil0::new(2, "b", 2));
        assert!(reg.is_full());
        assert!(!reg.register(AppBuildUtil0::new(3, "c", 3)));
    }
    #[test]
    fn test_appbuild_registry_score() {
        let mut reg = AppBuildRegistry::new(10);
        reg.register(AppBuildUtil0::new(1, "a", 10));
        reg.register(AppBuildUtil0::new(2, "b", 20));
        assert_eq!(reg.total_score(), 30);
    }
    #[test]
    fn test_appbuild_cache_hit_miss() {
        let mut cache = AppBuildCache::new();
        cache.insert("key1", 42);
        assert_eq!(cache.get("key1"), Some(42));
        assert_eq!(cache.get("key2"), None);
        assert_eq!(cache.hits, 1);
        assert_eq!(cache.misses, 1);
    }
    #[test]
    fn test_appbuild_cache_hit_rate() {
        let mut cache = AppBuildCache::new();
        cache.insert("k", 1);
        cache.get("k");
        cache.get("k");
        cache.get("nope");
        assert!((cache.hit_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_appbuild_cache_clear() {
        let mut cache = AppBuildCache::new();
        cache.insert("k", 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hits, 0);
    }
    #[test]
    fn test_appbuild_logger_basic() {
        let mut logger = AppBuildLogger::new(100);
        logger.log("msg1");
        logger.log("msg2");
        assert_eq!(logger.count(), 2);
        assert_eq!(logger.last(), Some("msg2"));
    }
    #[test]
    fn test_appbuild_logger_capacity() {
        let mut logger = AppBuildLogger::new(2);
        logger.log("a");
        logger.log("b");
        logger.log("c");
        assert_eq!(logger.count(), 2);
    }
    #[test]
    fn test_appbuild_stats_success() {
        let mut stats = AppBuildStats::new();
        stats.record_success(100);
        stats.record_success(200);
        assert_eq!(stats.total_ops, 2);
        assert_eq!(stats.successful_ops, 2);
        assert!((stats.success_rate() - 1.0).abs() < 1e-9);
    }
    #[test]
    fn test_appbuild_stats_failure() {
        let mut stats = AppBuildStats::new();
        stats.record_success(100);
        stats.record_failure();
        assert!((stats.success_rate() - 0.5).abs() < 1e-9);
    }
    #[test]
    fn test_appbuild_stats_merge() {
        let mut a = AppBuildStats::new();
        let mut b = AppBuildStats::new();
        a.record_success(100);
        b.record_failure();
        a.merge(&b);
        assert_eq!(a.total_ops, 2);
    }
    #[test]
    fn test_appbuild_priority_queue() {
        let mut pq = AppBuildPriorityQueue::new();
        pq.push(AppBuildUtil0::new(1, "low", 1), 1);
        pq.push(AppBuildUtil0::new(2, "high", 2), 100);
        let (_, p) = pq.pop().expect("collection should not be empty");
        assert_eq!(p, 100);
    }
    #[test]
    fn test_appbuild_hash() {
        let h1 = appbuild_hash("foo");
        let h2 = appbuild_hash("foo");
        assert_eq!(h1, h2);
        let h3 = appbuild_hash("bar");
        assert_ne!(h1, h3);
    }
    #[test]
    fn test_appbuild_valid_name() {
        assert!(appbuild_is_valid_name("foo_bar"));
        assert!(!appbuild_is_valid_name("foo-bar"));
        assert!(!appbuild_is_valid_name(""));
    }
    #[test]
    fn test_appbuild_join() {
        let parts = ["a", "b", "c"];
        assert_eq!(appbuild_join(&parts, ", "), "a, b, c");
    }
}
#[cfg(test)]
mod appbuilder_analysis_tests {
    use super::*;
    use crate::app_builder::*;
    #[test]
    fn test_appbuilder_result_ok() {
        let r = AppBuilderResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_appbuilder_result_err() {
        let r = AppBuilderResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_appbuilder_result_partial() {
        let r = AppBuilderResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_appbuilder_result_skipped() {
        let r = AppBuilderResult::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_appbuilder_analysis_pass_run() {
        let mut p = AppBuilderAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_appbuilder_analysis_pass_empty_input() {
        let mut p = AppBuilderAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_appbuilder_analysis_pass_success_rate() {
        let mut p = AppBuilderAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_appbuilder_analysis_pass_disable() {
        let mut p = AppBuilderAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_appbuilder_pipeline_basic() {
        let mut pipeline = AppBuilderPipeline::new("main_pipeline");
        pipeline.add_pass(AppBuilderAnalysisPass::new("pass1"));
        pipeline.add_pass(AppBuilderAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_appbuilder_pipeline_disabled_pass() {
        let mut pipeline = AppBuilderPipeline::new("partial");
        let mut p = AppBuilderAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(AppBuilderAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_appbuilder_diff_basic() {
        let mut d = AppBuilderDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_appbuilder_diff_summary() {
        let mut d = AppBuilderDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }
    #[test]
    fn test_appbuilder_config_set_get() {
        let mut cfg = AppBuilderConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_appbuilder_config_read_only() {
        let mut cfg = AppBuilderConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_appbuilder_config_remove() {
        let mut cfg = AppBuilderConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_appbuilder_diagnostics_basic() {
        let mut diag = AppBuilderDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_appbuilder_diagnostics_max_errors() {
        let mut diag = AppBuilderDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_appbuilder_diagnostics_clear() {
        let mut diag = AppBuilderDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_appbuilder_config_value_types() {
        let b = AppBuilderConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = AppBuilderConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = AppBuilderConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = AppBuilderConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = AppBuilderConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
#[cfg(test)]
mod app_builder_ext_tests_3700 {
    use super::*;
    use crate::app_builder::*;
    #[test]
    fn test_app_builder_ext_result_ok_3700() {
        let r = AppBuilderExtResult3700::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_app_builder_ext_result_err_3700() {
        let r = AppBuilderExtResult3700::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }
    #[test]
    fn test_app_builder_ext_result_partial_3700() {
        let r = AppBuilderExtResult3700::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }
    #[test]
    fn test_app_builder_ext_result_skipped_3700() {
        let r = AppBuilderExtResult3700::Skipped;
        assert!(r.is_skipped());
    }
    #[test]
    fn test_app_builder_ext_pass_run_3700() {
        let mut p = AppBuilderExtPass3700::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }
    #[test]
    fn test_app_builder_ext_pass_empty_3700() {
        let mut p = AppBuilderExtPass3700::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }
    #[test]
    fn test_app_builder_ext_pass_rate_3700() {
        let mut p = AppBuilderExtPass3700::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }
    #[test]
    fn test_app_builder_ext_pass_disable_3700() {
        let mut p = AppBuilderExtPass3700::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }
    #[test]
    fn test_app_builder_ext_pipeline_basic_3700() {
        let mut pipeline = AppBuilderExtPipeline3700::new("main_pipeline");
        pipeline.add_pass(AppBuilderExtPass3700::new("pass1"));
        pipeline.add_pass(AppBuilderExtPass3700::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_app_builder_ext_pipeline_disabled_3700() {
        let mut pipeline = AppBuilderExtPipeline3700::new("partial");
        let mut p = AppBuilderExtPass3700::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(AppBuilderExtPass3700::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_app_builder_ext_diff_basic_3700() {
        let mut d = AppBuilderExtDiff3700::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }
    #[test]
    fn test_app_builder_ext_config_set_get_3700() {
        let mut cfg = AppBuilderExtConfig3700::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }
    #[test]
    fn test_app_builder_ext_config_read_only_3700() {
        let mut cfg = AppBuilderExtConfig3700::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }
    #[test]
    fn test_app_builder_ext_config_remove_3700() {
        let mut cfg = AppBuilderExtConfig3700::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }
    #[test]
    fn test_app_builder_ext_diagnostics_basic_3700() {
        let mut diag = AppBuilderExtDiag3700::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }
    #[test]
    fn test_app_builder_ext_diagnostics_max_errors_3700() {
        let mut diag = AppBuilderExtDiag3700::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }
    #[test]
    fn test_app_builder_ext_diagnostics_clear_3700() {
        let mut diag = AppBuilderExtDiag3700::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }
    #[test]
    fn test_app_builder_ext_config_value_types_3700() {
        let b = AppBuilderExtConfigVal3700::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);
        let i = AppBuilderExtConfigVal3700::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));
        let f = AppBuilderExtConfigVal3700::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);
        let s = AppBuilderExtConfigVal3700::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));
        let l = AppBuilderExtConfigVal3700::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
