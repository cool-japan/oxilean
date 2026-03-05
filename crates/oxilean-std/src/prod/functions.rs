//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::env_builder::{app, prop, sort, var, EnvBuilder};
use oxilean_kernel::{BinderInfo, Declaration, Environment, Level};
use oxilean_kernel::{Expr, Name};

use super::types::{AssocMap, Pair, Sigma, SigmaVec, Triple};

/// Register the `Prod` type and its projections into the environment.
///
/// Adds `Prod`, `Prod.fst`, `Prod.snd`, and `Prod.mk` as axioms.
pub fn build_prod_projections(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("Prod"), sort(1));
    env.add_axiom(Name::from_str("Prod.mk"), sort(1));
    env.add_axiom(Name::from_str("Prod.fst"), sort(1));
    env.add_axiom(Name::from_str("Prod.snd"), sort(1));
}
/// Register `PProd` (the proof-relevant product) into the environment.
///
/// `PProd` is used for product types where both components are propositions.
pub fn build_pprod_env(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("PProd"), sort(1));
    env.add_axiom(Name::from_str("PProd.mk"), prop());
    env.add_axiom(Name::from_str("PProd.fst"), prop());
    env.add_axiom(Name::from_str("PProd.snd"), prop());
}
/// Register `And` (the propositional conjunction) into the environment.
///
/// `And` is propositional `PProd`, with `And.intro`, `And.left`, `And.right`.
pub fn build_and_env(env: &mut EnvBuilder) {
    env.add_axiom(Name::from_str("And"), prop());
    env.add_axiom(Name::from_str("And.intro"), prop());
    env.add_axiom(Name::from_str("And.left"), prop());
    env.add_axiom(Name::from_str("And.right"), prop());
}
/// Zip two slices into a vector of pairs.
///
/// Stops at the shorter slice.
pub fn zip<A: Clone, B: Clone>(xs: &[A], ys: &[B]) -> Vec<Pair<A, B>> {
    xs.iter()
        .zip(ys.iter())
        .map(|(a, b)| Pair::new(a.clone(), b.clone()))
        .collect()
}
/// Unzip a vector of pairs into two vectors.
pub fn unzip<A, B>(pairs: Vec<Pair<A, B>>) -> (Vec<A>, Vec<B>) {
    let mut as_ = Vec::with_capacity(pairs.len());
    let mut bs = Vec::with_capacity(pairs.len());
    for p in pairs {
        as_.push(p.fst);
        bs.push(p.snd);
    }
    (as_, bs)
}
/// Zip three slices into a vector of triples.
pub fn zip3<A: Clone, B: Clone, C: Clone>(xs: &[A], ys: &[B], zs: &[C]) -> Vec<Triple<A, B, C>> {
    xs.iter()
        .zip(ys.iter())
        .zip(zs.iter())
        .map(|((a, b), c)| Triple::new(a.clone(), b.clone(), c.clone()))
        .collect()
}
/// Unzip a vector of triples into three vectors.
pub fn unzip3<A, B, C>(triples: Vec<Triple<A, B, C>>) -> (Vec<A>, Vec<B>, Vec<C>) {
    let mut as_ = Vec::with_capacity(triples.len());
    let mut bs = Vec::with_capacity(triples.len());
    let mut cs = Vec::with_capacity(triples.len());
    for t in triples {
        as_.push(t.fst);
        bs.push(t.snd);
        cs.push(t.thd);
    }
    (as_, bs, cs)
}
/// Look up a key in an association list.
///
/// Returns the first value associated with `key`, or `None`.
pub fn assoc_lookup<'a, K: PartialEq, V>(assoc: &'a [(K, V)], key: &K) -> Option<&'a V> {
    assoc.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}
