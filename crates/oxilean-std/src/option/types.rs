//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// OptionComonad represents a comonadic structure for Option with a fixed default.
#[allow(dead_code)]
pub struct OptionComonad<T: Clone> {
    value: Option<T>,
    default: T,
}
impl<T: Clone> OptionComonad<T> {
    #[allow(dead_code)]
    pub fn new(value: Option<T>, default: T) -> Self {
        Self { value, default }
    }
    /// extract: comonadic extract (returns default if None).
    #[allow(dead_code)]
    pub fn extract(&self) -> T {
        self.value.clone().unwrap_or_else(|| self.default.clone())
    }
    /// extend: comonadic extend.
    #[allow(dead_code)]
    pub fn extend<B: Clone>(&self, f: impl Fn(&OptionComonad<T>) -> B) -> Option<B> {
        Some(f(self))
    }
    #[allow(dead_code)]
    pub fn duplicate_as_pair(&self) -> (T, Option<T>) {
        (self.default.clone(), self.value.clone())
    }
}
/// Memoizing wrapper around a function returning `Option<T>`.
///
/// Caches computed results to avoid recomputing them.
#[allow(dead_code)]
pub struct OptionMemo<K: std::hash::Hash + Eq, V: Clone> {
    cache: std::collections::HashMap<K, Option<V>>,
}
impl<K: std::hash::Hash + Eq, V: Clone> OptionMemo<K, V> {
    /// Create an empty memo table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }
    /// Get the memoized result for `key`, computing it with `f` if necessary.
    #[allow(dead_code)]
    pub fn get_or_compute(&mut self, key: K, f: impl FnOnce(&K) -> Option<V>) -> Option<V>
    where
        K: Clone,
    {
        if let Some(v) = self.cache.get(&key) {
            return v.clone();
        }
        let v = f(&key);
        self.cache.insert(key, v.clone());
        v
    }
    /// Clear the memo table.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Number of memoized entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
/// An iterator that yields the value inside `Some` once, or nothing for `None`.
#[allow(dead_code)]
pub struct OptionIter<T>(pub(super) Option<T>);
impl<T> OptionIter<T> {
    /// Create an iterator from an option.
    #[allow(dead_code)]
    pub fn new(opt: Option<T>) -> Self {
        Self(opt)
    }
}
/// OptionFunctor provides a functor over Option values.
#[allow(dead_code)]
pub struct OptionFunctor<T>(pub Option<T>);
impl<T> OptionFunctor<T> {
    #[allow(dead_code)]
    pub fn new(v: Option<T>) -> Self {
        Self(v)
    }
    #[allow(dead_code)]
    pub fn fmap<U>(self, f: impl FnOnce(T) -> U) -> OptionFunctor<U> {
        OptionFunctor(self.0.map(f))
    }
    #[allow(dead_code)]
    pub fn fmap_id(self) -> OptionFunctor<T> {
        OptionFunctor(self.0)
    }
    #[allow(dead_code)]
    pub fn inner(self) -> Option<T> {
        self.0
    }
}
/// OptionApplicative provides applicative structure over Option values.
#[allow(dead_code)]
pub struct OptionApplicative;
impl OptionApplicative {
    #[allow(dead_code)]
    pub fn pure<T>(v: T) -> Option<T> {
        Some(v)
    }
    #[allow(dead_code)]
    pub fn ap<A, B>(f: Option<impl FnOnce(A) -> B>, a: Option<A>) -> Option<B> {
        match (f, a) {
            (Some(func), Some(val)) => Some(func(val)),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn lift_a2<A, B, C>(f: impl FnOnce(A, B) -> C, a: Option<A>, b: Option<B>) -> Option<C> {
        match (a, b) {
            (Some(x), Some(y)) => Some(f(x, y)),
            _ => None,
        }
    }
}
/// A simple cache mapping keys to optional computed values.
///
/// If a key is present and `Some`, the cached value is returned.
/// If a key is absent, the computation is run and cached.
#[derive(Debug, Clone, Default)]
pub struct OptionCache<K: PartialEq, V> {
    entries: Vec<(K, Option<V>)>,
}
impl<K: PartialEq, V: Clone> OptionCache<K, V> {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Get or compute a value for `key`.
    ///
    /// If the key is already cached, returns the cached value.
    /// Otherwise runs `compute` and caches the result.
    pub fn get_or_insert_with(&mut self, key: K, compute: impl FnOnce() -> Option<V>) -> Option<V>
    where
        K: Clone,
    {
        if let Some(entry) = self.entries.iter().find(|(k, _)| k == &key) {
            return entry.1.clone();
        }
        let value = compute();
        self.entries.push((key, value.clone()));
        value
    }
    /// Explicitly insert a value for a key.
    pub fn insert(&mut self, key: K, value: Option<V>)
    where
        K: Clone,
        V: Clone,
    {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &key) {
            entry.1 = value;
        } else {
            self.entries.push((key, value));
        }
    }
    /// Look up a cached value.
    pub fn get(&self, key: &K) -> Option<Option<&V>> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_ref())
    }
    /// Invalidate (remove) an entry.
    pub fn invalidate(&mut self, key: &K) {
        self.entries.retain(|(k, _)| k != key);
    }
    /// Clear the entire cache.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A builder-style wrapper for chaining option operations.
///
/// Provides a fluent API for building complex option expressions.
#[allow(dead_code)]
pub struct OptionChain<T> {
    value: Option<T>,
}
impl<T> OptionChain<T> {
    /// Start a chain from an option value.
    #[allow(dead_code)]
    pub fn from(opt: Option<T>) -> Self {
        Self { value: opt }
    }
    /// Start a chain from a value (wraps in Some).
    #[allow(dead_code)]
    pub fn of(v: T) -> Self {
        Self { value: Some(v) }
    }
    /// Start an empty chain.
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self { value: None }
    }
    /// Apply a map operation.
    #[allow(dead_code)]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> OptionChain<U> {
        OptionChain {
            value: self.value.map(f),
        }
    }
    /// Apply a flatmap operation.
    #[allow(dead_code)]
    pub fn flat_map<U>(self, f: impl FnOnce(T) -> Option<U>) -> OptionChain<U> {
        OptionChain {
            value: self.value.and_then(f),
        }
    }
    /// Apply a filter operation.
    #[allow(dead_code)]
    pub fn filter(self, pred: impl FnOnce(&T) -> bool) -> Self {
        Self {
            value: self.value.filter(pred),
        }
    }
    /// Provide a fallback value.
    #[allow(dead_code)]
    pub fn or_else(self, fallback: Option<T>) -> Self {
        Self {
            value: self.value.or(fallback),
        }
    }
    /// Extract the final value.
    #[allow(dead_code)]
    pub fn get(self) -> Option<T> {
        self.value
    }
    /// Extract with a default.
    #[allow(dead_code)]
    pub fn get_or(self, default: T) -> T {
        self.value.unwrap_or(default)
    }
    /// Check if this chain has a value.
    #[allow(dead_code)]
    pub fn is_present(&self) -> bool {
        self.value.is_some()
    }
    /// Peek at the inner value without consuming.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&T> {
        self.value.as_ref()
    }
}
/// OptionWriter pairs an Option value with an accumulated log/writer monad value.
#[allow(dead_code)]
pub struct OptionWriter<T, W> {
    value: Option<T>,
    log: W,
}
impl<T, W: Default> OptionWriter<T, W> {
    #[allow(dead_code)]
    pub fn new(value: Option<T>, log: W) -> Self {
        Self { value, log }
    }
    #[allow(dead_code)]
    pub fn pure_some(value: T) -> Self {
        Self {
            value: Some(value),
            log: W::default(),
        }
    }
    #[allow(dead_code)]
    pub fn none_with_log(log: W) -> Self {
        Self { value: None, log }
    }
    #[allow(dead_code)]
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }
}
/// Utilities for converting between Option and Result in a typed fashion.
#[allow(dead_code)]
pub struct OptionResultBridge;
impl OptionResultBridge {
    /// Convert `Option<T>` to `Result<T, E>` with a supplied error.
    #[allow(dead_code)]
    pub fn to_result<T, E>(opt: Option<T>, err: E) -> Result<T, E> {
        opt.ok_or(err)
    }
    /// Convert `Option<T>` to `Result<T, E>` with a lazy error.
    #[allow(dead_code)]
    pub fn to_result_with<T, E>(opt: Option<T>, f: impl FnOnce() -> E) -> Result<T, E> {
        opt.ok_or_else(f)
    }
    /// Convert `Result<T, E>` to `Option<T>`, discarding the error.
    #[allow(dead_code)]
    pub fn from_result_ok<T, E>(r: Result<T, E>) -> Option<T> {
        r.ok()
    }
    /// Convert `Result<T, E>` to `Option<E>`, discarding the success.
    #[allow(dead_code)]
    pub fn from_result_err<T, E>(r: Result<T, E>) -> Option<E> {
        r.err()
    }
    /// Transpose `Result<Option<T>, E>` to `Option<Result<T, E>>`.
    #[allow(dead_code)]
    pub fn transpose_result<T, E>(r: Result<Option<T>, E>) -> Option<Result<T, E>> {
        match r {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
/// OptionProfunctor represents a profunctor-shaped wrapper over Option.
#[allow(dead_code)]
pub struct OptionProfunctor<A, B> {
    run: Box<dyn Fn(A) -> Option<B>>,
}
impl<A: 'static, B: 'static> OptionProfunctor<A, B> {
    #[allow(dead_code)]
    pub fn new(f: impl Fn(A) -> Option<B> + 'static) -> Self {
        Self { run: Box::new(f) }
    }
    #[allow(dead_code)]
    pub fn apply(&self, a: A) -> Option<B> {
        (self.run)(a)
    }
    #[allow(dead_code)]
    pub fn dimap<C: 'static, D: 'static>(
        self,
        pre: impl Fn(C) -> A + 'static,
        post: impl Fn(B) -> D + 'static,
    ) -> OptionProfunctor<C, D> {
        OptionProfunctor::new(move |c| (self.run)(pre(c)).map(|b| post(b)))
    }
}
/// A mapping from keys to optional values.
///
/// Wraps a `Vec<(K, Option<V>)>` and provides convenient access methods.
#[derive(Debug, Clone, Default)]
pub struct OptionMap<K: PartialEq, V> {
    entries: Vec<(K, Option<V>)>,
}
impl<K: PartialEq, V> OptionMap<K, V> {
    /// Create an empty `OptionMap`.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Set the value for `key` (may be `None` to mark as missing).
    pub fn set(&mut self, key: K, value: Option<V>) {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &key) {
            entry.1 = value;
        } else {
            self.entries.push((key, value));
        }
    }
    /// Get the value for `key`, or `None` if not present or explicitly set to `None`.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .and_then(|(_, v)| v.as_ref())
    }
    /// Returns `true` if the key exists (even if mapped to `None`).
    pub fn contains_key(&self, key: &K) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Collect all keys with `Some` values.
    pub fn some_keys(&self) -> Vec<&K> {
        self.entries
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, _)| k)
            .collect()
    }
    /// Collect all keys with `None` values.
    pub fn none_keys(&self) -> Vec<&K> {
        self.entries
            .iter()
            .filter(|(_, v)| v.is_none())
            .map(|(k, _)| k)
            .collect()
    }
    /// Iterate over all entries.
    pub fn iter(&self) -> impl Iterator<Item = &(K, Option<V>)> {
        self.entries.iter()
    }
}
/// A collection of option values with batch operations.
#[allow(dead_code)]
pub struct OptionVec<T> {
    pub(super) items: Vec<Option<T>>,
}
impl<T> OptionVec<T> {
    /// Create a new OptionVec.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Push an option.
    #[allow(dead_code)]
    pub fn push(&mut self, item: Option<T>) {
        self.items.push(item);
    }
    /// Sequence: succeed only if all are Some.
    #[allow(dead_code)]
    pub fn sequence(self) -> Option<Vec<T>> {
        let mut result = Vec::with_capacity(self.items.len());
        for item in self.items {
            result.push(item?);
        }
        Some(result)
    }
    /// Collect only the Some values.
    #[allow(dead_code)]
    pub fn collect_some(self) -> Vec<T> {
        self.items.into_iter().flatten().collect()
    }
    /// Count the Some values.
    #[allow(dead_code)]
    pub fn count_some(&self) -> usize {
        self.items.iter().filter(|o| o.is_some()).count()
    }
    /// Count the None values.
    #[allow(dead_code)]
    pub fn count_none(&self) -> usize {
        self.items.iter().filter(|o| o.is_none()).count()
    }
    /// Total number of items.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// True if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// True if all items are Some.
    #[allow(dead_code)]
    pub fn all_some(&self) -> bool {
        self.items.iter().all(|o| o.is_some())
    }
    /// True if any item is Some.
    #[allow(dead_code)]
    pub fn any_some(&self) -> bool {
        self.items.iter().any(|o| o.is_some())
    }
}
/// An option value tagged with a weight (used for ranked alternatives).
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedOption<T> {
    /// The weight (higher is better).
    pub weight: f64,
    /// The value, if present.
    pub value: Option<T>,
}
impl<T> WeightedOption<T> {
    /// Create a `WeightedOption` with a value.
    pub fn some(weight: f64, value: T) -> Self {
        Self {
            weight,
            value: Some(value),
        }
    }
    /// Create a `WeightedOption` with no value.
    pub fn none(weight: f64) -> Self {
        Self {
            weight,
            value: None,
        }
    }
    /// Select the higher-weight option.
    pub fn better(self, other: Self) -> Self {
        if self.weight >= other.weight {
            self
        } else {
            other
        }
    }
}
