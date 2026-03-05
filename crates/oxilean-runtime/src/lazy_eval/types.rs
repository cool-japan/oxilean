//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, Mutex, OnceLock};

use super::functions::TailFn;

/// A simple black-hole detector for named computations.
///
/// Records which thunks are currently being forced. If a thunk is
/// re-entered while already being forced, a black hole is detected.
#[derive(Default, Debug)]
pub struct BlackHoleDetector {
    in_progress: std::collections::HashSet<String>,
}
impl BlackHoleDetector {
    /// Create a new detector.
    pub fn new() -> Self {
        BlackHoleDetector::default()
    }
    /// Enter a thunk named `name`. Returns `Err` if a black hole is detected.
    pub fn enter(&mut self, name: impl Into<String>) -> Result<(), String> {
        let n = name.into();
        if self.in_progress.contains(&n) {
            return Err(format!("black hole detected in `{}`", n));
        }
        self.in_progress.insert(n);
        Ok(())
    }
    /// Exit a thunk named `name`.
    pub fn exit(&mut self, name: &str) {
        self.in_progress.remove(name);
    }
    /// Whether a given thunk is currently being forced.
    pub fn is_in_progress(&self, name: &str) -> bool {
        self.in_progress.contains(name)
    }
    /// Number of thunks currently being forced.
    pub fn depth(&self) -> usize {
        self.in_progress.len()
    }
}
/// A lazy map where each value is computed on first access and memoized.
pub struct LazyMap<K: Eq + std::hash::Hash + Clone, V> {
    computed: HashMap<K, V>,
    pending: HashMap<K, Box<dyn FnOnce() -> V>>,
}
impl<K: Eq + std::hash::Hash + Clone, V: Clone + fmt::Debug> LazyMap<K, V> {
    /// Create a new empty lazy map.
    pub fn new() -> Self {
        LazyMap {
            computed: HashMap::new(),
            pending: HashMap::new(),
        }
    }
    /// Insert a lazy value under `key`.
    pub fn insert_lazy<F>(&mut self, key: K, f: F)
    where
        F: FnOnce() -> V + 'static,
    {
        self.pending.insert(key, Box::new(f));
    }
    /// Insert an already-computed value.
    pub fn insert_ready(&mut self, key: K, value: V) {
        self.computed.insert(key, value);
    }
    /// Force and return the value for `key`, if it exists.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(f) = self.pending.remove(key) {
            let val = f();
            self.computed.insert(key.clone(), val);
        }
        self.computed.get(key)
    }
    /// Whether a value (pending or computed) exists under `key`.
    pub fn contains(&self, key: &K) -> bool {
        self.computed.contains_key(key) || self.pending.contains_key(key)
    }
    /// Number of entries (both pending and computed).
    pub fn len(&self) -> usize {
        self.computed.len() + self.pending.len()
    }
    /// Whether the map is empty.
    pub fn is_empty(&self) -> bool {
        self.computed.is_empty() && self.pending.is_empty()
    }
    /// Number of entries that have been computed.
    pub fn computed_count(&self) -> usize {
        self.computed.len()
    }
    /// Number of entries still pending.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}
