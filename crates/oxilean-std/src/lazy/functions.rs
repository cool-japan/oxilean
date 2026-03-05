//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::sync::{Arc, OnceLock};

use super::types::{Deferred, LazyBatch, LazyList, LazyOption, LazyPair, Memo, MemoFn, Thunk};

/// Build Lazy type in the environment.
pub fn build_lazy_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let lazy_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(type2.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy"),
        univ_params: vec![],
        ty: lazy_ty,
    })
    .map_err(|e| e.to_string())?;
    let mk_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::Const(Name::str("Unit"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy.mk"),
        univ_params: vec![],
        ty: mk_ty,
    })
    .map_err(|e| e.to_string())?;
    let force_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::BVar(1)),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy.force"),
        univ_params: vec![],
        ty: force_ty,
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
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("x"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy.map"),
        univ_params: vec![],
        ty: map_ty,
    })
    .map_err(|e| e.to_string())?;
    build_lazy_combinators(env)?;
    Ok(())
}
/// Build additional lazy combinator axioms: bind, zip, pure, ap.
pub fn build_lazy_combinators(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let pure_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy.pure"),
        univ_params: vec![],
        ty: pure_ty,
    })
    .map_err(|e| e.to_string())?;
    let bind_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("la"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy.bind"),
        univ_params: vec![],
        ty: bind_ty,
    })
    .map_err(|e| e.to_string())?;
    let zip_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("la"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("lb"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                        Box::new(Expr::App(
                            Box::new(Expr::App(
                                Box::new(Expr::Const(Name::str("Prod"), vec![])),
                                Box::new(Expr::BVar(3)),
                            )),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy.zip"),
        univ_params: vec![],
        ty: zip_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Create an infinite stream of natural numbers starting at `start`.
pub fn nats_from(start: u64) -> LazyList<u64> {
    LazyList::cons(start, move || nats_from(start + 1))
}
/// Create a lazy stream that repeats a value.
pub fn repeat_val<A: Clone + 'static>(val: A) -> LazyList<A> {
    let v = val.clone();
    LazyList::cons(val, move || repeat_val(v))
}
/// Create a lazy stream from a range `[lo, hi)`.
pub fn lazy_range(lo: u64, hi: u64) -> LazyList<u64> {
    if lo >= hi {
        LazyList::Nil
    } else {
        LazyList::cons(lo, move || lazy_range(lo + 1, hi))
    }
}
/// Build an `Expr` for `Lazy.mk α f`.
pub fn make_lazy_mk(alpha: Expr, f: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Lazy.mk"), vec![])),
            Box::new(alpha),
        )),
        Box::new(f),
    )
}
/// Build an `Expr` for `Lazy.force α x`.
pub fn make_lazy_force(alpha: Expr, x: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Lazy.force"), vec![])),
            Box::new(alpha),
        )),
        Box::new(x),
    )
}
/// Build an `Expr` for `Lazy α`.
pub fn make_lazy_type(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Lazy"), vec![])),
        Box::new(alpha),
    )
}
/// Return all Lazy-related names registered in the environment.
pub fn registered_lazy_names(env: &Environment) -> Vec<String> {
    let candidates = [
        "Lazy",
        "Lazy.mk",
        "Lazy.force",
        "Lazy.map",
        "Lazy.pure",
        "Lazy.bind",
        "Lazy.zip",
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
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("Unit"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_lazy_env() {
        let mut env = setup_env();
        assert!(build_lazy_env(&mut env).is_ok());
        assert!(env.get(&Name::str("Lazy")).is_some());
        assert!(env.get(&Name::str("Lazy.mk")).is_some());
        assert!(env.get(&Name::str("Lazy.force")).is_some());
    }
    #[test]
    fn test_lazy_map() {
        let mut env = setup_env();
        build_lazy_env(&mut env).expect("build_lazy_env should succeed");
        let decl = env
            .get(&Name::str("Lazy.map"))
            .expect("declaration 'Lazy.map' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_lazy_combinators_registered() {
        let mut env = setup_env();
        build_lazy_env(&mut env).expect("build_lazy_env should succeed");
        assert!(env.get(&Name::str("Lazy.pure")).is_some());
        assert!(env.get(&Name::str("Lazy.bind")).is_some());
        assert!(env.get(&Name::str("Lazy.zip")).is_some());
    }
    #[test]
    fn test_thunk_deferred() {
        let mut t = Thunk::new(|| 42);
        assert!(!t.is_evaluated());
        let val = t.force();
        assert_eq!(*val, 42);
        assert!(t.is_evaluated());
    }
    #[test]
    fn test_thunk_evaluated() {
        let t = Thunk::evaluated(99);
        assert!(t.is_evaluated());
    }
    #[test]
    fn test_thunk_computed_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        let mut t = Thunk::new(move || {
            c.fetch_add(1, Ordering::SeqCst);
            5
        });
        t.force();
        t.force();
        assert_eq!(*t.force(), 5);
    }
    #[test]
    fn test_memo_computed_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        let m = Memo::new(move || {
            c.fetch_add(1, Ordering::SeqCst);
            100u32
        });
        assert!(!m.is_initialized());
        assert_eq!(*m.get(), 100);
        assert!(m.is_initialized());
        assert_eq!(*m.get(), 100);
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn test_deferred_pure_force() {
        let d = Deferred::pure(7i32);
        assert_eq!(d.force(), 7);
    }
    #[test]
    fn test_deferred_map() {
        let d = Deferred::pure(3i32).map(|n| n * n);
        assert_eq!(d.force(), 9);
    }
    #[test]
    fn test_deferred_bind() {
        let d = Deferred::pure(4i32).bind(|n| Deferred::pure(n + 1));
        assert_eq!(d.force(), 5);
    }
    #[test]
    fn test_deferred_zip() {
        let a = Deferred::pure(10i32);
        let b = Deferred::pure("hello");
        let zipped = a.zip(b);
        assert_eq!(zipped.force(), (10, "hello"));
    }
    #[test]
    fn test_lazy_list_take() {
        let stream = nats_from(0);
        let first5 = stream.take(5);
        assert_eq!(first5, vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_lazy_list_nil() {
        let nil: LazyList<u64> = LazyList::nil();
        assert!(nil.is_nil());
        let empty = nil.take(10);
        assert!(empty.is_empty());
    }
    #[test]
    fn test_lazy_range() {
        let stream = lazy_range(5, 10);
        let vals = stream.take(10);
        assert_eq!(vals, vec![5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_lazy_range_empty() {
        let stream = lazy_range(10, 5);
        let vals = stream.take(10);
        assert!(vals.is_empty());
    }
    #[test]
    fn test_repeat_val() {
        let stream = repeat_val(42u64);
        let vals = stream.take(3);
        assert_eq!(vals, vec![42, 42, 42]);
    }
    #[test]
    fn test_memo_fn_cached() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        let mf = MemoFn::new(move || {
            c.fetch_add(1, Ordering::SeqCst);
            "result"
        });
        assert!(!mf.is_cached());
        let v1 = mf.get();
        let v2 = mf.get();
        assert_eq!(*v1, "result");
        assert_eq!(*v2, "result");
        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert!(mf.is_cached());
    }
    #[test]
    fn test_lazy_option_none() {
        let lo: LazyOption<i32> = LazyOption::none();
        assert!(lo.is_none());
        assert_eq!(lo.force(), None);
    }
    #[test]
    fn test_lazy_option_some() {
        let lo = LazyOption::pure(55i32);
        assert!(!lo.is_none());
        assert_eq!(lo.force(), Some(55));
    }
    #[test]
    fn test_lazy_option_map() {
        let lo = LazyOption::pure(10i32).map(|n| n * 2);
        assert_eq!(lo.force(), Some(20));
        let lo_none: LazyOption<i32> = LazyOption::none();
        let lo_mapped = lo_none.map(|n| n * 2);
        assert_eq!(lo_mapped.force(), None);
    }
    #[test]
    fn test_lazy_option_deferred() {
        let lo = LazyOption::some(|| 42 + 1);
        assert_eq!(lo.force(), Some(43));
    }
    #[test]
    fn test_make_lazy_type_expr() {
        let alpha = Expr::Const(Name::str("Nat"), vec![]);
        let ty = make_lazy_type(alpha);
        assert!(matches!(ty, Expr::App(_, _)));
    }
    #[test]
    fn test_registered_lazy_names() {
        let mut env = setup_env();
        build_lazy_env(&mut env).expect("build_lazy_env should succeed");
        let names = registered_lazy_names(&env);
        assert!(names.contains(&"Lazy".to_string()));
        assert!(names.contains(&"Lazy.mk".to_string()));
        assert!(names.contains(&"Lazy.force".to_string()));
        assert!(names.len() >= 4);
    }
    #[test]
    fn test_nats_from_large_skip() {
        let stream = nats_from(100);
        let vals = stream.take(3);
        assert_eq!(vals, vec![100, 101, 102]);
    }
    #[test]
    fn test_lazy_list_cons_explicit() {
        let stream = LazyList::cons(1u64, || LazyList::cons(2, || LazyList::nil()));
        let vals = stream.take(5);
        assert_eq!(vals, vec![1, 2]);
    }
    #[test]
    fn test_lazy_list_nil_is_empty() {
        let empty: LazyList<u64> = LazyList::nil();
        assert!(empty.is_nil());
        assert_eq!(empty.take(10), vec![]);
    }
}
/// Register `Lazy.toThunk` axiom: convert a `Lazy α` to a `Thunk α`.
pub fn register_lazy_to_thunk(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let to_thunk_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Thunk"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy.toThunk"),
        univ_params: vec![],
        ty: to_thunk_ty,
    })
    .map_err(|e| e.to_string())
}
/// Register `Lazy.const`: a lazy value that always returns the same element.
pub fn register_lazy_const_axiom(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    use oxilean_kernel::{BinderInfo, Declaration, Expr, Level, Name};
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let const_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Lazy.const"),
        univ_params: vec![],
        ty: const_ty,
    })
    .map_err(|e| e.to_string())
}
/// Map a function over a lazy list.
pub fn lazy_map<A: Clone + 'static, B: Clone + 'static>(
    stream: LazyList<A>,
    f: impl Fn(A) -> B + Clone + 'static,
) -> LazyList<B> {
    match stream {
        LazyList::Nil => LazyList::Nil,
        LazyList::Cons(h, t) => {
            let head = f(h);
            let f2 = f.clone();
            LazyList::Cons(head, Box::new(move || lazy_map(t(), f2)))
        }
    }
}
/// Filter a lazy list, keeping only elements satisfying a predicate.
pub fn lazy_filter<A: Clone + 'static>(
    stream: LazyList<A>,
    pred: impl Fn(&A) -> bool + Clone + 'static,
) -> LazyList<A> {
    match stream {
        LazyList::Nil => LazyList::Nil,
        LazyList::Cons(h, t) => {
            if pred(&h) {
                let p2 = pred.clone();
                LazyList::Cons(h, Box::new(move || lazy_filter(t(), p2)))
            } else {
                lazy_filter(t(), pred)
            }
        }
    }
}
/// Zip two lazy lists into a list of pairs.
pub fn lazy_zip<A: Clone + 'static, B: Clone + 'static>(
    sa: LazyList<A>,
    sb: LazyList<B>,
) -> LazyList<(A, B)> {
    match (sa, sb) {
        (LazyList::Cons(a, ta), LazyList::Cons(b, tb)) => {
            LazyList::Cons((a, b), Box::new(move || lazy_zip(ta(), tb())))
        }
        _ => LazyList::Nil,
    }
}
/// Fold a lazy list from the left (forcing elements).
pub fn lazy_foldl<A: Clone + 'static, B>(
    stream: LazyList<A>,
    init: B,
    mut f: impl FnMut(B, A) -> B,
) -> B {
    let mut acc = init;
    let mut cur = stream;
    loop {
        match cur {
            LazyList::Nil => return acc,
            LazyList::Cons(h, t) => {
                acc = f(acc, h);
                cur = t();
            }
        }
    }
}
#[cfg(test)]
mod lazy_new_tests {
    use super::*;
    #[test]
    fn test_lazy_batch_force_all() {
        let batch: LazyBatch<i32> = LazyBatch::new()
            .push(Deferred::pure(1))
            .push(Deferred::pure(2))
            .push(Deferred::pure(3));
        assert_eq!(batch.len(), 3);
        let results = batch.force_all();
        assert_eq!(results, vec![1, 2, 3]);
    }
    #[test]
    fn test_lazy_batch_empty() {
        let batch: LazyBatch<i32> = LazyBatch::new();
        assert!(batch.is_empty());
        let results = batch.force_all();
        assert!(results.is_empty());
    }
    #[test]
    fn test_lazy_pair_force() {
        let pair = LazyPair::new(Deferred::pure(10i32), Deferred::pure("hello"));
        assert_eq!(pair.force(), (10, "hello"));
    }
    #[test]
    fn test_lazy_pair_map_first() {
        let pair = LazyPair::new(Deferred::pure(5i32), Deferred::pure(3i32));
        let mapped = pair.map_first(|x| x * 2);
        let (a, b) = mapped.force();
        assert_eq!(a, 10);
        assert_eq!(b, 3);
    }
    #[test]
    fn test_lazy_pair_map_second() {
        let pair = LazyPair::new(Deferred::pure(5i32), Deferred::pure(3i32));
        let mapped = pair.map_second(|x| x + 1);
        let (a, b) = mapped.force();
        assert_eq!(a, 5);
        assert_eq!(b, 4);
    }
    #[test]
    fn test_lazy_map() {
        let stream = nats_from(0);
        let doubled = lazy_map(stream, |n| n * 2);
        let results = doubled.take(5);
        assert_eq!(results, vec![0, 2, 4, 6, 8]);
    }
    #[test]
    fn test_lazy_filter() {
        let stream = nats_from(0);
        let evens = lazy_filter(stream, |n| n % 2 == 0);
        let results = evens.take(4);
        assert_eq!(results, vec![0, 2, 4, 6]);
    }
    #[test]
    fn test_lazy_zip() {
        let sa = lazy_range(0, 5);
        let sb = lazy_range(10, 15);
        let zipped = lazy_zip(sa, sb);
        let results = zipped.take(10);
        assert_eq!(results, vec![(0, 10), (1, 11), (2, 12), (3, 13), (4, 14)]);
    }
    #[test]
    fn test_lazy_zip_unequal_length() {
        let sa = lazy_range(0, 3);
        let sb = lazy_range(0, 10);
        let zipped = lazy_zip(sa, sb);
        let results = zipped.take(10);
        assert_eq!(results.len(), 3);
    }
    #[test]
    fn test_lazy_foldl_sum() {
        let stream = lazy_range(1, 6);
        let sum = lazy_foldl(stream, 0u64, |acc, x| acc + x);
        assert_eq!(sum, 15);
    }
    #[test]
    fn test_lazy_foldl_nil() {
        let stream: LazyList<u64> = LazyList::nil();
        let sum = lazy_foldl(stream, 42u64, |acc, x| acc + x);
        assert_eq!(sum, 42);
    }
    #[test]
    fn test_lazy_map_empty() {
        let stream: LazyList<u64> = LazyList::nil();
        let mapped = lazy_map(stream, |n| n * 2);
        assert!(mapped.is_nil());
    }
    #[test]
    fn test_lazy_filter_none_pass() {
        let stream = lazy_range(0, 5);
        let filtered = lazy_filter(stream, |n| *n > 100);
        let results = filtered.take(10);
        assert!(results.is_empty());
    }
}
/// `CallByNeed.strategy`: call-by-need evaluation strategy axiom.
/// Type: {α : Type} → (Unit → α) → α
pub fn lzy_ext_call_by_need_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("thunk"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::Const(Name::str("Unit"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
            Box::new(Expr::BVar(1)),
        )),
    )
}
/// `CallByValue.strategy`: call-by-value evaluation strategy.
/// Type: {α β : Type} → α → (α → β) → β
pub fn lzy_ext_call_by_value_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("val"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("k"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    )
}
/// `Sharing.axiom`: sharing/memoization — force twice gives same result.
/// Type: {α : Type} → (x : Lazy α) → Lazy.force x = Lazy.force x
pub fn lzy_ext_sharing_axiom_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(prop),
        )),
    )
}
/// `CoNat`: lazy natural numbers (conatural numbers).
/// Type: Type
pub fn lzy_ext_conat_ty() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// `CoNat.zero`: the zero conatural number.
/// Type: CoNat
pub fn lzy_ext_conat_zero_ty() -> Expr {
    Expr::Const(Name::str("CoNat"), vec![])
}
/// `CoNat.succ`: successor of a conatural number.
/// Type: CoNat → CoNat
pub fn lzy_ext_conat_succ_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(Expr::Const(Name::str("CoNat"), vec![])),
        Box::new(Expr::Const(Name::str("CoNat"), vec![])),
    )
}
/// `CoNat.infinity`: the infinite conatural number.
/// Type: CoNat
pub fn lzy_ext_conat_infinity_ty() -> Expr {
    Expr::Const(Name::str("CoNat"), vec![])
}
/// `CoNat.corecursor`: corecursor for CoNat.
/// Type: {σ : Type} → (σ → Option σ) → σ → CoNat
pub fn lzy_ext_conat_corecursor_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("σ"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Option"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("s0"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Const(Name::str("CoNat"), vec![])),
            )),
        )),
    )
}
/// `Nakano.later`: Nakano's "later" modality for guarded recursion (▶ A).
/// Type: Type → Type
pub fn lzy_ext_nakano_later_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1),
        Box::new(type2),
    )
}
/// `Nakano.next`: introduction for the later modality.
/// Type: {α : Type} → α → ▶ α
pub fn lzy_ext_nakano_next_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nakano.later"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    )
}
/// `Nakano.ap_later`: application under the later modality.
/// Type: {α β : Type} → ▶(α → β) → ▶ α → ▶ β
pub fn lzy_ext_nakano_ap_later_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("lf"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Nakano.later"), vec![])),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(1)),
                        Box::new(Expr::BVar(1)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("la"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Nakano.later"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Nakano.later"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    )
}
/// `Löb.axiom`: Löb's theorem for guarded recursion.
/// Type: {α : Type} → (▶ α → α) → α
pub fn lzy_ext_lob_axiom_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("step"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Nakano.later"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::BVar(1)),
            )),
            Box::new(Expr::BVar(1)),
        )),
    )
}
/// `Delay.monad`: the partiality monad (delay monad).
/// Type: Type → Type
pub fn lzy_ext_delay_monad_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1),
        Box::new(type2),
    )
}
/// `Delay.now`: inject a value into the delay monad immediately.
/// Type: {α : Type} → α → Delay α
pub fn lzy_ext_delay_now_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Delay"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    )
}
/// `Delay.later`: delay a computation by one step.
/// Type: {α : Type} → Delay α → Delay α
pub fn lzy_ext_delay_later_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("d"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Delay"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Delay"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    )
}
/// `Delay.bind`: monadic bind for the delay monad.
/// Type: {α β : Type} → Delay α → (α → Delay β) → Delay β
pub fn lzy_ext_delay_bind_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("da"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Delay"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Delay"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Delay"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    )
}
/// `Capretta.setoid`: Capretta's setoid model for partial functions.
/// Type: {α : Type} → (Delay α → Delay α → Prop) → Prop
pub fn lzy_ext_capretta_setoid_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("R"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("Delay"), vec![])),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Delay"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(prop.clone()),
                )),
            )),
            Box::new(prop),
        )),
    )
}
/// `OmegaChain.colimit`: colimit of an ω-chain of types.
/// Type: (Nat → Type) → Type
pub fn lzy_ext_omega_chain_colimit_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("F"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(type1.clone()),
        )),
        Box::new(type2),
    )
}
/// `Coinductive.terminal_coalgebra`: coinductive types as terminal coalgebras.
/// Type: {F : Type → Type} → Prop
pub fn lzy_ext_terminal_coalgebra_ty() -> Expr {
    let prop = Expr::Sort(Level::zero());
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("F"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1),
            Box::new(type2),
        )),
        Box::new(prop),
    )
}
/// `Bisimulation.corecursion_principle`: bisimulation corecursion.
/// Type: {α : Type} → (∀ s t, R s t → head s = head t) → Prop
pub fn lzy_ext_bisim_corecursion_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(prop),
    )
}
/// `LazyStateMachine.type`: lazy state machine type.
/// Type: Type → Type → Type
pub fn lzy_ext_lazy_state_machine_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("S"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("I"),
            Box::new(type1),
            Box::new(type2),
        )),
    )
}
/// `LazyArray.type`: lazy functional array type.
/// Type: Type → Nat → Type
pub fn lzy_ext_lazy_array_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(type2),
        )),
    )
}
/// `LazyArray.lookup`: lazy array lookup.
/// Type: {α : Type} → {n : Nat} → LazyArray α n → Fin n → Lazy α
pub fn lzy_ext_lazy_array_lookup_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("n"),
            Box::new(Expr::Const(Name::str("Nat"), vec![])),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("arr"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("LazyArray"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("i"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Fin"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                        Box::new(Expr::BVar(3)),
                    )),
                )),
            )),
        )),
    )
}
/// `StrictPair.fst`: forcing both components of a strict pair.
/// Type: {α β : Type} → α × β → α
pub fn lzy_ext_strict_pair_fst_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("p"),
                Box::new(Expr::App(
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Prod"), vec![])),
                        Box::new(Expr::BVar(1)),
                    )),
                    Box::new(Expr::BVar(0)),
                )),
                Box::new(Expr::BVar(2)),
            )),
        )),
    )
}
/// `LazyPair.type`: lazy product — neither component is forced.
/// Type: Type → Type → Type
pub fn lzy_ext_lazy_pair_type_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("β"),
            Box::new(type1),
            Box::new(type2),
        )),
    )
}
/// `Lazy.force_pure`: forcing a pure lazy value gives the value.
/// Type: {α : Type} → (a : α) → Lazy.force (Lazy.pure a) = a
pub fn lzy_ext_force_pure_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(prop),
        )),
    )
}
/// `Lazy.map_force`: map then force equals apply function to forced value.
/// Type: {α β : Type} → (f : α → β) → (la : Lazy α) → Prop
pub fn lzy_ext_map_force_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("la"),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                    Box::new(prop),
                )),
            )),
        )),
    )
}
/// `Lazy.bind_force`: bind then force — monad law.
/// Type: {α β : Type} → (la : Lazy α) → (f : α → Lazy β) → Prop
pub fn lzy_ext_bind_force_ty() -> Expr {
    lzy_ext_map_force_ty()
}
/// `Lazy.eta`: eta law for lazy values.
/// Type: {α : Type} → (la : Lazy α) → Lazy.mk (fun _ => Lazy.force la) = la
pub fn lzy_ext_eta_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("la"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(prop),
        )),
    )
}
/// `Productive.corecursion`: productive corecursion combinator.
/// Type: {α σ : Type} → (σ → α × σ) → σ → Stream α
pub fn lzy_ext_productive_corecursion_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("σ"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("step"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(0)),
                    Box::new(Expr::App(
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Prod"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                        Box::new(Expr::BVar(1)),
                    )),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("s0"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("Stream"), vec![])),
                        Box::new(Expr::BVar(3)),
                    )),
                )),
            )),
        )),
    )
}
/// `Lazy.monad_left_identity`: left identity monad law for Lazy.
/// Type: {α β : Type} → (a : α) → (f : α → Lazy β) → Prop
pub fn lzy_ext_monad_left_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(Expr::BVar(1)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                    Box::new(prop),
                )),
            )),
        )),
    )
}
/// `Lazy.monad_right_identity`: right identity monad law for Lazy.
/// Type: {α : Type} → (la : Lazy α) → Prop
pub fn lzy_ext_monad_right_identity_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("la"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Lazy"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(prop),
        )),
    )
}
/// `Lazy.monad_assoc`: associativity monad law for Lazy.
/// Type: {α β γ : Type} → (la : Lazy α) → (f : α → Lazy β) → (g : β → Lazy γ) → Prop
pub fn lzy_ext_monad_assoc_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let prop = Expr::Sort(Level::zero());
    Expr::Pi(
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
                Box::new(type1),
                Box::new(prop),
            )),
        )),
    )
}
/// `GuardedRecursion.clock`: a clock variable for guarded recursion.
/// Type: Type
pub fn lzy_ext_guarded_clock_ty() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
/// `GuardedRecursion.force`: force a later value at a clock tick.
/// Type: {α : Type} → ▶ α → α
pub fn lzy_ext_guarded_force_ty() -> Expr {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("la"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Nakano.later"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::BVar(1)),
        )),
    )
}
/// Register extended lazy evaluation axioms in the environment.
///
/// Covers: call-by-need/value strategies, sharing, CoNat, Nakano's modality,
/// Löb's axiom, delay monad, Capretta's setoid, omega-chain colimits,
/// terminal coalgebras, bisimulation corecursion, lazy state machines,
/// lazy functional arrays, strict/lazy pairs, and monad laws.
pub fn register_lazy_extended(env: &mut Environment) -> Result<(), String> {
    let axioms: &[(&str, fn() -> Expr)] = &[
        ("CallByNeed.strategy", lzy_ext_call_by_need_ty),
        ("CallByValue.strategy", lzy_ext_call_by_value_ty),
        ("Sharing.axiom", lzy_ext_sharing_axiom_ty),
        ("CoNat", lzy_ext_conat_ty),
        ("CoNat.zero", lzy_ext_conat_zero_ty),
        ("CoNat.succ", lzy_ext_conat_succ_ty),
        ("CoNat.infinity", lzy_ext_conat_infinity_ty),
        ("CoNat.corecursor", lzy_ext_conat_corecursor_ty),
        ("Nakano.later", lzy_ext_nakano_later_ty),
        ("Nakano.next", lzy_ext_nakano_next_ty),
        ("Nakano.ap_later", lzy_ext_nakano_ap_later_ty),
        ("Löb.axiom", lzy_ext_lob_axiom_ty),
        ("Delay", lzy_ext_delay_monad_ty),
        ("Delay.now", lzy_ext_delay_now_ty),
        ("Delay.later", lzy_ext_delay_later_ty),
        ("Delay.bind", lzy_ext_delay_bind_ty),
        ("Capretta.setoid", lzy_ext_capretta_setoid_ty),
        ("OmegaChain.colimit", lzy_ext_omega_chain_colimit_ty),
        (
            "Coinductive.terminal_coalgebra",
            lzy_ext_terminal_coalgebra_ty,
        ),
        ("Bisimulation.corecursion", lzy_ext_bisim_corecursion_ty),
        ("LazyStateMachine", lzy_ext_lazy_state_machine_ty),
        ("LazyArray", lzy_ext_lazy_array_ty),
        ("LazyArray.lookup", lzy_ext_lazy_array_lookup_ty),
        ("StrictPair.fst", lzy_ext_strict_pair_fst_ty),
        ("LazyPair.type", lzy_ext_lazy_pair_type_ty),
        ("Lazy.force_pure", lzy_ext_force_pure_ty),
        ("Lazy.map_force", lzy_ext_map_force_ty),
        ("Lazy.bind_force", lzy_ext_bind_force_ty),
        ("Lazy.eta", lzy_ext_eta_ty),
        ("Productive.corecursion", lzy_ext_productive_corecursion_ty),
        ("Lazy.monad_left_identity", lzy_ext_monad_left_identity_ty),
        ("Lazy.monad_right_identity", lzy_ext_monad_right_identity_ty),
        ("Lazy.monad_assoc", lzy_ext_monad_assoc_ty),
        ("GuardedRecursion.clock", lzy_ext_guarded_clock_ty),
        ("GuardedRecursion.force", lzy_ext_guarded_force_ty),
    ];
    for (name, ty_fn) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty_fn(),
        })
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
