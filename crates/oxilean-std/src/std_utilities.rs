//! Extended standard library utility types: Span, Located, NameTable,
//! DiagnosticLevel, Diagnostic, DiagnosticBag, ScopeTable, Counter,
//! FreshNameGen, StringSet, MultiMap, Trie, BitSet64, MinHeap, DirectedGraph.

#![allow(dead_code)]
#![allow(missing_docs)]

// ── Span ─────────────────────────────────────────────────────────────────────

/// Utility type for carrying source-location metadata.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// Byte offset of the first character.
    pub start: usize,
    /// Byte offset one past the last character.
    pub end: usize,
    /// 1-based line number of the start.
    pub line: u32,
    /// 1-based column number of the start.
    pub column: u32,
}

impl Span {
    /// Create a new span.
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize, line: u32, column: u32) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }

    /// Create a dummy span (all zeros).
    #[allow(dead_code)]
    pub fn dummy() -> Self {
        Self {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }

    /// Return the length in bytes.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Return `true` if the span covers zero bytes.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Merge two spans: from the earlier start to the later end.
    #[allow(dead_code)]
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line.min(other.line),
            column: self.column,
        }
    }
}

/// A value annotated with a `Span`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Located<T> {
    /// The wrapped value.
    pub value: T,
    /// The source span.
    pub span: Span,
}

impl<T> Located<T> {
    /// Wrap `value` with a `span`.
    #[allow(dead_code)]
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    /// Wrap `value` with a dummy span.
    #[allow(dead_code)]
    pub fn dummy(value: T) -> Self {
        Self {
            value,
            span: Span::dummy(),
        }
    }

    /// Map over the inner value.
    #[allow(dead_code)]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Located<U> {
        Located {
            value: f(self.value),
            span: self.span,
        }
    }

    /// Return a reference to the inner value.
    #[allow(dead_code)]
    pub fn as_ref(&self) -> Located<&T> {
        Located {
            value: &self.value,
            span: self.span.clone(),
        }
    }
}

// ── Simple name-interning table ───────────────────────────────────────────────

/// A simple string-interning table backed by a `Vec`.
///
/// Useful for giving cheap `usize` IDs to string names during elaboration.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct NameTable {
    names: Vec<String>,
}

impl NameTable {
    /// Create an empty table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Intern `name` and return its ID.  If already present, returns the
    /// existing ID without inserting a duplicate.
    #[allow(dead_code)]
    pub fn intern(&mut self, name: &str) -> usize {
        if let Some(pos) = self.names.iter().position(|n| n == name) {
            return pos;
        }
        let id = self.names.len();
        self.names.push(name.to_owned());
        id
    }

    /// Look up the string for an ID.
    #[allow(dead_code)]
    pub fn lookup(&self, id: usize) -> Option<&str> {
        self.names.get(id).map(String::as_str)
    }

    /// Return the number of interned names.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Return `true` if the table is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Clear all entries.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.names.clear();
    }

    /// Return an iterator over `(id, name)` pairs.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (usize, &str)> {
        self.names.iter().enumerate().map(|(i, s)| (i, s.as_str()))
    }
}

// ── DiagnosticLevel ──────────────────────────────────────────────────────────

/// Severity levels for compiler diagnostics.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    /// Informational note; does not prevent compilation.
    Note,
    /// Warning; compilation continues.
    Warning,
    /// Error; compilation should stop.
    Error,
    /// Internal compiler error (ICE).
    Bug,
}

impl DiagnosticLevel {
    /// Return a short label string.
    #[allow(dead_code)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Bug => "internal compiler error",
        }
    }

    /// Return `true` if this level prevents a successful build.
    #[allow(dead_code)]
    pub fn is_fatal(self) -> bool {
        matches!(self, Self::Error | Self::Bug)
    }
}

