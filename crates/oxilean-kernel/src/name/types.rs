//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

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
/// A fresh-name generator.
///
/// Generates unique names by appending an incrementing counter suffix.
#[derive(Debug, Clone)]
pub struct NameGenerator {
    base: Name,
    counter: u64,
}
impl NameGenerator {
    /// Create a generator with the given base name.
    pub fn new(base: Name) -> Self {
        Self { base, counter: 0 }
    }
    /// Create a generator with a string base.
    pub fn with_base(s: impl Into<String>) -> Self {
        Self::new(Name::str(s))
    }
    /// Generate the next fresh name.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Name {
        let n = self.base.clone().append_num(self.counter);
        self.counter += 1;
        n
    }
    /// Peek at the next name without advancing.
    pub fn peek(&self) -> Name {
        self.base.clone().append_num(self.counter)
    }
    /// Reset the counter to 0.
    pub fn reset(&mut self) {
        self.counter = 0;
    }
    /// Current counter value.
    pub fn count(&self) -> u64 {
        self.counter
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
/// A trie (prefix tree) for efficient name lookups.
///
/// Each node in the trie corresponds to one component of a hierarchical name.
/// Useful for namespace browsing and completion in the LSP server.
#[derive(Debug, Clone)]
pub struct NameTrie<V> {
    /// Value stored at this node (if any).
    pub(super) value: Option<V>,
    /// Children indexed by string component.
    pub(super) string_children: Vec<(String, NameTrie<V>)>,
    /// Children indexed by numeric component.
    pub(super) num_children: Vec<(u64, NameTrie<V>)>,
}
impl<V: Clone> NameTrie<V> {
    /// Create an empty trie.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a name-value pair into the trie.
    pub fn insert(&mut self, name: &Name, value: V) {
        match name {
            Name::Anonymous => {
                self.value = Some(value);
            }
            Name::Str(parent, s) => {
                let child = if let Some(idx) = self.string_children.iter().position(|(k, _)| k == s)
                {
                    &mut self.string_children[idx].1
                } else {
                    self.string_children.push((s.clone(), NameTrie::new()));
                    let last = self.string_children.len() - 1;
                    &mut self.string_children[last].1
                };
                child.insert(parent, value);
            }
            Name::Num(parent, n) => {
                let child = if let Some(idx) = self.num_children.iter().position(|(k, _)| k == n) {
                    &mut self.num_children[idx].1
                } else {
                    self.num_children.push((*n, NameTrie::new()));
                    let last = self.num_children.len() - 1;
                    &mut self.num_children[last].1
                };
                child.insert(parent, value);
            }
        }
    }
    /// Look up a name in the trie.
    pub fn lookup(&self, name: &Name) -> Option<&V> {
        match name {
            Name::Anonymous => self.value.as_ref(),
            Name::Str(parent, s) => {
                let child = self
                    .string_children
                    .iter()
                    .find(|(k, _)| k == s)
                    .map(|(_, v)| v)?;
                child.lookup(parent)
            }
            Name::Num(parent, n) => {
                let child = self
                    .num_children
                    .iter()
                    .find(|(k, _)| k == n)
                    .map(|(_, v)| v)?;
                child.lookup(parent)
            }
        }
    }
    /// Check whether the trie contains the given name.
    pub fn contains(&self, name: &Name) -> bool {
        self.lookup(name).is_some()
    }
    /// Count all values stored in the trie.
    pub fn count(&self) -> usize {
        let self_count = if self.value.is_some() { 1 } else { 0 };
        let str_count: usize = self.string_children.iter().map(|(_, c)| c.count()).sum();
        let num_count: usize = self.num_children.iter().map(|(_, c)| c.count()).sum();
        self_count + str_count + num_count
    }
    /// Collect all (name, value) pairs in the trie.
    pub fn to_vec(&self) -> Vec<(Name, V)> {
        let mut result = Vec::new();
        self.collect_all(Name::Anonymous, &mut result);
        result
    }
    fn collect_all(&self, prefix: Name, result: &mut Vec<(Name, V)>) {
        if let Some(v) = &self.value {
            result.push((prefix.clone(), v.clone()));
        }
        for (s, child) in &self.string_children {
            let name = prefix.clone().append_str(s.as_str());
            child.collect_all(name, result);
        }
        for (n, child) in &self.num_children {
            let name = prefix.clone().append_num(*n);
            child.collect_all(name, result);
        }
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
/// A mapping from `Name` to `V`.
///
/// Wraps `HashMap<Name, V>` for convenient use in the elaborator and kernel.
#[derive(Debug, Clone, Default)]
pub struct NameMap<V> {
    pub(super) inner: HashMap<Name, V>,
}
impl<V> NameMap<V> {
    /// Create an empty `NameMap`.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    /// Create a `NameMap` with the given capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(cap),
        }
    }
    /// Insert or overwrite a mapping.
    pub fn insert(&mut self, name: Name, value: V) -> Option<V> {
        self.inner.insert(name, value)
    }
    /// Get a reference to the value for `name`.
    pub fn get(&self, name: &Name) -> Option<&V> {
        self.inner.get(name)
    }
    /// Get a mutable reference to the value for `name`.
    pub fn get_mut(&mut self, name: &Name) -> Option<&mut V> {
        self.inner.get_mut(name)
    }
    /// Remove and return the value for `name`.
    pub fn remove(&mut self, name: &Name) -> Option<V> {
        self.inner.remove(name)
    }
    /// Check whether `name` has a mapping.
    pub fn contains_key(&self, name: &Name) -> bool {
        self.inner.contains_key(name)
    }
    /// Number of mappings.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Iterate over key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&Name, &V)> {
        self.inner.iter()
    }
    /// Iterate over keys.
    pub fn keys(&self) -> impl Iterator<Item = &Name> {
        self.inner.keys()
    }
    /// Iterate over values.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }
    /// Filter entries by a predicate on the name.
    pub fn filter_by_namespace(&self, ns: &Name) -> Vec<(&Name, &V)> {
        self.inner
            .iter()
            .filter(|(n, _)| n.has_prefix(ns))
            .collect()
    }
    /// Collect all names in sorted order.
    pub fn sorted_names(&self) -> Vec<Name> {
        let mut names: Vec<Name> = self.inner.keys().cloned().collect();
        names.sort();
        names
    }
    /// Get or insert a default value.
    pub fn entry_or_insert(&mut self, name: Name, value: V) -> &mut V {
        self.inner.entry(name).or_insert(value)
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
/// A name with an optional namespace alias.
///
/// Used for name resolution when the user writes `open Nat` and then `add`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedName {
    /// The full canonical name.
    pub canonical: Name,
    /// An optional shorter alias (e.g. after `open`).
    pub alias: Option<Name>,
}
impl QualifiedName {
    /// Create a new `QualifiedName` with no alias.
    pub fn new(canonical: Name) -> Self {
        Self {
            canonical,
            alias: None,
        }
    }
    /// Create a `QualifiedName` with an alias.
    pub fn with_alias(canonical: Name, alias: Name) -> Self {
        Self {
            canonical,
            alias: Some(alias),
        }
    }
    /// The preferred display name (alias if present, otherwise canonical).
    pub fn preferred(&self) -> &Name {
        self.alias.as_ref().unwrap_or(&self.canonical)
    }
}
/// An interning pool for `Name` strings.
#[allow(dead_code)]
pub struct NamePool {
    names: Vec<String>,
    index: std::collections::HashMap<String, usize>,
}
#[allow(dead_code)]
impl NamePool {
    /// Creates an empty name pool.
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            index: std::collections::HashMap::new(),
        }
    }
    /// Interns `name` and returns its ID.
    pub fn intern(&mut self, name: &str) -> usize {
        if let Some(&id) = self.index.get(name) {
            return id;
        }
        let id = self.names.len();
        self.names.push(name.to_string());
        self.index.insert(name.to_string(), id);
        id
    }
    /// Returns the name for `id`, or `None`.
    pub fn get(&self, id: usize) -> Option<&str> {
        self.names.get(id).map(|s| s.as_str())
    }
    /// Returns the number of interned names.
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Returns `true` if the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}
/// Resolves unqualified names to fully-qualified names given a namespace.
#[allow(dead_code)]
pub struct NameResolver {
    namespace: Vec<String>,
    registered: std::collections::HashSet<String>,
}
#[allow(dead_code)]
impl NameResolver {
    /// Creates a new resolver in the root namespace.
    pub fn new() -> Self {
        Self {
            namespace: Vec::new(),
            registered: std::collections::HashSet::new(),
        }
    }
    /// Registers a fully-qualified name.
    pub fn register(&mut self, fqn: impl Into<String>) {
        self.registered.insert(fqn.into());
    }
    /// Enters a sub-namespace.
    pub fn enter(&mut self, ns: impl Into<String>) {
        self.namespace.push(ns.into());
    }
    /// Exits the current sub-namespace.
    pub fn exit(&mut self) {
        self.namespace.pop();
    }
    /// Returns the current namespace as a dot-separated string.
    pub fn current_ns(&self) -> String {
        self.namespace.join(".")
    }
    /// Resolves `name` to a fully-qualified name.
    pub fn resolve(&self, name: &str) -> String {
        if self.namespace.is_empty() {
            return name.to_string();
        }
        let fqn = format!("{}.{}", self.namespace.join("."), name);
        if self.registered.contains(&fqn) {
            fqn
        } else {
            name.to_string()
        }
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
/// A hierarchical name.
///
/// Names are used to identify constants, inductives, and other declarations.
/// They form a tree structure: `Nat.add.comm` is represented as
/// `Str(Str(Str(Anonymous, "Nat"), "add"), "comm")`.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum Name {
    /// The anonymous (root) name.
    #[default]
    Anonymous,
    /// A string component: parent name + string.
    Str(Box<Name>, String),
    /// A numeric component: parent name + number.
    Num(Box<Name>, u64),
}
impl Name {
    /// Create a simple string name (no parent).
    pub fn str(s: impl Into<String>) -> Self {
        Name::Str(Box::new(Name::Anonymous), s.into())
    }
    /// Append a string component to this name.
    pub fn append_str(self, s: impl Into<String>) -> Self {
        Name::Str(Box::new(self), s.into())
    }
    /// Append a numeric component to this name.
    pub fn append_num(self, n: u64) -> Self {
        Name::Num(Box::new(self), n)
    }
    /// Check if this is the anonymous name.
    pub fn is_anonymous(&self) -> bool {
        matches!(self, Name::Anonymous)
    }
    /// Create a `Name` from a dot-separated string.
    ///
    /// `Name::from_str("Nat.add.comm")` produces the same as
    /// `Name::str("Nat").append_str("add").append_str("comm")`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let mut parts = s.split('.');
        let first = match parts.next() {
            None | Some("") => return Name::Anonymous,
            Some(f) => f,
        };
        let mut name = Name::str(first);
        for part in parts {
            if part.is_empty() {
                continue;
            }
            if let Ok(n) = part.parse::<u64>() {
                name = name.append_num(n);
            } else {
                name = name.append_str(part);
            }
        }
        name
    }
    /// Returns the depth (number of components) of this name.
    ///
    /// `Anonymous` has depth 0, `Name::str("Nat")` has depth 1,
    /// `Name::str("Nat").append_str("add")` has depth 2.
    pub fn depth(&self) -> usize {
        match self {
            Name::Anonymous => 0,
            Name::Str(parent, _) | Name::Num(parent, _) => 1 + parent.depth(),
        }
    }
    /// Return the last string component of this name, if any.
    ///
    /// For `Nat.add`, returns `Some("add")`.
    pub fn last_str(&self) -> Option<&str> {
        match self {
            Name::Anonymous => None,
            Name::Str(_, s) => Some(s.as_str()),
            Name::Num(parent, _) => parent.last_str(),
        }
    }
    /// Return the last numeric component, if any.
    pub fn last_num(&self) -> Option<u64> {
        match self {
            Name::Num(_, n) => Some(*n),
            Name::Str(parent, _) => parent.last_num(),
            Name::Anonymous => None,
        }
    }
    /// Return the root (top-level) component as a string.
    ///
    /// For `Nat.add.comm`, returns `"Nat"`.
    pub fn root(&self) -> Option<&str> {
        match self {
            Name::Anonymous => None,
            Name::Str(parent, s) => {
                if parent.is_anonymous() {
                    Some(s.as_str())
                } else {
                    parent.root()
                }
            }
            Name::Num(parent, _) => parent.root(),
        }
    }
    /// Return the parent name (prefix with last component removed).
    pub fn prefix(&self) -> Name {
        match self {
            Name::Anonymous => Name::Anonymous,
            Name::Str(parent, _) | Name::Num(parent, _) => *parent.clone(),
        }
    }
    /// Check whether this name has `prefix` as a (strict) prefix.
    ///
    /// `Nat.add.comm.has_prefix(Nat)` is `true`.
    pub fn has_prefix(&self, prefix: &Name) -> bool {
        if self == prefix {
            return false;
        }
        let mut current = self;
        loop {
            match current {
                Name::Anonymous => return false,
                Name::Str(parent, _) | Name::Num(parent, _) => {
                    if parent.as_ref() == prefix {
                        return true;
                    }
                    current = parent;
                }
            }
        }
    }
    /// Collect all components from root to leaf.
    ///
    /// Returns a vector of `(is_num, string_or_num)` pairs.
    pub fn components(&self) -> Vec<String> {
        let mut comps = Vec::new();
        let mut current = self;
        loop {
            match current {
                Name::Anonymous => break,
                Name::Str(parent, s) => {
                    comps.push(s.clone());
                    current = parent;
                }
                Name::Num(parent, n) => {
                    comps.push(n.to_string());
                    current = parent;
                }
            }
        }
        comps.reverse();
        comps
    }
    /// Reconstruct a `Name` from a list of string components.
    ///
    /// Numeric strings are converted to `Num` components.
    pub fn from_components(comps: &[String]) -> Self {
        let mut name = Name::Anonymous;
        for comp in comps {
            if let Ok(n) = comp.parse::<u64>() {
                name = name.append_num(n);
            } else {
                name = name.append_str(comp.as_str());
            }
        }
        name
    }
    /// Replace the last string component with `new_last`.
    ///
    /// If the name ends in a numeric component, appends `new_last` instead.
    pub fn replace_last(self, new_last: impl Into<String>) -> Self {
        match self {
            Name::Anonymous => Name::str(new_last),
            Name::Str(parent, _) => Name::Str(parent, new_last.into()),
            Name::Num(parent, _) => Name::Str(parent, new_last.into()),
        }
    }
    /// Produce a "fresh" version of this name by appending a suffix number.
    ///
    /// Used to avoid name collisions during elaboration.
    pub fn freshen(self, suffix: u64) -> Self {
        self.append_num(suffix)
    }
    /// Check whether this name is in the given namespace.
    ///
    /// A name is in namespace `ns` if it has `ns` as a strict prefix.
    pub fn in_namespace(&self, ns: &Name) -> bool {
        self.has_prefix(ns)
    }
    /// Append a suffix string to this name.
    pub fn with_suffix(self, suffix: impl Into<String>) -> Self {
        self.append_str(suffix)
    }
    /// Convert to a string suitable for use as a Rust identifier.
    ///
    /// Dots and special characters are replaced with underscores.
    pub fn to_ident_string(&self) -> String {
        self.to_string()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }
    /// Return a new name with the given prefix prepended.
    pub fn prepend(self, prefix: Name) -> Self {
        let comps = self.components();
        let mut name = prefix;
        for comp in comps {
            if let Ok(n) = comp.parse::<u64>() {
                name = name.append_num(n);
            } else {
                name = name.append_str(comp);
            }
        }
        name
    }
    /// Check whether this name is a string name (last component is a string).
    pub fn is_str_name(&self) -> bool {
        matches!(self, Name::Str(_, _))
    }
    /// Check whether this name is a numeric name (last component is a number).
    pub fn is_num_name(&self) -> bool {
        matches!(self, Name::Num(_, _))
    }
}
/// A bidirectional mapping between names and numeric IDs.
#[allow(dead_code)]
pub struct NameMapping {
    name_to_id: std::collections::HashMap<String, u32>,
    id_to_name: std::collections::HashMap<u32, String>,
    next_id: u32,
}
#[allow(dead_code)]
impl NameMapping {
    /// Creates an empty name mapping.
    pub fn new() -> Self {
        Self {
            name_to_id: std::collections::HashMap::new(),
            id_to_name: std::collections::HashMap::new(),
            next_id: 0,
        }
    }
    /// Registers `name` and returns its ID (or the existing ID).
    pub fn register(&mut self, name: impl Into<String>) -> u32 {
        let name = name.into();
        if let Some(&id) = self.name_to_id.get(&name) {
            return id;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.name_to_id.insert(name.clone(), id);
        self.id_to_name.insert(id, name);
        id
    }
    /// Returns the ID for `name`, or `None`.
    pub fn id_of(&self, name: &str) -> Option<u32> {
        self.name_to_id.get(name).copied()
    }
    /// Returns the name for `id`, or `None`.
    pub fn name_of(&self, id: u32) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }
    /// Returns the total number of registered names.
    pub fn len(&self) -> usize {
        self.name_to_id.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.name_to_id.is_empty()
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
/// A set of names with fast membership testing.
#[allow(dead_code)]
pub struct NameSetExt {
    inner: std::collections::HashSet<String>,
}
#[allow(dead_code)]
impl NameSetExt {
    /// Creates an empty name set.
    pub fn new() -> Self {
        Self {
            inner: std::collections::HashSet::new(),
        }
    }
    /// Inserts `name`.
    pub fn insert(&mut self, name: impl Into<String>) {
        self.inner.insert(name.into());
    }
    /// Returns `true` if `name` is in the set.
    pub fn contains(&self, name: &str) -> bool {
        self.inner.contains(name)
    }
    /// Removes `name`.
    pub fn remove(&mut self, name: &str) {
        self.inner.remove(name);
    }
    /// Returns the number of names.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Returns the union with another set.
    pub fn union(&self, other: &NameSetExt) -> NameSetExt {
        let mut result = NameSetExt::new();
        for n in &self.inner {
            result.insert(n.as_str());
        }
        for n in &other.inner {
            result.insert(n.as_str());
        }
        result
    }
    /// Returns the intersection with another set.
    pub fn intersect(&self, other: &NameSetExt) -> NameSetExt {
        let mut result = NameSetExt::new();
        for n in &self.inner {
            if other.contains(n) {
                result.insert(n.as_str());
            }
        }
        result
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
/// A trie (prefix tree) for efficient prefix search over names.
#[allow(dead_code)]
pub struct NameTrieExt {
    children: std::collections::HashMap<char, NameTrieExt>,
    terminal: bool,
    name: Option<String>,
}
#[allow(dead_code)]
impl NameTrieExt {
    /// Creates an empty trie.
    pub fn new() -> Self {
        Self {
            children: std::collections::HashMap::new(),
            terminal: false,
            name: None,
        }
    }
    /// Inserts a name into the trie.
    pub fn insert(&mut self, name: &str) {
        let mut node = self;
        for c in name.chars() {
            node = node.children.entry(c).or_default();
        }
        node.terminal = true;
        node.name = Some(name.to_string());
    }
    /// Returns `true` if `name` is in the trie.
    pub fn contains(&self, name: &str) -> bool {
        let mut node = self;
        for c in name.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.terminal
    }
    /// Collects all names with the given prefix.
    pub fn with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return Vec::new(),
            }
        }
        let mut results = Vec::new();
        node.collect_all(&mut results);
        results
    }
    fn collect_all(&self, out: &mut Vec<String>) {
        if let Some(ref n) = self.name {
            out.push(n.clone());
        }
        for child in self.children.values() {
            let c: &NameTrieExt = child;
            c.collect_all(out);
        }
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
/// A set of `Name` values.
///
/// Wraps `HashSet<Name>` for convenient use in the elaborator.
#[derive(Debug, Clone, Default)]
pub struct NameSet {
    pub(super) inner: HashSet<Name>,
}
impl NameSet {
    /// Create an empty `NameSet`.
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
    /// Insert a name. Returns `true` if it was newly inserted.
    pub fn insert(&mut self, name: Name) -> bool {
        self.inner.insert(name)
    }
    /// Remove a name. Returns `true` if it was present.
    pub fn remove(&mut self, name: &Name) -> bool {
        self.inner.remove(name)
    }
    /// Check whether the set contains `name`.
    pub fn contains(&self, name: &Name) -> bool {
        self.inner.contains(name)
    }
    /// Number of names in the set.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Iterate over all names.
    pub fn iter(&self) -> impl Iterator<Item = &Name> {
        self.inner.iter()
    }
    /// Compute the union with another `NameSet`.
    pub fn union(&self, other: &NameSet) -> NameSet {
        NameSet {
            inner: self.inner.union(&other.inner).cloned().collect(),
        }
    }
    /// Compute the intersection with another `NameSet`.
    pub fn intersection(&self, other: &NameSet) -> NameSet {
        NameSet {
            inner: self.inner.intersection(&other.inner).cloned().collect(),
        }
    }
    /// Compute the difference `self \ other`.
    pub fn difference(&self, other: &NameSet) -> NameSet {
        NameSet {
            inner: self.inner.difference(&other.inner).cloned().collect(),
        }
    }
    /// Filter to names in a given namespace.
    pub fn in_namespace(&self, ns: &Name) -> NameSet {
        NameSet {
            inner: self
                .inner
                .iter()
                .filter(|n| n.has_prefix(ns))
                .cloned()
                .collect(),
        }
    }
    /// Convert to a sorted vector.
    pub fn to_sorted_vec(&self) -> Vec<Name> {
        let mut v: Vec<Name> = self.inner.iter().cloned().collect();
        v.sort();
        v
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
/// Generates fresh unique names.
#[allow(dead_code)]
pub struct NameGeneratorExt {
    prefix: String,
    counter: u64,
}
#[allow(dead_code)]
impl NameGeneratorExt {
    /// Creates a generator using the given prefix.
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            counter: 0,
        }
    }
    /// Returns the next fresh name.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> String {
        let n = self.counter;
        self.counter += 1;
        format!("{}{}", self.prefix, n)
    }
    /// Returns the number of names generated.
    pub fn count(&self) -> u64 {
        self.counter
    }
    /// Resets the counter to zero.
    pub fn reset(&mut self) {
        self.counter = 0;
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
/// A dot-separated qualified name (e.g. `Nat.succ`).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct QualifiedNameExt {
    /// Component parts of the name.
    pub parts: Vec<String>,
}
#[allow(dead_code)]
impl QualifiedNameExt {
    /// Creates a qualified name from a dot-separated string.
    pub fn from_dot_str(s: &str) -> Self {
        s.parse().unwrap_or_else(|_| unreachable!())
    }
    /// Creates a single-component name.
    pub fn simple(name: impl Into<String>) -> Self {
        Self {
            parts: vec![name.into()],
        }
    }
    /// Returns the last component (unqualified name).
    pub fn unqualified(&self) -> &str {
        self.parts.last().map(|s| s.as_str()).unwrap_or("")
    }
    /// Returns the namespace (all but the last component), or `None`.
    pub fn namespace(&self) -> Option<QualifiedNameExt> {
        if self.parts.len() <= 1 {
            return None;
        }
        let parts = self.parts[..self.parts.len() - 1].to_vec();
        Some(QualifiedNameExt { parts })
    }
    /// Returns `true` if this name is a sub-name of `other`.
    pub fn is_sub_of(&self, other: &QualifiedNameExt) -> bool {
        self.parts.starts_with(&other.parts)
    }
    /// Returns the number of components.
    pub fn depth(&self) -> usize {
        self.parts.len()
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
