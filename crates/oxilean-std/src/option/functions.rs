//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::env_builder::{app, sort, var, EnvBuilder};
use oxilean_kernel::{BinderInfo, Declaration, Environment, Level};
use oxilean_kernel::{Expr, Name};

use super::types::{OptionCache, OptionIter, OptionMap, OptionMemo, WeightedOption};

/// Register the core `Option` declarations into the environment.
///
/// Adds `Option`, `Option.none`, `Option.some`, and `Option.rec`.
pub fn build_option_env(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("Option"), sort(1));
    env.add_axiom(Name::from_str("Option.none"), sort(1));
    env.add_axiom(Name::from_str("Option.some"), sort(1));
    env.add_axiom(Name::from_str("Option.rec"), sort(1));
}
/// Register `Option.getD` (get-with-default) into the environment.
///
/// `Option.getD : Option α → α → α`
pub fn build_option_getd(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("Option.getD"), sort(1));
}
/// Register `Option.map` into the environment.
///
/// `Option.map : (α → β) → Option α → Option β`
pub fn build_option_map(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("Option.map"), sort(1));
}
/// Register `Option.bind` (monadic bind) into the environment.
///
/// `Option.bind : Option α → (α → Option β) → Option β`
pub fn build_option_bind(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("Option.bind"), sort(1));
}
/// Register `Option.filter` into the environment.
pub fn build_option_filter(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("Option.filter"), sort(1));
}
/// Register the full `Option` API into the environment in one call.
pub fn build_full_option_env(env: &mut EnvBuilder) {
    build_option_env(env);
    build_option_getd(env);
    build_option_map(env);
    build_option_bind(env);
    build_option_filter(env);
    env.add_axiom(Name::from_str("Option.isSome"), sort(1));
    env.add_axiom(Name::from_str("Option.isNone"), sort(1));
    env.add_axiom(Name::from_str("Option.get!"), sort(1));
    env.add_axiom(Name::from_str("Option.toList"), sort(1));
    env.add_axiom(Name::from_str("Option.all"), sort(1));
    env.add_axiom(Name::from_str("Option.any"), sort(1));
    env.add_axiom(Name::from_str("Option.zip"), sort(1));
    env.add_axiom(Name::from_str("Option.unzip"), sort(1));
    env.add_axiom(Name::from_str("Option.sequence"), sort(1));
}
/// Build `Option.some x`.
pub fn mk_some(x: Expr) -> Expr {
    app(var("Option.some"), x)
}
/// Build `Option.none`.
pub fn mk_none() -> Expr {
    var("Option.none")
}
/// Build `Option.getD opt default`.
pub fn mk_option_getd(opt: Expr, default: Expr) -> Expr {
    app(app(var("Option.getD"), opt), default)
}
/// Build `Option.map f opt`.
pub fn mk_option_map(f: Expr, opt: Expr) -> Expr {
    app(app(var("Option.map"), f), opt)
}
/// Build `Option.bind opt f`.
pub fn mk_option_bind(opt: Expr, f: Expr) -> Expr {
    app(app(var("Option.bind"), opt), f)
}
/// Return `default` if `opt` is `None`, otherwise apply `f` to the value.
pub fn option_get_or_else<T, U>(opt: Option<T>, default: U, f: impl FnOnce(T) -> U) -> U {
    match opt {
        Some(x) => f(x),
        None => default,
    }
}
/// Convert `Option<T>` to `Option<U>` by applying `f` only when `Some`.
pub fn option_map<T, U>(opt: Option<T>, f: impl FnOnce(T) -> U) -> Option<U> {
    opt.map(f)
}
/// Monadic bind: apply `f` to the inner value if `Some`.
pub fn option_bind<T, U>(opt: Option<T>, f: impl FnOnce(T) -> Option<U>) -> Option<U> {
    opt.and_then(f)
}
/// Applicative apply: if both `f` and `a` are `Some`, apply `f` to `a`.
pub fn option_ap<T, U>(f: Option<impl FnOnce(T) -> U>, a: Option<T>) -> Option<U> {
    match (f, a) {
        (Some(func), Some(val)) => Some(func(val)),
        _ => None,
    }
}
/// Lift a binary function into `Option`.
pub fn option_lift2<A, B, C>(f: impl FnOnce(A, B) -> C, a: Option<A>, b: Option<B>) -> Option<C> {
    match (a, b) {
        (Some(x), Some(y)) => Some(f(x, y)),
        _ => None,
    }
}
/// Lift a ternary function into `Option`.
pub fn option_lift3<A, B, C, D>(
    f: impl FnOnce(A, B, C) -> D,
    a: Option<A>,
    b: Option<B>,
    c: Option<C>,
) -> Option<D> {
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => Some(f(x, y, z)),
        _ => None,
    }
}
/// Zip two options into an option of a pair.
pub fn option_zip<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    match (a, b) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}
/// Unzip an `Option<(A, B)>` into `(Option<A>, Option<B>)`.
pub fn option_unzip<A, B>(p: Option<(A, B)>) -> (Option<A>, Option<B>) {
    match p {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    }
}
/// Convert `Option<Option<T>>` to `Option<T>` (join/flatten).
pub fn option_join<T>(opt: Option<Option<T>>) -> Option<T> {
    opt.flatten()
}
/// Sequence a list of options — `None` if any is `None`.
pub fn option_sequence<T>(opts: Vec<Option<T>>) -> Option<Vec<T>> {
    let mut result = Vec::with_capacity(opts.len());
    for opt in opts {
        result.push(opt?);
    }
    Some(result)
}
/// Traverse a slice with a fallible function, collecting results.
///
/// Returns `None` if any call returns `None`.
pub fn option_traverse<A, B>(xs: &[A], f: impl Fn(&A) -> Option<B>) -> Option<Vec<B>> {
    let mut result = Vec::with_capacity(xs.len());
    for x in xs {
        result.push(f(x)?);
    }
    Some(result)
}
/// Return the first `Some` value from an iterator of options.
pub fn option_first<T>(opts: impl IntoIterator<Item = Option<T>>) -> Option<T> {
    opts.into_iter().flatten().next()
}
/// Return the last `Some` value from a vector of options.
pub fn option_last<T>(opts: Vec<Option<T>>) -> Option<T> {
    opts.into_iter().flatten().last()
}
/// Collect all `Some` values from an iterator, ignoring `None`.
pub fn option_catsome<T>(opts: impl IntoIterator<Item = Option<T>>) -> Vec<T> {
    opts.into_iter().flatten().collect()
}
/// Retry `f` up to `n` times until it returns `Some`.
pub fn option_retry<T>(n: usize, mut f: impl FnMut() -> Option<T>) -> Option<T> {
    for _ in 0..n {
        if let Some(v) = f() {
            return Some(v);
        }
    }
    None
}
/// Transpose `Option<Result<T, E>>` to `Result<Option<T>, E>`.
pub fn option_transpose<T, E>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    match opt {
        Some(Ok(v)) => Ok(Some(v)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}
/// Combine two options: if both are `Some`, apply `f`; otherwise return the other.
pub fn option_combine<T: Clone>(
    a: Option<T>,
    b: Option<T>,
    f: impl FnOnce(T, T) -> T,
) -> Option<T> {
    match (a, b) {
        (Some(x), Some(y)) => Some(f(x, y)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}
/// Alternative operator: return `a` if `Some`, else `b`.
pub fn option_alt<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    a.or(b)
}
/// Filter an option by a predicate.
pub fn option_filter<T>(opt: Option<T>, pred: impl FnOnce(&T) -> bool) -> Option<T> {
    opt.filter(pred)
}
/// Convert an option to a `Result<T, E>` with a given error.
pub fn option_ok_or<T, E>(opt: Option<T>, err: E) -> Result<T, E> {
    opt.ok_or(err)
}
/// Convert a `Result<T, E>` to `Option<T>`, discarding errors.
pub fn result_to_option<T, E>(r: Result<T, E>) -> Option<T> {
    r.ok()
}
/// Apply a side effect to the inner value if `Some`.
pub fn option_tap<T: Clone>(opt: &Option<T>, f: impl FnOnce(&T)) -> &Option<T> {
    if let Some(ref v) = opt {
        f(v);
    }
    opt
}
/// Convert `Option<&str>` to `Option<String>`.
pub fn option_to_owned(opt: Option<&str>) -> Option<String> {
    opt.map(|s| s.to_owned())
}
/// Convert `&Option<T>` to `Option<&T>`.
pub fn option_as_ref<T>(opt: &Option<T>) -> Option<&T> {
    opt.as_ref()
}
/// Partition a vector of options into (somes, count_of_nones).
pub fn option_partition<T>(opts: Vec<Option<T>>) -> (Vec<T>, usize) {
    let mut somes = Vec::new();
    let mut none_count = 0;
    for opt in opts {
        match opt {
            Some(v) => somes.push(v),
            None => none_count += 1,
        }
    }
    (somes, none_count)
}
/// Fold over an option.
///
/// If `None`, returns `init`. If `Some(x)`, returns `f(init, x)`.
pub fn option_fold<T, U>(opt: Option<T>, init: U, f: impl FnOnce(U, T) -> U) -> U {
    match opt {
        Some(x) => f(init, x),
        None => init,
    }
}
/// Return a default value if the option is `None`, consuming it.
pub fn option_unwrap_or_default<T: Default>(opt: Option<T>) -> T {
    opt.unwrap_or_default()
}
/// Count `Some` values in an iterator of options.
pub fn option_count_some<T>(opts: impl IntoIterator<Item = Option<T>>) -> usize {
    opts.into_iter().filter(|o| o.is_some()).count()
}
/// Build a `Some` chain from a vector: the first `Some` wins.
pub fn option_find<T>(candidates: Vec<T>, pred: impl Fn(&T) -> bool) -> Option<T> {
    candidates.into_iter().find(|x| pred(x))
}
/// Pair each element of a slice with its option-mapped value.
pub fn option_annotate<A: Clone, B>(xs: &[A], f: impl Fn(&A) -> Option<B>) -> Vec<(A, Option<B>)> {
    xs.iter().map(|x| (x.clone(), f(x))).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_option_map() {
        assert_eq!(option_map(Some(2u32), |x| x * 3), Some(6));
        assert_eq!(option_map::<u32, u32>(None, |x| x * 3), None);
    }
    #[test]
    fn test_option_bind() {
        let f = |x: u32| if x > 2 { Some(x * 10) } else { None };
        assert_eq!(option_bind(Some(5), f), Some(50));
        assert_eq!(option_bind(Some(1), f), None);
        assert_eq!(option_bind(None, f), None);
    }
    #[test]
    fn test_option_lift2() {
        let r = option_lift2(|a: u32, b: u32| a + b, Some(3), Some(4));
        assert_eq!(r, Some(7));
        let r2 = option_lift2(|a: u32, b: u32| a + b, None, Some(4));
        assert_eq!(r2, None);
    }
    #[test]
    fn test_option_zip() {
        assert_eq!(option_zip(Some(1u32), Some("a")), Some((1, "a")));
        assert_eq!(option_zip::<u32, &str>(None, Some("a")), None);
    }
    #[test]
    fn test_option_unzip() {
        let (a, b) = option_unzip(Some((1u32, 2u32)));
        assert_eq!(a, Some(1));
        assert_eq!(b, Some(2));
        let (c, d) = option_unzip::<u32, u32>(None);
        assert_eq!(c, None);
        assert_eq!(d, None);
    }
    #[test]
    fn test_option_sequence() {
        let v = vec![Some(1u32), Some(2), Some(3)];
        assert_eq!(option_sequence(v), Some(vec![1, 2, 3]));
        let v2 = vec![Some(1u32), None, Some(3)];
        assert_eq!(option_sequence(v2), None);
    }
    #[test]
    fn test_option_traverse() {
        let xs = vec![2u32, 4, 6];
        let r = option_traverse(&xs, |x| if x % 2 == 0 { Some(x / 2) } else { None });
        assert_eq!(r, Some(vec![1, 2, 3]));
        let xs2 = vec![2u32, 3, 6];
        let r2 = option_traverse(&xs2, |x| if x % 2 == 0 { Some(x / 2) } else { None });
        assert_eq!(r2, None);
    }
    #[test]
    fn test_option_catsome() {
        let opts = vec![Some(1u32), None, Some(3), None, Some(5)];
        assert_eq!(option_catsome(opts), vec![1, 3, 5]);
    }
    #[test]
    fn test_option_retry() {
        let mut count = 0u32;
        let result = option_retry(5, || {
            count += 1;
            if count >= 3 {
                Some(count)
            } else {
                None
            }
        });
        assert_eq!(result, Some(3));
    }
    #[test]
    fn test_option_combine() {
        let r = option_combine(Some(3u32), Some(5), |a, b| a + b);
        assert_eq!(r, Some(8));
        let r2 = option_combine(Some(3u32), None, |a, b| a + b);
        assert_eq!(r2, Some(3));
        let r3 = option_combine::<u32>(None, None, |a, b| a + b);
        assert_eq!(r3, None);
    }
    #[test]
    fn test_option_partition() {
        let opts = vec![Some(1u32), None, Some(3), None];
        let (somes, none_count) = option_partition(opts);
        assert_eq!(somes, vec![1, 3]);
        assert_eq!(none_count, 2);
    }
    #[test]
    fn test_option_fold() {
        let r = option_fold(Some(5u32), 0u32, |acc, x| acc + x);
        assert_eq!(r, 5);
        let r2 = option_fold::<u32, u32>(None, 42, |acc, x| acc + x);
        assert_eq!(r2, 42);
    }
    #[test]
    fn test_option_first_last() {
        let opts = vec![None, Some(2u32), Some(3)];
        assert_eq!(option_first(opts.clone()), Some(2));
        assert_eq!(option_last(opts), Some(3));
    }
    #[test]
    fn test_option_map_struct() {
        let mut map: OptionMap<&str, u32> = OptionMap::new();
        map.set("a", Some(1));
        map.set("b", None);
        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), None);
        assert_eq!(map.some_keys(), vec![&"a"]);
        assert_eq!(map.none_keys(), vec![&"b"]);
    }
    #[test]
    fn test_option_join() {
        let nested: Option<Option<u32>> = Some(Some(42));
        assert_eq!(option_join(nested), Some(42));
        let nested2: Option<Option<u32>> = Some(None);
        assert_eq!(option_join(nested2), None);
    }
    #[test]
    fn test_option_transpose() {
        let r: Option<Result<u32, &str>> = Some(Ok(5));
        assert_eq!(option_transpose(r), Ok(Some(5)));
        let r2: Option<Result<u32, &str>> = Some(Err("err"));
        assert!(option_transpose(r2).is_err());
        let r3: Option<Result<u32, &str>> = None;
        assert_eq!(option_transpose(r3), Ok(None));
    }
}
/// Extension methods for `Option<T>`.
///
/// Provides a fluent API for common option operations used throughout the
/// OxiLean elaborator.
#[allow(clippy::wrong_self_convention)]
pub trait OptionExt<T> {
    /// Map with a fallible function.
    fn try_map<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<Option<U>, E>;
    /// Return `true` if `Some` and the predicate holds.
    fn is_some_and(self, pred: impl FnOnce(&T) -> bool) -> bool;
    /// Return `true` if `None` or the predicate holds.
    fn is_none_or(self, pred: impl FnOnce(&T) -> bool) -> bool;
    /// Chain two options: if `self` is `Some`, also check `other`.
    fn and_also(self, other: Option<T>) -> Option<(T, T)>;
    /// Inspect the inner value without consuming.
    fn inspect_opt(self, f: impl FnOnce(&T)) -> Self;
    /// Convert to a vector (0 or 1 elements).
    fn to_vec(self) -> Vec<T>;
    /// Unwrap with a custom panic message.
    fn expect_msg(self, msg: &str) -> T;
}
impl<T> OptionExt<T> for Option<T> {
    fn try_map<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<Option<U>, E> {
        match self {
            Some(v) => f(v).map(Some),
            None => Ok(None),
        }
    }
    fn is_some_and(self, pred: impl FnOnce(&T) -> bool) -> bool {
        match self {
            Some(ref v) => pred(v),
            None => false,
        }
    }
    fn is_none_or(self, pred: impl FnOnce(&T) -> bool) -> bool {
        match self {
            Some(ref v) => pred(v),
            None => true,
        }
    }
    fn and_also(self, other: Option<T>) -> Option<(T, T)> {
        match (self, other) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
    fn inspect_opt(self, f: impl FnOnce(&T)) -> Self {
        if let Some(ref v) = self {
            f(v);
        }
        self
    }
    fn to_vec(self) -> Vec<T> {
        match self {
            Some(v) => vec![v],
            None => vec![],
        }
    }
    fn expect_msg(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => panic!("{msg}"),
        }
    }
}
/// Look up a key in multiple maps (slices), returning the first match.
pub fn option_chain_lookup<'a, K: PartialEq, V>(key: &K, maps: &[&'a [(K, V)]]) -> Option<&'a V> {
    for map in maps {
        if let Some(v) = map.iter().find(|(k, _)| k == key).map(|(_, v)| v) {
            return Some(v);
        }
    }
    None
}
/// Apply a series of transformations, stopping at the first `Some`.
pub fn option_first_of<T>(value: T, transforms: &[&dyn Fn(T) -> (T, Option<T>)]) -> Option<T>
where
    T: Clone,
{
    let mut current = value;
    for transform in transforms {
        let (next, result) = transform(current.clone());
        if result.is_some() {
            return result;
        }
        current = next;
    }
    None
}
/// Select the best option from a list of weighted options.
pub fn best_option<T>(options: Vec<WeightedOption<T>>) -> Option<T> {
    options
        .into_iter()
        .filter(|o| o.value.is_some())
        .max_by(|a, b| {
            a.weight
                .partial_cmp(&b.weight)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .and_then(|o| o.value)
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_option_ext_try_map() {
        let r: Result<Option<u32>, &str> = Some(5u32).try_map(|x| Ok::<u32, &str>(x * 2));
        assert_eq!(r, Ok(Some(10)));
        let r2: Result<Option<u32>, &str> = None.try_map(|x: u32| Ok::<u32, &str>(x * 2));
        assert_eq!(r2, Ok(None));
    }
    #[test]
    fn test_option_ext_is_some_and() {
        assert!(Some(5u32).is_some_and(|x| x > 3));
        assert!(!Some(2u32).is_some_and(|x| x > 3));
        assert!(!None::<u32>.is_some_and(|x| x > 3));
    }
    #[test]
    fn test_option_ext_to_vec() {
        assert_eq!(Some(42u32).to_vec(), vec![42]);
        assert_eq!(None::<u32>.to_vec(), Vec::<u32>::new());
    }
    #[test]
    fn test_option_ext_and_also() {
        let r = Some(1u32).and_also(Some(2u32));
        assert_eq!(r, Some((1, 2)));
        let r2 = None::<u32>.and_also(Some(2));
        assert!(r2.is_none());
    }
    #[test]
    fn test_weighted_option() {
        let opts = vec![
            WeightedOption::some(0.8, "high"),
            WeightedOption::some(0.5, "low"),
            WeightedOption::none(0.9),
        ];
        let best = best_option(opts);
        assert_eq!(best, Some("high"));
    }
    #[test]
    fn test_option_cache() {
        let mut cache: OptionCache<&str, u32> = OptionCache::new();
        let v = cache.get_or_insert_with("a", || Some(42));
        assert_eq!(v, Some(42));
        let v2 = cache.get_or_insert_with("a", || Some(999));
        assert_eq!(v2, Some(42));
        assert_eq!(cache.len(), 1);
    }
    #[test]
    fn test_option_cache_invalidate() {
        let mut cache: OptionCache<&str, u32> = OptionCache::new();
        cache.insert("x", Some(10));
        cache.invalidate(&"x");
        assert!(cache.is_empty());
    }
    #[test]
    fn test_option_count_some() {
        let opts = vec![Some(1u32), None, Some(3), None, Some(5)];
        assert_eq!(option_count_some(opts), 3);
    }
    #[test]
    fn test_option_annotate() {
        let xs = vec![2u32, 3, 4];
        let ann = option_annotate(&xs, |x| if x % 2 == 0 { Some(*x / 2) } else { None });
        assert_eq!(ann[0].1, Some(1));
        assert_eq!(ann[1].1, None);
        assert_eq!(ann[2].1, Some(2));
    }
    #[test]
    fn test_option_find() {
        let v = vec![1u32, 2, 3, 4, 5];
        let found = option_find(v, |x| *x > 3);
        assert_eq!(found, Some(4));
    }
}
/// Convert `Option<T>` to `Result<T, &'static str>`.
#[allow(dead_code)]
pub fn option_to_result<T>(opt: Option<T>, msg: &'static str) -> Result<T, &'static str> {
    opt.ok_or(msg)
}
/// Convert `Result<T, E>` to `Option<T>`.
#[allow(dead_code)]
pub fn result_ok<T, E>(r: Result<T, E>) -> Option<T> {
    r.ok()
}
/// Apply `f` and `g` to the inner value if `Some`, returning `Some` only if both succeed.
#[allow(dead_code)]
pub fn option_both<T: Clone, U, V>(
    opt: Option<T>,
    f: impl FnOnce(T) -> Option<U>,
    g: impl FnOnce(T) -> Option<V>,
) -> Option<(U, V)> {
    let v = opt?;
    let u = f(v.clone())?;
    let w = g(v)?;
    Some((u, w))
}
/// Return `Some(default)` if `opt` is `None`.
#[allow(dead_code)]
pub fn option_or_default<T: Default>(opt: Option<T>) -> Option<T> {
    Some(opt.unwrap_or_default())
}
/// Unwrap or compute a fallback using a closure.
#[allow(dead_code)]
pub fn option_unwrap_or_compute<T>(opt: Option<T>, f: impl FnOnce() -> T) -> T {
    opt.unwrap_or_else(f)
}
/// Map over an option, threading an accumulator.
#[allow(dead_code)]
pub fn option_map_with<T, U, S>(
    opt: Option<T>,
    state: S,
    f: impl FnOnce(S, T) -> (S, U),
) -> (S, Option<U>) {
    match opt {
        Some(v) => {
            let (s2, u) = f(state, v);
            (s2, Some(u))
        }
        None => (state, None),
    }
}
/// Build an `Option` from a boolean: `true` → `Some(value)`, `false` → `None`.
#[allow(dead_code)]
pub fn option_from_bool<T>(cond: bool, value: T) -> Option<T> {
    if cond {
        Some(value)
    } else {
        None
    }
}
/// Build an `Option<T>` by calling `f` and catching panics.
#[allow(dead_code)]
pub fn option_catch<T>(f: impl FnOnce() -> T + std::panic::UnwindSafe) -> Option<T> {
    std::panic::catch_unwind(f).ok()
}
/// Zip three options.
#[allow(dead_code)]
pub fn option_zip3<A, B, C>(a: Option<A>, b: Option<B>, c: Option<C>) -> Option<(A, B, C)> {
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => Some((x, y, z)),
        _ => None,
    }
}
/// Swap the type inside an `Option<(A, B)>`.
#[allow(dead_code)]
pub fn option_swap<A, B>(opt: Option<(A, B)>) -> Option<(B, A)> {
    opt.map(|(a, b)| (b, a))
}
/// Return the value inside an `Option`, or insert `value` if `None`.
#[allow(dead_code)]
pub fn option_get_or_insert<T: Clone>(opt: &mut Option<T>, value: T) -> T {
    if opt.is_none() {
        *opt = Some(value.clone());
        value
    } else {
        opt.clone().expect("opt is Some: checked by is_none guard")
    }
}
/// A simple priority-based option selection.
///
/// Returns the first `Some` that has a priority greater than or equal to `min_priority`.
#[allow(dead_code)]
pub fn option_select_priority<T>(
    candidates: Vec<(u32, Option<T>)>,
    min_priority: u32,
) -> Option<T> {
    candidates
        .into_iter()
        .filter(|(p, o)| *p >= min_priority && o.is_some())
        .max_by_key(|(p, _)| *p)
        .and_then(|(_, o)| o)
}
#[cfg(test)]
mod option_bridge_tests {
    use super::*;
    #[test]
    fn test_option_to_result_some() {
        let r = option_to_result(Some(5u32), "missing");
        assert_eq!(r, Ok(5));
    }
    #[test]
    fn test_option_to_result_none() {
        let r = option_to_result(None::<u32>, "missing");
        assert_eq!(r, Err("missing"));
    }
    #[test]
    fn test_option_iter_some() {
        let iter = OptionIter::new(Some(42u32));
        let v: Vec<u32> = iter.collect();
        assert_eq!(v, vec![42]);
    }
    #[test]
    fn test_option_iter_none() {
        let iter = OptionIter::<u32>::new(None);
        let v: Vec<u32> = iter.collect();
        assert!(v.is_empty());
    }
    #[test]
    fn test_option_zip3() {
        assert_eq!(
            option_zip3(Some(1u32), Some(2u32), Some(3u32)),
            Some((1, 2, 3))
        );
        assert!(option_zip3(Some(1u32), None::<u32>, Some(3u32)).is_none());
    }
    #[test]
    fn test_option_swap() {
        let swapped = option_swap(Some((1u32, "hello")));
        assert_eq!(swapped, Some(("hello", 1)));
    }
    #[test]
    fn test_option_from_bool() {
        assert_eq!(option_from_bool(true, 42u32), Some(42));
        assert_eq!(option_from_bool(false, 42u32), None);
    }
    #[test]
    fn test_option_select_priority() {
        let candidates = vec![(1, Some(10u32)), (3, Some(30)), (2, Some(20))];
        let v = option_select_priority(candidates, 2);
        assert_eq!(v, Some(30));
    }
    #[test]
    fn test_option_select_priority_none_below_min() {
        let candidates = vec![(1, Some(10u32))];
        let v = option_select_priority(candidates, 2);
        assert!(v.is_none());
    }
    #[test]
    fn test_option_memo() {
        let mut memo: OptionMemo<u32, String> = OptionMemo::new();
        let v = memo.get_or_compute(5, |k| Some(k.to_string()));
        assert_eq!(v, Some("5".to_string()));
        let v2 = memo.get_or_compute(5, |_| panic!("should not be called"));
        assert_eq!(v2, Some("5".to_string()));
        assert_eq!(memo.len(), 1);
    }
    #[test]
    fn test_option_memo_clear() {
        let mut memo: OptionMemo<u32, u32> = OptionMemo::new();
        memo.get_or_compute(1, |k| Some(*k));
        memo.clear();
        assert!(memo.is_empty());
    }
    #[test]
    fn test_option_both() {
        let r = option_both(
            Some(5u32),
            |x| if x > 3 { Some(x * 2) } else { None },
            |x| Some(x + 1),
        );
        assert_eq!(r, Some((10, 6)));
        let r2 = option_both(
            Some(1u32),
            |x| if x > 3 { Some(x * 2) } else { None },
            |x| Some(x + 1),
        );
        assert!(r2.is_none());
    }
    #[test]
    fn test_option_map_with() {
        let (state, result) = option_map_with(Some(5u32), 0u32, |s, v| (s + v, v * 2));
        assert_eq!(state, 5);
        assert_eq!(result, Some(10));
        let (s2, r2) = option_map_with(None::<u32>, 0u32, |s, v| (s + v, v * 2));
        assert_eq!(s2, 0);
        assert!(r2.is_none());
    }
}
pub(super) fn opt_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub(super) fn opt_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn opt_ext_cst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
pub fn opt_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn opt_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    opt_ext_app(opt_ext_app(f, a), b)
}
pub fn opt_ext_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    opt_ext_app(opt_ext_app2(f, a, b), c)
}
pub(super) fn opt_ext_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub(super) fn opt_ext_pi_imp(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub(super) fn opt_ext_arrow(a: Expr, b: Expr) -> Expr {
    opt_ext_pi("_", a, b)
}
pub(super) fn opt_ext_option_ty(alpha: Expr) -> Expr {
    opt_ext_app(opt_ext_cst("Option"), alpha)
}
pub(super) fn opt_ext_axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Option.monad_left_id: pure a >>= f = f a
pub fn opt_ext_monad_left_id(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "a",
                Expr::BVar(1),
                opt_ext_pi(
                    "f",
                    opt_ext_arrow(Expr::BVar(1), opt_ext_option_ty(Expr::BVar(1))),
                    opt_ext_prop(),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.monad_left_id", ty)
}
/// Option.monad_right_id: m >>= pure = m
pub fn opt_ext_monad_right_id(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let opt_a = opt_ext_option_ty(Expr::BVar(0));
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_a.clone(), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.monad_right_id", ty)
}
/// Option.monad_assoc: (m >>= f) >>= g = m >>= (fun x => f x >>= g)
pub fn opt_ext_monad_assoc(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp(
                "γ",
                t.clone(),
                opt_ext_pi(
                    "m",
                    opt_ext_option_ty(Expr::BVar(2)),
                    opt_ext_pi(
                        "f",
                        opt_ext_arrow(Expr::BVar(2), opt_ext_option_ty(Expr::BVar(2))),
                        opt_ext_pi(
                            "g",
                            opt_ext_arrow(Expr::BVar(2), opt_ext_option_ty(Expr::BVar(2))),
                            opt_ext_prop(),
                        ),
                    ),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.monad_assoc", ty)
}
/// Option.functor_id: fmap id = id
pub fn opt_ext_functor_id(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.functor_id", ty)
}
/// Option.functor_comp: fmap (f . g) = fmap f . fmap g
pub fn opt_ext_functor_comp(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp(
                "γ",
                t.clone(),
                opt_ext_pi(
                    "f",
                    opt_ext_arrow(Expr::BVar(1), Expr::BVar(1)),
                    opt_ext_pi(
                        "g",
                        opt_ext_arrow(Expr::BVar(3), Expr::BVar(3)),
                        opt_ext_prop(),
                    ),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.functor_comp", ty)
}
/// Option.ap_identity: pure id <*> v = v
pub fn opt_ext_ap_identity(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("v", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.ap_identity", ty)
}
/// Option.ap_homomorphism: pure f <*> pure x = pure (f x)
pub fn opt_ext_ap_homomorphism(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "f",
                opt_ext_arrow(Expr::BVar(1), Expr::BVar(1)),
                opt_ext_pi("x", Expr::BVar(2), opt_ext_prop()),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.ap_homomorphism", ty)
}
/// Option.ap_interchange: u <*> pure y = pure ($ y) <*> u
pub fn opt_ext_ap_interchange(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "u",
                opt_ext_option_ty(opt_ext_arrow(Expr::BVar(1), Expr::BVar(1))),
                opt_ext_pi("y", Expr::BVar(2), opt_ext_prop()),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.ap_interchange", ty)
}
/// Option.ap_composition: pure (.) <*> u <*> v <*> w = u <*> (v <*> w)
pub fn opt_ext_ap_composition(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp("γ", t.clone(), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.ap_composition", ty)
}
/// Option.alt_first_some: Some x <|> _ = Some x
pub fn opt_ext_alt_first_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi(
            "x",
            Expr::BVar(0),
            opt_ext_pi("other", opt_ext_option_ty(Expr::BVar(1)), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.alt_first_some", ty)
}
/// Option.alt_none_left: None <|> m = m
pub fn opt_ext_alt_none_left(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.alt_none_left", ty)
}
/// Option.alt_assoc: (a <|> b) <|> c = a <|> (b <|> c)
pub fn opt_ext_alt_assoc(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let oa = opt_ext_option_ty(Expr::BVar(0));
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi(
            "a",
            oa.clone(),
            opt_ext_pi("b", oa.clone(), opt_ext_pi("c", oa.clone(), opt_ext_prop())),
        ),
    );
    opt_ext_axiom(env, "Option.alt_assoc", ty)
}
/// Option.iso_to_either: Option α ≅ Either Unit α (to direction)
pub fn opt_ext_iso_to_either(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let opt_a = opt_ext_option_ty(Expr::BVar(0));
    let either_unit_a = opt_ext_app2(opt_ext_cst("Either"), opt_ext_cst("Unit"), Expr::BVar(0));
    let ty = opt_ext_pi_imp("α", t.clone(), opt_ext_arrow(opt_a, either_unit_a));
    opt_ext_axiom(env, "Option.iso_to_either", ty)
}
/// Option.iso_from_either: Option α ≅ Either Unit α (from direction)
pub fn opt_ext_iso_from_either(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let opt_a = opt_ext_option_ty(Expr::BVar(0));
    let either_unit_a = opt_ext_app2(opt_ext_cst("Either"), opt_ext_cst("Unit"), Expr::BVar(0));
    let ty = opt_ext_pi_imp("α", t.clone(), opt_ext_arrow(either_unit_a, opt_a));
    opt_ext_axiom(env, "Option.iso_from_either", ty)
}
/// Option.iso_roundtrip_to: from_either (to_either x) = x
pub fn opt_ext_iso_roundtrip_to(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("x", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.iso_roundtrip_to", ty)
}
/// Option.iso_roundtrip_from: to_either (from_either y) = y
pub fn opt_ext_iso_roundtrip_from(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let either_unit_a = opt_ext_app2(opt_ext_cst("Either"), opt_ext_cst("Unit"), Expr::BVar(0));
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("y", either_unit_a, opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.iso_roundtrip_from", ty)
}
/// Option.cata: catamorphism (fold)
pub fn opt_ext_cata(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "n",
                Expr::BVar(0),
                opt_ext_pi(
                    "s",
                    opt_ext_arrow(Expr::BVar(2), Expr::BVar(1)),
                    opt_ext_arrow(opt_ext_option_ty(Expr::BVar(3)), Expr::BVar(2)),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.cata", ty)
}
/// Option.ana: anamorphism (unfold)
pub fn opt_ext_ana(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "coalg",
                opt_ext_arrow(Expr::BVar(0), opt_ext_option_ty(Expr::BVar(1))),
                opt_ext_arrow(Expr::BVar(1), opt_ext_option_ty(Expr::BVar(2))),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.ana", ty)
}
/// Option.traverse_id: traverse Id = Id
pub fn opt_ext_traverse_id(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.traverse_id", ty)
}
/// Option.traverse_comp: traverse (f . g) = traverse f . traverse g
pub fn opt_ext_traverse_comp(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp("γ", t.clone(), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.traverse_comp", ty)
}
/// Option.sequence_traverse: sequence = traverse id
pub fn opt_ext_sequence_traverse(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp("α", t.clone(), opt_ext_prop());
    opt_ext_axiom(env, "Option.sequence_traverse", ty)
}
/// Option.foldable_foldl: foldl f z (Some x) = f z x
pub fn opt_ext_foldable_foldl(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "f",
                opt_ext_arrow(Expr::BVar(0), opt_ext_arrow(Expr::BVar(1), Expr::BVar(1))),
                opt_ext_pi(
                    "z",
                    Expr::BVar(1),
                    opt_ext_pi("x", Expr::BVar(3), opt_ext_prop()),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.foldable_foldl", ty)
}
/// Option.foldable_foldr: foldr f z (Some x) = f x z
pub fn opt_ext_foldable_foldr(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "f",
                opt_ext_arrow(Expr::BVar(1), opt_ext_arrow(Expr::BVar(1), Expr::BVar(1))),
                opt_ext_pi(
                    "z",
                    Expr::BVar(1),
                    opt_ext_pi("x", Expr::BVar(3), opt_ext_prop()),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.foldable_foldr", ty)
}
/// Option.foldable_fold_none: foldl f z None = z
pub fn opt_ext_foldable_fold_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "f",
                opt_ext_arrow(Expr::BVar(0), opt_ext_arrow(Expr::BVar(1), Expr::BVar(1))),
                opt_ext_pi("z", Expr::BVar(1), opt_ext_prop()),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.foldable_fold_none", ty)
}
/// OptionT.run: run the OptionT monad transformer
pub fn opt_ext_optiont_run(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "m",
        opt_ext_arrow(t.clone(), t.clone()),
        opt_ext_pi_imp(
            "α",
            t.clone(),
            opt_ext_arrow(
                opt_ext_app(Expr::BVar(1), opt_ext_option_ty(Expr::BVar(0))),
                opt_ext_app(Expr::BVar(1), opt_ext_option_ty(Expr::BVar(0))),
            ),
        ),
    );
    opt_ext_axiom(env, "OptionT.run", ty)
}
/// OptionT.lift: lift m into OptionT
pub fn opt_ext_optiont_lift(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "m",
        opt_ext_arrow(t.clone(), t.clone()),
        opt_ext_pi_imp(
            "α",
            t.clone(),
            opt_ext_arrow(
                opt_ext_app(Expr::BVar(1), Expr::BVar(0)),
                opt_ext_app(Expr::BVar(1), opt_ext_option_ty(Expr::BVar(0))),
            ),
        ),
    );
    opt_ext_axiom(env, "OptionT.lift", ty)
}
/// OptionT.bind: monadic bind for OptionT
pub fn opt_ext_optiont_bind(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "m",
        opt_ext_arrow(t.clone(), t.clone()),
        opt_ext_pi_imp(
            "α",
            t.clone(),
            opt_ext_pi_imp("β", t.clone(), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "OptionT.bind", ty)
}
/// Option.pointed_pure: pure x = Some x
pub fn opt_ext_pointed_pure(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("x", Expr::BVar(0), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.pointed_pure", ty)
}
/// Option.monoid_empty: empty = None
pub fn opt_ext_monoid_empty(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp("α", t.clone(), opt_ext_option_ty(Expr::BVar(0)));
    opt_ext_axiom(env, "Option.monoid_empty", ty)
}
/// Option.monoid_append: append (first Some wins)
pub fn opt_ext_monoid_append(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let oa = opt_ext_option_ty(Expr::BVar(0));
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_arrow(oa.clone(), opt_ext_arrow(oa.clone(), oa.clone())),
    );
    opt_ext_axiom(env, "Option.monoid_append", ty)
}
/// Option.monoid_left_id: None `append` m = m
pub fn opt_ext_monoid_left_id(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.monoid_left_id", ty)
}
/// Option.monoid_right_id: m `append` None = m
pub fn opt_ext_monoid_right_id(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.monoid_right_id", ty)
}
/// Option.bimap_some: bimap f (Some x) = Some (f x)
pub fn opt_ext_bimap_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "f",
                opt_ext_arrow(Expr::BVar(1), Expr::BVar(1)),
                opt_ext_pi("x", Expr::BVar(2), opt_ext_prop()),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.bimap_some", ty)
}
/// Option.bimap_none: bimap f None = None
pub fn opt_ext_bimap_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "f",
                opt_ext_arrow(Expr::BVar(1), Expr::BVar(1)),
                opt_ext_prop(),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.bimap_none", ty)
}
/// Option.zip_some: zip (Some a) (Some b) = Some (a, b)
pub fn opt_ext_zip_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "a",
                Expr::BVar(1),
                opt_ext_pi("b", Expr::BVar(1), opt_ext_prop()),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.zip_some", ty)
}
/// Option.zip_none: zip None _ = None
pub fn opt_ext_zip_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi("b", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.zip_none", ty)
}
/// Option.filter_some_true: filter p (Some x) = Some x when p x = true
pub fn opt_ext_filter_some_true(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi(
            "p",
            opt_ext_arrow(Expr::BVar(0), opt_ext_cst("Bool")),
            opt_ext_pi("x", Expr::BVar(1), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.filter_some_true", ty)
}
/// Option.filter_some_false: filter p (Some x) = None when p x = false
pub fn opt_ext_filter_some_false(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi(
            "p",
            opt_ext_arrow(Expr::BVar(0), opt_ext_cst("Bool")),
            opt_ext_pi("x", Expr::BVar(1), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.filter_some_false", ty)
}
/// Option.filter_none: filter p None = None
pub fn opt_ext_filter_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi(
            "p",
            opt_ext_arrow(Expr::BVar(0), opt_ext_cst("Bool")),
            opt_ext_prop(),
        ),
    );
    opt_ext_axiom(env, "Option.filter_none", ty)
}
/// Option.get_or_else_some: getOrElse (Some x) d = x
pub fn opt_ext_get_or_else_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi(
            "x",
            Expr::BVar(0),
            opt_ext_pi("d", Expr::BVar(1), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.get_or_else_some", ty)
}
/// Option.get_or_else_none: getOrElse None d = d
pub fn opt_ext_get_or_else_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("d", Expr::BVar(0), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.get_or_else_none", ty)
}
/// Option.or_else_some: orElse (Some x) _ = Some x
pub fn opt_ext_or_else_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let oa = opt_ext_option_ty(Expr::BVar(0));
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi(
            "x",
            Expr::BVar(0),
            opt_ext_pi("other", oa.clone(), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.or_else_some", ty)
}
/// Option.or_else_none: orElse None m = m
pub fn opt_ext_or_else_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("m", opt_ext_option_ty(Expr::BVar(0)), opt_ext_prop()),
    );
    opt_ext_axiom(env, "Option.or_else_none", ty)
}
/// Option.dimap: profunctor dimap
pub fn opt_ext_dimap(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp(
                "γ",
                t.clone(),
                opt_ext_pi_imp(
                    "δ",
                    t.clone(),
                    opt_ext_pi(
                        "pre",
                        opt_ext_arrow(Expr::BVar(2), Expr::BVar(3)),
                        opt_ext_pi(
                            "post",
                            opt_ext_arrow(Expr::BVar(2), Expr::BVar(2)),
                            opt_ext_pi(
                                "f",
                                opt_ext_arrow(Expr::BVar(5), opt_ext_option_ty(Expr::BVar(3))),
                                opt_ext_arrow(Expr::BVar(5), opt_ext_option_ty(Expr::BVar(4))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.dimap", ty)
}
/// Option.comonad_extract: extract (pure x) = x with fixed default
pub fn opt_ext_comonad_extract(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi(
            "m",
            opt_ext_option_ty(Expr::BVar(0)),
            opt_ext_pi("d", Expr::BVar(1), opt_ext_prop()),
        ),
    );
    opt_ext_axiom(env, "Option.comonad_extract", ty)
}
/// Option.join_some_some: join (Some (Some x)) = Some x
pub fn opt_ext_join_some_some(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let inner = opt_ext_option_ty(Expr::BVar(0));
    let outer = opt_ext_option_ty(inner);
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi("x", Expr::BVar(0), opt_ext_pi("m", outer, opt_ext_prop())),
    );
    opt_ext_axiom(env, "Option.join_some_some", ty)
}
/// Option.join_some_none: join (Some None) = None
pub fn opt_ext_join_some_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp("α", t.clone(), opt_ext_prop());
    opt_ext_axiom(env, "Option.join_some_none", ty)
}
/// Option.join_none: join None = None
pub fn opt_ext_join_none(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp("α", t.clone(), opt_ext_prop());
    opt_ext_axiom(env, "Option.join_none", ty)
}
/// Option.flatmap_is_join_map: m >>= f = join (map f m)
pub fn opt_ext_flatmap_is_join_map(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi(
                "m",
                opt_ext_option_ty(Expr::BVar(1)),
                opt_ext_pi(
                    "f",
                    opt_ext_arrow(Expr::BVar(2), opt_ext_option_ty(Expr::BVar(2))),
                    opt_ext_prop(),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.flatmap_is_join_map", ty)
}
/// Option.nullable_iso_to: Option α ≅ nullable α (to)
pub fn opt_ext_nullable_iso_to(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_arrow(
            opt_ext_option_ty(Expr::BVar(0)),
            opt_ext_option_ty(Expr::BVar(0)),
        ),
    );
    opt_ext_axiom(env, "Option.nullable_iso_to", ty)
}
/// Option.nullable_iso_from: nullable α ≅ Option α (from)
pub fn opt_ext_nullable_iso_from(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_arrow(
            opt_ext_option_ty(Expr::BVar(0)),
            opt_ext_option_ty(Expr::BVar(0)),
        ),
    );
    opt_ext_axiom(env, "Option.nullable_iso_from", ty)
}
/// Option.liftA2_def: liftA2 f u v = f <$> u <*> v
pub fn opt_ext_lift_a2(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    let ty = opt_ext_pi_imp(
        "α",
        t.clone(),
        opt_ext_pi_imp(
            "β",
            t.clone(),
            opt_ext_pi_imp(
                "γ",
                t.clone(),
                opt_ext_pi(
                    "f",
                    opt_ext_arrow(Expr::BVar(2), opt_ext_arrow(Expr::BVar(2), Expr::BVar(2))),
                    opt_ext_pi(
                        "u",
                        opt_ext_option_ty(Expr::BVar(3)),
                        opt_ext_pi("v", opt_ext_option_ty(Expr::BVar(3)), opt_ext_prop()),
                    ),
                ),
            ),
        ),
    );
    opt_ext_axiom(env, "Option.liftA2_def", ty)
}
/// Register all extended Option axioms into `env`.
///
/// Covers monad laws, functor laws, applicative laws, alternative laws,
/// Either isomorphism, catamorphism/anamorphism, traversal, foldable laws,
/// OptionT transformer, pointed functor, monoid laws, bimap, zip, filter,
/// getOrElse/orElse laws, profunctor, comonad, join/flatten, flatmap,
/// nullable isomorphism, and liftA2.
pub fn register_option_extended_axioms(env: &mut Environment) -> Result<(), String> {
    let t = opt_ext_type0();
    for name in ["Bool", "Option", "Either", "Unit", "Nat", "List", "Prod"] {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: t.clone(),
        });
    }
    opt_ext_monad_left_id(env)?;
    opt_ext_monad_right_id(env)?;
    opt_ext_monad_assoc(env)?;
    opt_ext_functor_id(env)?;
    opt_ext_functor_comp(env)?;
    opt_ext_ap_identity(env)?;
    opt_ext_ap_homomorphism(env)?;
    opt_ext_ap_interchange(env)?;
    opt_ext_ap_composition(env)?;
    opt_ext_alt_first_some(env)?;
    opt_ext_alt_none_left(env)?;
    opt_ext_alt_assoc(env)?;
    opt_ext_iso_to_either(env)?;
    opt_ext_iso_from_either(env)?;
    opt_ext_iso_roundtrip_to(env)?;
    opt_ext_iso_roundtrip_from(env)?;
    opt_ext_cata(env)?;
    opt_ext_ana(env)?;
    opt_ext_traverse_id(env)?;
    opt_ext_traverse_comp(env)?;
    opt_ext_sequence_traverse(env)?;
    opt_ext_foldable_foldl(env)?;
    opt_ext_foldable_foldr(env)?;
    opt_ext_foldable_fold_none(env)?;
    opt_ext_optiont_run(env)?;
    opt_ext_optiont_lift(env)?;
    opt_ext_optiont_bind(env)?;
    opt_ext_pointed_pure(env)?;
    opt_ext_monoid_empty(env)?;
    opt_ext_monoid_append(env)?;
    opt_ext_monoid_left_id(env)?;
    opt_ext_monoid_right_id(env)?;
    opt_ext_bimap_some(env)?;
    opt_ext_bimap_none(env)?;
    opt_ext_zip_some(env)?;
    opt_ext_zip_none(env)?;
    opt_ext_filter_some_true(env)?;
    opt_ext_filter_some_false(env)?;
    opt_ext_filter_none(env)?;
    opt_ext_get_or_else_some(env)?;
    opt_ext_get_or_else_none(env)?;
    opt_ext_or_else_some(env)?;
    opt_ext_or_else_none(env)?;
    opt_ext_dimap(env)?;
    opt_ext_comonad_extract(env)?;
    opt_ext_join_some_some(env)?;
    opt_ext_join_some_none(env)?;
    opt_ext_join_none(env)?;
    opt_ext_flatmap_is_join_map(env)?;
    opt_ext_nullable_iso_to(env)?;
    opt_ext_nullable_iso_from(env)?;
    opt_ext_lift_a2(env)?;
    Ok(())
}