/// A single compiler diagnostic.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Severity level.
    pub level: DiagnosticLevel,
    /// Human-readable message.
    pub message: String,
    /// Optional source span.
    pub span: Option<Span>,
    /// Optional help/hint text.
    pub help: Option<String>,
}

impl Diagnostic {
    /// Construct an error diagnostic.
    #[allow(dead_code)]
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    /// Construct a warning.
    #[allow(dead_code)]
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    /// Construct a note.
    #[allow(dead_code)]
    pub fn note(message: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Note,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    /// Attach a source span.
    #[allow(dead_code)]
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Attach a help string.
    #[allow(dead_code)]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Return `true` if this diagnostic is fatal.
    #[allow(dead_code)]
    pub fn is_fatal(&self) -> bool {
        self.level.is_fatal()
    }
}

/// Accumulator for multiple diagnostics.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DiagnosticBag {
    items: Vec<Diagnostic>,
}

impl DiagnosticBag {
    /// Create an empty bag.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a diagnostic.
    #[allow(dead_code)]
    pub fn push(&mut self, diag: Diagnostic) {
        self.items.push(diag);
    }

    /// Return `true` if there are any fatal diagnostics.
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.is_fatal())
    }

    /// Return the number of accumulated diagnostics.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Return `true` if the bag is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Drain all diagnostics, returning them in order.
    #[allow(dead_code)]
    pub fn drain(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.items)
    }

    /// Iterate over diagnostics.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.items.iter()
    }
}

// ── Simple scoped symbol table ────────────────────────────────────────────────

/// A scoped symbol table supporting nested scopes.
///
/// Each `push_scope` / `pop_scope` pair delimits a lexical scope.  Lookups
/// search from the innermost scope outward.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ScopeTable<K, V> {
    scopes: Vec<Vec<(K, V)>>,
}

impl<K: Eq, V: Clone> ScopeTable<K, V> {
    /// Create a table with a single (global) scope.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
        }
    }

    /// Push a new nested scope.
    #[allow(dead_code)]
    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    /// Pop the innermost scope, discarding its bindings.
    /// Panics if called on the root scope.
    #[allow(dead_code)]
    pub fn pop_scope(&mut self) {
        assert!(self.scopes.len() > 1, "cannot pop root scope");
        self.scopes.pop();
    }

    /// Bind `key` → `value` in the current (innermost) scope.
    #[allow(dead_code)]
    pub fn define(&mut self, key: K, value: V) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.push((key, value));
        }
    }

    /// Look up `key`, searching from innermost to outermost scope.
    #[allow(dead_code)]
    pub fn lookup(&self, key: &K) -> Option<&V> {
        for scope in self.scopes.iter().rev() {
            for (k, v) in scope.iter().rev() {
                if k == key {
                    return Some(v);
                }
            }
        }
        None
    }

    /// Return `true` if `key` is defined in the current scope only.
    #[allow(dead_code)]
    pub fn defined_locally(&self, key: &K) -> bool {
        if let Some(scope) = self.scopes.last() {
            scope.iter().any(|(k, _)| k == key)
        } else {
            false
        }
    }

    /// Current depth (1 = global scope only).
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
}

impl<K: Eq, V: Clone> Default for ScopeTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// ── Counter utilities ─────────────────────────────────────────────────────────

/// A monotonically increasing counter, useful for generating fresh variable IDs.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Counter {
    next: u64,
}

impl Counter {
    /// Create a counter starting at zero.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a counter starting at `start`.
    #[allow(dead_code)]
    pub fn starting_at(start: u64) -> Self {
        Self { next: start }
    }

    /// Return the next value and advance the counter.
    #[allow(dead_code)]
    pub fn next(&mut self) -> u64 {
        let val = self.next;
        self.next += 1;
        val
    }

    /// Peek at the current value without advancing.
    #[allow(dead_code)]
    pub fn peek(&self) -> u64 {
        self.next
    }

    /// Reset the counter to zero.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.next = 0;
    }
}

// ── FreshName generator ───────────────────────────────────────────────────────

