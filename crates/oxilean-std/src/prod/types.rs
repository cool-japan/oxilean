//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
/// CurriedFn represents a curried function A → B → C.
#[allow(dead_code)]
pub struct CurriedFn<A, B, C> {
    f: Box<dyn Fn(A, B) -> C>,
}
impl<A: Clone + 'static, B: 'static, C: 'static> CurriedFn<A, B, C> {
    #[allow(dead_code)]
    pub fn new(f: impl Fn(A, B) -> C + 'static) -> Self {
        Self { f: Box::new(f) }
    }
    #[allow(dead_code)]
    pub fn apply(&self, a: A, b: B) -> C {
        (self.f)(a, b)
    }
    #[allow(dead_code)]
    pub fn curry_apply(&self, a: A) -> impl Fn(B) -> C + '_ {
        move |b| (self.f)(a.clone(), b)
    }
    #[allow(dead_code)]
    pub fn uncurried(&self, pair: (A, B)) -> C {
        (self.f)(pair.0, pair.1)
    }
}
/// A simple association map with ordered insertion semantics.
///
/// Keys are maintained in insertion order; operations are O(n).
/// Suitable for small collections used in elaboration.
#[derive(Debug, Clone, Default)]
pub struct AssocMap<K: PartialEq, V> {
    data: Vec<(K, V)>,
}
impl<K: PartialEq, V> AssocMap<K, V> {
    /// Create an empty `AssocMap`.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    /// Create an `AssocMap` with initial capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
        }
    }
    /// Insert or update a key-value pair.
    pub fn insert(&mut self, key: K, value: V)
    where
        K: Clone,
        V: Clone,
    {
        assoc_insert(&mut self.data, key, value);
    }
    /// Look up a key.
    pub fn get(&self, key: &K) -> Option<&V> {
        assoc_lookup(&self.data, key)
    }
    /// Remove a key, returning its value.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        assoc_remove(&mut self.data, key)
    }
    /// Check whether a key is present.
    pub fn contains_key(&self, key: &K) -> bool {
        assoc_mem(&self.data, key)
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Iterate over key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.data.iter()
    }
    /// Collect all keys.
    pub fn keys(&self) -> Vec<&K> {
        self.data.iter().map(|(k, _)| k).collect()
    }
    /// Collect all values.
    pub fn values(&self) -> Vec<&V> {
        self.data.iter().map(|(_, v)| v).collect()
    }
    /// Map all values in place.
    pub fn map_values<W>(self, f: impl Fn(V) -> W) -> AssocMap<K, W> {
        AssocMap {
            data: self.data.into_iter().map(|(k, v)| (k, f(v))).collect(),
        }
    }
    /// Remove all entries where the predicate on keys returns `false`.
    pub fn retain_keys(&mut self, pred: impl Fn(&K) -> bool) {
        self.data.retain(|(k, _)| pred(k));
    }
}
/// A vector of sigma pairs, grouping witnesses and their proofs.
#[derive(Debug, Clone, Default)]
pub struct SigmaVec<A, B> {
    items: Vec<Sigma<A, B>>,
}
impl<A, B> SigmaVec<A, B> {
    /// Create an empty `SigmaVec`.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Push a new witness-proof pair.
    pub fn push(&mut self, witness: A, proof: B) {
        self.items.push(Sigma::new(witness, proof));
    }
    /// Number of elements.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Get a reference to the sigma pair at `index`.
    pub fn get(&self, index: usize) -> Option<&Sigma<A, B>> {
        self.items.get(index)
    }
    /// Iterate over all sigma pairs.
    pub fn iter(&self) -> impl Iterator<Item = &Sigma<A, B>> {
        self.items.iter()
    }
    /// Collect all witnesses.
    pub fn witnesses(&self) -> Vec<&A> {
        self.items.iter().map(|s| &s.fst).collect()
    }
    /// Collect all proofs.
    pub fn proofs(&self) -> Vec<&B> {
        self.items.iter().map(|s| &s.snd).collect()
    }
}
/// A dependent pair (sigma type) `Σ (a : A), B(a)`.
///
/// At the Rust meta-level, `B` is represented as a plain type `B` rather
/// than a type family, but conceptually this encodes dependent pairs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sigma<A, B> {
    /// The first component (the "witness").
    pub fst: A,
    /// The second component (the "proof" or dependent value).
    pub snd: B,
}
impl<A, B> Sigma<A, B> {
    /// Construct a sigma pair.
    pub fn new(fst: A, snd: B) -> Self {
        Self { fst, snd }
    }
    /// Destruct into components.
    pub fn into_parts(self) -> (A, B) {
        (self.fst, self.snd)
    }
    /// Map the proof component (second element) through `f`.
    pub fn map_snd<C>(self, f: impl FnOnce(B) -> C) -> Sigma<A, C> {
        Sigma::new(self.fst, f(self.snd))
    }
    /// Map the witness (first element) through `f`.
    pub fn map_fst<C>(self, f: impl FnOnce(A) -> C) -> Sigma<C, B> {
        Sigma::new(f(self.fst), self.snd)
    }
    /// Apply a function to both components.
    pub fn elim<C>(self, f: impl FnOnce(A, B) -> C) -> C {
        f(self.fst, self.snd)
    }
}
/// A non-empty pair iterator: iterate over a pair's components.
#[derive(Debug, Clone)]
pub struct PairIter<T> {
    pub(super) first: T,
    pub(super) second: T,
    pub(super) idx: usize,
}
impl<T: Clone> PairIter<T> {
    /// Create an iterator over the two components of a `Pair`.
    pub fn new(fst: T, snd: T) -> Self {
        Self {
            first: fst,
            second: snd,
            idx: 0,
        }
    }
}
/// LexPair represents a lexicographically ordered pair.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexPair<A: Ord, B: Ord> {
    pub fst: A,
    pub snd: B,
}
impl<A: Ord, B: Ord> LexPair<A, B> {
    #[allow(dead_code)]
    pub fn new(fst: A, snd: B) -> Self {
        Self { fst, snd }
    }
}
/// A generic ordered triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Triple<A, B, C> {
    /// First component.
    pub fst: A,
    /// Second component.
    pub snd: B,
    /// Third component.
    pub thd: C,
}
impl<A, B, C> Triple<A, B, C> {
    /// Construct a triple.
    pub fn new(fst: A, snd: B, thd: C) -> Self {
        Self { fst, snd, thd }
    }
    /// Construct from a Rust 3-tuple.
    pub fn from_tuple((a, b, c): (A, B, C)) -> Self {
        Self::new(a, b, c)
    }
    /// Convert to a Rust 3-tuple.
    pub fn into_tuple(self) -> (A, B, C) {
        (self.fst, self.snd, self.thd)
    }
    /// Map all three components.
    pub fn trimap<D, E, F>(
        self,
        f: impl FnOnce(A) -> D,
        g: impl FnOnce(B) -> E,
        h: impl FnOnce(C) -> F,
    ) -> Triple<D, E, F> {
        Triple::new(f(self.fst), g(self.snd), h(self.thd))
    }
    /// Project the first pair.
    pub fn fst_pair(self) -> Pair<A, B> {
        Pair::new(self.fst, self.snd)
    }
    /// Project the last pair.
    pub fn snd_pair(self) -> Pair<B, C> {
        Pair::new(self.snd, self.thd)
    }
}
/// ProdBimap applies two functions to the two components of a product simultaneously.
#[allow(dead_code)]
pub struct ProdBimap<A, B, C, D> {
    pub f: Box<dyn Fn(A) -> C>,
    pub g: Box<dyn Fn(B) -> D>,
}
impl<A: 'static, B: 'static, C: 'static, D: 'static> ProdBimap<A, B, C, D> {
    #[allow(dead_code)]
    pub fn new(f: impl Fn(A) -> C + 'static, g: impl Fn(B) -> D + 'static) -> Self {
        Self {
            f: Box::new(f),
            g: Box::new(g),
        }
    }
    #[allow(dead_code)]
    pub fn apply(&self, pair: (A, B)) -> (C, D) {
        ((self.f)(pair.0), (self.g)(pair.1))
    }
}
/// ProdCone encodes the cone of a product diagram: morphisms into both projections.
#[allow(dead_code)]
pub struct ProdCone<A, B, Z> {
    /// Projection to first component.
    pub proj1: Box<dyn Fn(Z) -> A>,
    /// Projection to second component.
    pub proj2: Box<dyn Fn(Z) -> B>,
}
impl<A: 'static, B: 'static, Z: Clone + 'static> ProdCone<A, B, Z> {
    #[allow(dead_code)]
    pub fn new(p1: impl Fn(Z) -> A + 'static, p2: impl Fn(Z) -> B + 'static) -> Self {
        Self {
            proj1: Box::new(p1),
            proj2: Box::new(p2),
        }
    }
    /// Universal mediating morphism into the product.
    #[allow(dead_code)]
    pub fn mediate(&self, z: Z) -> (A, B) {
        let z2 = z.clone();
        ((self.proj1)(z), (self.proj2)(z2))
    }
}
/// A generic ordered pair.
///
/// This is the Rust meta-level analogue of `Prod` in Lean 4.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Pair<A, B> {
    /// First component.
    pub fst: A,
    /// Second component.
    pub snd: B,
}
impl<A, B> Pair<A, B> {
    /// Construct a new `Pair`.
    pub fn new(fst: A, snd: B) -> Self {
        Self { fst, snd }
    }
    /// Construct from a Rust tuple.
    pub fn from_tuple((a, b): (A, B)) -> Self {
        Self::new(a, b)
    }
    /// Convert to a Rust tuple.
    pub fn into_tuple(self) -> (A, B) {
        (self.fst, self.snd)
    }
    /// Map the first component.
    pub fn map_fst<C>(self, f: impl FnOnce(A) -> C) -> Pair<C, B> {
        Pair::new(f(self.fst), self.snd)
    }
    /// Map the second component.
    pub fn map_snd<C>(self, f: impl FnOnce(B) -> C) -> Pair<A, C> {
        Pair::new(self.fst, f(self.snd))
    }
    /// Map both components.
    pub fn bimap<C, D>(self, f: impl FnOnce(A) -> C, g: impl FnOnce(B) -> D) -> Pair<C, D> {
        Pair::new(f(self.fst), g(self.snd))
    }
    /// Swap the components.
    pub fn swap(self) -> Pair<B, A> {
        Pair::new(self.snd, self.fst)
    }
    /// Apply a function expecting two arguments.
    pub fn uncurry<C>(self, f: impl FnOnce(A, B) -> C) -> C {
        f(self.fst, self.snd)
    }
    /// Get references to both components.
    pub fn as_refs(&self) -> (&A, &B) {
        (&self.fst, &self.snd)
    }
}
impl<T: Clone> Pair<T, T> {
    /// Iterate over the two homogeneous components.
    pub fn iter(&self) -> PairIter<T> {
        PairIter::new(self.fst.clone(), self.snd.clone())
    }
}
/// AssocTriple provides the associativity isomorphism for products.
#[allow(dead_code)]
pub struct AssocTriple<A, B, C> {
    pub value: ((A, B), C),
}
impl<A, B, C> AssocTriple<A, B, C> {
    #[allow(dead_code)]
    pub fn new(value: ((A, B), C)) -> Self {
        Self { value }
    }
    /// Associate right: ((a, b), c) → (a, (b, c))
    #[allow(dead_code)]
    pub fn assoc_right(self) -> (A, (B, C)) {
        let ((a, b), c) = self.value;
        (a, (b, c))
    }
    #[allow(dead_code)]
    pub fn from_right(triple: (A, (B, C))) -> Self {
        let (a, (b, c)) = triple;
        Self { value: ((a, b), c) }
    }
}
