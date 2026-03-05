//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::Expr;
use std::collections::{HashMap, HashSet, VecDeque};

/// Maps expression hashes to equivalence class IDs.
#[allow(dead_code)]
pub struct EquivalenceTable {
    hash_to_id: std::collections::HashMap<u64, usize>,
    uf: UnionFind,
    next_id: usize,
}
#[allow(dead_code)]
impl EquivalenceTable {
    /// Creates an empty table with pre-allocated capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            hash_to_id: std::collections::HashMap::with_capacity(capacity),
            uf: UnionFind::new(capacity),
            next_id: 0,
        }
    }
    /// Registers a hash and returns its class ID.
    pub fn register(&mut self, hash: u64) -> usize {
        if let Some(&id) = self.hash_to_id.get(&hash) {
            return id;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.hash_to_id.insert(hash, id);
        id
    }
    /// Merges the classes of `h1` and `h2`.
    pub fn merge(&mut self, h1: u64, h2: u64) {
        let id1 = self.register(h1);
        let id2 = self.register(h2);
        self.uf.union(id1, id2);
    }
    /// Returns `true` if `h1` and `h2` are in the same class.
    pub fn equiv(&mut self, h1: u64, h2: u64) -> bool {
        let id1 = self.register(h1);
        let id2 = self.register(h2);
        self.uf.same(id1, id2)
    }
}
/// A monotonic timestamp in microseconds.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);
#[allow(dead_code)]
impl Timestamp {
    /// Creates a timestamp from microseconds.
    pub const fn from_us(us: u64) -> Self {
        Self(us)
    }
    /// Returns the timestamp in microseconds.
    pub fn as_us(self) -> u64 {
        self.0
    }
    /// Returns the duration between two timestamps.
    pub fn elapsed_since(self, earlier: Timestamp) -> u64 {
        self.0.saturating_sub(earlier.0)
    }
}
/// A generation counter for validity tracking.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Generation(u32);
#[allow(dead_code)]
impl Generation {
    /// The initial generation.
    pub const INITIAL: Generation = Generation(0);
    /// Advances to the next generation.
    pub fn advance(self) -> Generation {
        Generation(self.0 + 1)
    }
    /// Returns the raw generation number.
    pub fn number(self) -> u32 {
        self.0
    }
}
/// A clock that measures elapsed time in a loop.
#[allow(dead_code)]
pub struct LoopClock {
    start: std::time::Instant,
    iters: u64,
}
#[allow(dead_code)]
impl LoopClock {
    /// Starts the clock.
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
            iters: 0,
        }
    }
    /// Records one iteration.
    pub fn tick(&mut self) {
        self.iters += 1;
    }
    /// Returns the elapsed time in microseconds.
    pub fn elapsed_us(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1e6
    }
    /// Returns the average microseconds per iteration.
    pub fn avg_us_per_iter(&self) -> f64 {
        if self.iters == 0 {
            return 0.0;
        }
        self.elapsed_us() / self.iters as f64
    }
    /// Returns the number of iterations.
    pub fn iters(&self) -> u64 {
        self.iters
    }
}
/// A bidirectional map between two types.
#[allow(dead_code)]
pub struct BiMap<A: std::hash::Hash + Eq + Clone, B: std::hash::Hash + Eq + Clone> {
    forward: std::collections::HashMap<A, B>,
    backward: std::collections::HashMap<B, A>,
}
#[allow(dead_code)]
impl<A: std::hash::Hash + Eq + Clone, B: std::hash::Hash + Eq + Clone> BiMap<A, B> {
    /// Creates an empty bidirectional map.
    pub fn new() -> Self {
        Self {
            forward: std::collections::HashMap::new(),
            backward: std::collections::HashMap::new(),
        }
    }
    /// Inserts a pair `(a, b)`.
    pub fn insert(&mut self, a: A, b: B) {
        self.forward.insert(a.clone(), b.clone());
        self.backward.insert(b, a);
    }
    /// Looks up `b` for a given `a`.
    pub fn get_b(&self, a: &A) -> Option<&B> {
        self.forward.get(a)
    }
    /// Looks up `a` for a given `b`.
    pub fn get_a(&self, b: &B) -> Option<&A> {
        self.backward.get(b)
    }
    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.forward.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }
}
/// A pair of values useful for before/after comparisons.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BeforeAfter<T> {
    /// Value before the transformation.
    pub before: T,
    /// Value after the transformation.
    pub after: T,
}
#[allow(dead_code)]
impl<T: PartialEq> BeforeAfter<T> {
    /// Creates a new before/after pair.
    pub fn new(before: T, after: T) -> Self {
        Self { before, after }
    }
    /// Returns `true` if before equals after (no change).
    pub fn unchanged(&self) -> bool {
        self.before == self.after
    }
}
/// A simple LRU cache backed by a linked list + hash map.
#[allow(dead_code)]
pub struct SimpleLruCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    capacity: usize,
    map: std::collections::HashMap<K, usize>,
    keys: Vec<K>,
    vals: Vec<V>,
    order: Vec<usize>,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq + Clone, V: Clone> SimpleLruCache<K, V> {
    /// Creates a new LRU cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            map: std::collections::HashMap::new(),
            keys: Vec::new(),
            vals: Vec::new(),
            order: Vec::new(),
        }
    }
    /// Inserts or updates a key-value pair.
    pub fn put(&mut self, key: K, val: V) {
        if let Some(&idx) = self.map.get(&key) {
            self.vals[idx] = val;
            self.order.retain(|&x| x != idx);
            self.order.insert(0, idx);
            return;
        }
        if self.keys.len() >= self.capacity {
            let evict_idx = *self
                .order
                .last()
                .expect("order list must be non-empty before eviction");
            self.map.remove(&self.keys[evict_idx]);
            self.order.pop();
            self.keys[evict_idx] = key.clone();
            self.vals[evict_idx] = val;
            self.map.insert(key, evict_idx);
            self.order.insert(0, evict_idx);
        } else {
            let idx = self.keys.len();
            self.keys.push(key.clone());
            self.vals.push(val);
            self.map.insert(key, idx);
            self.order.insert(0, idx);
        }
    }
    /// Returns a reference to the value for `key`, promoting it.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let idx = *self.map.get(key)?;
        self.order.retain(|&x| x != idx);
        self.order.insert(0, idx);
        Some(&self.vals[idx])
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}
/// A simple sparse bit set.
#[allow(dead_code)]
pub struct SparseBitSet {
    words: Vec<u64>,
}
#[allow(dead_code)]
impl SparseBitSet {
    /// Creates a new bit set that can hold at least `capacity` bits.
    pub fn new(capacity: usize) -> Self {
        let words = (capacity + 63) / 64;
        Self {
            words: vec![0u64; words],
        }
    }
    /// Sets bit `i`.
    pub fn set(&mut self, i: usize) {
        let word = i / 64;
        let bit = i % 64;
        if word < self.words.len() {
            self.words[word] |= 1u64 << bit;
        }
    }
    /// Clears bit `i`.
    pub fn clear(&mut self, i: usize) {
        let word = i / 64;
        let bit = i % 64;
        if word < self.words.len() {
            self.words[word] &= !(1u64 << bit);
        }
    }
    /// Returns `true` if bit `i` is set.
    pub fn get(&self, i: usize) -> bool {
        let word = i / 64;
        let bit = i % 64;
        self.words.get(word).is_some_and(|w| w & (1u64 << bit) != 0)
    }
    /// Returns the number of set bits.
    pub fn count_ones(&self) -> u32 {
        self.words.iter().map(|w| w.count_ones()).sum()
    }
    /// Returns the union with another bit set.
    pub fn union(&self, other: &SparseBitSet) -> SparseBitSet {
        let len = self.words.len().max(other.words.len());
        let mut result = SparseBitSet {
            words: vec![0u64; len],
        };
        for i in 0..self.words.len() {
            result.words[i] |= self.words[i];
        }
        for i in 0..other.words.len() {
            result.words[i] |= other.words[i];
        }
        result
    }
}
/// Tracks the frequency of items.
#[allow(dead_code)]
pub struct FrequencyTable<T: std::hash::Hash + Eq + Clone> {
    counts: std::collections::HashMap<T, u64>,
}
#[allow(dead_code)]
impl<T: std::hash::Hash + Eq + Clone> FrequencyTable<T> {
    /// Creates a new empty frequency table.
    pub fn new() -> Self {
        Self {
            counts: std::collections::HashMap::new(),
        }
    }
    /// Records one occurrence of `item`.
    pub fn record(&mut self, item: T) {
        *self.counts.entry(item).or_insert(0) += 1;
    }
    /// Returns the frequency of `item`.
    pub fn freq(&self, item: &T) -> u64 {
        self.counts.get(item).copied().unwrap_or(0)
    }
    /// Returns the item with the highest frequency.
    pub fn most_frequent(&self) -> Option<(&T, u64)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }
    /// Returns the total number of recordings.
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }
    /// Returns the number of distinct items.
    pub fn distinct(&self) -> usize {
        self.counts.len()
    }
}
/// A simple LIFO work queue.
#[allow(dead_code)]
pub struct WorkStack<T> {
    items: Vec<T>,
}
#[allow(dead_code)]
impl<T> WorkStack<T> {
    /// Creates a new empty stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Pushes a work item.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    /// Pops the next work item.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Returns the number of pending work items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
/// A symmetric binary relation stored as a set of unordered pairs.
#[allow(dead_code)]
pub struct SymmetricRelation {
    pairs: std::collections::HashSet<(u64, u64)>,
}
#[allow(dead_code)]
impl SymmetricRelation {
    /// Creates an empty relation.
    pub fn new() -> Self {
        Self {
            pairs: std::collections::HashSet::new(),
        }
    }
    fn key(a: u64, b: u64) -> (u64, u64) {
        if a <= b {
            (a, b)
        } else {
            (b, a)
        }
    }
    /// Adds the pair `(a, b)`.
    pub fn add(&mut self, a: u64, b: u64) {
        self.pairs.insert(Self::key(a, b));
    }
    /// Returns `true` if `(a, b)` is in the relation.
    pub fn contains(&self, a: u64, b: u64) -> bool {
        self.pairs.contains(&Self::key(a, b))
    }
    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
/// A memoized computation slot that stores a cached value.
#[allow(dead_code)]
pub struct MemoSlot<T: Clone> {
    cached: Option<T>,
}
#[allow(dead_code)]
impl<T: Clone> MemoSlot<T> {
    /// Creates an uncomputed memo slot.
    pub fn new() -> Self {
        Self { cached: None }
    }
    /// Returns the cached value, computing it with `f` if absent.
    pub fn get_or_compute(&mut self, f: impl FnOnce() -> T) -> &T {
        if self.cached.is_none() {
            self.cached = Some(f());
        }
        self.cached
            .as_ref()
            .expect("cached value must be initialized before access")
    }
    /// Invalidates the cached value.
    pub fn invalidate(&mut self) {
        self.cached = None;
    }
    /// Returns `true` if the value has been computed.
    pub fn is_cached(&self) -> bool {
        self.cached.is_some()
    }
}
/// A checked equivalence with its witness proof.
#[allow(dead_code)]
pub struct CheckedEquiv {
    lhs: u64,
    rhs: u64,
    proof: EquivProofTerm,
}
#[allow(dead_code)]
impl CheckedEquiv {
    /// Creates a reflexive equivalence.
    pub fn refl(id: u64) -> Self {
        Self {
            lhs: id,
            rhs: id,
            proof: EquivProofTerm::Refl(id),
        }
    }
    /// Creates a named-axiom equivalence.
    pub fn by_axiom(lhs: u64, rhs: u64, name: impl Into<String>) -> Self {
        Self {
            lhs,
            rhs,
            proof: EquivProofTerm::Axiom(name.into()),
        }
    }
    /// Returns the LHS hash.
    pub fn lhs(&self) -> u64 {
        self.lhs
    }
    /// Returns the RHS hash.
    pub fn rhs(&self) -> u64 {
        self.rhs
    }
    /// Returns a reference to the proof term.
    pub fn proof(&self) -> &EquivProofTerm {
        &self.proof
    }
    /// Returns `true` if both sides are identical.
    pub fn is_trivial(&self) -> bool {
        self.lhs == self.rhs
    }
}
/// A simple event counter with named events.
#[allow(dead_code)]
pub struct EventCounter {
    counts: std::collections::HashMap<String, u64>,
}
#[allow(dead_code)]
impl EventCounter {
    /// Creates a new empty counter.
    pub fn new() -> Self {
        Self {
            counts: std::collections::HashMap::new(),
        }
    }
    /// Increments the counter for `event`.
    pub fn inc(&mut self, event: &str) {
        *self.counts.entry(event.to_string()).or_insert(0) += 1;
    }
    /// Adds `n` to the counter for `event`.
    pub fn add(&mut self, event: &str, n: u64) {
        *self.counts.entry(event.to_string()).or_insert(0) += n;
    }
    /// Returns the count for `event`.
    pub fn get(&self, event: &str) -> u64 {
        self.counts.get(event).copied().unwrap_or(0)
    }
    /// Returns the total count across all events.
    pub fn total(&self) -> u64 {
        self.counts.values().sum()
    }
    /// Resets all counters.
    pub fn reset(&mut self) {
        self.counts.clear();
    }
    /// Returns all event names.
    pub fn event_names(&self) -> Vec<&str> {
        self.counts.keys().map(|s| s.as_str()).collect()
    }
}
/// A key-value annotation table for arbitrary metadata.
#[allow(dead_code)]
pub struct AnnotationTable {
    map: std::collections::HashMap<String, Vec<String>>,
}
#[allow(dead_code)]
impl AnnotationTable {
    /// Creates an empty annotation table.
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }
    /// Adds an annotation value for the given key.
    pub fn annotate(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.map.entry(key.into()).or_default().push(val.into());
    }
    /// Returns all annotations for `key`.
    pub fn get_all(&self, key: &str) -> &[String] {
        self.map.get(key).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Returns the number of distinct annotation keys.
    pub fn num_keys(&self) -> usize {
        self.map.len()
    }
    /// Returns `true` if the table has any annotations for `key`.
    pub fn has(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}
/// A FIFO work queue.
#[allow(dead_code)]
pub struct WorkQueue<T> {
    items: std::collections::VecDeque<T>,
}
#[allow(dead_code)]
impl<T> WorkQueue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self {
            items: std::collections::VecDeque::new(),
        }
    }
    /// Enqueues a work item.
    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }
    /// Dequeues the next work item.
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Returns the number of pending items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
/// A counter that dispenses monotonically increasing `TypedId` values.
#[allow(dead_code)]
pub struct IdDispenser<T> {
    next: u32,
    _phantom: std::marker::PhantomData<T>,
}
#[allow(dead_code)]
impl<T> IdDispenser<T> {
    /// Creates a new dispenser starting from zero.
    pub fn new() -> Self {
        Self {
            next: 0,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Dispenses the next ID.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> TypedId<T> {
        let id = TypedId::new(self.next);
        self.next += 1;
        id
    }
    /// Returns the number of IDs dispensed.
    pub fn count(&self) -> u32 {
        self.next
    }
}
/// A slot that can hold a value, with lazy initialization.
#[allow(dead_code)]
pub struct Slot<T> {
    inner: Option<T>,
}
#[allow(dead_code)]
impl<T> Slot<T> {
    /// Creates an empty slot.
    pub fn empty() -> Self {
        Self { inner: None }
    }
    /// Fills the slot with `val`.  Panics if already filled.
    pub fn fill(&mut self, val: T) {
        assert!(self.inner.is_none(), "Slot: already filled");
        self.inner = Some(val);
    }
    /// Returns the slot's value, or `None`.
    pub fn get(&self) -> Option<&T> {
        self.inner.as_ref()
    }
    /// Returns `true` if the slot is filled.
    pub fn is_filled(&self) -> bool {
        self.inner.is_some()
    }
    /// Takes the value out of the slot.
    pub fn take(&mut self) -> Option<T> {
        self.inner.take()
    }
    /// Fills the slot if empty, returning a reference to the value.
    pub fn get_or_fill_with(&mut self, f: impl FnOnce() -> T) -> &T {
        if self.inner.is_none() {
            self.inner = Some(f());
        }
        self.inner
            .as_ref()
            .expect("inner value must be initialized before access")
    }
}
/// A single equivalence class with a canonical representative.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct EquivClass {
    /// The canonical (representative) element.
    pub repr: usize,
    /// All elements in this class (including the representative).
    pub members: Vec<usize>,
}
#[allow(dead_code)]
impl EquivClass {
    /// Creates a new singleton class.
    pub fn singleton(id: usize) -> Self {
        Self {
            repr: id,
            members: vec![id],
        }
    }
    /// Returns `true` if `id` is a member of this class.
    pub fn contains(&self, id: usize) -> bool {
        self.members.contains(&id)
    }
    /// Returns the size of the class.
    pub fn size(&self) -> usize {
        self.members.len()
    }
}
/// A proof term witnessing an equivalence.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum EquivProofTerm {
    /// Reflexivity: `a ≡ a`.
    Refl(u64),
    /// Symmetry: if `a ≡ b` then `b ≡ a`.
    Symm(Box<EquivProofTerm>),
    /// Transitivity: if `a ≡ b` and `b ≡ c` then `a ≡ c`.
    Trans(Box<EquivProofTerm>, Box<EquivProofTerm>),
    /// A named axiom.
    Axiom(String),
}
#[allow(dead_code)]
impl EquivProofTerm {
    /// Returns the depth of the proof term.
    pub fn depth(&self) -> usize {
        match self {
            EquivProofTerm::Refl(_) => 0,
            EquivProofTerm::Axiom(_) => 0,
            EquivProofTerm::Symm(p) => 1 + p.depth(),
            EquivProofTerm::Trans(p, q) => 1 + p.depth().max(q.depth()),
        }
    }
    /// Returns `true` if the proof is trivially reflexive.
    pub fn is_refl(&self) -> bool {
        matches!(self, EquivProofTerm::Refl(_))
    }
}
/// Union-find node for expression equivalence classes.
#[derive(Debug)]
struct UnionFindEntry {
    parent: usize,
    rank: u32,
}
/// Statistics about the equivalence manager usage.
#[derive(Clone, Debug, Default)]
pub struct EquivStats {
    /// Total queries performed.
    pub total_queries: u64,
    /// Queries that hit the equiv cache.
    pub equiv_hits: u64,
    /// Queries that hit the failure cache.
    pub failure_hits: u64,
    /// Queries that required computation.
    pub cache_misses: u64,
    /// Number of times add_equiv was called.
    pub equiv_additions: u64,
    /// Number of times add_failure was called.
    pub failure_additions: u64,
}
impl EquivStats {
    /// Create new zeroed statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a cache hit on the equiv set.
    pub fn record_equiv_hit(&mut self) {
        self.total_queries += 1;
        self.equiv_hits += 1;
    }
    /// Record a cache hit on the failure set.
    pub fn record_failure_hit(&mut self) {
        self.total_queries += 1;
        self.failure_hits += 1;
    }
    /// Record a cache miss.
    pub fn record_miss(&mut self) {
        self.total_queries += 1;
        self.cache_misses += 1;
    }
    /// Record an addition.
    pub fn record_equiv_addition(&mut self) {
        self.equiv_additions += 1;
    }
    /// Record a failure addition.
    pub fn record_failure_addition(&mut self) {
        self.failure_additions += 1;
    }
    /// Compute the cache hit rate.
    pub fn hit_rate(&self) -> f64 {
        if self.total_queries == 0 {
            return 1.0;
        }
        (self.equiv_hits + self.failure_hits) as f64 / self.total_queries as f64
    }
}
/// A set of non-overlapping integer intervals.
#[allow(dead_code)]
pub struct IntervalSet {
    intervals: Vec<(i64, i64)>,
}
#[allow(dead_code)]
impl IntervalSet {
    /// Creates an empty interval set.
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }
    /// Adds the interval `[lo, hi]` to the set.
    pub fn add(&mut self, lo: i64, hi: i64) {
        if lo > hi {
            return;
        }
        let mut new_lo = lo;
        let mut new_hi = hi;
        let mut i = 0;
        while i < self.intervals.len() {
            let (il, ih) = self.intervals[i];
            if ih < new_lo - 1 {
                i += 1;
                continue;
            }
            if il > new_hi + 1 {
                break;
            }
            new_lo = new_lo.min(il);
            new_hi = new_hi.max(ih);
            self.intervals.remove(i);
        }
        self.intervals.insert(i, (new_lo, new_hi));
    }
    /// Returns `true` if `x` is in the set.
    pub fn contains(&self, x: i64) -> bool {
        self.intervals.iter().any(|&(lo, hi)| x >= lo && x <= hi)
    }
    /// Returns the number of intervals.
    pub fn num_intervals(&self) -> usize {
        self.intervals.len()
    }
    /// Returns the total count of integers covered.
    pub fn cardinality(&self) -> i64 {
        self.intervals.iter().map(|&(lo, hi)| hi - lo + 1).sum()
    }
}
/// Equivalence manager for caching def_eq results during type checking.
///
/// Provides two layers of caching:
/// 1. A set of expression pairs known to be definitionally equal.
/// 2. A set of expression pairs known to NOT be definitionally equal (failure cache).
///
/// The failure cache is critical for performance: when lazy delta reduction
/// tries multiple unfolding strategies, caching failures avoids redundant work.
#[derive(Debug)]
pub struct EquivManager {
    /// Known equal pairs.
    equiv_set: HashSet<(Expr, Expr)>,
    /// Known unequal pairs (failure cache).
    failure_set: HashSet<(Expr, Expr)>,
    /// Union-find for transitive closure.
    /// Maps expression to its representative index.
    expr_to_idx: HashMap<Expr, usize>,
    /// Union-find entries.
    entries: Vec<UnionFindEntry>,
}
impl EquivManager {
    /// Create a new empty equivalence manager.
    pub fn new() -> Self {
        EquivManager {
            equiv_set: HashSet::new(),
            failure_set: HashSet::new(),
            expr_to_idx: HashMap::new(),
            entries: Vec::new(),
        }
    }
    /// Record that two expressions are definitionally equal.
    pub fn add_equiv(&mut self, a: &Expr, b: &Expr) {
        if a == b {
            return;
        }
        let pair = canonicalize_pair(a.clone(), b.clone());
        self.equiv_set.insert(pair);
        let idx_a = self.get_or_create_idx(a);
        let idx_b = self.get_or_create_idx(b);
        self.union(idx_a, idx_b);
    }
    /// Check if two expressions are known to be definitionally equal.
    pub fn is_equiv(&mut self, a: &Expr, b: &Expr) -> bool {
        if a == b {
            return true;
        }
        let pair = canonicalize_pair(a.clone(), b.clone());
        if self.equiv_set.contains(&pair) {
            return true;
        }
        if let (Some(&idx_a), Some(&idx_b)) = (self.expr_to_idx.get(a), self.expr_to_idx.get(b)) {
            return self.find(idx_a) == self.find(idx_b);
        }
        false
    }
    /// Record that two expressions are NOT definitionally equal.
    pub fn add_failure(&mut self, a: &Expr, b: &Expr) {
        let pair = canonicalize_pair(a.clone(), b.clone());
        self.failure_set.insert(pair);
    }
    /// Check if two expressions are known to NOT be definitionally equal.
    pub fn is_failure(&self, a: &Expr, b: &Expr) -> bool {
        let pair = canonicalize_pair(a.clone(), b.clone());
        self.failure_set.contains(&pair)
    }
    /// Clear all cached results.
    pub fn clear(&mut self) {
        self.equiv_set.clear();
        self.failure_set.clear();
        self.expr_to_idx.clear();
        self.entries.clear();
    }
    /// Get the number of cached equivalences.
    pub fn num_equiv(&self) -> usize {
        self.equiv_set.len()
    }
    /// Get the number of cached failures.
    pub fn num_failures(&self) -> usize {
        self.failure_set.len()
    }
    fn get_or_create_idx(&mut self, e: &Expr) -> usize {
        if let Some(&idx) = self.expr_to_idx.get(e) {
            return idx;
        }
        let idx = self.entries.len();
        self.entries.push(UnionFindEntry {
            parent: idx,
            rank: 0,
        });
        self.expr_to_idx.insert(e.clone(), idx);
        idx
    }
    fn find(&mut self, mut x: usize) -> usize {
        while self.entries[x].parent != x {
            let grandparent = self.entries[self.entries[x].parent].parent;
            self.entries[x].parent = grandparent;
            x = grandparent;
        }
        x
    }
    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        match self.entries[rx].rank.cmp(&self.entries[ry].rank) {
            std::cmp::Ordering::Less => {
                self.entries[rx].parent = ry;
            }
            std::cmp::Ordering::Greater => {
                self.entries[ry].parent = rx;
            }
            std::cmp::Ordering::Equal => {
                self.entries[ry].parent = rx;
                self.entries[rx].rank += 1;
            }
        }
    }
}
/// A simple equivalence relation over integer indices.
#[derive(Clone, Debug, Default)]
pub struct IndexEquivManager {
    parent: Vec<usize>,
    rank: Vec<u32>,
}
impl IndexEquivManager {
    /// Create a new manager for n elements.
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    /// Find the representative of element `x`.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    /// Merge the classes of `x` and `y`.
    pub fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }
    }
    /// Check if two elements are in the same class.
    pub fn same_class(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    /// Count the number of distinct equivalence classes.
    pub fn num_classes(&mut self) -> usize {
        let n = self.parent.len();
        let roots: std::collections::HashSet<usize> = (0..n).map(|i| self.find(i)).collect();
        roots.len()
    }
}
/// A key-value store for diagnostic metadata.
#[allow(dead_code)]
pub struct DiagMeta {
    pub(super) entries: Vec<(String, String)>,
}
#[allow(dead_code)]
impl DiagMeta {
    /// Creates an empty metadata store.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Adds a key-value pair.
    pub fn add(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.entries.push((key.into(), val.into()));
    }
    /// Returns the value for `key`, or `None`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A type-safe wrapper around a `u32` identifier.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedId<T> {
    pub(super) id: u32,
    _phantom: std::marker::PhantomData<T>,
}
#[allow(dead_code)]
impl<T> TypedId<T> {
    /// Creates a new typed ID.
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Returns the raw `u32` ID.
    pub fn raw(&self) -> u32 {
        self.id
    }
}
/// A counted-access cache that tracks hit and miss statistics.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct StatCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    /// The inner LRU cache.
    pub inner: SimpleLruCache<K, V>,
    /// Number of cache hits.
    pub hits: u64,
    /// Number of cache misses.
    pub misses: u64,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq + Clone, V: Clone> StatCache<K, V> {
    /// Creates a new stat cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: SimpleLruCache::new(capacity),
            hits: 0,
            misses: 0,
        }
    }
    /// Performs a lookup, tracking hit/miss.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let result = self.inner.get(key);
        if result.is_some() {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
        None
    }
    /// Inserts a key-value pair.
    pub fn put(&mut self, key: K, val: V) {
        self.inner.put(key, val);
    }
    /// Returns the hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
}
/// A simple stack-based scope tracker.
#[allow(dead_code)]
pub struct ScopeStack {
    names: Vec<String>,
}
#[allow(dead_code)]
impl ScopeStack {
    /// Creates a new empty scope stack.
    pub fn new() -> Self {
        Self { names: Vec::new() }
    }
    /// Pushes a scope name.
    pub fn push(&mut self, name: impl Into<String>) {
        self.names.push(name.into());
    }
    /// Pops the current scope.
    pub fn pop(&mut self) -> Option<String> {
        self.names.pop()
    }
    /// Returns the current (innermost) scope name, or `None`.
    pub fn current(&self) -> Option<&str> {
        self.names.last().map(|s| s.as_str())
    }
    /// Returns the depth of the scope stack.
    pub fn depth(&self) -> usize {
        self.names.len()
    }
    /// Returns `true` if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Returns the full path as a dot-separated string.
    pub fn path(&self) -> String {
        self.names.join(".")
    }
}
/// A sequence number that can be compared for ordering.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeqNum(u64);
#[allow(dead_code)]
impl SeqNum {
    /// Creates sequence number zero.
    pub const ZERO: SeqNum = SeqNum(0);
    /// Advances the sequence number by one.
    pub fn next(self) -> SeqNum {
        SeqNum(self.0 + 1)
    }
    /// Returns the raw value.
    pub fn value(self) -> u64 {
        self.0
    }
}
/// An instrumented equivalence manager that collects statistics.
#[derive(Debug, Default)]
pub struct InstrumentedEquivManager {
    /// Underlying manager.
    inner: EquivManager,
    /// Usage statistics.
    stats: EquivStats,
}
impl InstrumentedEquivManager {
    /// Create a new instrumented manager.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an equivalence.
    pub fn add_equiv(&mut self, a: &Expr, b: &Expr) {
        self.stats.record_equiv_addition();
        self.inner.add_equiv(a, b);
    }
    /// Add a failure.
    pub fn add_failure(&mut self, a: &Expr, b: &Expr) {
        self.stats.record_failure_addition();
        self.inner.add_failure(a, b);
    }
    /// Check if two expressions are equivalent.
    pub fn is_equiv(&mut self, a: &Expr, b: &Expr) -> bool {
        let result = self.inner.is_equiv(a, b);
        if result {
            self.stats.record_equiv_hit();
        } else {
            self.stats.record_miss();
        }
        result
    }
    /// Check if a failure is cached.
    pub fn is_failure(&self, a: &Expr, b: &Expr) -> bool {
        self.inner.is_failure(a, b)
    }
    /// Get the statistics.
    pub fn stats(&self) -> &EquivStats {
        &self.stats
    }
    /// Clear both the manager and statistics.
    pub fn clear(&mut self) {
        self.inner.clear();
        self.stats = EquivStats::new();
    }
}
/// Statistics for the equivalence manager.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct EquivManagerStats {
    /// Number of equivalence checks performed.
    pub checks: u64,
    /// Number of cache hits.
    pub hits: u64,
    /// Number of merges performed.
    pub merges: u64,
    /// Number of distinct equivalence classes remaining.
    pub classes: usize,
}
#[allow(dead_code)]
impl EquivManagerStats {
    /// Creates zeroed stats.
    pub fn new() -> Self {
        Self {
            checks: 0,
            hits: 0,
            merges: 0,
            classes: 0,
        }
    }
    /// Returns the cache hit rate (0.0–1.0).
    pub fn hit_rate(&self) -> f64 {
        if self.checks == 0 {
            return 0.0;
        }
        self.hits as f64 / self.checks as f64
    }
}
/// A read-only view into an `EquivManager` for query-only access.
pub struct EquivQuery<'a> {
    manager: &'a EquivManager,
}
impl<'a> EquivQuery<'a> {
    /// Create a new query handle.
    pub fn new(manager: &'a EquivManager) -> Self {
        Self { manager }
    }
    /// Check if two expressions are in the failure cache.
    pub fn is_known_failure(&self, a: &Expr, b: &Expr) -> bool {
        self.manager.is_failure(a, b)
    }
    /// Return the number of cached equivalences.
    pub fn num_equiv(&self) -> usize {
        self.manager.num_equiv()
    }
    /// Return the number of cached failures.
    pub fn num_failures(&self) -> usize {
        self.manager.num_failures()
    }
}
/// A persistent equivalence manager that supports efficient batch updates.
///
/// Unlike `EquivManager`, this stores equivalences as a sorted list for
/// predictable serialization.
#[derive(Clone, Debug, Default)]
pub struct PersistentEquivManager {
    pairs: Vec<(String, String)>,
}
impl PersistentEquivManager {
    /// Create a new empty persistent manager.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an equivalence.
    pub fn add_equiv(&mut self, a: &Expr, b: &Expr) {
        let ka = format!("{:?}", a);
        let kb = format!("{:?}", b);
        let pair = if ka <= kb { (ka, kb) } else { (kb, ka) };
        if !self.pairs.contains(&pair) {
            self.pairs.push(pair);
            self.pairs.sort();
        }
    }
    /// Check if two expressions are known equivalent.
    pub fn is_equiv(&self, a: &Expr, b: &Expr) -> bool {
        if a == b {
            return true;
        }
        let ka = format!("{:?}", a);
        let kb = format!("{:?}", b);
        let pair = if ka <= kb {
            (ka.clone(), kb.clone())
        } else {
            (kb.clone(), ka.clone())
        };
        self.pairs.contains(&pair)
    }
    /// Return the number of stored equivalences.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    /// Check if the manager is empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
    /// Serialize the equivalences to a list of string pairs.
    pub fn serialize(&self) -> Vec<(String, String)> {
        self.pairs.clone()
    }
}
/// A cache that records proven expression equalities.
#[allow(dead_code)]
pub struct ExprEquivCache {
    proven: std::collections::HashSet<(u64, u64)>,
    disproven: std::collections::HashSet<(u64, u64)>,
}
#[allow(dead_code)]
impl ExprEquivCache {
    /// Creates an empty cache.
    pub fn new() -> Self {
        Self {
            proven: std::collections::HashSet::new(),
            disproven: std::collections::HashSet::new(),
        }
    }
    /// Records that `a` and `b` are equal.
    pub fn mark_equal(&mut self, a: u64, b: u64) {
        let key = if a <= b { (a, b) } else { (b, a) };
        self.proven.insert(key);
    }
    /// Records that `a` and `b` are unequal.
    pub fn mark_unequal(&mut self, a: u64, b: u64) {
        let key = if a <= b { (a, b) } else { (b, a) };
        self.disproven.insert(key);
    }
    /// Returns `Some(true)` if equal, `Some(false)` if unequal, `None` if unknown.
    pub fn query(&self, a: u64, b: u64) -> Option<bool> {
        let key = if a <= b { (a, b) } else { (b, a) };
        if self.proven.contains(&key) {
            return Some(true);
        }
        if self.disproven.contains(&key) {
            return Some(false);
        }
        None
    }
    /// Returns the number of proven equalities.
    pub fn proven_count(&self) -> usize {
        self.proven.len()
    }
}
/// Interns strings to save memory (each unique string stored once).
#[allow(dead_code)]
pub struct StringInterner {
    strings: Vec<String>,
    map: std::collections::HashMap<String, u32>,
}
#[allow(dead_code)]
impl StringInterner {
    /// Creates a new string interner.
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            map: std::collections::HashMap::new(),
        }
    }
    /// Interns `s` and returns its ID.
    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&id) = self.map.get(s) {
            return id;
        }
        let id = self.strings.len() as u32;
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), id);
        id
    }
    /// Returns the string for `id`.
    pub fn get(&self, id: u32) -> Option<&str> {
        self.strings.get(id as usize).map(|s| s.as_str())
    }
    /// Returns the total number of interned strings.
    pub fn len(&self) -> usize {
        self.strings.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }
}
/// A union-find (disjoint-set) data structure with path compression.
#[allow(dead_code)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}
#[allow(dead_code)]
impl UnionFind {
    /// Creates a new union-find with `n` elements (each its own class).
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }
    /// Finds the representative of `x` with path compression.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    /// Unions the classes of `x` and `y`.  Returns `true` if they were distinct.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
        self.count -= 1;
        true
    }
    /// Returns `true` if `x` and `y` are in the same class.
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    /// Returns the number of distinct classes.
    pub fn num_classes(&self) -> usize {
        self.count
    }
    /// Returns the total number of elements.
    pub fn num_elements(&self) -> usize {
        self.parent.len()
    }
}
/// A scoped equivalence manager that supports push/pop of equivalence frames.
///
/// This allows backtracking during type checking — when a unification attempt
/// fails, all equivalences added in that scope are rolled back.
#[derive(Debug, Default)]
pub struct ScopedEquivManager {
    /// Current equivalences, grouped by scope.
    scopes: Vec<Vec<(Expr, Expr)>>,
    /// The underlying equivalence manager.
    inner: EquivManager,
}
impl ScopedEquivManager {
    /// Create a new scoped manager with one initial scope.
    pub fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
            inner: EquivManager::new(),
        }
    }
    /// Push a new scope.
    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }
    /// Pop the current scope, rolling back all equivalences added in it.
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            self.inner.clear();
            for scope in &self.scopes {
                for (a, b) in scope {
                    self.inner.add_equiv(a, b);
                }
            }
        }
    }
    /// Add an equivalence in the current scope.
    pub fn add_equiv(&mut self, a: &Expr, b: &Expr) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.push((a.clone(), b.clone()));
        }
        self.inner.add_equiv(a, b);
    }
    /// Check if two expressions are equivalent.
    pub fn is_equiv(&mut self, a: &Expr, b: &Expr) -> bool {
        self.inner.is_equiv(a, b)
    }
    /// Return the current scope depth.
    pub fn scope_depth(&self) -> usize {
        self.scopes.len()
    }
    /// Return the total number of equivalences across all scopes.
    pub fn total_equivs(&self) -> usize {
        self.scopes.iter().map(|s| s.len()).sum()
    }
}
/// A growable ring buffer with fixed maximum capacity.
#[allow(dead_code)]
pub struct RingBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    count: usize,
    capacity: usize,
}
#[allow(dead_code)]
impl<T> RingBuffer<T> {
    /// Creates a new ring buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(None);
        }
        Self {
            data,
            head: 0,
            tail: 0,
            count: 0,
            capacity,
        }
    }
    /// Pushes a value, overwriting the oldest if full.
    pub fn push(&mut self, val: T) {
        if self.count == self.capacity {
            self.data[self.head] = Some(val);
            self.head = (self.head + 1) % self.capacity;
            self.tail = (self.tail + 1) % self.capacity;
        } else {
            self.data[self.tail] = Some(val);
            self.tail = (self.tail + 1) % self.capacity;
            self.count += 1;
        }
    }
    /// Pops the oldest value.
    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let val = self.data[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;
        val
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Returns `true` if at capacity.
    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }
}