/// Generates fresh name strings of the form `prefix_N`.
#[allow(dead_code)]
#[derive(Debug)]
pub struct FreshNameGen {
    prefix: String,
    counter: Counter,
}

impl FreshNameGen {
    /// Create a generator with the given prefix.
    #[allow(dead_code)]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            counter: Counter::new(),
        }
    }

    /// Return the next fresh name.
    #[allow(dead_code)]
    pub fn fresh(&mut self) -> String {
        let n = self.counter.next();
        format!("{}_{}", self.prefix, n)
    }

    /// Return the next fresh name as a `&'static str`-compatible owned `String`.
    #[allow(dead_code)]
    pub fn fresh_str(&mut self) -> String {
        self.fresh()
    }

    /// Reset the counter (reuse names — use with caution).
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.counter.reset();
    }
}

// ── StringSet (ordered, for deterministic output) ────────────────────────────

/// A set of `String` values backed by a sorted `Vec` for deterministic output.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct StringSet {
    items: Vec<String>,
}

impl StringSet {
    /// Create an empty set.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert `item`.  No-op if already present.  Returns `true` if new.
    #[allow(dead_code)]
    pub fn insert(&mut self, item: impl Into<String>) -> bool {
        let s = item.into();
        match self.items.binary_search(&s) {
            Ok(_) => false,
            Err(pos) => {
                self.items.insert(pos, s);
                true
            }
        }
    }

    /// Return `true` if `item` is in the set.
    #[allow(dead_code)]
    pub fn contains(&self, item: &str) -> bool {
        self.items
            .binary_search_by_key(&item, String::as_str)
            .is_ok()
    }

    /// Remove `item`.  Returns `true` if it was present.
    #[allow(dead_code)]
    pub fn remove(&mut self, item: &str) -> bool {
        match self.items.binary_search_by_key(&item, String::as_str) {
            Ok(pos) => {
                self.items.remove(pos);
                true
            }
            Err(_) => false,
        }
    }

    /// Return the number of elements.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Return `true` if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterate over items in sorted order.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.items.iter().map(String::as_str)
    }

    /// Compute the union of `self` and `other`.
    #[allow(dead_code)]
    pub fn union(&self, other: &StringSet) -> StringSet {
        let mut result = self.clone();
        for item in other.iter() {
            result.insert(item);
        }
        result
    }

    /// Compute the intersection of `self` and `other`.
    #[allow(dead_code)]
    pub fn intersection(&self, other: &StringSet) -> StringSet {
        let mut result = StringSet::new();
        for item in self.iter() {
            if other.contains(item) {
                result.insert(item);
            }
        }
        result
    }

    /// Compute the difference `self \ other`.
    #[allow(dead_code)]
    pub fn difference(&self, other: &StringSet) -> StringSet {
        let mut result = StringSet::new();
        for item in self.iter() {
            if !other.contains(item) {
                result.insert(item);
            }
        }
        result
    }
}

// ── Multi-map ─────────────────────────────────────────────────────────────────

/// A simple multi-map: each key may map to multiple values.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MultiMap<K, V> {
    inner: Vec<(K, Vec<V>)>,
}

impl<K, V> Default for MultiMap<K, V> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<K: Eq, V> MultiMap<K, V> {
    /// Create an empty multi-map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a `(key, value)` pair.
    #[allow(dead_code)]
    pub fn insert(&mut self, key: K, value: V) {
        for (k, vs) in &mut self.inner {
            if k == &key {
                vs.push(value);
                return;
            }
        }
        self.inner.push((key, vec![value]));
    }

    /// Return all values associated with `key`.
    #[allow(dead_code)]
    pub fn get(&self, key: &K) -> &[V] {
        for (k, vs) in &self.inner {
            if k == key {
                return vs;
            }
        }
        &[]
    }