/// Generates prime numbers lazily using a stream-based sieve.
pub struct LazySieve {
    primes: Vec<u64>,
    candidate: u64,
}
impl LazySieve {
    /// Create a new lazy prime sieve starting from 2.
    pub fn new() -> Self {
        LazySieve {
            primes: Vec::new(),
            candidate: 2,
        }
    }
    /// Get the next prime number.
    pub fn next_prime(&mut self) -> u64 {
        loop {
            let c = self.candidate;
            self.candidate += 1;
            let is_prime = self.primes.iter().all(|&p| c % p != 0);
            if is_prime {
                self.primes.push(c);
                return c;
            }
        }
    }
    /// Get the first `n` prime numbers.
    pub fn take_primes(&mut self, n: usize) -> Vec<u64> {
        (0..n).map(|_| self.next_prime()).collect()
    }
}
/// A simple memoization cache keyed by u64 hash.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MemoCache<V> {
    entries: std::collections::HashMap<u64, V>,
    pub(crate) hits: usize,
    pub(crate) misses: usize,
}
#[allow(dead_code)]
impl<V: Clone> MemoCache<V> {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    pub fn get(&mut self, key: u64) -> Option<&V> {
        if self.entries.contains_key(&key) {
            self.hits += 1;
            self.entries.get(&key)
        } else {
            self.misses += 1;
            None
        }
    }
    pub fn insert(&mut self, key: u64, value: V) {
        self.entries.insert(key, value);
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }
}
/// Internal state of a single-threaded thunk.
pub(super) enum ThunkState<T> {
    /// Not yet evaluated; holds the suspended computation.
    Unevaluated(Box<dyn FnOnce() -> T>),
    /// Already evaluated; holds the memoized result.
    Evaluated(T),
    /// Evaluation is in progress (detects infinite loops / black holes).
    BlackHole,
}
/// An accumulator that collects values lazily (only when iterated).
pub struct LazyAccumulator<T> {
    pending: Vec<Box<dyn FnOnce() -> T>>,
    collected: Vec<T>,
}
impl<T: Clone + fmt::Debug> LazyAccumulator<T> {
    /// Create a new empty accumulator.
    pub fn new() -> Self {
        LazyAccumulator {
            pending: Vec::new(),
            collected: Vec::new(),
        }
    }
    /// Add a lazy value.
    pub fn add<F: FnOnce() -> T + 'static>(&mut self, f: F) {
        self.pending.push(Box::new(f));
    }
    /// Force all pending values.
    pub fn flush(&mut self) {
        while let Some(f) = self.pending.pop() {
            self.collected.push(f());
        }
    }
    /// Get all collected values (after flushing).
    pub fn collect(&mut self) -> &[T] {
        self.flush();
        &self.collected
    }
    /// Number of pending items.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Number of collected items.
    pub fn collected_count(&self) -> usize {
        self.collected.len()
    }
}
/// An infinite stream backed by a generating function.
///
/// The generator is called with a mutable state value to produce
/// the next element.
pub struct StreamThunk<S: Clone, T> {
    state: S,
    gen: Box<dyn Fn(&S) -> (T, S)>,
}
impl<S: Clone + fmt::Debug, T: Clone + fmt::Debug> StreamThunk<S, T> {
    /// Create a new stream with initial state `s` and generator `f`.
    ///
    /// `f(state)` returns `(next_value, next_state)`.
    pub fn new<F>(init: S, f: F) -> Self
    where
        F: Fn(&S) -> (T, S) + 'static,
    {
        StreamThunk {
            state: init,
            gen: Box::new(f),
        }
    }
    /// Produce the next value and advance the stream.
    pub fn next(&mut self) -> T {
        let (val, new_state) = (self.gen)(&self.state);
        self.state = new_state;
        val
    }
    /// Take `n` elements and return them as a vector.
    pub fn take_n(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
    /// Peek at the current state without advancing.
    pub fn current_state(&self) -> &S {
        &self.state
    }
}
/// A value that may be either:
/// - Already computed (`Ready`),
/// - Deferred to be computed later (`Deferred` with a name tag),
/// - Failed to compute (`Failed` with an error message).
#[derive(Clone, Debug)]
pub enum DeferredValue<T> {
    /// The value is ready.
    Ready(T),
    /// The value is not yet computed; `name` identifies the computation.
    Deferred { name: String },
    /// Computation failed with this error message.
    Failed { error: String },
}
impl<T: Clone + fmt::Debug> DeferredValue<T> {
    /// Unwrap a ready value, panicking if not ready.
    pub fn unwrap_ready(self) -> T {
        match self {
            DeferredValue::Ready(v) => v,
            DeferredValue::Deferred { name } => {
                panic!(
                    "DeferredValue::unwrap_ready: value '{}' not yet computed",
                    name
                )
            }
            DeferredValue::Failed { error } => {
                panic!("DeferredValue::unwrap_ready: computation failed: {}", error)
            }
        }
    }
    /// Whether the value is ready.
    pub fn is_ready(&self) -> bool {
        matches!(self, DeferredValue::Ready(_))
    }
    /// Whether the value is still deferred.
    pub fn is_deferred(&self) -> bool {
        matches!(self, DeferredValue::Deferred { .. })
    }
    /// Whether the computation failed.
    pub fn is_failed(&self) -> bool {
        matches!(self, DeferredValue::Failed { .. })
    }
    /// Map over a ready value.
    pub fn map<U, F>(self, f: F) -> DeferredValue<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            DeferredValue::Ready(v) => DeferredValue::Ready(f(v)),
            DeferredValue::Deferred { name } => DeferredValue::Deferred { name },
            DeferredValue::Failed { error } => DeferredValue::Failed { error },
        }
    }
    /// Get the ready value, if any.
    pub fn get(&self) -> Option<&T> {
        match self {
            DeferredValue::Ready(v) => Some(v),
            _ => None,
        }
    }
}
/// A lazy iterator over an arithmetic range [start, end) with a step.
pub struct LazyRange {
    pub(super) current: i64,
    pub(super) end: i64,
    pub(super) step: i64,
}
impl LazyRange {
    /// Create a range [start..end) with the given step.
    pub fn new(start: i64, end: i64, step: i64) -> Self {
        assert!(step != 0, "step must be non-zero");
        LazyRange {
            current: start,
            end,
            step,
        }
    }
    /// Create a simple range [0..n).
    pub fn up_to(n: i64) -> Self {
        Self::new(0, n, 1)
    }
    /// Collect all values.
    pub fn collect_all(self) -> Vec<i64> {
        self.collect()
    }
}
/// A named cache of lazy values.
///
/// Useful for memoizing top-level definitions that should only be evaluated
/// once during a program run.
pub struct ThunkCache {
    pub(super) entries: HashMap<String, Arc<dyn std::any::Any + Send + Sync>>,
}
impl ThunkCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        ThunkCache {
            entries: HashMap::new(),
        }
    }
    /// Insert a lazily-computed entry under `name`.
    ///
    /// If `name` is already in the cache the old entry is replaced.
    pub fn insert<T, F>(&mut self, name: impl Into<String>, f: F)
    where
        T: Clone + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        let thunk = Arc::new(SharedThunk::new(f));
        self.entries.insert(name.into(), thunk);
    }
    /// Insert a pre-evaluated value under `name`.
    pub fn insert_pure<T>(&mut self, name: impl Into<String>, value: T)
    where
        T: Clone + fmt::Debug + Send + Sync + 'static,
    {
        let thunk = Arc::new(SharedThunk::pure(value));
        self.entries.insert(name.into(), thunk);
    }
    /// Force and return the value under `name`, if it exists.
    pub fn force<T>(&self, name: &str) -> Option<T>
    where
        T: Clone + fmt::Debug + Send + Sync + 'static,
    {
        let entry = self.entries.get(name)?;
        let thunk = entry.downcast_ref::<SharedThunk<T>>()?;
        Some(thunk.force())
    }
    /// Returns `true` if an entry with `name` exists in the cache.
    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }
    /// Number of entries in the cache.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Iterator produced by [`LazyList::take`].
