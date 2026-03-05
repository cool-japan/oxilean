//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
#![allow(clippy::items_after_test_module)]

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{ContM, Either, Identity, Maybe, Reader, State, Writer};

/// Build Monad type class in the environment.
pub fn build_monad_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let monad_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(type2.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Monad"),
        univ_params: vec![],
        ty: monad_ty,
    })
    .map_err(|e| e.to_string())?;
    let pure_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Monad"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("a"),
                    Box::new(Expr::BVar(0)),
                    Box::new(Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(1)))),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Monad.pure"),
        univ_params: vec![],
        ty: pure_ty,
    })
    .map_err(|e| e.to_string())?;
    let bind_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Monad"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("β"),
                    Box::new(type1.clone()),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("ma"),
                        Box::new(Expr::App(Box::new(Expr::BVar(4)), Box::new(Expr::BVar(1)))),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("f"),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("_"),
                                Box::new(Expr::BVar(2)),
                                Box::new(Expr::App(
                                    Box::new(Expr::BVar(6)),
                                    Box::new(Expr::BVar(2)),
                                )),
                            )),
                            Box::new(Expr::App(Box::new(Expr::BVar(6)), Box::new(Expr::BVar(3)))),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Monad.bind"),
        univ_params: vec![],
        ty: bind_ty,
    })
    .map_err(|e| e.to_string())?;
    build_functor_env(env)?;
    build_applicative_env(env)?;
    build_monad_extras(env)?;
    Ok(())
}
/// Build Functor type class in the environment.
pub fn build_functor_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let functor_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(type2.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Functor"),
        univ_params: vec![],
        ty: functor_ty,
    })
    .map_err(|e| e.to_string())?;
    let map_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_inst"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Functor"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("β"),
                    Box::new(type1.clone()),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("g"),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("_"),
                            Box::new(Expr::BVar(1)),
                            Box::new(Expr::BVar(1)),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("fa"),
                            Box::new(Expr::App(Box::new(Expr::BVar(5)), Box::new(Expr::BVar(2)))),
                            Box::new(Expr::App(Box::new(Expr::BVar(6)), Box::new(Expr::BVar(2)))),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Functor.map"),
        univ_params: vec![],
        ty: map_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Build Applicative type class in the environment.