    /// Return `true` if `key` has at least one associated value.
    #[allow(dead_code)]
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.iter().any(|(k, _)| k == key)
    }

    /// Return the number of distinct keys.
    #[allow(dead_code)]
    pub fn key_count(&self) -> usize {
        self.inner.len()
    }

    /// Remove all entries for `key`.  Returns the removed values.
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &K) -> Vec<V> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < self.inner.len() {
            if &self.inner[i].0 == key {
                let (_, vs) = self.inner.remove(i);
                result = vs;
            } else {
                i += 1;
            }
        }
        result
    }

    /// Iterate over `(key, values)` pairs.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (&K, &[V])> {
        self.inner.iter().map(|(k, vs)| (k, vs.as_slice()))
    }
}

// ── Trie (prefix tree) ────────────────────────────────────────────────────────

/// A simple trie mapping byte strings to values.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Trie<V> {
    children: Vec<(u8, Trie<V>)>,
    value: Option<V>,
}

impl<V> Trie<V> {
    /// Create an empty trie node.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            value: None,
        }
    }

    /// Insert `key` → `value`.
    #[allow(dead_code)]
    pub fn insert(&mut self, key: &[u8], value: V) {
        if let Some((first, rest)) = key.split_first() {
            let child = self.get_or_create_child(*first);
            child.insert(rest, value);
        } else {
            self.value = Some(value);
        }
    }

    /// Look up `key` and return a reference to the associated value, if any.
    #[allow(dead_code)]
    pub fn get(&self, key: &[u8]) -> Option<&V> {
        if let Some((first, rest)) = key.split_first() {
            for (b, child) in &self.children {
                if *b == *first {
                    return child.get(rest);
                }
            }
            None
        } else {
            self.value.as_ref()
        }
    }

    /// Return `true` if `key` is present.
    #[allow(dead_code)]
    pub fn contains(&self, key: &[u8]) -> bool {
        self.get(key).is_some()
    }

    /// Return all keys with the given `prefix`.
    #[allow(dead_code)]
    pub fn keys_with_prefix(&self, prefix: &[u8]) -> Vec<Vec<u8>> {
        match prefix.split_first() {
            Some((first, rest)) => {
                for (b, child) in &self.children {
                    if *b == *first {
                        return child
                            .keys_with_prefix(rest)
                            .into_iter()
                            .map(|mut k| {
                                k.insert(0, *first);
                                k
                            })
                            .collect();
                    }
                }
                Vec::new()
            }
            None => self.collect_all(Vec::new()),
        }
    }

    fn get_or_create_child(&mut self, byte: u8) -> &mut Trie<V> {
        for i in 0..self.children.len() {
            if self.children[i].0 == byte {
                return &mut self.children[i].1;
            }
        }
        self.children.push((byte, Trie::new()));
        let last = self.children.len() - 1;
        &mut self.children[last].1
    }

    fn collect_all(&self, prefix: Vec<u8>) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        if self.value.is_some() {
            result.push(prefix.clone());
        }
        for (b, child) in &self.children {
            let mut p = prefix.clone();
            p.push(*b);
            result.extend(child.collect_all(p));
        }
        result
    }
}

impl<V> Default for Trie<V> {
    fn default() -> Self {
        Self::new()
    }
}

// ── BitSet (fixed-width 64-bit) ───────────────────────────────────────────────

/// A fixed-size bit set backed by a single `u64`.  Supports positions 0..63.
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BitSet64(u64);

impl BitSet64 {
    /// Empty set.
    #[allow(dead_code)]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Full set (all 64 bits set).
    #[allow(dead_code)]
    pub const fn full() -> Self {
        Self(u64::MAX)
    }

    /// Set the bit at `pos`.
    #[allow(dead_code)]
    pub fn set(&mut self, pos: u8) {
        debug_assert!(pos < 64);
        self.0 |= 1u64 << pos;
    }

    /// Clear the bit at `pos`.
    #[allow(dead_code)]
    pub fn clear(&mut self, pos: u8) {
        debug_assert!(pos < 64);
        self.0 &= !(1u64 << pos);
    }