pub struct TakeIter<T: Clone + 'static> {
    pub(super) current: Option<(T, Option<TailFn<T>>)>,
    pub(super) remaining: usize,
}
/// A chain of lazy computations that can be built up and then forced.
///
/// Each step maps the previous value to the next. This is a simplified
/// synchronous "future" / continuation chain.
pub struct FutureChain<T> {
    /// The initial computation.
    initial: Box<dyn FnOnce() -> T>,
    /// The chain of transformations.
    steps: Vec<Box<dyn FnOnce(T) -> T>>,
}
impl<T: 'static> FutureChain<T> {
    /// Create a chain starting from `init`.
    pub fn new<F: FnOnce() -> T + 'static>(init: F) -> Self {
        FutureChain {
            initial: Box::new(init),
            steps: Vec::new(),
        }
    }
    /// Add a transformation step.
    pub fn then<F: FnOnce(T) -> T + 'static>(mut self, f: F) -> Self {
        self.steps.push(Box::new(f));
        self
    }
    /// Force the chain, running all computations in order.
    pub fn force(self) -> T {
        let mut val = (self.initial)();
        for step in self.steps {
            val = step(val);
        }
        val
    }
    /// Number of steps in the chain.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}
/// A batch evaluator that runs a collection of named thunks in sequence.
pub struct BatchEval<T> {
    tasks: Vec<(String, Box<dyn FnOnce() -> T>)>,
    results: Vec<(String, T)>,
}
impl<T: Clone + fmt::Debug> BatchEval<T> {
    /// Create a new empty batch.
    pub fn new() -> Self {
        BatchEval {
            tasks: Vec::new(),
            results: Vec::new(),
        }
    }
    /// Add a named task to the batch.
    pub fn add<F: FnOnce() -> T + 'static>(&mut self, name: impl Into<String>, f: F) {
        self.tasks.push((name.into(), Box::new(f)));
    }
    /// Run all tasks and collect results.
    pub fn run_all(&mut self) {
        while let Some((name, f)) = self.tasks.pop() {
            let val = f();
            self.results.push((name, val));
        }
    }
    /// Get the result for a given name, if it was run.
    pub fn result(&self, name: &str) -> Option<&T> {
        self.results.iter().find(|(n, _)| n == name).map(|(_, v)| v)
    }
    /// Number of tasks pending.
    pub fn pending(&self) -> usize {
        self.tasks.len()
    }
    /// Number of completed tasks.
    pub fn completed(&self) -> usize {
        self.results.len()
    }
    /// All results.
    pub fn all_results(&self) -> &[(String, T)] {
        &self.results
    }
}
/// Lazy once-cell that tracks how many times it was forced.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct TrackedCell<T> {
    value: Option<T>,
    force_count: usize,
}
#[allow(dead_code)]
impl<T: Clone> TrackedCell<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            force_count: 0,
        }
    }
    pub fn set(&mut self, val: T) {
        if self.value.is_none() {
            self.value = Some(val);
        }
    }
    pub fn get(&mut self) -> Option<&T> {
        if self.value.is_some() {
            self.force_count += 1;
        }
        self.value.as_ref()
    }
    pub fn force_count(&self) -> usize {
        self.force_count
    }
    pub fn is_initialized(&self) -> bool {
        self.value.is_some()
    }
}
/// Memoize a two-argument function over a HashMap.
pub struct MemoFn2<I1, I2, O> {
    cache: HashMap<(I1, I2), O>,
    func: Box<dyn Fn(I1, I2) -> O>,
}
impl<I1, I2, O> MemoFn2<I1, I2, O>
where
    I1: Eq + std::hash::Hash + Clone,
    I2: Eq + std::hash::Hash + Clone,
    O: Clone,
{
    /// Create a memoized 2-argument function.
    pub fn new<F: Fn(I1, I2) -> O + 'static>(f: F) -> Self {
        MemoFn2 {
            cache: HashMap::new(),
            func: Box::new(f),
        }
    }
    /// Call the memoized function.
    pub fn call(&mut self, a: I1, b: I2) -> O {
        let key = (a.clone(), b.clone());
        if let Some(v) = self.cache.get(&key) {
            return v.clone();
        }
        let result = (self.func)(a, b);
        self.cache.insert(key, result.clone());
        result
    }
    /// Number of cached results.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
    /// Clear the cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