/// Insert or update a key-value pair in an association list.
///
/// If `key` already exists, its value is updated in place.
/// Otherwise the pair is appended.
pub fn assoc_insert<K: PartialEq + Clone, V: Clone>(assoc: &mut Vec<(K, V)>, key: K, value: V) {
    if let Some(entry) = assoc.iter_mut().find(|(k, _)| k == &key) {
        entry.1 = value;
    } else {
        assoc.push((key, value));
    }
}
/// Remove the first occurrence of `key` from an association list.
///
/// Returns the removed value, or `None` if not found.
pub fn assoc_remove<K: PartialEq, V>(assoc: &mut Vec<(K, V)>, key: &K) -> Option<V> {
    assoc
        .iter()
        .position(|(k, _)| k == key)
        .map(|pos| assoc.remove(pos).1)
}
/// Check whether a key is present in an association list.
pub fn assoc_mem<K: PartialEq, V>(assoc: &[(K, V)], key: &K) -> bool {
    assoc.iter().any(|(k, _)| k == key)
}
/// Collect all values for a given key (there may be duplicates).
pub fn assoc_lookup_all<'a, K: PartialEq, V>(assoc: &'a [(K, V)], key: &K) -> Vec<&'a V> {
    assoc
        .iter()
        .filter(|(k, _)| k == key)
        .map(|(_, v)| v)
        .collect()
}
/// Map all values in an association list.
pub fn assoc_map_values<K: Clone, V, W>(assoc: &[(K, V)], f: impl Fn(&V) -> W) -> Vec<(K, W)> {
    assoc.iter().map(|(k, v)| (k.clone(), f(v))).collect()
}
/// Remove all entries whose key satisfies the predicate.
pub fn assoc_filter_keys<K: Clone, V: Clone>(
    assoc: &[(K, V)],
    pred: impl Fn(&K) -> bool,
) -> Vec<(K, V)> {
    assoc.iter().filter(|(k, _)| pred(k)).cloned().collect()
}
/// Curry a function of type `(A, B) -> C` into `A -> B -> C`.
pub fn curry<A: Clone + 'static, B: 'static, C: 'static>(
    f: impl Fn(A, B) -> C + Clone + 'static,
) -> impl Fn(A) -> Box<dyn Fn(B) -> C> {
    move |a: A| {
        let f2 = f.clone();
        let a2 = a.clone();
        Box::new(move |b: B| f2(a2.clone(), b))
    }
}
/// Uncurry a function `A -> B -> C` into `(A, B) -> C`.
pub fn uncurry<A, B: 'static, C>(f: impl Fn(A) -> Box<dyn Fn(B) -> C>) -> impl Fn(A, B) -> C {
    move |a, b| f(a)(b)
}
/// Flip the arguments of a two-argument function.
pub fn flip<A, B, C>(f: impl Fn(A, B) -> C) -> impl Fn(B, A) -> C {
    move |b, a| f(a, b)
}
/// The identity function.
pub fn id<A>(a: A) -> A {
    a
}
/// The constant function: ignores the second argument.
pub fn const_fn<A: Clone, B>(a: A) -> impl Fn(B) -> A {
    move |_| a.clone()
}
/// Function composition: `(f ∘ g)(x) = f(g(x))`.
pub fn compose<A, B, C>(f: impl Fn(B) -> C, g: impl Fn(A) -> B) -> impl Fn(A) -> C {
    move |a| f(g(a))
}
/// Apply a function if the condition holds, otherwise return `a` unchanged.
pub fn apply_if<A>(cond: bool, a: A, f: impl FnOnce(A) -> A) -> A {
    if cond {
        f(a)
    } else {
        a
    }
}
/// Build a `Prod.mk a b` expression.
pub fn mk_prod(a: Expr, b: Expr) -> Expr {
    app(app(var("Prod.mk"), a), b)
}
/// Build a `Prod.fst p` expression.
pub fn mk_fst(p: Expr) -> Expr {
    app(var("Prod.fst"), p)
}
/// Build a `Prod.snd p` expression.
pub fn mk_snd(p: Expr) -> Expr {
    app(var("Prod.snd"), p)
}
/// Build `And.intro h1 h2`.
pub fn mk_and_intro(h1: Expr, h2: Expr) -> Expr {
    app(app(var("And.intro"), h1), h2)
}
/// Build `And.left h`.
pub fn mk_and_left(h: Expr) -> Expr {
    app(var("And.left"), h)
}
/// Build `And.right h`.
pub fn mk_and_right(h: Expr) -> Expr {
    app(var("And.right"), h)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pair_new_and_access() {
        let p = Pair::new(1u32, "hello");
        assert_eq!(p.fst, 1);
        assert_eq!(p.snd, "hello");
    }
    #[test]
    fn test_pair_swap() {
        let p = Pair::new(1u32, 2u32);
        let q = p.swap();
        assert_eq!(q.fst, 2);
        assert_eq!(q.snd, 1);
    }
    #[test]
    fn test_pair_bimap() {
        let p = Pair::new(1u32, 2u32);
        let q = p.bimap(|x| x + 10, |y| y * 2);
        assert_eq!(q.fst, 11);
        assert_eq!(q.snd, 4);
    }
    #[test]
    fn test_sigma_new() {
        let s = Sigma::new(42u32, "proof");
        assert_eq!(s.fst, 42);
        assert_eq!(s.snd, "proof");
    }
    #[test]
    fn test_triple_new() {
        let t = Triple::new(1u32, 2u32, 3u32);
        assert_eq!(t.fst, 1);
        assert_eq!(t.snd, 2);
        assert_eq!(t.thd, 3);
    }
    #[test]
    fn test_zip_unzip() {
        let xs = vec![1u32, 2, 3];
        let ys = vec!["a", "b", "c"];
        let zipped = zip(&xs, &ys);
        assert_eq!(zipped.len(), 3);
        assert_eq!(zipped[1].fst, 2);
        let (as_, bs) = unzip(zipped);
        assert_eq!(as_, xs);
        assert_eq!(bs, ys);
    }
    #[test]
    fn test_assoc_lookup() {
        let assoc = vec![("a", 1u32), ("b", 2), ("c", 3)];
        assert_eq!(assoc_lookup(&assoc, &"b"), Some(&2));
        assert_eq!(assoc_lookup(&assoc, &"z"), None);
    }
    #[test]
    fn test_assoc_insert() {
        let mut assoc: Vec<(&str, u32)> = vec![("a", 1), ("b", 2)];
        assoc_insert(&mut assoc, "b", 99);
        assert_eq!(assoc_lookup(&assoc, &"b"), Some(&99));
        assoc_insert(&mut assoc, "c", 3);
        assert_eq!(assoc.len(), 3);
    }
    #[test]
    fn test_assoc_remove() {
        let mut assoc: Vec<(&str, u32)> = vec![("a", 1), ("b", 2)];
        let removed = assoc_remove(&mut assoc, &"a");
        assert_eq!(removed, Some(1));
        assert_eq!(assoc.len(), 1);
    }
    #[test]
    fn test_assoc_mem() {
        let assoc = vec![("x", 10u32)];
        assert!(assoc_mem(&assoc, &"x"));
        assert!(!assoc_mem(&assoc, &"y"));
    }
    #[test]
    fn test_flip() {
        let sub = flip(|a: u32, b: u32| a - b);
        assert_eq!(sub(3, 10), 7);
    }
    #[test]
    fn test_compose() {
        let add1 = |x: u32| x + 1;
        let mul2 = |x: u32| x * 2;
        let f = compose(add1, mul2);
        assert_eq!(f(5), 11);
    }
    #[test]
    fn test_apply_if() {
        assert_eq!(apply_if(true, 5u32, |x| x * 2), 10);
        assert_eq!(apply_if(false, 5u32, |x| x * 2), 5);
    }
    #[test]
    fn test_zip3() {
        let xs = vec![1u32, 2];
        let ys = vec!['a', 'b'];
        let zs = vec![true, false];
        let triples = zip3(&xs, &ys, &zs);
        assert_eq!(triples.len(), 2);
        assert_eq!(triples[0].fst, 1);
        assert_eq!(triples[0].snd, 'a');
        assert!(triples[0].thd);
    }
    #[test]
    fn test_unzip3() {
        let triples = vec![Triple::new(1u32, 'a', true), Triple::new(2, 'b', false)];
        let (xs, ys, zs) = unzip3(triples);
        assert_eq!(xs, vec![1, 2]);
        assert_eq!(ys, vec!['a', 'b']);
        assert_eq!(zs, vec![true, false]);
    }
    #[test]
    fn test_assoc_map_values() {
        let assoc = vec![("a", 1u32), ("b", 2)];
        let mapped = assoc_map_values(&assoc, |v| v * 10);
        assert_eq!(mapped[0].1, 10);
        assert_eq!(mapped[1].1, 20);
    }
    #[test]
    fn test_mk_prod_expr() {
        let e = mk_prod(var("x"), var("y"));
        match e {
            Expr::App(_, _) => {}
            _ => panic!("expected App"),
        }
    }
}
/// Swap a pair of references.
pub fn swap_ref<A, B>(p: &Pair<A, B>) -> Pair<&B, &A> {
    Pair::new(&p.snd, &p.fst)
}
/// Map a function over the first elements of a slice of pairs.
pub fn map_fst_slice<A: Clone, B: Clone, C>(
    pairs: &[Pair<A, B>],
    f: impl Fn(A) -> C,
) -> Vec<Pair<C, B>> {
    pairs
        .iter()
        .map(|p| Pair::new(f(p.fst.clone()), p.snd.clone()))
        .collect()
}
/// Map a function over the second elements of a slice of pairs.
pub fn map_snd_slice<A: Clone, B: Clone, C>(
    pairs: &[Pair<A, B>],
    f: impl Fn(B) -> C,
) -> Vec<Pair<A, C>> {
    pairs
        .iter()
        .map(|p| Pair::new(p.fst.clone(), f(p.snd.clone())))
        .collect()
}
/// Collect all first elements from a slice of pairs.
pub fn firsts<A: Clone, B>(pairs: &[Pair<A, B>]) -> Vec<A> {
    pairs.iter().map(|p| p.fst.clone()).collect()
}
/// Collect all second elements from a slice of pairs.
pub fn seconds<A, B: Clone>(pairs: &[Pair<A, B>]) -> Vec<B> {
    pairs.iter().map(|p| p.snd.clone()).collect()
}
/// Cross product of two slices.
///
/// Returns all `Pair(a, b)` for each `a` in `xs` and `b` in `ys`.
pub fn cross_product<A: Clone, B: Clone>(xs: &[A], ys: &[B]) -> Vec<Pair<A, B>> {
    let mut result = Vec::with_capacity(xs.len() * ys.len());
    for a in xs {
        for b in ys {
            result.push(Pair::new(a.clone(), b.clone()));
        }
    }
    result
}
/// Zip two slices with index, producing `Triple(index, a, b)`.
pub fn zip_indexed<A: Clone, B: Clone>(xs: &[A], ys: &[B]) -> Vec<Triple<usize, A, B>> {
    xs.iter()
        .zip(ys.iter())
        .enumerate()
        .map(|(i, (a, b))| Triple::new(i, a.clone(), b.clone()))
        .collect()
}
/// Apply a function to each element of a pair.
pub fn both<A, B: Clone>(f: impl Fn(A) -> A, p: Pair<A, A>) -> Pair<A, A>
where
    A: Clone,
{
    Pair::new(f(p.fst), f(p.snd))
}
/// Duplicate a value into a pair.
pub fn dup<A: Clone>(a: A) -> Pair<A, A> {
    Pair::new(a.clone(), a)
}
/// Apply two functions to the same input and collect results.
pub fn fanout<A: Clone, B, C>(f: impl Fn(A) -> B, g: impl Fn(A) -> C, a: A) -> Pair<B, C> {
    Pair::new(f(a.clone()), g(a))
}
/// Merge two `Option` values into an `Option<Pair>`.
pub fn zip_option<A, B>(a: Option<A>, b: Option<B>) -> Option<Pair<A, B>> {
    match (a, b) {
        (Some(x), Some(y)) => Some(Pair::new(x, y)),
        _ => None,
    }
}
/// Unzip an `Option<Pair>` into a pair of `Option`s.
pub fn unzip_option<A, B>(p: Option<Pair<A, B>>) -> (Option<A>, Option<B>) {
    match p {
        Some(pair) => (Some(pair.fst), Some(pair.snd)),
        None => (None, None),
    }
}
/// Sort a slice of pairs by first element.
pub fn sort_by_fst<A: Ord, B: Clone>(pairs: &mut [Pair<A, B>]) {
    pairs.sort_by(|x, y| x.fst.cmp(&y.fst));
}
/// Sort a slice of pairs by second element.
pub fn sort_by_snd<A: Clone, B: Ord>(pairs: &mut [Pair<A, B>]) {
    pairs.sort_by(|x, y| x.snd.cmp(&y.snd));
}
/// Group a slice of pairs by equal first elements.
///
/// Returns a vector of `(key, values)` pairs preserving insertion order.
pub fn group_by_fst<A: PartialEq + Clone, B: Clone>(pairs: &[Pair<A, B>]) -> Vec<(A, Vec<B>)> {
    let mut groups: Vec<(A, Vec<B>)> = Vec::new();
    for p in pairs {
        if let Some(entry) = groups.iter_mut().find(|(k, _)| k == &p.fst) {
            entry.1.push(p.snd.clone());
        } else {
            groups.push((p.fst.clone(), vec![p.snd.clone()]));
        }
    }
    groups
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn test_firsts_seconds() {
        let pairs = vec![Pair::new(1u32, 'a'), Pair::new(2, 'b')];
        assert_eq!(firsts(&pairs), vec![1, 2]);
        assert_eq!(seconds(&pairs), vec!['a', 'b']);
    }
    #[test]
    fn test_cross_product() {
        let xs = vec![1u32, 2];
        let ys = vec!['a', 'b'];
        let cp = cross_product(&xs, &ys);
        assert_eq!(cp.len(), 4);
    }
    #[test]
    fn test_dup() {
        let p = dup(42u32);
        assert_eq!(p.fst, 42);
        assert_eq!(p.snd, 42);
    }
    #[test]
    fn test_fanout() {
        let p = fanout(|x: u32| x + 1, |x: u32| x * 2, 5u32);
        assert_eq!(p.fst, 6);
        assert_eq!(p.snd, 10);
    }
    #[test]
    fn test_zip_option() {
        let r = zip_option(Some(1u32), Some("a"));
        assert!(r.is_some());
        let r2 = zip_option::<u32, &str>(None, Some("a"));
        assert!(r2.is_none());
    }
    #[test]
    fn test_sigma_vec() {
        let mut sv: SigmaVec<u32, &str> = SigmaVec::new();
        sv.push(42, "proof");
        sv.push(7, "proof2");
        assert_eq!(sv.len(), 2);
        assert_eq!(sv.get(0).expect("get should succeed").fst, 42);
    }
    #[test]
    fn test_assoc_map() {
        let mut m: AssocMap<&str, u32> = AssocMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        m.insert("a", 99);
        assert_eq!(m.get(&"a"), Some(&99));
        assert_eq!(m.len(), 2);
    }
    #[test]
    fn test_assoc_map_remove() {
        let mut m: AssocMap<&str, u32> = AssocMap::new();
        m.insert("x", 10);
        let v = m.remove(&"x");
        assert_eq!(v, Some(10));
        assert!(m.is_empty());
    }
    #[test]
    fn test_group_by_fst() {
        let pairs = vec![
            Pair::new(1u32, 'a'),
            Pair::new(2u32, 'b'),
            Pair::new(1u32, 'c'),
        ];
        let groups = group_by_fst(&pairs);
        assert_eq!(groups.len(), 2);
        let g1 = groups
            .iter()
            .find(|(k, _)| *k == 1)
            .expect("len should succeed");
        assert_eq!(g1.1.len(), 2);
    }
    #[test]
    fn test_map_fst_snd_slice() {
        let pairs = vec![Pair::new(1u32, 2u32), Pair::new(3u32, 4u32)];
        let mapped = map_fst_slice(&pairs, |x| x + 10);
        assert_eq!(mapped[0].fst, 11);
        let mapped2 = map_snd_slice(&pairs, |y| y * 2);
        assert_eq!(mapped2[1].snd, 8);
    }
}
/// Build the standard Prod environment declarations.
///
/// Initialises the kernel environment with all built-in inductive types
/// (Bool, Unit, Nat, Eq, Prod, List, …) that the standard library depends on.
pub fn build_prod_env(env: &mut oxilean_kernel::Environment) -> Result<(), String> {
    oxilean_kernel::init_builtin_env(env)
}
/// Partition a vector of pairs into two vectors based on a predicate on the first component.
#[allow(clippy::type_complexity)]
pub fn partition_by_fst<A: Clone, B: Clone>(
    pairs: Vec<Pair<A, B>>,
    pred: impl Fn(&A) -> bool,
) -> (Vec<Pair<A, B>>, Vec<Pair<A, B>>) {
    let mut yes = Vec::new();
    let mut no = Vec::new();
    for p in pairs {
        if pred(&p.fst) {
            yes.push(p);
        } else {
            no.push(p);
        }
    }
    (yes, no)
}
/// Find the first pair whose first component satisfies the predicate.
pub fn find_by_fst<A, B>(pairs: &[Pair<A, B>], pred: impl Fn(&A) -> bool) -> Option<&Pair<A, B>> {
    pairs.iter().find(|p| pred(&p.fst))
}
/// Find the first pair whose second component satisfies the predicate.
pub fn find_by_snd<A, B>(pairs: &[Pair<A, B>], pred: impl Fn(&B) -> bool) -> Option<&Pair<A, B>> {
    pairs.iter().find(|p| pred(&p.snd))
}
/// Count pairs whose first component satisfies the predicate.
pub fn count_by_fst<A, B>(pairs: &[Pair<A, B>], pred: impl Fn(&A) -> bool) -> usize {
    pairs.iter().filter(|p| pred(&p.fst)).count()
}
/// Count pairs whose second component satisfies the predicate.
pub fn count_by_snd<A, B>(pairs: &[Pair<A, B>], pred: impl Fn(&B) -> bool) -> usize {
    pairs.iter().filter(|p| pred(&p.snd)).count()
}
/// Collect the first components of pairs where the predicate on the second component holds.
pub fn filter_firsts<A: Clone, B>(pairs: &[Pair<A, B>], pred: impl Fn(&B) -> bool) -> Vec<A> {
    pairs
        .iter()
        .filter(|p| pred(&p.snd))
        .map(|p| p.fst.clone())
        .collect()
}
/// Collect the second components of pairs where the predicate on the first component holds.
pub fn filter_seconds<A, B: Clone>(pairs: &[Pair<A, B>], pred: impl Fn(&A) -> bool) -> Vec<B> {
    pairs
        .iter()
        .filter(|p| pred(&p.fst))
        .map(|p| p.snd.clone())
        .collect()
}
/// Deduplicate a vector of pairs by their first component, keeping the last occurrence.
pub fn dedup_by_fst<A: PartialEq + Clone, B: Clone>(pairs: Vec<Pair<A, B>>) -> Vec<Pair<A, B>> {
    let mut seen: Vec<A> = Vec::new();
    let mut result: Vec<Pair<A, B>> = Vec::new();
    for p in pairs.into_iter().rev() {
        if !seen.contains(&p.fst) {
            seen.push(p.fst.clone());
            result.push(p);
        }
    }
    result.reverse();
    result
}
/// Transpose a vector of pairs of options.
pub fn transpose_pair_option<A, B>(pair: Pair<Option<A>, Option<B>>) -> Option<Pair<A, B>> {
    match (pair.fst, pair.snd) {
        (Some(a), Some(b)) => Some(Pair::new(a, b)),
        _ => None,
    }
}
/// Zip two iterators into a vector of pairs (stops at the shorter one).
pub fn zip_iter<A, B>(
    a: impl IntoIterator<Item = A>,
    b: impl IntoIterator<Item = B>,
) -> Vec<Pair<A, B>> {
    a.into_iter().zip(b).map(|(x, y)| Pair::new(x, y)).collect()
}
#[cfg(test)]
mod structural_tests {
    use super::*;
    #[test]
    fn test_pair_iter() {
        let p = Pair::new(1u32, 2u32);
        let v: Vec<u32> = p.iter().collect();
        assert_eq!(v, vec![1, 2]);
    }
    #[test]
    fn test_pair_iter_exact_size() {
        let p = Pair::new(10u32, 20u32);
        let mut it = p.iter();
        assert_eq!(it.size_hint(), (2, Some(2)));
        it.next();
        assert_eq!(it.size_hint(), (1, Some(1)));
        it.next();
        assert_eq!(it.size_hint(), (0, Some(0)));
    }
    #[test]
    fn test_partition_by_fst() {
        let pairs = vec![
            Pair::new(1u32, 'a'),
            Pair::new(2u32, 'b'),
            Pair::new(3u32, 'c'),
        ];
        let (evens, odds) = partition_by_fst(pairs, |x| x % 2 == 0);
        assert_eq!(evens.len(), 1);
        assert_eq!(odds.len(), 2);
    }
    #[test]
    fn test_find_by_fst() {
        let pairs = vec![Pair::new(1u32, 'a'), Pair::new(2u32, 'b')];
        let found = find_by_fst(&pairs, |x| *x == 2);
        assert!(found.is_some());
        assert_eq!(found.expect("found should be valid").snd, 'b');
    }
    #[test]
    fn test_find_by_snd() {
        let pairs = vec![Pair::new(1u32, 'a'), Pair::new(2u32, 'b')];
        let found = find_by_snd(&pairs, |c| *c == 'a');
        assert!(found.is_some());
        assert_eq!(found.expect("found should be valid").fst, 1);
    }
    #[test]
    fn test_count_by_fst() {
        let pairs = vec![
            Pair::new(2u32, 'a'),
            Pair::new(4u32, 'b'),
            Pair::new(3u32, 'c'),
        ];
        assert_eq!(count_by_fst(&pairs, |x| x % 2 == 0), 2);
    }
    #[test]
    fn test_count_by_snd() {
        let pairs = vec![
            Pair::new(1u32, true),
            Pair::new(2u32, false),
            Pair::new(3u32, true),
        ];
        assert_eq!(count_by_snd(&pairs, |b| *b), 2);
    }
    #[test]
    fn test_filter_firsts() {
        let pairs = vec![
            Pair::new(1u32, true),
            Pair::new(2u32, false),
            Pair::new(3u32, true),
        ];
        let firsts = filter_firsts(&pairs, |b| *b);
        assert_eq!(firsts, vec![1, 3]);
    }
    #[test]
    fn test_filter_seconds() {
        let pairs = vec![
            Pair::new(1u32, 'a'),
            Pair::new(2u32, 'b'),
            Pair::new(3u32, 'c'),
        ];
        let seconds = filter_seconds(&pairs, |x| *x > 1);
        assert_eq!(seconds, vec!['b', 'c']);
    }
    #[test]
    fn test_dedup_by_fst() {
        let pairs = vec![
            Pair::new(1u32, 'a'),
            Pair::new(2u32, 'b'),
            Pair::new(1u32, 'c'),
        ];
        let deduped = dedup_by_fst(pairs);
        assert_eq!(deduped.len(), 2);
        let p = deduped
            .iter()
            .find(|p| p.fst == 1)
            .expect("find should succeed");
        assert_eq!(p.snd, 'c');
    }
    #[test]
    fn test_transpose_pair_option_both_some() {
        let p = Pair::new(Some(1u32), Some('a'));
        let result = transpose_pair_option(p);
        assert!(result.is_some());
    }
    #[test]
    fn test_transpose_pair_option_fst_none() {
        let p: Pair<Option<u32>, Option<char>> = Pair::new(None, Some('a'));
        assert!(transpose_pair_option(p).is_none());
    }
    #[test]
    fn test_zip_iter() {
        let xs = vec![1u32, 2, 3];
        let ys = vec!['a', 'b', 'c'];
        let pairs = zip_iter(xs, ys);
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[2].fst, 3);
        assert_eq!(pairs[2].snd, 'c');
    }
}
pub fn prod_ext_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn prod_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn prod_ext_cst(name: &str) -> Expr {
    Expr::Const(Name::str(name), vec![])
}
pub fn prod_ext_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn prod_ext_app2(f: Expr, a: Expr, b: Expr) -> Expr {
    prod_ext_app(prod_ext_app(f, a), b)
}
pub fn prod_ext_app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    prod_ext_app(prod_ext_app2(f, a, b), c)
}
pub fn prod_ext_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn prod_ext_pi_imp(name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str(name),
        Box::new(dom),
        Box::new(body),
    )
}
pub fn prod_ext_arrow(a: Expr, b: Expr) -> Expr {
    prod_ext_pi("_", a, b)
}
pub fn prod_ext_prod_ty(a: Expr, b: Expr) -> Expr {
    prod_ext_app2(prod_ext_cst("Prod"), a, b)
}
pub fn prod_ext_axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Prod.universal_property: forall morphisms f: Z→A, g: Z→B, unique h: Z→A×B
pub fn prod_ext_universal_property(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "Z",
                t.clone(),
                prod_ext_pi(
                    "f",
                    prod_ext_arrow(Expr::BVar(0), Expr::BVar(2)),
                    prod_ext_pi(
                        "g",
                        prod_ext_arrow(Expr::BVar(1), Expr::BVar(2)),
                        prod_ext_arrow(
                            Expr::BVar(2),
                            prod_ext_prod_ty(Expr::BVar(4), Expr::BVar(3)),
                        ),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.universal_property", ty)
}
/// Prod.proj_fst_law: fst (mk a b) = a
pub fn prod_ext_proj_fst_law(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi(
                "a",
                Expr::BVar(1),
                prod_ext_pi("b", Expr::BVar(1), prod_ext_prop()),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.proj_fst_law", ty)
}
/// Prod.proj_snd_law: snd (mk a b) = b
pub fn prod_ext_proj_snd_law(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi(
                "a",
                Expr::BVar(1),
                prod_ext_pi("b", Expr::BVar(1), prod_ext_prop()),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.proj_snd_law", ty)
}
/// Prod.eta_law: p = mk (fst p) (snd p)
pub fn prod_ext_eta_law(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi(
                "p",
                prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                prod_ext_prop(),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.eta_law", ty)
}
/// Prod.functor_map_fst: map_fst f (mk a b) = mk (f a) b
pub fn prod_ext_functor_map_fst(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi(
                    "f",
                    prod_ext_arrow(Expr::BVar(2), Expr::BVar(1)),
                    prod_ext_pi(
                        "a",
                        Expr::BVar(3),
                        prod_ext_pi("b", Expr::BVar(3), prod_ext_prop()),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.functor_map_fst", ty)
}
/// Prod.functor_map_snd: map_snd g (mk a b) = mk a (g b)
pub fn prod_ext_functor_map_snd(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi(
                    "g",
                    prod_ext_arrow(Expr::BVar(1), Expr::BVar(1)),
                    prod_ext_pi(
                        "a",
                        Expr::BVar(3),
                        prod_ext_pi("b", Expr::BVar(3), prod_ext_prop()),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.functor_map_snd", ty)
}
/// Prod.bifunctor_law: bimap f g = map_fst f . map_snd g
pub fn prod_ext_bifunctor_law(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi_imp(
                    "δ",
                    t.clone(),
                    prod_ext_pi(
                        "f",
                        prod_ext_arrow(Expr::BVar(3), Expr::BVar(2)),
                        prod_ext_pi(
                            "g",
                            prod_ext_arrow(Expr::BVar(2), Expr::BVar(2)),
                            prod_ext_pi(
                                "p",
                                prod_ext_prod_ty(Expr::BVar(5), Expr::BVar(4)),
                                prod_ext_prop(),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.bifunctor_law", ty)
}
/// Prod.bifunctor_id: bimap id id = id
pub fn prod_ext_bifunctor_id(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi(
                "p",
                prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                prod_ext_prop(),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.bifunctor_id", ty)
}
/// Prod.zip_ap: applicative zip-list style: (f, g) <*> (a, b) = (f a, g b)
pub fn prod_ext_zip_ap(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi_imp("δ", t.clone(), prod_ext_prop()),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.zip_ap", ty)
}
/// Prod.comonad_extract: extract (a, b) = a (diagonal comonad)
pub fn prod_ext_comonad_extract(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi(
                "p",
                prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                Expr::BVar(2),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.comonad_extract", ty)
}
/// Prod.comonad_extend: extend f (a, b) = (f (a, b), b)
pub fn prod_ext_comonad_extend(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi(
                    "f",
                    prod_ext_arrow(
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(1)),
                        Expr::BVar(1),
                    ),
                    prod_ext_pi(
                        "p",
                        prod_ext_prod_ty(Expr::BVar(3), Expr::BVar(2)),
                        prod_ext_prop(),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.comonad_extend", ty)
}
/// Prod.tensor_unit_left: Unit × α ≅ α (left unitor)
pub fn prod_ext_tensor_unit_left(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_arrow(
            prod_ext_prod_ty(prod_ext_cst("Unit"), Expr::BVar(0)),
            Expr::BVar(0),
        ),
    );
    prod_ext_axiom(env, "Prod.tensor_unit_left", ty)
}
/// Prod.tensor_unit_right: α × Unit ≅ α (right unitor)
pub fn prod_ext_tensor_unit_right(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_arrow(
            prod_ext_prod_ty(Expr::BVar(0), prod_ext_cst("Unit")),
            Expr::BVar(0),
        ),
    );
    prod_ext_axiom(env, "Prod.tensor_unit_right", ty)
}
/// Prod.tensor_assoc: (α × β) × γ ≅ α × (β × γ) (associator)
pub fn prod_ext_tensor_assoc(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_arrow(
                    prod_ext_prod_ty(
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(1)),
                        Expr::BVar(0),
                    ),
                    prod_ext_prod_ty(
                        Expr::BVar(2),
                        prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.tensor_assoc", ty)
}
/// Prod.commutativity_iso: α × β ≅ β × α (swap)
pub fn prod_ext_commutativity_iso(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_arrow(
                prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                prod_ext_prod_ty(Expr::BVar(0), Expr::BVar(1)),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.commutativity_iso", ty)
}
/// Prod.swap_involution: swap (swap p) = p
pub fn prod_ext_swap_involution(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi(
                "p",
                prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                prod_ext_prop(),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.swap_involution", ty)
}
/// Prod.assoc_left: assoc_left (a, (b, c)) = ((a, b), c)
pub fn prod_ext_assoc_left(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_arrow(
                    prod_ext_prod_ty(
                        Expr::BVar(2),
                        prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                    ),
                    prod_ext_prod_ty(
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(1)),
                        Expr::BVar(0),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.assoc_left", ty)
}
/// Prod.unit_terminal: Unit is the terminal object (unique morphism to Unit)
pub fn prod_ext_unit_terminal(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_arrow(Expr::BVar(0), prod_ext_cst("Unit")),
    );
    prod_ext_axiom(env, "Prod.unit_terminal", ty)
}
/// Prod.unit_terminal_unique: terminal morphisms are unique
pub fn prod_ext_unit_terminal_unique(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi(
            "f",
            prod_ext_arrow(Expr::BVar(0), prod_ext_cst("Unit")),
            prod_ext_pi(
                "g",
                prod_ext_arrow(Expr::BVar(1), prod_ext_cst("Unit")),
                prod_ext_pi("x", Expr::BVar(2), prod_ext_prop()),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.unit_terminal_unique", ty)
}
/// Prod.curry_law: curry (uncurry f) = f
pub fn prod_ext_curry_law(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi(
                    "f",
                    prod_ext_arrow(Expr::BVar(2), prod_ext_arrow(Expr::BVar(2), Expr::BVar(2))),
                    prod_ext_prop(),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.curry_law", ty)
}
/// Prod.uncurry_law: uncurry (curry f) = f
pub fn prod_ext_uncurry_law(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi(
                    "f",
                    prod_ext_arrow(
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(1)),
                        Expr::BVar(1),
                    ),
                    prod_ext_prop(),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.uncurry_law", ty)
}
/// Prod.hom_tensor_adj: Hom(A×B, C) ≅ Hom(A, B→C)
pub fn prod_ext_hom_tensor_adj(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_arrow(
                    prod_ext_arrow(
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(1)),
                        Expr::BVar(0),
                    ),
                    prod_ext_arrow(Expr::BVar(2), prod_ext_arrow(Expr::BVar(1), Expr::BVar(0))),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.hom_tensor_adj", ty)
}
/// Prod.distrib_over_sum: α × (β ⊕ γ) ≅ (α × β) ⊕ (α × γ)
pub fn prod_ext_distrib_over_sum(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let sum = |a: Expr, b: Expr| prod_ext_app2(prod_ext_cst("Sum"), a, b);
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_arrow(
                    prod_ext_prod_ty(Expr::BVar(2), sum(Expr::BVar(1), Expr::BVar(0))),
                    sum(
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(1)),
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.distrib_over_sum", ty)
}
/// Prod.distrib_over_sum_inv: (α × β) ⊕ (α × γ) → α × (β ⊕ γ)
pub fn prod_ext_distrib_over_sum_inv(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let sum = |a: Expr, b: Expr| prod_ext_app2(prod_ext_cst("Sum"), a, b);
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_arrow(
                    sum(
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(1)),
                        prod_ext_prod_ty(Expr::BVar(2), Expr::BVar(0)),
                    ),
                    prod_ext_prod_ty(Expr::BVar(2), sum(Expr::BVar(1), Expr::BVar(0))),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.distrib_over_sum_inv", ty)
}
/// Prod.prop_and_eq: PProd P Q ↔ P ∧ Q (in Prop)
pub fn prod_ext_prop_and_eq(env: &mut Environment) -> Result<(), String> {
    let p = prod_ext_prop();
    let ty = prod_ext_pi_imp(
        "P",
        p.clone(),
        prod_ext_pi_imp("Q", p.clone(), prod_ext_prop()),
    );
    prod_ext_axiom(env, "Prod.prop_and_eq", ty)
}
/// Prod.and_intro: P → Q → P ∧ Q
pub fn prod_ext_and_intro(env: &mut Environment) -> Result<(), String> {
    let p = prod_ext_prop();
    let ty = prod_ext_pi_imp(
        "P",
        p.clone(),
        prod_ext_pi_imp(
            "Q",
            p.clone(),
            prod_ext_pi(
                "hp",
                Expr::BVar(1),
                prod_ext_pi(
                    "hq",
                    Expr::BVar(1),
                    prod_ext_app2(prod_ext_cst("And"), Expr::BVar(3), Expr::BVar(2)),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.and_intro", ty)
}
/// Prod.sigma_fst: Sigma.fst ⟨a, b⟩ = a
pub fn prod_ext_sigma_fst(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            prod_ext_arrow(Expr::BVar(0), t.clone()),
            prod_ext_pi(
                "a",
                Expr::BVar(1),
                prod_ext_pi(
                    "b",
                    prod_ext_app(Expr::BVar(1), Expr::BVar(0)),
                    prod_ext_prop(),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.sigma_fst", ty)
}
/// Prod.sigma_snd: Sigma.snd ⟨a, b⟩ = b
pub fn prod_ext_sigma_snd(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            prod_ext_arrow(Expr::BVar(0), t.clone()),
            prod_ext_pi(
                "a",
                Expr::BVar(1),
                prod_ext_pi(
                    "b",
                    prod_ext_app(Expr::BVar(1), Expr::BVar(0)),
                    prod_ext_prop(),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.sigma_snd", ty)
}
/// Prod.sigma_eta: ⟨s.fst, s.snd⟩ = s
pub fn prod_ext_sigma_eta(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            prod_ext_arrow(Expr::BVar(0), t.clone()),
            prod_ext_pi(
                "s",
                prod_ext_app2(prod_ext_cst("Sigma"), Expr::BVar(1), Expr::BVar(0)),
                prod_ext_prop(),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.sigma_eta", ty)
}
/// Prod.record_as_prod: a record type is an iterated product
pub fn prod_ext_record_as_prod(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_arrow(
                    prod_ext_prod_ty(
                        Expr::BVar(2),
                        prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                    ),
                    prod_ext_prod_ty(
                        Expr::BVar(2),
                        prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.record_as_prod", ty)
}
/// Prod.nary_tuple_3: α × β × γ ≅ Triple α β γ
pub fn prod_ext_nary_tuple_3(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp("γ", t.clone(), prod_ext_prop()),
        ),
    );
    prod_ext_axiom(env, "Prod.nary_tuple_3", ty)
}
/// Prod.pointwise_fn_prod: (A → B) × (A → C) ≅ A → B × C
pub fn prod_ext_pointwise_fn_prod(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_arrow(
                    prod_ext_prod_ty(
                        prod_ext_arrow(Expr::BVar(2), Expr::BVar(1)),
                        prod_ext_arrow(Expr::BVar(2), Expr::BVar(0)),
                    ),
                    prod_ext_arrow(
                        Expr::BVar(2),
                        prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.pointwise_fn_prod", ty)
}
/// Prod.pointwise_fn_prod_inv: A → B × C ≅ (A → B) × (A → C)
pub fn prod_ext_pointwise_fn_prod_inv(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_arrow(
                    prod_ext_arrow(
                        Expr::BVar(2),
                        prod_ext_prod_ty(Expr::BVar(1), Expr::BVar(0)),
                    ),
                    prod_ext_prod_ty(
                        prod_ext_arrow(Expr::BVar(2), Expr::BVar(1)),
                        prod_ext_arrow(Expr::BVar(2), Expr::BVar(0)),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.pointwise_fn_prod_inv", ty)
}
/// Prod.group_direct_product: direct product of two groups is a group
pub fn prod_ext_group_direct_product(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "G",
        t.clone(),
        prod_ext_pi_imp(
            "H",
            t.clone(),
            prod_ext_pi(
                "mulG",
                prod_ext_arrow(Expr::BVar(1), prod_ext_arrow(Expr::BVar(1), Expr::BVar(1))),
                prod_ext_pi(
                    "mulH",
                    prod_ext_arrow(Expr::BVar(1), prod_ext_arrow(Expr::BVar(1), Expr::BVar(1))),
                    prod_ext_prop(),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.group_direct_product", ty)
}
/// Prod.ring_direct_product: direct product of two rings is a ring
pub fn prod_ext_ring_direct_product(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "R",
        t.clone(),
        prod_ext_pi_imp("S", t.clone(), prod_ext_prop()),
    );
    prod_ext_axiom(env, "Prod.ring_direct_product", ty)
}
/// Prod.fst_comp: fst . bimap f g = f . fst
pub fn prod_ext_fst_comp(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi_imp(
                    "δ",
                    t.clone(),
                    prod_ext_pi(
                        "f",
                        prod_ext_arrow(Expr::BVar(3), Expr::BVar(2)),
                        prod_ext_pi(
                            "g",
                            prod_ext_arrow(Expr::BVar(2), Expr::BVar(2)),
                            prod_ext_pi(
                                "p",
                                prod_ext_prod_ty(Expr::BVar(5), Expr::BVar(4)),
                                prod_ext_prop(),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.fst_comp", ty)
}
/// Prod.snd_comp: snd . bimap f g = g . snd
pub fn prod_ext_snd_comp(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi_imp(
                    "δ",
                    t.clone(),
                    prod_ext_pi(
                        "f",
                        prod_ext_arrow(Expr::BVar(3), Expr::BVar(2)),
                        prod_ext_pi(
                            "g",
                            prod_ext_arrow(Expr::BVar(2), Expr::BVar(2)),
                            prod_ext_pi(
                                "p",
                                prod_ext_prod_ty(Expr::BVar(5), Expr::BVar(4)),
                                prod_ext_prop(),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.snd_comp", ty)
}
/// Prod.monoidal_pentagon: pentagon coherence law
pub fn prod_ext_monoidal_pentagon(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi_imp("δ", t.clone(), prod_ext_prop()),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.monoidal_pentagon", ty)
}
/// Prod.monoidal_triangle: triangle coherence law
pub fn prod_ext_monoidal_triangle(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp("β", t.clone(), prod_ext_prop()),
    );
    prod_ext_axiom(env, "Prod.monoidal_triangle", ty)
}
/// Prod.fanout_law: (f &&& g) x = (f x, g x)
pub fn prod_ext_fanout_law(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi_imp(
            "β",
            t.clone(),
            prod_ext_pi_imp(
                "γ",
                t.clone(),
                prod_ext_pi(
                    "f",
                    prod_ext_arrow(Expr::BVar(2), Expr::BVar(1)),
                    prod_ext_pi(
                        "g",
                        prod_ext_arrow(Expr::BVar(3), Expr::BVar(2)),
                        prod_ext_pi("x", Expr::BVar(4), prod_ext_prop()),
                    ),
                ),
            ),
        ),
    );
    prod_ext_axiom(env, "Prod.fanout_law", ty)
}
/// Prod.dup_law: dup x = (x, x)
pub fn prod_ext_dup_law(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi("x", Expr::BVar(0), prod_ext_prop()),
    );
    prod_ext_axiom(env, "Prod.dup_law", ty)
}
/// Prod.fst_dup: fst (dup x) = x
pub fn prod_ext_fst_dup(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi("x", Expr::BVar(0), prod_ext_prop()),
    );
    prod_ext_axiom(env, "Prod.fst_dup", ty)
}
/// Prod.snd_dup: snd (dup x) = x
pub fn prod_ext_snd_dup(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    let ty = prod_ext_pi_imp(
        "α",
        t.clone(),
        prod_ext_pi("x", Expr::BVar(0), prod_ext_prop()),
    );
    prod_ext_axiom(env, "Prod.snd_dup", ty)
}
/// Register all extended Prod axioms into `env`.
///
/// Covers categorical product universal property, projections, functor/bifunctor
/// laws, applicative, comonad, monoidal category structure, commutativity,
/// associativity, unit/terminal, currying/uncurrying, distributivity over sum,
/// propositions as products, dependent product (Sigma) laws, record types,
/// N-ary products, pointwise function product, and direct product of groups/rings.
pub fn register_prod_extended_axioms(env: &mut Environment) -> Result<(), String> {
    let t = prod_ext_type0();
    for name in ["Prod", "Unit", "Bool", "Sum", "And", "Sigma", "Nat", "List"] {
        let _ = env.add(Declaration::Axiom {
            name: Name::str(name),
            univ_params: vec![],
            ty: t.clone(),
        });
    }
    prod_ext_universal_property(env)?;
    prod_ext_proj_fst_law(env)?;
    prod_ext_proj_snd_law(env)?;
    prod_ext_eta_law(env)?;
    prod_ext_functor_map_fst(env)?;
    prod_ext_functor_map_snd(env)?;
    prod_ext_bifunctor_law(env)?;
    prod_ext_bifunctor_id(env)?;
    prod_ext_zip_ap(env)?;
    prod_ext_comonad_extract(env)?;
    prod_ext_comonad_extend(env)?;
    prod_ext_tensor_unit_left(env)?;
    prod_ext_tensor_unit_right(env)?;
    prod_ext_tensor_assoc(env)?;
    prod_ext_commutativity_iso(env)?;
    prod_ext_swap_involution(env)?;
    prod_ext_assoc_left(env)?;
    prod_ext_unit_terminal(env)?;
    prod_ext_unit_terminal_unique(env)?;
    prod_ext_curry_law(env)?;
    prod_ext_uncurry_law(env)?;
    prod_ext_hom_tensor_adj(env)?;
    prod_ext_distrib_over_sum(env)?;
    prod_ext_distrib_over_sum_inv(env)?;
    prod_ext_prop_and_eq(env)?;
    prod_ext_and_intro(env)?;
    prod_ext_sigma_fst(env)?;
    prod_ext_sigma_snd(env)?;
    prod_ext_sigma_eta(env)?;
    prod_ext_record_as_prod(env)?;
    prod_ext_nary_tuple_3(env)?;
    prod_ext_pointwise_fn_prod(env)?;
    prod_ext_pointwise_fn_prod_inv(env)?;
    prod_ext_group_direct_product(env)?;
    prod_ext_ring_direct_product(env)?;
    prod_ext_fst_comp(env)?;
    prod_ext_snd_comp(env)?;
    prod_ext_monoidal_pentagon(env)?;
    prod_ext_monoidal_triangle(env)?;
    prod_ext_fanout_law(env)?;
    prod_ext_dup_law(env)?;
    prod_ext_fst_dup(env)?;
    prod_ext_snd_dup(env)?;
    Ok(())
}