    /// Test whether the bit at `pos` is set.
    #[allow(dead_code)]
    pub fn test(&self, pos: u8) -> bool {
        debug_assert!(pos < 64);
        (self.0 >> pos) & 1 == 1
    }

    /// Return the number of set bits.
    #[allow(dead_code)]
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Return `true` if no bits are set.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Compute bitwise AND.
    #[allow(dead_code)]
    pub fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Compute bitwise OR.
    #[allow(dead_code)]
    pub fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Compute bitwise XOR.
    #[allow(dead_code)]
    pub fn xor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Compute bitwise NOT.
    #[allow(dead_code)]
    pub fn not(self) -> Self {
        Self(!self.0)
    }

    /// Iterate over set bit positions.
    #[allow(dead_code)]
    pub fn iter_ones(self) -> impl Iterator<Item = u8> {
        (0u8..64).filter(move |&i| self.test(i))
    }
}

// ── PriorityQueue ─────────────────────────────────────────────────────────────

/// A min-heap priority queue.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MinHeap<P: Ord, V> {
    heap: Vec<(P, V)>,
}

impl<P: Ord, V> MinHeap<P, V> {
    /// Create an empty heap.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    /// Push `(priority, value)` onto the heap.
    #[allow(dead_code)]
    pub fn push(&mut self, priority: P, value: V) {
        self.heap.push((priority, value));
        let mut i = self.heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.heap[parent].0 > self.heap[i].0 {
                self.heap.swap(parent, i);
                i = parent;
            } else {
                break;
            }
        }
    }

    /// Pop the minimum-priority element.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<(P, V)> {
        if self.heap.is_empty() {
            return None;
        }
        let n = self.heap.len();
        self.heap.swap(0, n - 1);
        let min = self.heap.pop();
        self.sift_down(0);
        min
    }

    /// Peek at the minimum-priority element without removing it.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<(&P, &V)> {
        self.heap.first().map(|(p, v)| (p, v))
    }

    /// Return the number of elements.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Return `true` if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;
            if left < n && self.heap[left].0 < self.heap[smallest].0 {
                smallest = left;
            }
            if right < n && self.heap[right].0 < self.heap[smallest].0 {
                smallest = right;
            }
            if smallest == i {
                break;
            }
            self.heap.swap(i, smallest);
            i = smallest;
        }
    }
}

// ── Graph (adjacency list) ────────────────────────────────────────────────────

/// A directed graph with `n` nodes, represented as an adjacency list.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DirectedGraph {
    adj: Vec<Vec<usize>>,
}

