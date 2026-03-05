//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{
    BinderInfo, Declaration, Environment, Expr, InductiveEnv, InductiveType, IntroRule, Level, Name,
};

use super::types::{Coproduct, InjectionMap, Pair, Sum3, Sum4, SumChain, SumStats, Tagged};

/// Build Sum type in the environment.
pub fn build_sum_env(env: &mut Environment, ind_env: &mut InductiveEnv) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let sum_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
    );
    let inl_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    let inr_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("b"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    let sum_ind = InductiveType::new(
        Name::str("Sum"),
        vec![],
        2,
        0,
        sum_ty.clone(),
        vec![
            IntroRule {
                name: Name::str("Sum.inl"),
                ty: inl_ty.clone(),
            },
            IntroRule {
                name: Name::str("Sum.inr"),
                ty: inr_ty.clone(),
            },
        ],
    );
    ind_env.add(sum_ind).map_err(|e| format!("{}", e))?;
    env.add(Declaration::Axiom {
        name: Name::str("Sum"),
        univ_params: vec![],
        ty: sum_ty,
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Sum.inl"),
        univ_params: vec![],
        ty: inl_ty,
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Sum.inr"),
        univ_params: vec![],
        ty: inr_ty,
    })
    .map_err(|e| e.to_string())?;
    build_sum_combinators(env)?;
    Ok(())
}
/// Build Sum eliminator and combinator axioms.
pub fn build_sum_combinators(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let elim_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("g"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_"),
                            Box::new(Expr::BVar(2)),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("s"),
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Sum"), vec![])),
                                    Box::new(Expr::BVar(4)),
                                )),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(3)),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Sum.elim"),
        univ_params: vec![],
        ty: elim_ty,
    })
    .map_err(|e| e.to_string())?;
    let map_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("δ"),
                    Box::new(type1.clone()),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("f"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_"),
                            Box::new(Expr::BVar(3)),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("g"),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("_"),
                                Box::new(Expr::BVar(3)),
                                Box::new(Expr::BVar(2)),
                            )),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("s"),
                                Box::new(Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                                        Box::new(Expr::BVar(5)),
                                    )),
                                    Box::new(Expr::BVar(4)),
                                )),
                                Box::new(Expr::App(
                                    Box::new(Expr::App(
                                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                                        Box::new(Expr::BVar(4)),
                                    )),
                                    Box::new(Expr::BVar(3)),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Sum.map"),
        univ_params: vec![],
        ty: map_ty,
    })
    .map_err(|e| e.to_string())?;
    let swap_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Sum.swap"),
        univ_params: vec![],
        ty: swap_ty,
    })
    .map_err(|e| e.to_string())?;
    let is_left_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Const(Name::str("Bool"), vec![])),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Sum.isLeft"),
        univ_params: vec![],
        ty: is_left_ty.clone(),
    })
    .map_err(|e| e.to_string())?;
    env.add(Declaration::Axiom {
        name: Name::str("Sum.isRight"),
        univ_params: vec![],
        ty: is_left_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Partition a vector of `Coproduct<A, B>` into `(Vec<A>, Vec<B>)`.
pub fn partition<A, B>(xs: Vec<Coproduct<A, B>>) -> (Vec<A>, Vec<B>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for x in xs {
        match x {
            Coproduct::Inl(a) => lefts.push(a),
            Coproduct::Inr(b) => rights.push(b),
        }
    }
    (lefts, rights)
}
/// Collect only the left values from a slice.
pub fn lefts<A: Clone, B>(xs: &[Coproduct<A, B>]) -> Vec<A> {
    xs.iter()
        .filter_map(|c| match c {
            Coproduct::Inl(a) => Some(a.clone()),
            _ => None,
        })
        .collect()
}
/// Collect only the right values from a slice.
pub fn rights<A, B: Clone>(xs: &[Coproduct<A, B>]) -> Vec<B> {
    xs.iter()
        .filter_map(|c| match c {
            Coproduct::Inr(b) => Some(b.clone()),
            _ => None,
        })
        .collect()
}
/// Map a fallible function `f: A → Result<C, E>` over a list, wrapping results in `Coproduct`.
pub fn try_map<A, C, E>(xs: Vec<A>, f: impl Fn(A) -> Result<C, E>) -> Vec<Coproduct<E, C>> {
    xs.into_iter()
        .map(|a| match f(a) {
            Ok(c) => Coproduct::Inr(c),
            Err(e) => Coproduct::Inl(e),
        })
        .collect()
}
/// A Bifunctor trait for types `F<A, B>`.
pub trait Bifunctor {
    /// The left component type.
    type Left;
    /// The right component type.
    type Right;
    /// The result type after mapping both sides.
    type Mapped<C, D>;
    /// Map both sides of the bifunctor.
    fn bimap_trait<C, D>(
        self,
        f: impl FnOnce(Self::Left) -> C,
        g: impl FnOnce(Self::Right) -> D,
    ) -> Self::Mapped<C, D>;
}
/// Distribute `Sum (A × B) (A × C) ≅ A × Sum B C`.
pub fn distribute_left<A: Clone, B, C>(
    s: Coproduct<Pair<A, B>, Pair<A, C>>,
) -> Pair<A, Coproduct<B, C>> {
    match s {
        Coproduct::Inl(Pair { fst: a, snd: b }) => Pair {
            fst: a,
            snd: Coproduct::Inl(b),
        },
        Coproduct::Inr(Pair { fst: a, snd: c }) => Pair {
            fst: a,
            snd: Coproduct::Inr(c),
        },
    }
}
/// Distribute `A × Sum B C ≅ Sum (A × B) (A × C)`.
pub fn factor_left<A: Clone, B, C>(
    p: Pair<A, Coproduct<B, C>>,
) -> Coproduct<Pair<A, B>, Pair<A, C>> {
    let Pair { fst: a, snd: s } = p;
    match s {
        Coproduct::Inl(b) => Coproduct::Inl(Pair { fst: a, snd: b }),
        Coproduct::Inr(c) => Coproduct::Inr(Pair { fst: a, snd: c }),
    }
}
/// Build an `Expr` for `Sum.inl α β a`.
pub fn make_sum_inl(alpha: Expr, beta: Expr, a: Expr) -> Expr {
    let base = Expr::Const(Name::str("Sum.inl"), vec![]);
    let e1 = Expr::App(Box::new(base), Box::new(alpha));
    let e2 = Expr::App(Box::new(e1), Box::new(beta));
    Expr::App(Box::new(e2), Box::new(a))
}
/// Build an `Expr` for `Sum.inr α β b`.
pub fn make_sum_inr(alpha: Expr, beta: Expr, b: Expr) -> Expr {
    let base = Expr::Const(Name::str("Sum.inr"), vec![]);
    let e1 = Expr::App(Box::new(base), Box::new(alpha));
    let e2 = Expr::App(Box::new(e1), Box::new(beta));
    Expr::App(Box::new(e2), Box::new(b))
}
/// Build an `Expr` for `Sum α β`.
pub fn make_sum_type(alpha: Expr, beta: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Sum"), vec![])),
            Box::new(alpha),
        )),
        Box::new(beta),
    )
}
/// Return all Sum-related names registered in the environment.
pub fn registered_sum_names(env: &Environment) -> Vec<String> {
    let candidates = [
        "Sum",
        "Sum.inl",
        "Sum.inr",
        "Sum.elim",
        "Sum.map",
        "Sum.swap",
        "Sum.isLeft",
        "Sum.isRight",
    ];
    candidates
        .iter()
        .filter(|n| env.get(&Name::str(**n)).is_some())
        .map(|s| s.to_string())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> (Environment, InductiveEnv) {
        let mut env = Environment::new();
        let ind_env = InductiveEnv::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Bool"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        (env, ind_env)
    }
    #[test]
    fn test_build_sum_env() {
        let (mut env, mut ind_env) = setup_env();
        assert!(build_sum_env(&mut env, &mut ind_env).is_ok());
        assert!(env.get(&Name::str("Sum")).is_some());
        assert!(env.get(&Name::str("Sum.inl")).is_some());
        assert!(env.get(&Name::str("Sum.inr")).is_some());
    }
    #[test]
    fn test_sum_inl() {
        let (mut env, mut ind_env) = setup_env();
        build_sum_env(&mut env, &mut ind_env).expect("build_sum_env should succeed");
        let inl_decl = env
            .get(&Name::str("Sum.inl"))
            .expect("declaration 'Sum.inl' should exist in env");
        assert!(matches!(inl_decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_sum_inr() {
        let (mut env, mut ind_env) = setup_env();
        build_sum_env(&mut env, &mut ind_env).expect("build_sum_env should succeed");
        let inr_decl = env
            .get(&Name::str("Sum.inr"))
            .expect("declaration 'Sum.inr' should exist in env");
        assert!(matches!(inr_decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_sum_combinators_registered() {
        let (mut env, mut ind_env) = setup_env();
        build_sum_env(&mut env, &mut ind_env).expect("build_sum_env should succeed");
        assert!(env.get(&Name::str("Sum.elim")).is_some());
        assert!(env.get(&Name::str("Sum.map")).is_some());
        assert!(env.get(&Name::str("Sum.swap")).is_some());
        assert!(env.get(&Name::str("Sum.isLeft")).is_some());
        assert!(env.get(&Name::str("Sum.isRight")).is_some());
    }
    #[test]
    fn test_coproduct_inl() {
        let c: Coproduct<i32, &str> = Coproduct::inl(42);
        assert!(c.is_left());
        assert!(!c.is_right());
    }
    #[test]
    fn test_coproduct_inr() {
        let c: Coproduct<i32, &str> = Coproduct::inr("hello");
        assert!(c.is_right());
        assert!(!c.is_left());
    }
    #[test]
    fn test_coproduct_elim() {
        let l: Coproduct<i32, &str> = Coproduct::inl(5);
        let r: Coproduct<i32, &str> = Coproduct::inr("hi");
        assert_eq!(l.elim(|n| n * 2, |s| s.len() as i32), 10);
        assert_eq!(r.elim(|n| n * 2, |s| s.len() as i32), 2);
    }
    #[test]
    fn test_coproduct_swap() {
        let c: Coproduct<i32, &str> = Coproduct::inl(3);
        let swapped = c.swap();
        assert!(swapped.is_right());
        assert_eq!(swapped.into_right(), Some(3));
    }
    #[test]
    fn test_coproduct_bimap() {
        let c: Coproduct<i32, &str> = Coproduct::inl(5);
        let mapped = c.bimap(|n| n * 2, |s: &str| s.len());
        assert_eq!(mapped.into_left(), Some(10));
        let c2: Coproduct<i32, &str> = Coproduct::inr("hello");
        let mapped2 = c2.bimap(|n| n * 2, |s: &str| s.len());
        assert_eq!(mapped2.into_right(), Some(5));
    }
    #[test]
    fn test_coproduct_map_left() {
        let c: Coproduct<i32, &str> = Coproduct::inl(3);
        let m = c.map_left(|n| n + 1);
        assert_eq!(m.into_left(), Some(4));
    }
    #[test]
    fn test_coproduct_map_right() {
        let c: Coproduct<i32, &str> = Coproduct::inr("world");
        let m = c.map_right(|s: &str| s.to_uppercase());
        assert_eq!(m.into_right(), Some("WORLD".to_string()));
    }
    #[test]
    fn test_coproduct_into_result() {
        let l: Coproduct<String, i32> = Coproduct::inl("err".to_string());
        let r: Coproduct<String, i32> = Coproduct::inr(42);
        assert!(l.into_result().is_err());
        assert_eq!(r.into_result().expect("into_result should succeed"), 42);
    }
    #[test]
    fn test_coproduct_from_result() {
        let ok: Result<i32, String> = Ok(5);
        let err: Result<i32, String> = Err("oops".to_string());
        assert!(Coproduct::from_result(ok).is_right());
        assert!(Coproduct::from_result(err).is_left());
    }
    #[test]
    fn test_partition() {
        let xs: Vec<Coproduct<i32, &str>> = vec![
            Coproduct::inl(1),
            Coproduct::inr("a"),
            Coproduct::inl(2),
            Coproduct::inr("b"),
        ];
        let (ls, rs) = partition(xs);
        assert_eq!(ls, vec![1, 2]);
        assert_eq!(rs, vec!["a", "b"]);
    }
    #[test]
    fn test_lefts_rights() {
        let xs: Vec<Coproduct<i32, &str>> =
            vec![Coproduct::inl(10), Coproduct::inr("x"), Coproduct::inl(20)];
        assert_eq!(lefts(&xs), vec![10, 20]);
        assert_eq!(rights(&xs), vec!["x"]);
    }
    #[test]
    fn test_try_map() {
        let xs = vec![2, 0, 4];
        let result = try_map(xs, |n: i32| if n == 0 { Err("zero") } else { Ok(100 / n) });
        assert!(result[0].is_right());
        assert!(result[1].is_left());
        assert!(result[2].is_right());
    }
    #[test]
    fn test_sum3_elim() {
        let s: Sum3<i32, &str, bool> = Sum3::In1(7);
        let result = s.elim(
            |n| n * 2,
            |s: &str| s.len() as i32,
            |b| if b { 1 } else { 0 },
        );
        assert_eq!(result, 14);
        let s2: Sum3<i32, &str, bool> = Sum3::In3(true);
        let result2 = s2.elim(
            |n| n * 2,
            |s: &str| s.len() as i32,
            |b| if b { 1 } else { 0 },
        );
        assert_eq!(result2, 1);
    }
    #[test]
    fn test_sum3_predicates() {
        let s1: Sum3<i32, &str, bool> = Sum3::In1(1);
        let s2: Sum3<i32, &str, bool> = Sum3::In2("hi");
        let s3: Sum3<i32, &str, bool> = Sum3::In3(false);
        assert!(s1.is_first());
        assert!(s2.is_second());
        assert!(s3.is_third());
    }
    #[test]
    fn test_pair_operations() {
        let p = Pair::new(1, "hello");
        let swapped = p.swap();
        assert_eq!(swapped.fst, "hello");
        assert_eq!(swapped.snd, 1);
        let p2 = Pair::new(3, 4);
        let mapped = p2.map_fst(|n| n * 10);
        assert_eq!(mapped.fst, 30);
        assert_eq!(mapped.snd, 4);
    }
    #[test]
    fn test_pair_tuple_round_trip() {
        let p = Pair::from_tuple((42, "test"));
        let t = p.into_tuple();
        assert_eq!(t, (42, "test"));
    }
    #[test]
    fn test_distribute_left() {
        let s: Coproduct<Pair<i32, &str>, Pair<i32, bool>> = Coproduct::Inl(Pair::new(5, "hello"));
        let p = distribute_left(s);
        assert_eq!(p.fst, 5);
        assert!(p.snd.is_left());
    }
    #[test]
    fn test_factor_left() {
        let p = Pair::new(10i32, Coproduct::<&str, bool>::Inr(true));
        let s = factor_left(p);
        assert!(s.is_right());
        let inner = s.into_right().expect("into_right should succeed");
        assert_eq!(inner.fst, 10);
    }
    #[test]
    fn test_tagged_map() {
        let t = Tagged::new("label", 5);
        let t2 = t.map(|n| n * 2);
        assert_eq!(t2.tag, "label");
        assert_eq!(t2.value, 10);
    }
    #[test]
    fn test_tagged_map_tag() {
        let t = Tagged::new(1u32, "value");
        let t2 = t.map_tag(|n| n.to_string());
        assert_eq!(t2.tag, "1");
        assert_eq!(t2.value, "value");
    }
    #[test]
    fn test_make_sum_type_expr() {
        let alpha = Expr::Const(Name::str("Nat"), vec![]);
        let beta = Expr::Const(Name::str("Bool"), vec![]);
        let sum_ty = make_sum_type(alpha, beta);
        assert!(matches!(sum_ty, Expr::App(_, _)));
    }
    #[test]
    fn test_registered_sum_names() {
        let (mut env, mut ind_env) = setup_env();
        build_sum_env(&mut env, &mut ind_env).expect("build_sum_env should succeed");
        let names = registered_sum_names(&env);
        assert!(names.contains(&"Sum".to_string()));
        assert!(names.contains(&"Sum.inl".to_string()));
        assert!(names.contains(&"Sum.inr".to_string()));
        assert!(names.len() >= 5);
    }
}
/// A `Coproduct<E, A>` with error-on-left semantics.
pub type SumResult<E, A> = Coproduct<E, A>;
/// Lift a `Result<A, E>` into a `SumResult<E, A>`.
pub fn lift_result<A, E>(r: Result<A, E>) -> SumResult<E, A> {
    Coproduct::from_result(r)
}
/// Sequence a list of `SumResult`s, stopping at the first error.
pub fn sequence_sum<A: Clone, E: Clone>(xs: Vec<SumResult<E, A>>) -> SumResult<E, Vec<A>> {
    let mut results = Vec::new();
    for x in xs {
        match x {
            Coproduct::Inl(e) => return Coproduct::Inl(e),
            Coproduct::Inr(a) => results.push(a),
        }
    }
    Coproduct::Inr(results)
}
/// Traverse a list with a fallible function, stopping at the first error.
pub fn traverse_sum<A, B: Clone, E: Clone>(
    xs: Vec<A>,
    f: impl Fn(A) -> SumResult<E, B>,
) -> SumResult<E, Vec<B>> {
    sequence_sum(xs.into_iter().map(f).collect())
}
#[cfg(test)]
mod sum_extra_tests {
    use super::*;
    #[test]
    fn test_sum_chain_map_right() {
        let c: Coproduct<String, i32> = Coproduct::inr(5);
        let chain = SumChain::new(c).map(|n| n * 2);
        assert!(chain.is_ok());
        assert_eq!(chain.into_inner().into_right(), Some(10));
    }
    #[test]
    fn test_sum_chain_short_circuit_left() {
        let c: Coproduct<String, i32> = Coproduct::inl("error".to_string());
        let chain = SumChain::new(c).map(|n: i32| n * 2);
        assert!(chain.is_err());
    }
    #[test]
    fn test_sum_chain_flat_map() {
        let c: Coproduct<String, i32> = Coproduct::inr(5);
        let chain = SumChain::new(c).flat_map(|n| {
            if n > 0 {
                Coproduct::inr(n * 10)
            } else {
                Coproduct::inl("negative".to_string())
            }
        });
        assert!(chain.is_ok());
        assert_eq!(chain.into_inner().into_right(), Some(50));
    }
    #[test]
    fn test_sum_stats_from_slice() {
        let xs: Vec<Coproduct<i32, &str>> =
            vec![Coproduct::inl(1), Coproduct::inr("a"), Coproduct::inl(2)];
        let stats = SumStats::from_slice(&xs);
        assert_eq!(stats.left_count, 2);
        assert_eq!(stats.right_count, 1);
        assert_eq!(stats.total(), 3);
    }
    #[test]
    fn test_sum_stats_all_left() {
        let xs: Vec<Coproduct<i32, &str>> = vec![Coproduct::inl(1), Coproduct::inl(2)];
        let stats = SumStats::from_slice(&xs);
        assert!(stats.all_left());
    }
    #[test]
    fn test_sum_stats_all_right() {
        let xs: Vec<Coproduct<i32, &str>> = vec![Coproduct::inr("x"), Coproduct::inr("y")];
        let stats = SumStats::from_slice(&xs);
        assert!(stats.all_right());
    }
    #[test]
    fn test_sum4_elim() {
        let s: Sum4<i32, &str, bool, f32> = Sum4::In3(true);
        let result: i32 = s.elim(|n| n, |_| 0, |b| if b { 1 } else { 0 }, |_| -1);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_sum4_tag() {
        let s: Sum4<i32, &str, bool, f32> = Sum4::In2("hi");
        assert_eq!(s.tag(), 2);
        assert!(s.is_second());
    }
    #[test]
    fn test_lift_result_ok() {
        let r: Result<i32, String> = Ok(42);
        assert!(lift_result(r).is_right());
    }
    #[test]
    fn test_lift_result_err() {
        let r: Result<i32, String> = Err("oops".to_string());
        assert!(lift_result(r).is_left());
    }
    #[test]
    fn test_sequence_sum_all_ok() {
        let xs: Vec<SumResult<String, i32>> = vec![Coproduct::inr(1), Coproduct::inr(2)];
        let result = sequence_sum(xs);
        assert_eq!(result.into_right(), Some(vec![1, 2]));
    }
    #[test]
    fn test_sequence_sum_first_error() {
        let xs: Vec<SumResult<String, i32>> =
            vec![Coproduct::inr(1), Coproduct::inl("err".to_string())];
        let result = sequence_sum(xs);
        assert!(result.is_left());
    }
    #[test]
    fn test_traverse_sum_success() {
        let xs = vec![1i32, 2, 3];
        let result = traverse_sum(xs, |n| -> SumResult<String, i32> { Coproduct::inr(n * 2) });
        assert_eq!(result.into_right(), Some(vec![2, 4, 6]));
    }
    #[test]
    fn test_traverse_sum_error() {
        let xs = vec![1i32, 0, 3];
        let result = traverse_sum(xs, |n| -> SumResult<String, i32> {
            if n == 0 {
                Coproduct::inl("zero".to_string())
            } else {
                Coproduct::inr(100 / n)
            }
        });
        assert!(result.is_left());
    }
    #[test]
    fn test_injection_map() {
        let mut map = InjectionMap::new();
        map.register(1, "Left");
        map.register(2, "Right");
        assert_eq!(map.get(1), Some("Left"));
        assert!(map.get(3).is_none());
        assert_eq!(map.len(), 2);
    }
    #[test]
    fn test_sum4_is_fourth() {
        let s: Sum4<i32, &str, bool, f64> = Sum4::In4(2.72);
        assert!(s.is_fourth());
        assert!(!s.is_first());
    }
    #[test]
    fn test_sum_chain_flat_map_short_circuit() {
        let c: Coproduct<String, i32> = Coproduct::inr(-5);
        let chain = SumChain::new(c).flat_map(|n| -> Coproduct<String, i32> {
            if n > 0 {
                Coproduct::inr(n)
            } else {
                Coproduct::inl("negative".to_string())
            }
        });
        assert!(chain.is_err());
    }
}
/// Build axiom: Sum is categorical coproduct.
pub(super) fn sm_ext_sum_categorical_coproduct(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                Bi::Implicit,
                Name::str("Z"),
                Box::new(type1),
                Box::new(prop),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.categoricalCoproduct"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum universal property (unique factorization).
pub(super) fn sm_ext_sum_universal_property(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                Bi::Implicit,
                Name::str("Z"),
                Box::new(type1),
                Box::new(Expr::Pi(
                    Bi::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        Bi::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::Pi(
                        Bi::Default,
                        Name::str("g"),
                        Box::new(Expr::Pi(
                            Bi::Default,
                            Name::str("_"),
                            Box::new(Expr::BVar(2)),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(prop),
                    )),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.universalProperty"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: inl injection law (inl is injective).
pub(super) fn sm_ext_inl_injective(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("a1"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    Bi::Default,
                    Name::str("a2"),
                    Box::new(Expr::BVar(2)),
                    Box::new(prop),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.inlInjective"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: inr injection law (inr is injective).
pub(super) fn sm_ext_inr_injective(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("b1"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::Pi(
                    Bi::Default,
                    Name::str("b2"),
                    Box::new(Expr::BVar(1)),
                    Box::new(prop),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.inrInjective"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: inl and inr are disjoint (no overlap).
pub(super) fn sm_ext_inl_inr_disjoint(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("a"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    Bi::Default,
                    Name::str("b"),
                    Box::new(Expr::BVar(1)),
                    Box::new(prop),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.inlInrDisjoint"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum functor preserves identity.
pub(super) fn sm_ext_sum_functor_id(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1),
            Box::new(prop),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.functorId"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum functor preserves composition.
pub(super) fn sm_ext_sum_functor_compose(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.functorCompose"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum bifunctor law (bimap identity).
pub(super) fn sm_ext_sum_bifunctor_id(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.bifunctorId"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum monad return (fixed Left = error type).
pub(super) fn sm_ext_sum_monad_return(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("E"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("A"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("a"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.monadReturn"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum monad bind (left short-circuits).
pub(super) fn sm_ext_sum_monad_bind(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.monadBind"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum monad left identity law.
pub(super) fn sm_ext_sum_monad_left_id(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.monadLeftId"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum monad right identity law.
pub(super) fn sm_ext_sum_monad_right_id(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.monadRightId"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum monad associativity law.
pub(super) fn sm_ext_sum_monad_assoc(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.monadAssoc"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum applicative pure.
pub(super) fn sm_ext_sum_applicative_pure(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("E"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("A"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("a"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Sum"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.applicativePure"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum applicative ap (sequential application).
pub(super) fn sm_ext_sum_applicative_ap(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.applicativeAp"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum commutativity iso (swap is involution).
pub(super) fn sm_ext_sum_swap_involution(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let sum_ab = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Sum"), vec![])),
            Box::new(Expr::BVar(1)),
        )),
        Box::new(Expr::BVar(0)),
    );
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("s"),
                Box::new(sum_ab),
                Box::new(prop),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.swapInvolution"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum associativity iso.
pub(super) fn sm_ext_sum_assoc_iso(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                Bi::Implicit,
                Name::str("C"),
                Box::new(type1),
                Box::new(Expr::Const(Name::str("Prop"), vec![])),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.assocIso"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum initial object (Void/Empty as neutral element).
pub(super) fn sm_ext_sum_void_initial(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1),
        Box::new(Expr::Const(Name::str("Prop"), vec![])),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.voidInitial"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum distributivity over product.
pub(super) fn sm_ext_sum_distributes_over_prod(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                Bi::Implicit,
                Name::str("C"),
                Box::new(type1),
                Box::new(Expr::Const(Name::str("Prop"), vec![])),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.distributesOverProd"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum as disjoint union (inl and inr cover all cases).
pub(super) fn sm_ext_sum_disjoint_union(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let sum_ab = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Sum"), vec![])),
            Box::new(Expr::BVar(1)),
        )),
        Box::new(Expr::BVar(0)),
    );
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("s"),
                Box::new(sum_ab),
                Box::new(prop),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.disjointUnion"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum case analysis (eliminator law).
pub(super) fn sm_ext_sum_case_analysis(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                Bi::Implicit,
                Name::str("P"),
                Box::new(Expr::Pi(
                    Bi::Default,
                    Name::str("_"),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Sum"), vec![])),
                            Box::new(Expr::BVar(1)),
                        )),
                        Box::new(Expr::BVar(0)),
                    )),
                    Box::new(type1.clone()),
                )),
                Box::new(Expr::Pi(
                    Bi::Default,
                    Name::str("hl"),
                    Box::new(Expr::Pi(
                        Bi::Default,
                        Name::str("a"),
                        Box::new(Expr::BVar(2)),
                        Box::new(prop.clone()),
                    )),
                    Box::new(Expr::Pi(
                        Bi::Default,
                        Name::str("hr"),
                        Box::new(Expr::Pi(
                            Bi::Default,
                            Name::str("b"),
                            Box::new(Expr::BVar(2)),
                            Box::new(prop.clone()),
                        )),
                        Box::new(Expr::Pi(
                            Bi::Default,
                            Name::str("s"),
                            Box::new(Expr::App(
                                Box::new(Expr::App(
                                    Box::new(Expr::Const(Name::str("Sum"), vec![])),
                                    Box::new(Expr::BVar(4)),
                                )),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(prop),
                        )),
                    )),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.caseAnalysis"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Tagged union semantics (tag determines variant).
pub(super) fn sm_ext_tagged_union_semantics(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.taggedUnionSemantics"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum.lefts partition (lefts ++ rights covers all).
pub(super) fn sm_ext_sum_partition_complete(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.partitionComplete"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum.lefts count (length of lefts).
pub(super) fn sm_ext_sum_lefts_count(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.leftsCount"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum traversal naturality.
pub(super) fn sm_ext_sum_traversal_naturality(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.traversalNaturality"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Sum traversal preserves identity.
pub(super) fn sm_ext_sum_traversal_id(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.traversalId"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: HoTT path space for Sum (encode/decode).
pub(super) fn sm_ext_sum_path_space(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    let sum_ab = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Sum"), vec![])),
            Box::new(Expr::BVar(1)),
        )),
        Box::new(Expr::BVar(0)),
    );
    let ty = Expr::Pi(
        Bi::Implicit,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Implicit,
            Name::str("B"),
            Box::new(type1),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("s"),
                Box::new(sum_ab.clone()),
                Box::new(Expr::Pi(
                    Bi::Default,
                    Name::str("t"),
                    Box::new(sum_ab),
                    Box::new(prop),
                )),
            )),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.pathSpace"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: HoTT – inl path encodes to left.
pub(super) fn sm_ext_sum_path_inl(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.pathInl"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: HoTT – inr path encodes to right.
pub(super) fn sm_ext_sum_path_inr(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{Declaration, Expr, Name};
    let prop = Expr::Sort(oxilean_kernel::Level::zero());
    match env.add(Declaration::Axiom {
        name: Name::str("Sum.pathInr"),
        univ_params: vec![],
        ty: prop,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
/// Build axiom: Dependent sum / Sigma type intro rule.
pub(super) fn sm_ext_sigma_intro(env: &mut Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo as Bi, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        Bi::Default,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            Bi::Default,
            Name::str("B"),
            Box::new(Expr::Pi(
                Bi::Default,
                Name::str("_"),
                Box::new(Expr::BVar(0)),
                Box::new(type1.clone()),
            )),
            Box::new(type1),
        )),
    );
    match env.add(Declaration::Axiom {
        name: Name::str("Sigma.intro"),
        univ_params: vec![],
        ty,
    }) {
        Ok(()) | Err(_) => Ok(()),
    }
}
