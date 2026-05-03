//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{count_default_modules, count_total_modules, modules_for_phase};

/// A simple multi-map: each key may map to multiple values.
#[allow(dead_code)]
#[derive(Debug)]
pub struct MultiMap<K, V> {
    pub(super) inner: Vec<(K, Vec<V>)>,
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
/// A category tag for standard library modules.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StdCategory {
    /// Core arithmetic (Nat, Int).
    Arithmetic,
    /// Logic (Prop, And, Or, Not, Iff).
    Logic,
    /// Data structures (List, Array, Option, etc.).
    Data,
    /// Type classes (Eq, Ord, Functor, etc.).
    TypeClass,
    /// IO and system operations.
    IO,
    /// String operations.
    String,
    /// Tactics and proof automation.
    Tactic,
    /// Universe polymorphism.
    Universe,
}
impl StdCategory {
    /// Human-readable label.
    #[allow(dead_code)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Arithmetic => "Arithmetic",
            Self::Logic => "Logic",
            Self::Data => "Data",
            Self::TypeClass => "TypeClass",
            Self::IO => "IO",
            Self::String => "String",
            Self::Tactic => "Tactic",
            Self::Universe => "Universe",
        }
    }
}
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
        let mut visited = vec![false; n];
        let mut finish_order = Vec::new();
        for start in 0..n {
            if !visited[start] {
                self.dfs_finish(start, &mut visited, &mut finish_order);
            }
        }
        let mut rev = vec![Vec::new(); n];
        for u in 0..n {
            for &v in &self.adj[u] {
                rev[v].push(u);
            }
        }
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
/// A dependency pair: (dependent, dependency).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ModuleDep {
    /// The module that depends on another.
    pub dependent: &'static str,
    /// The module that must be built first.
    pub dependency: &'static str,
}
/// Feature flags for optional standard library components.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StdFeatures {
    /// Enable classical logic axioms (excluded middle, choice).
    pub classical: bool,
    /// Enable propext (propositional extensionality).
    pub propext: bool,
    /// Enable funext (function extensionality).
    pub funext: bool,
    /// Enable quotient types.
    pub quotient: bool,
    /// Enable experimental category theory module.
    pub category_theory: bool,
    /// Enable topology module.
    pub topology: bool,
    /// Enable real number support.
    pub reals: bool,
}
impl StdFeatures {
    /// Create the default feature set (classical logic enabled by default).
    #[allow(dead_code)]
    pub fn default_features() -> Self {
        Self {
            classical: true,
            propext: true,
            funext: true,
            quotient: false,
            category_theory: false,
            topology: false,
            reals: false,
        }
    }
    /// Create the full feature set.
    #[allow(dead_code)]
    pub fn full() -> Self {
        Self {
            classical: true,
            propext: true,
            funext: true,
            quotient: true,
            category_theory: true,
            topology: true,
            reals: true,
        }
    }
    /// Count how many features are enabled.
    #[allow(dead_code)]
    pub fn count_enabled(&self) -> usize {
        [
            self.classical,
            self.propext,
            self.funext,
            self.quotient,
            self.category_theory,
            self.topology,
            self.reals,
        ]
        .iter()
        .filter(|&&v| v)
        .count()
    }
}
/// Standard library module statistics.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StdLibStats {
    /// Total modules registered.
    pub total_modules: usize,
    /// Modules enabled by default.
    pub default_modules: usize,
    /// Modules per build phase.
    pub per_phase: [usize; 5],
}
impl StdLibStats {
    /// Compute statistics from the registry.
    #[allow(dead_code)]
    pub fn compute() -> Self {
        let total = count_total_modules();
        let defaults = count_default_modules();
        let phases = [
            modules_for_phase(BuildPhase::Primitives).len(),
            modules_for_phase(BuildPhase::Collections).len(),
            modules_for_phase(BuildPhase::TypeClasses).len(),
            modules_for_phase(BuildPhase::Theorems).len(),
            modules_for_phase(BuildPhase::Automation).len(),
        ];
        Self {
            total_modules: total,
            default_modules: defaults,
            per_phase: phases,
        }
    }
    /// Check if all phases have at least one module.
    #[allow(dead_code)]
    pub fn all_phases_populated(&self) -> bool {
        self.per_phase.iter().all(|&n| n > 0)
    }
    /// Get total modules across all phases.
    #[allow(dead_code)]
    pub fn phase_total(&self) -> usize {
        self.per_phase.iter().sum()
    }
}
/// Represents a phase in the standard library build process.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildPhase {
    /// Phase 1: Primitive types (Nat, Bool, Char, etc.)
    Primitives,
    /// Phase 2: Collection types (List, Array, Set, etc.)
    Collections,
    /// Phase 3: Type class definitions (Eq, Ord, Show, etc.)
    TypeClasses,
    /// Phase 4: Core theorems and lemmas.
    Theorems,
    /// Phase 5: Automation (tactic lemmas, decision procedures).
    Automation,
}
impl BuildPhase {
    /// Returns all phases in build order.
    #[allow(dead_code)]
    pub fn all_in_order() -> &'static [BuildPhase] {
        &[
            BuildPhase::Primitives,
            BuildPhase::Collections,
            BuildPhase::TypeClasses,
            BuildPhase::Theorems,
            BuildPhase::Automation,
        ]
    }
    /// Returns a human-readable name for this phase.
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            BuildPhase::Primitives => "primitives",
            BuildPhase::Collections => "collections",
            BuildPhase::TypeClasses => "type_classes",
            BuildPhase::Theorems => "theorems",
            BuildPhase::Automation => "automation",
        }
    }
}
/// A registry entry describing one standard library module.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StdModuleEntry {
    /// Fully qualified module name.
    pub qualified_name: &'static str,
    /// Build phase this module belongs to.
    pub phase: BuildPhase,
    /// Whether this module is loaded by default.
    pub default_load: bool,
    /// Brief description of module purpose.
    pub description: &'static str,
}
/// Version information for the OxiLean standard library.
#[allow(dead_code)]
pub struct StdVersion {
    /// Major version number.
    pub major: u32,
    /// Minor version number.
    pub minor: u32,
    /// Patch version number.
    pub patch: u32,
    /// Pre-release label (empty for stable).
    pub pre: &'static str,
}
impl StdVersion {
    /// The current standard library version.
    #[allow(dead_code)]
    pub const CURRENT: StdVersion = StdVersion {
        major: 0,
        minor: 1,
        patch: 0,
        pre: "alpha",
    };
    /// Format as a semver string.
    #[allow(dead_code)]
    pub fn format_version(&self) -> String {
        self.to_string()
    }
    /// Check if this is a stable release.
    #[allow(dead_code)]
    pub fn is_stable(&self) -> bool {
        self.pre.is_empty()
    }
}
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
/// A record of a single OxiLean standard library theorem or definition
/// that the elaborator knows about.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StdEntry {
    /// Qualified name (e.g., `Nat.add_comm`).
    pub name: &'static str,
    /// Which module this entry belongs to.
    pub module: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Whether this is a theorem (vs. a definition).
    pub is_theorem: bool,
}
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