impl DirectedGraph {
    /// Create a graph with `n` nodes and no edges.
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
        }
    }

    /// Add a directed edge `u → v`.
    #[allow(dead_code)]
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }

    /// Return the number of nodes.
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.adj.len()
    }

    /// Return the out-degree of node `u`.
    #[allow(dead_code)]
    pub fn out_degree(&self, u: usize) -> usize {
        self.adj[u].len()
    }

    /// Iterate over the successors of `u`.
    #[allow(dead_code)]
    pub fn successors(&self, u: usize) -> &[usize] {
        &self.adj[u]
    }

    /// Compute a topological ordering using Kahn's algorithm.
    /// Returns `None` if the graph contains a cycle.
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.adj.len();
        let mut in_deg = vec![0usize; n];
        for u in 0..n {
            for &v in &self.adj[u] {
                in_deg[v] += 1;
            }
        }
        let mut queue: std::collections::VecDeque<usize> =
            (0..n).filter(|&u| in_deg[u] == 0).collect();
        let mut order = Vec::new();
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &self.adj[u] {
                in_deg[v] -= 1;
                if in_deg[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        if order.len() == n {
            Some(order)
        } else {
            None
        }
    }

    /// Compute strongly connected components using Kosaraju's algorithm.
    #[allow(dead_code)]
    pub fn strongly_connected_components(&self) -> Vec<Vec<usize>> {
        let n = self.adj.len();
        // Pass 1: finish-time order
        let mut visited = vec![false; n];
        let mut finish_order = Vec::new();
        for start in 0..n {
            if !visited[start] {
                self.dfs_finish(start, &mut visited, &mut finish_order);
            }
        }
        // Build reverse graph
        let mut rev = vec![Vec::new(); n];
        for u in 0..n {
            for &v in &self.adj[u] {
                rev[v].push(u);
            }
        }
        // Pass 2: assign SCCs in reverse finish order
        let mut comp = vec![usize::MAX; n];
        let mut scc_id = 0;
        for &start in finish_order.iter().rev() {
            if comp[start] == usize::MAX {
                let mut stack = vec![start];
                while let Some(u) = stack.pop() {
                    if comp[u] != usize::MAX {
                        continue;
                    }
                    comp[u] = scc_id;
                    for &v in &rev[u] {
                        if comp[v] == usize::MAX {
                            stack.push(v);
                        }
                    }
                }
                scc_id += 1;
            }
        }
        let mut sccs: Vec<Vec<usize>> = vec![Vec::new(); scc_id];
        for u in 0..n {
            sccs[comp[u]].push(u);
        }
        sccs
    }

    fn dfs_finish(&self, u: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        let mut stack = vec![(u, 0usize)];
        while let Some((node, idx)) = stack.last_mut() {
            let _node = *node;
            if !visited[_node] {
                visited[_node] = true;
            }
            if *idx < self.adj[_node].len() {
                let next = self.adj[_node][*idx];
                *idx += 1;
                if !visited[next] {
                    stack.push((next, 0));
                }
            } else {
                order.push(_node);
                stack.pop();
            }
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_merge() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(3, 10, 1, 4);
        let m = a.merge(&b);
        assert_eq!(m.start, 0);
        assert_eq!(m.end, 10);
    }

    #[test]
    fn test_located_map() {
        let l = Located::dummy(42u32);
        let l2 = l.map(|x| x * 2);
        assert_eq!(l2.value, 84);
    }

    #[test]
    fn test_name_table() {
        let mut t = NameTable::new();
        let id_a = t.intern("alpha");
        let id_b = t.intern("beta");
        let id_a2 = t.intern("alpha");
        assert_eq!(id_a, id_a2);
        assert_ne!(id_a, id_b);
        assert_eq!(t.lookup(id_a), Some("alpha"));
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn test_diagnostic_bag() {
        let mut bag = DiagnosticBag::new();
        assert!(!bag.has_errors());
        bag.push(Diagnostic::warning("minor issue"));
        assert!(!bag.has_errors());
        bag.push(Diagnostic::error("fatal problem"));
        assert!(bag.has_errors());
        assert_eq!(bag.len(), 2);
        let drained = bag.drain();
        assert_eq!(drained.len(), 2);
        assert!(bag.is_empty());
    }

    #[test]
    fn test_scope_table() {
        let mut s: ScopeTable<&str, u32> = ScopeTable::new();
        s.define("x", 1);
        s.push_scope();
        s.define("x", 2);
        assert_eq!(s.lookup(&"x"), Some(&2));
        s.pop_scope();
        assert_eq!(s.lookup(&"x"), Some(&1));
    }

    #[test]
    fn test_counter_and_fresh_name() {
        let mut c = Counter::new();
        assert_eq!(c.next(), 0);
        assert_eq!(c.next(), 1);
        assert_eq!(c.peek(), 2);
        c.reset();
        assert_eq!(c.next(), 0);

        let mut gen = FreshNameGen::new("var");
        let n0 = gen.fresh();
        let n1 = gen.fresh();
        assert_eq!(n0, "var_0");
        assert_eq!(n1, "var_1");
    }

    #[test]
    fn test_string_set_operations() {
        let mut s = StringSet::new();
        assert!(s.insert("banana"));
        assert!(s.insert("apple"));
        assert!(!s.insert("apple")); // duplicate
        assert!(s.contains("apple"));
        assert!(!s.contains("cherry"));
        assert_eq!(s.len(), 2);
        assert!(s.remove("apple"));
        assert!(!s.contains("apple"));
        let mut t = StringSet::new();
        t.insert("cherry");
        t.insert("banana");
        let u = s.union(&t);
        assert!(u.contains("banana"));
        assert!(u.contains("cherry"));
    }

    #[test]
    fn test_multi_map() {
        let mut m: MultiMap<&str, u32> = MultiMap::new();
        m.insert("key", 1);
        m.insert("key", 2);
        m.insert("other", 3);
        assert_eq!(m.get(&"key"), &[1, 2]);
        assert_eq!(m.key_count(), 2);
        let removed = m.remove(&"key");
        assert_eq!(removed, vec![1, 2]);
        assert!(!m.contains_key(&"key"));
    }

    #[test]
    fn test_trie() {
        let mut t: Trie<u32> = Trie::new();
        t.insert(b"hello", 1);
        t.insert(b"help", 2);
        t.insert(b"world", 3);
        assert_eq!(t.get(b"hello"), Some(&1));
        assert_eq!(t.get(b"help"), Some(&2));
        assert!(t.get(b"helo").is_none());
        assert!(t.contains(b"world"));
        let pfx = t.keys_with_prefix(b"hel");
        assert_eq!(pfx.len(), 2);
    }

    #[test]
    fn test_bitset64() {
        let mut bs = BitSet64::empty();
        assert!(bs.is_empty());
        bs.set(5);
        bs.set(10);
        assert!(bs.test(5));
        assert!(bs.test(10));
        assert!(!bs.test(0));
        assert_eq!(bs.count(), 2);
        bs.clear(5);
        assert!(!bs.test(5));
        let ones: Vec<u8> = bs.iter_ones().collect();
        assert_eq!(ones, vec![10]);
    }

    #[test]
    fn test_min_heap() {
        let mut heap: MinHeap<u32, &str> = MinHeap::new();
        heap.push(5, "five");
        heap.push(1, "one");
        heap.push(3, "three");
        assert_eq!(heap.len(), 3);
        let (p, v) = heap.pop().expect("pop should succeed");
        assert_eq!(p, 1);
        assert_eq!(v, "one");
        let (p2, _) = heap.pop().expect("pop should succeed");
        assert_eq!(p2, 3);
    }

    #[test]
    fn test_directed_graph_topo_sort() {
        let mut g = DirectedGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        let order = g.topological_sort().expect("should be a DAG");
        assert_eq!(order.len(), 4);
        // 0 must come before 1,2; 1 and 2 before 3
        let pos: Vec<usize> = {
            let mut p = vec![0usize; 4];
            for (i, &node) in order.iter().enumerate() {
                p[node] = i;
            }
            p
        };
        assert!(pos[0] < pos[1]);
        assert!(pos[0] < pos[2]);
        assert!(pos[1] < pos[3]);
        assert!(pos[2] < pos[3]);
    }

    #[test]
    fn test_directed_graph_scc() {
        // 0 → 1 → 2 → 0, 3 (separate)
        let mut g = DirectedGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0);
        // node 3 is isolated
        let sccs = g.strongly_connected_components();
        // Should have 2 SCCs: {0,1,2} and {3}
        assert_eq!(sccs.len(), 2);
    }

    #[test]
    fn test_diagnostic_level_ordering() {
        assert!(DiagnosticLevel::Note < DiagnosticLevel::Warning);
        assert!(DiagnosticLevel::Warning < DiagnosticLevel::Error);
        assert!(DiagnosticLevel::Error < DiagnosticLevel::Bug);
        assert!(DiagnosticLevel::Error.is_fatal());
        assert!(!DiagnosticLevel::Warning.is_fatal());
    }
}