/// A vector where elements are computed lazily on first access.
pub struct ThunkVec<T> {
    thunks: Vec<std::cell::RefCell<ThunkVecState<T>>>,
}
impl<T: Clone + fmt::Debug> ThunkVec<T> {
    /// Create a new empty ThunkVec.
    pub fn new() -> Self {
        ThunkVec { thunks: Vec::new() }
    }
    /// Push a lazy element.
    pub fn push_lazy<F: FnOnce() -> T + 'static>(&mut self, f: F) {
        self.thunks
            .push(std::cell::RefCell::new(ThunkVecState::Pending(Box::new(f))));
    }
    /// Push an already-computed element.
    pub fn push_ready(&mut self, value: T) {
        self.thunks
            .push(std::cell::RefCell::new(ThunkVecState::Computed(value)));
    }
    /// Get the element at `index`, forcing it if necessary.
    pub fn get(&self, index: usize) -> Option<T> {
        let cell = self.thunks.get(index)?;
        let state = cell.borrow();
        if let ThunkVecState::Computed(ref v) = *state {
            return Some(v.clone());
        }
        drop(state);
        let old = cell.replace(ThunkVecState::Computed(
            match cell.replace(ThunkVecState::Computed(unsafe { std::mem::zeroed() })) {
                ThunkVecState::Pending(f) => f(),
                ThunkVecState::Computed(v) => v,
            },
        ));
        drop(old);
        let state2 = cell.borrow();
        if let ThunkVecState::Computed(ref v) = *state2 {
            Some(v.clone())
        } else {
            None
        }
    }
    /// Length of the vec.
    pub fn len(&self) -> usize {
        self.thunks.len()
    }
    /// Whether the vec is empty.
    pub fn is_empty(&self) -> bool {
        self.thunks.is_empty()
    }
    /// Force all thunks and return a Vec.
    pub fn force_all(&self) -> Vec<T> {
        (0..self.len()).filter_map(|i| self.get(i)).collect()
    }
}
/// A memoizing evaluator that safely handles mutual recursion by returning
/// a fallback value when a cycle is detected (instead of panicking).
pub struct CycleSafeMemo<T: Clone + Default> {
    cache: HashMap<String, T>,
    in_progress: std::collections::HashSet<String>,
}
impl<T: Clone + Default + fmt::Debug> CycleSafeMemo<T> {
    /// Create a new cycle-safe memo.
    pub fn new() -> Self {
        CycleSafeMemo {
            cache: HashMap::new(),
            in_progress: std::collections::HashSet::new(),
        }
    }
    /// Compute or retrieve the value for `key`. If a cycle is detected,
    /// returns `T::default()` as a fallback.
    pub fn get_or_compute<F>(&mut self, key: impl Into<String>, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let k = key.into();
        if let Some(v) = self.cache.get(&k) {
            return v.clone();
        }
        if self.in_progress.contains(&k) {
            return T::default();
        }
        self.in_progress.insert(k.clone());
        let result = f(self);
        self.in_progress.remove(&k);
        self.cache.insert(k, result.clone());
        result
    }
    /// Number of cached entries.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}
/// Thread-safe lazy value backed by `Arc<Mutex<...>>`.
///
/// Use this when a thunk needs to be sent across threads (the closure and
/// the memoized value must both be `Send + Sync`).
pub struct SharedThunk<T> {
    pub(super) inner: Arc<Mutex<SharedThunkInner<T>>>,
}
impl<T: Clone + fmt::Debug + Send + Sync + 'static> SharedThunk<T> {
    /// Create a new thread-safe thunk.
    pub fn new<F: FnOnce() -> T + Send + 'static>(f: F) -> Self {
        SharedThunk {
            inner: Arc::new(Mutex::new(SharedThunkInner {
                value: OnceLock::new(),
                thunk: Some(Box::new(f)),
            })),
        }
    }
    /// Create an already-evaluated shared thunk.
    pub fn pure(value: T) -> Self {
        let lock = OnceLock::new();
        let _ = lock.set(value);
        SharedThunk {
            inner: Arc::new(Mutex::new(SharedThunkInner {
                value: lock,
                thunk: None,
            })),
        }
    }
    /// Force the thunk (thread-safe, memoized).
    pub fn force(&self) -> T {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = guard.value.get() {
            return v.clone();
        }
        let f = guard.thunk.take().expect("thunk already consumed");
        let result = f();
        let _ = guard.value.set(result.clone());
        result
    }
    /// Returns `true` if already evaluated.
    pub fn is_evaluated(&self) -> bool {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.value.get().is_some()
    }
    /// Clone the `Arc` handle, giving a second handle to the *same* thunk.
    pub fn share(&self) -> Self {
        SharedThunk {
            inner: Arc::clone(&self.inner),
        }
    }
}
/// Internal state for a single thunk in a ThunkVec.
enum ThunkVecState<T> {
    Pending(Box<dyn FnOnce() -> T>),
    Computed(T),
}
/// A rose tree node with lazily-computed children.
pub struct LazyTree<T: Clone + fmt::Debug + 'static> {
    /// Value at this node.
    pub value: T,
    /// Lazily-computed children.
    children_thunk: Option<Arc<dyn Fn() -> Vec<LazyTree<T>> + Send + Sync>>,
}
impl<T: Clone + fmt::Debug + 'static> LazyTree<T> {
    /// Create a leaf node.
    pub fn leaf(value: T) -> Self {
        LazyTree {
            value,
            children_thunk: None,
        }
    }
    /// Create an internal node with lazily-computed children.
    pub fn node<F>(value: T, children: F) -> Self
    where
        F: Fn() -> Vec<LazyTree<T>> + Send + Sync + 'static,
    {
        LazyTree {
            value,
            children_thunk: Some(Arc::new(children)),
        }
    }
    /// Force and return the children.
    pub fn children(&self) -> Vec<LazyTree<T>> {
        match &self.children_thunk {
            None => Vec::new(),
            Some(f) => f(),
        }
    }
    /// Whether this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.children_thunk.is_none()
    }
    /// Depth-first traversal, collecting values.
    pub fn dfs(&self) -> Vec<T> {
        let mut result = vec![self.value.clone()];
        for child in self.children() {
            result.extend(child.dfs());
        }
        result
    }
}
/// A memoization table that stores computed results keyed by a string.
pub struct MemoTable {
    pub(super) entries: HashMap<String, Box<dyn std::any::Any>>,
}
impl MemoTable {
    /// Create a new empty table.
    pub fn new() -> Self {
        MemoTable {
            entries: HashMap::new(),
        }
    }
    /// Insert a precomputed value.
    pub fn insert<V: 'static>(&mut self, key: impl Into<String>, value: V) {
        self.entries.insert(key.into(), Box::new(value));
    }
    /// Get a value by key and type.
    pub fn get<V: 'static>(&self, key: &str) -> Option<&V> {
        self.entries.get(key)?.downcast_ref::<V>()
    }
    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }
    /// Remove a key.
    pub fn remove(&mut self, key: &str) -> bool {
        self.entries.remove(key).is_some()
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// A potentially-infinite list whose tail is lazy.
///
/// ```
/// # use oxilean_runtime::lazy_eval::LazyList;
/// let nats = LazyList::from_fn(0u64, |n| n + 1);
/// let first5: Vec<u64> = nats.take(5).collect();
/// assert_eq!(first5, vec![0, 1, 2, 3, 4]);
/// ```
pub struct LazyList<T: Clone + 'static> {
    pub(super) head: Option<T>,
    pub(super) tail: Option<Arc<dyn Fn() -> LazyList<T> + Send + Sync>>,
}
impl<T: Clone + fmt::Debug + 'static> LazyList<T> {
    /// Create an empty lazy list.
    pub fn empty() -> Self {
        LazyList {
            head: None,
            tail: None,
        }
    }
    /// Create a cons cell.
    pub fn cons<F>(head: T, tail: F) -> Self
    where
        F: Fn() -> LazyList<T> + Send + Sync + 'static,
    {
        LazyList {
            head: Some(head),
            tail: Some(Arc::new(tail)),
        }
    }
    /// Build an infinite list using a seed value and a successor function.
    pub fn from_fn(seed: T, next: impl Fn(T) -> T + Send + Sync + Clone + 'static) -> Self
    where
        T: Send + Sync,
    {
        let next2 = next.clone();
        let seed2 = next(seed.clone());
        LazyList::cons(seed, move || {
            LazyList::from_fn(seed2.clone(), next2.clone())
        })
    }
    /// Whether this list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /// Force the head element.
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref()
    }
    /// Force the tail, returning the next lazy list.
    pub fn tail(&self) -> LazyList<T> {
        match &self.tail {
            Some(f) => f(),
            None => LazyList::empty(),
        }
    }
    /// Take (at most) the first `n` elements, returning an iterator.
    pub fn take(&self, n: usize) -> TakeIter<T> {
        TakeIter {
            current: self.head.clone().map(|h| {
                let tail = self.tail.clone();
                (h, tail)
            }),
            remaining: n,
        }
    }
}
/// Lazy filter adapter.
pub struct LazyFilter<'a, A> {
    pub(super) data: &'a [A],
    pub(super) pred: Box<dyn Fn(&A) -> bool>,
    pub(super) index: usize,
}
impl<'a, A: Clone + fmt::Debug> LazyFilter<'a, A> {
    /// Create a new lazy filter.
    pub fn new<F: Fn(&A) -> bool + 'static>(data: &'a [A], pred: F) -> Self {
        LazyFilter {
            data,
            pred: Box::new(pred),
            index: 0,
        }
    }
    /// Collect all passing elements.
    pub fn collect_all(&self) -> Vec<A> {
        self.data
            .iter()
            .filter(|x| (self.pred)(x))
            .cloned()
            .collect()
    }
}
/// A value that is initialized at most once, returning a reference
/// on first access.
///
/// Unlike [`Thunk`], this variant uses an `Option` and never panics on
/// black-hole — it returns `None` if not yet initialized.
pub struct LazyCell<T> {
    pub(super) inner: std::cell::OnceCell<T>,
}
impl<T: Clone + fmt::Debug> LazyCell<T> {
    /// Create a new uninitialized lazy cell.
    pub fn new() -> Self {
        LazyCell {
            inner: std::cell::OnceCell::new(),
        }
    }
    /// Initialize the cell with `value`. Returns `Ok(())` on success
    /// or `Err(value)` if already initialized.
    pub fn init(&self, value: T) -> Result<(), T> {
        self.inner.set(value)
    }
    /// Get the value if initialized.
    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    }
    /// Get or initialize with `f`.
    pub fn get_or_init<F: FnOnce() -> T>(&self, f: F) -> &T {
        self.inner.get_or_init(f)
    }
    /// Whether this cell has been initialized.
    pub fn is_initialized(&self) -> bool {
        self.inner.get().is_some()
    }
}
/// A lazy string that is only concatenated when needed.
pub struct LazyString {
    parts: Vec<String>,
}
impl LazyString {
    /// Create a new empty lazy string.
    pub fn new() -> Self {
        LazyString { parts: Vec::new() }
    }
    /// Append a string part.
    pub fn push(mut self, s: impl Into<String>) -> Self {
        self.parts.push(s.into());
        self
    }
    /// Force the string, concatenating all parts.
    pub fn build(self) -> String {
        self.parts.concat()
    }
    /// Number of parts.
    pub fn part_count(&self) -> usize {
        self.parts.len()
    }
}
/// A thread-safe value computed at most once.
pub struct Once<T: Clone + Send + Sync + 'static> {
    pub(super) inner: OnceLock<T>,
}
impl<T: Clone + fmt::Debug + Send + Sync + 'static> Once<T> {
    /// Create a new uninitialized Once.
    pub fn new() -> Self {
        Once {
            inner: OnceLock::new(),
        }
    }
    /// Get or initialize the value.
    pub fn get_or_init<F: FnOnce() -> T>(&self, f: F) -> &T {
        self.inner.get_or_init(f)
    }
    /// Get the value if initialized.
    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    }
    /// Whether initialized.
    pub fn is_initialized(&self) -> bool {
        self.inner.get().is_some()
    }
}
/// A memoized function that caches all previous results.
///
/// ```
/// # use oxilean_runtime::lazy_eval::MemoFn;
/// let mut fib = MemoFn::new(|n: u64| {
///     if n <= 1 { n } else { n } // simplified; see tests for recursive version
/// });
/// assert_eq!(fib.call(10), 10);
/// assert_eq!(fib.call(10), 10); // from cache
/// ```
pub struct MemoFn<I, O> {
    cache: HashMap<I, O>,
    func: Box<dyn Fn(I) -> O>,
}
impl<I: Eq + std::hash::Hash + Clone, O: Clone> MemoFn<I, O> {
    /// Wrap `f` in a memoized function.
    pub fn new<F: Fn(I) -> O + 'static>(f: F) -> Self {
        MemoFn {
            cache: HashMap::new(),
            func: Box::new(f),
        }
    }
    /// Call the memoized function; uses cached result if available.
    pub fn call(&mut self, arg: I) -> O {
        if let Some(v) = self.cache.get(&arg) {
            return v.clone();
        }
        let result = (self.func)(arg.clone());
        self.cache.insert(arg, result.clone());
        result
    }
    /// Clear the memoization cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    /// Number of cached entries.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}