pub fn build_applicative_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let app_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(type2.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Applicative"),
        univ_params: vec![],
        ty: app_ty,
    })
    .map_err(|e| e.to_string())?;
    let seq_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_inst"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Applicative"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Implicit,
                    Name::str("β"),
                    Box::new(type1.clone()),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("fg"),
                        Box::new(Expr::App(
                            Box::new(Expr::BVar(4)),
                            Box::new(Expr::Pi(
                                BinderInfo::Default,
                                Name::str("_"),
                                Box::new(Expr::BVar(1)),
                                Box::new(Expr::BVar(1)),
                            )),
                        )),
                        Box::new(Expr::Pi(
                            BinderInfo::Default,
                            Name::str("fa"),
                            Box::new(Expr::App(Box::new(Expr::BVar(5)), Box::new(Expr::BVar(2)))),
                            Box::new(Expr::App(Box::new(Expr::BVar(6)), Box::new(Expr::BVar(2)))),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Applicative.seq"),
        univ_params: vec![],
        ty: seq_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Build extra monad combinators: join, mapM, sequence, guard, etc.
pub fn build_monad_extras(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let join_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_inst"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Monad"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("α"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("mma"),
                    Box::new(Expr::App(
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::App(Box::new(Expr::BVar(2)), Box::new(Expr::BVar(0)))),
                    )),
                    Box::new(Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(1)))),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Monad.join"),
        univ_params: vec![],
        ty: join_ty,
    })
    .map_err(|e| e.to_string())?;
    let alt_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Alternative"),
        univ_params: vec![],
        ty: alt_ty,
    })
    .map_err(|e| e.to_string())?;
    let state_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("σ"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
            Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MonadState"),
        univ_params: vec![],
        ty: state_ty,
    })
    .map_err(|e| e.to_string())?;
    let reader_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("ρ"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
            Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MonadReader"),
        univ_params: vec![],
        ty: reader_ty,
    })
    .map_err(|e| e.to_string())?;
    let writer_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("ω"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
            Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MonadWriter"),
        univ_params: vec![],
        ty: writer_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Kleisli composition: `f >=> g = fun a -> f(a) >>= g`
pub fn kleisli_compose<A, B, C>(
    f: impl FnOnce(A) -> Maybe<B>,
    g: impl FnOnce(B) -> Maybe<C>,
) -> impl FnOnce(A) -> Maybe<C> {
    move |a| f(a).bind(g)
}
/// Sequence a list of `Maybe` values. Returns `Maybe(Some(vec))` only if all are `Just`.
pub fn sequence_maybe<A>(maybes: Vec<Maybe<A>>) -> Maybe<Vec<A>> {
    let mut results = Vec::with_capacity(maybes.len());
    for m in maybes {
        match m.into_option() {
            Some(a) => results.push(a),
            None => return Maybe::nothing(),
        }
    }
    Maybe::just(results)
}
/// `mapM`: apply `f` to each element and collect results.
pub fn map_maybe<A, B>(xs: Vec<A>, f: impl Fn(A) -> Maybe<B>) -> Maybe<Vec<B>> {
    let maybes: Vec<Maybe<B>> = xs.into_iter().map(f).collect();
    sequence_maybe(maybes)
}
/// `filterM` for `Maybe`: keep elements where `f(x)` is `Just true`.
pub fn filter_maybe<A: Clone>(xs: Vec<A>, f: impl Fn(&A) -> Maybe<bool>) -> Maybe<Vec<A>> {
    let mut result = Vec::new();
    for x in &xs {
        match f(x).into_option() {
            Some(true) => result.push(x.clone()),
            Some(false) => {}
            None => return Maybe::nothing(),
        }
    }
    Maybe::just(result)
}
/// Fold a list monadically using `Maybe`.
pub fn fold_maybe<A, B: Clone>(xs: Vec<A>, init: B, f: impl Fn(B, A) -> Maybe<B>) -> Maybe<B> {
    let mut acc = init;
    for x in xs {
        match f(acc, x).into_option() {
            Some(b) => acc = b,
            None => return Maybe::nothing(),
        }
    }
    Maybe::just(acc)
}
/// Build an `Expr` for `Monad.pure m inst α a`.
pub fn make_monad_pure(m: Expr, inst: Expr, alpha: Expr, a: Expr) -> Expr {
    let base = Expr::Const(Name::str("Monad.pure"), vec![]);
    let e1 = Expr::App(Box::new(base), Box::new(m));
    let e2 = Expr::App(Box::new(e1), Box::new(inst));
    let e3 = Expr::App(Box::new(e2), Box::new(alpha));
    Expr::App(Box::new(e3), Box::new(a))
}
/// Build an `Expr` for `Monad.bind m inst α β ma f`.
#[allow(clippy::too_many_arguments)]
pub fn make_monad_bind(m: Expr, inst: Expr, alpha: Expr, beta: Expr, ma: Expr, f: Expr) -> Expr {
    let base = Expr::Const(Name::str("Monad.bind"), vec![]);
    let e1 = Expr::App(Box::new(base), Box::new(m));
    let e2 = Expr::App(Box::new(e1), Box::new(inst));
    let e3 = Expr::App(Box::new(e2), Box::new(alpha));
    let e4 = Expr::App(Box::new(e3), Box::new(beta));
    let e5 = Expr::App(Box::new(e4), Box::new(ma));
    Expr::App(Box::new(e5), Box::new(f))
}
/// Return all monad-related names registered in the environment.
pub fn registered_monad_names(env: &Environment) -> Vec<String> {
    let candidates = [
        "Monad",
        "Monad.pure",
        "Monad.bind",
        "Monad.join",
        "Functor",
        "Functor.map",
        "Applicative",
        "Applicative.seq",
        "Alternative",
        "MonadState",
        "MonadReader",
        "MonadWriter",
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
    #[test]
    fn test_build_monad_env() {
        let mut env = Environment::new();
        assert!(build_monad_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Monad")).is_some());
        assert!(env.get(&Name::str("Monad.pure")).is_some());
        assert!(env.get(&Name::str("Monad.bind")).is_some());
    }
    #[test]
    fn test_functor_registered() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        assert!(env.get(&Name::str("Functor")).is_some());
        assert!(env.get(&Name::str("Functor.map")).is_some());
    }
    #[test]
    fn test_applicative_registered() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        assert!(env.get(&Name::str("Applicative")).is_some());
        assert!(env.get(&Name::str("Applicative.seq")).is_some());
    }
    #[test]
    fn test_monad_extras_registered() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        assert!(env.get(&Name::str("Monad.join")).is_some());
        assert!(env.get(&Name::str("Alternative")).is_some());
        assert!(env.get(&Name::str("MonadState")).is_some());
        assert!(env.get(&Name::str("MonadReader")).is_some());
        assert!(env.get(&Name::str("MonadWriter")).is_some());
    }
    #[test]
    fn test_identity_pure_bind() {
        let x = Identity::pure(5).bind(|n| Identity::pure(n * 2));
        assert_eq!(x.run(), 10);
    }
    #[test]
    fn test_identity_map() {
        let x = Identity::pure(3).map(|n| n + 1);
        assert_eq!(x.run(), 4);
    }
    #[test]
    fn test_maybe_just_bind() {
        let m = Maybe::just(10).bind(|n| Maybe::just(n + 5));
        assert_eq!(m.into_option(), Some(15));
    }
    #[test]
    fn test_maybe_nothing_bind() {
        let m: Maybe<i32> = Maybe::nothing().bind(|n: i32| Maybe::just(n + 5));
        assert!(m.is_nothing());
    }
    #[test]
    fn test_maybe_fmap() {
        let m = Maybe::just(7).fmap(|n| n * 3);
        assert_eq!(m.into_option(), Some(21));
    }
    #[test]
    fn test_maybe_or_else() {
        let a: Maybe<i32> = Maybe::nothing();
        let b = Maybe::just(42);
        assert_eq!(a.or_else(b).into_option(), Some(42));
        let c = Maybe::just(1);
        let d = Maybe::just(2);
        assert_eq!(c.or_else(d).into_option(), Some(1));
    }
    #[test]
    fn test_either_right_bind() {
        let e: Either<String, i32> = Either::right(10);
        let result = e.bind(|n| Either::right(n * 2));
        assert!(result.is_right());
        assert_eq!(
            result.into_result().expect("into_result should succeed"),
            20
        );
    }
    #[test]
    fn test_either_left_propagates() {
        let e: Either<String, i32> = Either::left("error".to_string());
        let result = e.bind(|n| Either::right(n * 2));
        assert!(result.is_left());
    }
    #[test]
    fn test_either_fmap() {
        let e: Either<String, i32> = Either::right(5);
        let r = e.fmap(|n| n.to_string());
        assert_eq!(r.unwrap_right(), "5");
    }
    #[test]
    fn test_state_get_put() {
        let computation = State::<i32, i32>::get()
            .bind(|s| State::<i32, ()>::put(s + 1).bind(|_| State::<i32, i32>::get()));
        let (val, state) = computation.run(10);
        assert_eq!(val, 11);
        assert_eq!(state, 11);
    }
    #[test]
    fn test_state_modify() {
        let computation = State::<i32, ()>::modify(|s| s * 2);
        let final_state = computation.exec(5);
        assert_eq!(final_state, 10);
    }
    #[test]
    fn test_state_eval_exec() {
        let comp = State::<i32, i32>::new(|s| (s + 100, s * 2));
        assert_eq!(comp.eval(3), 103);
        let comp2 = State::<i32, i32>::new(|s| (s + 100, s * 2));
        assert_eq!(comp2.exec(3), 6);
    }
    #[test]
    fn test_writer_new_bind() {
        let w = Writer::new(5, vec!["start".to_string()]);
        let w2 = w.bind(|n| Writer::new(n * 2, vec!["doubled".to_string()]));
        assert_eq!(w2.value, 10);
        assert_eq!(w2.log, vec!["start".to_string(), "doubled".to_string()]);
    }
    #[test]
    fn test_writer_tell() {
        let w = Writer::<Vec<String>, ()>::tell("hello".to_string())
            .bind(|_| Writer::<Vec<String>, ()>::tell("world".to_string()));
        assert_eq!(w.log, vec!["hello".to_string(), "world".to_string()]);
    }
    #[test]
    fn test_writer_fmap() {
        let w = Writer::new(3, vec!["log".to_string()]);
        let w2 = w.fmap(|n| n * n);
        assert_eq!(w2.value, 9);
        assert_eq!(w2.log, vec!["log".to_string()]);
    }
    #[test]
    fn test_reader_pure_run() {
        let r = Reader::<String, i32>::pure(42);
        assert_eq!(r.run(&"env".to_string()), 42);
    }
    #[test]
    fn test_reader_ask() {
        let r = Reader::<i32, i32>::ask();
        assert_eq!(r.run(&99), 99);
    }
    #[test]
    fn test_reader_asks() {
        let r = Reader::<Vec<i32>, usize>::asks(|v: &Vec<i32>| v.len());
        assert_eq!(r.run(&vec![1, 2, 3]), 3);
    }
    #[test]
    fn test_reader_bind() {
        let r = Reader::<i32, i32>::ask().bind(|n| Reader::pure(n * 2));
        assert_eq!(r.run(&5), 10);
    }
    #[test]
    fn test_sequence_maybe_all_just() {
        let ms = vec![Maybe::just(1), Maybe::just(2), Maybe::just(3)];
        let result = sequence_maybe(ms);
        assert_eq!(result.into_option(), Some(vec![1, 2, 3]));
    }
    #[test]
    fn test_sequence_maybe_with_nothing() {
        let ms = vec![Maybe::just(1), Maybe::nothing(), Maybe::just(3)];
        let result = sequence_maybe(ms);
        assert!(result.is_nothing());
    }
    #[test]
    fn test_map_maybe() {
        let xs = vec![1, 2, 3, 4];
        let result = map_maybe(xs, |n| {
            if n % 2 == 0 {
                Maybe::just(n * 10)
            } else {
                Maybe::nothing()
            }
        });
        assert!(result.is_nothing());
    }
    #[test]
    fn test_map_maybe_all_succeed() {
        let xs = vec![2, 4, 6];
        let result = map_maybe(xs, |n| Maybe::just(n / 2));
        assert_eq!(result.into_option(), Some(vec![1, 2, 3]));
    }
    #[test]
    fn test_filter_maybe() {
        let xs = vec![1, 2, 3, 4, 5];
        let result = filter_maybe(xs, |n| Maybe::just(*n % 2 == 0));
        assert_eq!(result.into_option(), Some(vec![2, 4]));
    }
    #[test]
    fn test_fold_maybe() {
        let xs = vec![1, 2, 3, 4];
        let result = fold_maybe(xs, 0, |acc, x| Maybe::just(acc + x));
        assert_eq!(result.into_option(), Some(10));
    }
    #[test]
    fn test_fold_maybe_failure() {
        let xs = vec![1, 2, 3, 4];
        let result = fold_maybe(xs, 0i32, |acc, x| {
            if x == 3 {
                Maybe::nothing()
            } else {
                Maybe::just(acc + x)
            }
        });
        assert!(result.is_nothing());
    }
    #[test]
    fn test_kleisli_compose() {
        let f = |n: i32| {
            if n > 0 {
                Maybe::just(n * 2)
            } else {
                Maybe::nothing()
            }
        };
        let g = |n: i32| {
            if n < 100 {
                Maybe::just(n + 1)
            } else {
                Maybe::nothing()
            }
        };
        let fg = kleisli_compose(f, g);
        assert_eq!(fg(5).into_option(), Some(11));
    }
    #[test]
    fn test_registered_monad_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        let names = registered_monad_names(&env);
        assert!(names.contains(&"Monad".to_string()));
        assert!(names.contains(&"Functor".to_string()));
        assert!(names.len() >= 6);
    }
}
/// Build the monad-as-monoid-in-endofunctors axiom.
/// States that every monad (M, return, join) is a monoid in
/// the category of endofunctors under functor composition.
pub fn mnd_ext_monad_monoid_in_endofunctors(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MonadAsMonoid"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Kleisli category identity axiom.
/// In Kleisli(M), the identity morphism on A is `return : A -> M A`.
pub fn mnd_ext_kleisli_identity(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_inst"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Monad"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("A"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("a"),
                    Box::new(Expr::BVar(0)),
                    Box::new(Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(1)))),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("KleisliId"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Kleisli composition axiom.
/// (f >=> g) a = f a >>= g where f: A -> M B, g: B -> M C
pub fn mnd_ext_kleisli_composition(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Monad"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("KleisliComp"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Kleisli left-identity law.
/// return >=> f = f (pure is left identity in Kleisli category)
pub fn mnd_ext_kleisli_left_identity_law(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Kleisli.left_id"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Kleisli right-identity law.
/// f >=> return = f
pub fn mnd_ext_kleisli_right_identity_law(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Kleisli.right_id"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Kleisli associativity law.
/// (f >=> g) >=> h = f >=> (g >=> h)
pub fn mnd_ext_kleisli_assoc_law(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Kleisli.assoc"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the join/flatten left-unit law.
/// join . fmap return = id
pub fn mnd_ext_join_left_unit(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Join.left_unit"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the join/flatten right-unit law.
/// join . return = id
pub fn mnd_ext_join_right_unit(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Join.right_unit"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the join associativity law.
/// join . join = join . fmap join
pub fn mnd_ext_join_assoc(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Join.assoc"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the join naturality axiom.
/// join . fmap (fmap f) = fmap f . join
pub fn mnd_ext_join_naturality(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Join.naturality"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the T-algebra (Eilenberg-Moore) axiom.
/// A T-algebra for monad T is (A, h: TA -> A) satisfying h . return_A = id
/// and h . T(h) = h . join_A
pub fn mnd_ext_t_algebra(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("T"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("A"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("TAlgebra"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Eilenberg-Moore unit law axiom.
/// h . η_A = id_A (algebra map respects unit)
pub fn mnd_ext_eilenberg_moore_unit(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("T"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("EilenbergMoore.unit_law"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Eilenberg-Moore multiplication law axiom.
/// h . T(h) = h . μ_A (algebra map respects multiplication)
pub fn mnd_ext_eilenberg_moore_mult(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("T"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("EilenbergMoore.mult_law"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the adjunction-monad relationship axiom.
/// Every adjunction F ⊣ G gives a monad T = G ∘ F with
/// unit η: Id -> GF and counit ε: FG -> Id.
pub fn mnd_ext_adjunction_monad(env: &mut Environment) -> Result<(), String> {
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    env.add(Declaration::Axiom {
        name: Name::str("AdjunctionMonad"),
        univ_params: vec![],
        ty: type2,
    })
    .map_err(|e| e.to_string())
}
/// Build the adjunction unit axiom.
/// η : Id -> G ∘ F (the unit of the adjunction)
pub fn mnd_ext_adjunction_unit(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(type1.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Adjunction.unit"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the adjunction counit axiom.
/// ε : F ∘ G -> Id (the counit of the adjunction)
pub fn mnd_ext_adjunction_counit(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("A"),
        Box::new(type1.clone()),
        Box::new(type1.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Adjunction.counit"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the comonad type class axiom.
/// Comonad is dual to Monad: extract :: W a -> a, duplicate :: W a -> W (W a)
pub fn mnd_ext_comonad(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("w"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Comonad"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the comonad extract axiom.
/// extract :: W a -> a (dual to pure/return)
pub fn mnd_ext_comonad_extract(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("w"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Comonad"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("A"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("wa"),
                    Box::new(Expr::App(Box::new(Expr::BVar(3)), Box::new(Expr::BVar(0)))),
                    Box::new(Expr::BVar(1)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Comonad.extract"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the comonad duplicate axiom.
/// duplicate :: W a -> W (W a) (dual to join)
pub fn mnd_ext_comonad_duplicate(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("w"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::InstImplicit,
            Name::str("_"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Comonad"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Comonad.duplicate"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the comonad left-identity law.
/// extract . duplicate = id
pub fn mnd_ext_comonad_left_id(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("w"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Comonad.left_id"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the MonadTrans lift axiom.
/// lift :: m a -> t m a (monad transformer lifting)
pub fn mnd_ext_monad_trans_lift(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let trans_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
        )),
        Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MonadTrans"),
        univ_params: vec![],
        ty: trans_ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the MonadTrans lift-pure law.
/// lift (pure a) = pure a
pub fn mnd_ext_monad_trans_lift_pure(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MonadTrans.lift_pure"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the MonadTrans lift-bind law.
/// lift (m >>= f) = lift m >>= (lift . f)
pub fn mnd_ext_monad_trans_lift_bind(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("t"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MonadTrans.lift_bind"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the State monad get axiom.
/// get :: State s s  (reads current state)
pub fn mnd_ext_state_get(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("s"),
        Box::new(type1.clone()),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("MonadState"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Const(Name::str("StateM"), vec![])),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("StateGet"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the State monad put axiom.
/// put :: s -> State s ()  (overwrites state)
pub fn mnd_ext_state_put(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("s"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("new_s"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Sort(Level::zero())),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("StatePut"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the State monad modify axiom.
/// modify :: (s -> s) -> State s ()
pub fn mnd_ext_state_modify(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("s"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::BVar(1)),
            )),
            Box::new(Expr::Sort(Level::zero())),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("StateModify"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the State get-put law.
/// get >>= put = return ()
pub fn mnd_ext_state_get_put_law(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("State.get_put_law"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the State put-get law.
/// put s >> get = put s >> return s
pub fn mnd_ext_state_put_get_law(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("State.put_get_law"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Writer monad tell axiom.
/// tell :: w -> Writer w ()
pub fn mnd_ext_writer_tell(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("w"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("msg"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::Sort(Level::zero())),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("WriterTell"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Writer monad listen axiom.
/// listen :: Writer w a -> Writer w (a, w)
pub fn mnd_ext_writer_listen(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("w"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::zero())),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("WriterListen"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Writer monad pass axiom.
/// pass :: Writer w (a, w -> w) -> Writer w a
pub fn mnd_ext_writer_pass(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("w"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::zero())),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("WriterPass"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Writer tell-unit law.
/// tell mempty >> m = m
pub fn mnd_ext_writer_tell_unit(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("w"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Writer.tell_unit_law"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Reader monad ask axiom.
/// ask :: Reader r r  (retrieve environment)
pub fn mnd_ext_reader_ask(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("r"),
        Box::new(type1.clone()),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("MonadReader"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Const(Name::str("ReaderM"), vec![])),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("ReaderAsk"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Reader monad local axiom.
/// local :: (r -> r) -> Reader r a -> Reader r a
pub fn mnd_ext_reader_local(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("r"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::zero())),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("ReaderLocal"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Reader ask-ask law.
/// ask >>= \r -> ask >>= \r' -> f r r' = ask >>= \r -> f r r
pub fn mnd_ext_reader_ask_ask_law(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Reader.ask_ask_law"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Continuation monad callCC axiom.
/// callCC :: ((a -> Cont r b) -> Cont r a) -> Cont r a
pub fn mnd_ext_cont_callcc(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let cont_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("ContM"),
        univ_params: vec![],
        ty: cont_ty,
    })
    .map_err(|e| e.to_string())?;
    let callcc_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("r"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("callCC"),
        univ_params: vec![],
        ty: callcc_ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the callCC abort law.
/// callCC f >>= k = callCC (\c -> f (\a -> c a >>= k))
pub fn mnd_ext_cont_callcc_abort(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("callCC.abort_law"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the IO monad as free monad axiom.
/// IO is modeled as the free monad over the IO functor.
pub fn mnd_ext_io_free_monad(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let io_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("a"),
        Box::new(type1.clone()),
        Box::new(type1.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO"),
        univ_params: vec![],
        ty: io_ty,
    })
    .map_err(|e| e.to_string())
}