/// A single-threaded lazy value with memoization.
///
/// ```
/// # use oxilean_runtime::lazy_eval::Thunk;
/// let thunk = Thunk::new(|| 6 * 7);
/// assert_eq!(*thunk.force(), 42);
/// // Subsequent forces return the cached value without recomputing.
/// assert_eq!(*thunk.force(), 42);
/// ```
pub struct Thunk<T> {
    pub(super) state: RefCell<ThunkState<T>>,
}
impl<T: Clone + fmt::Debug> Thunk<T> {
    /// Create a new unevaluated thunk wrapping `f`.
    pub fn new<F: FnOnce() -> T + 'static>(f: F) -> Self {
        Thunk {
            state: RefCell::new(ThunkState::Unevaluated(Box::new(f))),
        }
    }
    /// Create an already-evaluated thunk (no suspension needed).
    pub fn pure(value: T) -> Self {
        Thunk {
            state: RefCell::new(ThunkState::Evaluated(value)),
        }
    }
    /// Force the thunk, memoizing the result.
    ///
    /// # Panics
    ///
    /// Panics if a cycle is detected (black-hole detection).
    pub fn force(&self) -> std::cell::Ref<'_, T> {
        {
            let s = self.state.borrow();
            if matches!(*s, ThunkState::Evaluated(_)) {
                return std::cell::Ref::map(s, |state| {
                    if let ThunkState::Evaluated(ref v) = state {
                        v
                    } else {
                        unreachable!()
                    }
                });
            }
        }
        let old_state = self.state.replace(ThunkState::BlackHole);
        let result = match old_state {
            ThunkState::Unevaluated(f) => f(),
            ThunkState::BlackHole => {
                panic!("lazy evaluation cycle detected (black hole)")
            }
            ThunkState::Evaluated(_) => unreachable!(),
        };
        self.state.replace(ThunkState::Evaluated(result));
        std::cell::Ref::map(self.state.borrow(), |state| {
            if let ThunkState::Evaluated(ref v) = state {
                v
            } else {
                unreachable!()
            }
        })
    }
    /// Returns `true` if this thunk has already been evaluated.
    pub fn is_evaluated(&self) -> bool {
        matches!(*self.state.borrow(), ThunkState::Evaluated(_))
    }
}
pub(super) struct SharedThunkInner<T> {
    pub(super) value: OnceLock<T>,
    thunk: Option<Box<dyn FnOnce() -> T + Send>>,
}
/// Map a function lazily over a slice, producing results on demand.
pub struct LazyMap2<'a, A, B> {
    pub(super) data: &'a [A],
    pub(super) func: Box<dyn Fn(&A) -> B>,
    pub(super) index: usize,
}
impl<'a, A, B: fmt::Debug> LazyMap2<'a, A, B> {
    /// Create a new lazy map over `data` using `f`.
    pub fn new<F: Fn(&A) -> B + 'static>(data: &'a [A], f: F) -> Self {
        LazyMap2 {
            data,
            func: Box::new(f),
            index: 0,
        }
    }
    /// Get the element at `i` (applies `f` each time; not memoized).
    pub fn get(&self, i: usize) -> Option<B> {
        self.data.get(i).map(|x| (self.func)(x))
    }
    /// Collect all results into a Vec.
    pub fn collect_all(&self) -> Vec<B> {
        self.data.iter().map(|x| (self.func)(x)).collect()
    }
}
