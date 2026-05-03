//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// A node in a persistent binary search tree.
#[derive(Debug, Clone)]
enum PersistentNode<K, V> {
    Leaf,
    Node(K, V, Box<PersistentNode<K, V>>, Box<PersistentNode<K, V>>),
}
/// A simple interval map for ordered key types.
///
/// Stores a list of disjoint intervals, each associated with a value.
/// Lookup is O(n) by linear scan.
#[derive(Debug, Clone)]
pub struct IntervalMap<T: Ord + Clone, V: Clone> {
    pub(super) intervals: Vec<IntervalEntry<T, V>>,
}
impl<T: Ord + Clone, V: Clone> IntervalMap<T, V> {
    /// Create an empty interval map.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert an interval \[lo, hi\] with a value.
    pub fn insert(&mut self, lo: T, hi: T, value: V) {
        self.intervals.push(IntervalEntry { lo, hi, value });
    }
    /// Query a point, returning all values whose interval contains the point.
    pub fn query(&self, point: &T) -> Vec<&V> {
        self.intervals
            .iter()
            .filter(|e| &e.lo <= point && point <= &e.hi)
            .map(|e| &e.value)
            .collect()
    }
    /// Number of intervals.
    pub fn len(&self) -> usize {
        self.intervals.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }
    /// Clear all intervals.
    pub fn clear(&mut self) {
        self.intervals.clear();
    }
}
/// Simple LRU cache backed by a HashMap.
#[allow(dead_code)]
pub struct LRUCacheHm<K, V> {
    pub capacity: usize,
    pub store: std::collections::HashMap<K, V>,
    pub order: std::collections::VecDeque<K>,
}
/// A map that associates each key with a list of values.
#[derive(Debug, Clone)]
pub struct MultiMap<K: PartialEq + Clone, V: Clone> {
    inner: AssocMap<K, Vec<V>>,
}
impl<K: PartialEq + Clone, V: Clone> MultiMap<K, V> {
    /// Create an empty multi-map.
    pub fn new() -> Self {
        Self {
            inner: AssocMap::new(),
        }
    }
    /// Insert a value for a key (appends if key already exists).
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(vals) = self.inner.get_mut(&key) {
            vals.push(value);
        } else {
            self.inner.insert(key, vec![value]);
        }
    }
    /// Get all values for a key.
    pub fn get(&self, key: &K) -> &[V] {
        self.inner.get(key).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Remove all values for a key.
    pub fn remove(&mut self, key: &K) -> Vec<V> {
        self.inner.remove(key).unwrap_or_default()
    }
    /// Check if a key has any values.
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
    /// Total number of key-value associations.
    pub fn total_count(&self) -> usize {
        self.inner.values().map(|v| v.len()).sum()
    }
    /// Number of distinct keys.
    pub fn key_count(&self) -> usize {
        self.inner.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Clear the multi-map.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
/// A simple association list map.
///
/// Provides O(n) lookup but is useful in contexts where the number of entries
/// is small and allocation overhead should be minimal.
#[derive(Debug, Clone, PartialEq)]
pub struct AssocMap<K, V> {
    /// Stored entries in insertion order.
    entries: Vec<(K, V)>,
}
impl<K: PartialEq + Clone, V: Clone> AssocMap<K, V> {
    /// Create an empty association map.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Insert a key-value pair, replacing any existing entry for that key.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &key) {
            entry.1 = value;
        } else {
            self.entries.push((key, value));
        }
    }
    /// Look up a value by key.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
    /// Look up a value by key (mutable).
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.entries
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
    /// Remove a key-value pair by key.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == key) {
            Some(self.entries.remove(pos).1)
        } else {
            None
        }
    }
    /// Check if the map contains a key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate over key-value pairs in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.entries.iter()
    }
    /// Iterate over keys in insertion order.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(k, _)| k)
    }
    /// Iterate over values in insertion order.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, v)| v)
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Convert to a Vec of key-value pairs.
    pub fn into_vec(self) -> Vec<(K, V)> {
        self.entries
    }
    /// Merge another map into this one (entries from `other` overwrite).
    pub fn merge(&mut self, other: &AssocMap<K, V>) {
        for (k, v) in &other.entries {
            self.insert(k.clone(), v.clone());
        }
    }
    /// Retain only entries satisfying the predicate.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        self.entries.retain(|(k, v)| f(k, v));
    }
}
/// Multi-map: each key maps to a list of values.
#[allow(dead_code)]
pub struct MultiMapHm<K, V> {
    pub inner: std::collections::HashMap<K, Vec<V>>,
}
/// An entry in an interval map.
#[derive(Debug, Clone)]
pub struct IntervalEntry<T: Ord, V> {
    /// Lower bound of the interval (inclusive).
    pub lo: T,
    /// Upper bound of the interval (inclusive).
    pub hi: T,
    /// Value associated with this interval.
    pub value: V,
}
/// Immutable frozen snapshot of a HashMap.
#[allow(dead_code)]
pub struct FrozenHashMap<K, V> {
    pub inner: std::collections::HashMap<K, V>,
    pub frozen: bool,
}
/// A bidirectional map where both keys and values are unique.
///
/// Provides O(n) forward and reverse lookup, useful for managing bijections
/// between names and indices in the elaboration context.
#[derive(Debug, Clone, PartialEq)]
pub struct BiMap<K: PartialEq + Clone, V: PartialEq + Clone> {
    forward: AssocMap<K, V>,
    backward: AssocMap<V, K>,
}
impl<K: PartialEq + Clone, V: PartialEq + Clone> BiMap<K, V> {
    /// Create an empty bidirectional map.
    pub fn new() -> Self {
        Self {
            forward: AssocMap::new(),
            backward: AssocMap::new(),
        }
    }
    /// Insert a (key, value) pair, evicting any existing entries that conflict.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(old_val) = self.forward.remove(&key) {
            self.backward.remove(&old_val);
        }
        if let Some(old_key) = self.backward.remove(&value) {
            self.forward.remove(&old_key);
        }
        self.forward.insert(key.clone(), value.clone());
        self.backward.insert(value, key);
    }
    /// Forward lookup: key → value.
    pub fn get_by_key(&self, key: &K) -> Option<&V> {
        self.forward.get(key)
    }
    /// Reverse lookup: value → key.
    pub fn get_by_val(&self, val: &V) -> Option<&K> {
        self.backward.get(val)
    }
    /// Remove by key.
    pub fn remove_by_key(&mut self, key: &K) -> Option<V> {
        if let Some(val) = self.forward.remove(key) {
            self.backward.remove(&val);
            Some(val)
        } else {
            None
        }
    }
    /// Remove by value.
    pub fn remove_by_val(&mut self, val: &V) -> Option<K> {
        if let Some(key) = self.backward.remove(val) {
            self.forward.remove(&key);
            Some(key)
        } else {
            None
        }
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.forward.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }
    /// Check if the key exists.
    pub fn contains_key(&self, key: &K) -> bool {
        self.forward.contains_key(key)
    }
    /// Check if the value exists.
    pub fn contains_val(&self, val: &V) -> bool {
        self.backward.contains_key(val)
    }
    /// Iterate over (key, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.forward.iter()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.forward.clear();
        self.backward.clear();
    }
}
/// A map with LRU (least-recently-used) eviction policy.
///
/// When the capacity is exceeded, the least-recently-used entry is removed.
/// Useful for memoization caches inside the type checker.
#[derive(Debug, Clone)]
pub struct LruMap<K: PartialEq + Clone, V: Clone> {
    capacity: usize,
    entries: Vec<(K, V)>,
}
impl<K: PartialEq + Clone, V: Clone> LruMap<K, V> {
    /// Create an LRU map with the given capacity (must be > 0).
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "LruMap capacity must be positive");
        Self {
            capacity,
            entries: Vec::with_capacity(capacity),
        }
    }
    /// Get a value by key, promoting it to the front (most recently used).
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let pos = self.entries.iter().position(|(k, _)| k == key)?;
        let entry = self.entries.remove(pos);
        self.entries.insert(0, entry);
        self.entries.first().map(|(_, v)| v)
    }
    /// Insert a key-value pair, evicting the LRU entry if at capacity.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &key) {
            self.entries.remove(pos);
        }
        if self.entries.len() >= self.capacity {
            self.entries.pop();
        }
        self.entries.insert(0, (key, value));
    }
    /// Peek at a value without updating LRU order.
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Evict all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Current capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    /// Iterate entries in MRU order.
    pub fn iter_mru(&self) -> impl Iterator<Item = &(K, V)> {
        self.entries.iter()
    }
}
/// HashMap viewed as a monoid under union (left-biased merge).
#[allow(dead_code)]
pub struct HashMapMonoid<K, V> {
    pub inner: std::collections::HashMap<K, V>,
}
/// An immutable persistent map based on a binary search tree.
///
/// Operations return new map instances sharing structure with the original.
/// Suitable for use in backtracking algorithms where previous states must
/// be preserved.
#[derive(Debug, Clone)]
pub struct PersistentMap<K: Ord + Clone, V: Clone> {
    root: PersistentNode<K, V>,
    size: usize,
}
impl<K: Ord + Clone, V: Clone> PersistentMap<K, V> {
    /// Create an empty persistent map.
    pub fn empty() -> Self {
        Self {
            root: PersistentNode::Leaf,
            size: 0,
        }
    }
    /// Look up a value by key.
    pub fn get(&self, key: &K) -> Option<&V> {
        Self::get_node(&self.root, key)
    }
    fn get_node<'a>(node: &'a PersistentNode<K, V>, key: &K) -> Option<&'a V> {
        match node {
            PersistentNode::Leaf => None,
            PersistentNode::Node(k, v, left, right) => {
                use std::cmp::Ordering;
                match key.cmp(k) {
                    Ordering::Equal => Some(v),
                    Ordering::Less => Self::get_node(left, key),
                    Ordering::Greater => Self::get_node(right, key),
                }
            }
        }
    }
    /// Insert a key-value pair, returning a new map.
    pub fn insert(&self, key: K, value: V) -> Self {
        let (new_root, added) = Self::insert_node(&self.root, key, value);
        Self {
            root: new_root,
            size: if added { self.size + 1 } else { self.size },
        }
    }
    fn insert_node(node: &PersistentNode<K, V>, key: K, value: V) -> (PersistentNode<K, V>, bool) {
        match node {
            PersistentNode::Leaf => (
                PersistentNode::Node(
                    key,
                    value,
                    Box::new(PersistentNode::Leaf),
                    Box::new(PersistentNode::Leaf),
                ),
                true,
            ),
            PersistentNode::Node(k, v, left, right) => {
                use std::cmp::Ordering;
                match key.cmp(k) {
                    Ordering::Equal => (
                        PersistentNode::Node(k.clone(), value, left.clone(), right.clone()),
                        false,
                    ),
                    Ordering::Less => {
                        let (new_left, added) = Self::insert_node(left, key, value);
                        (
                            PersistentNode::Node(
                                k.clone(),
                                v.clone(),
                                Box::new(new_left),
                                right.clone(),
                            ),
                            added,
                        )
                    }
                    Ordering::Greater => {
                        let (new_right, added) = Self::insert_node(right, key, value);
                        (
                            PersistentNode::Node(
                                k.clone(),
                                v.clone(),
                                left.clone(),
                                Box::new(new_right),
                            ),
                            added,
                        )
                    }
                }
            }
        }
    }
    /// Check if the map contains a key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.size
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    /// Collect all key-value pairs in sorted order.
    pub fn to_sorted_vec(&self) -> Vec<(K, V)> {
        let mut result = Vec::new();
        Self::collect_inorder(&self.root, &mut result);
        result
    }
    fn collect_inorder(node: &PersistentNode<K, V>, out: &mut Vec<(K, V)>) {
        if let PersistentNode::Node(k, v, left, right) = node {
            Self::collect_inorder(left, out);
            out.push((k.clone(), v.clone()));
            Self::collect_inorder(right, out);
        }
    }
}
/// A map supporting scoped bindings with push/pop semantics.
///
/// Useful for implementing the typing context in the elaborator,
/// where entering a binder pushes a new scope and exiting pops it.
#[derive(Debug, Clone)]
pub struct StackMap<K: PartialEq + Clone, V: Clone> {
    scopes: Vec<Vec<(K, V)>>,
}
impl<K: PartialEq + Clone, V: Clone> StackMap<K, V> {
    /// Create a new `StackMap` with a single (global) scope.
    pub fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
        }
    }
    /// Push a new scope.
    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }
    /// Pop the innermost scope, discarding its bindings.
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    /// Insert into the current (innermost) scope.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(entry) = scope.iter_mut().find(|(k, _)| k == &key) {
                entry.1 = value;
                return;
            }
            scope.push((key, value));
        }
    }
    /// Look up by key, searching from innermost to outermost scope.
    pub fn get(&self, key: &K) -> Option<&V> {
        for scope in self.scopes.iter().rev() {
            if let Some((_, v)) = scope.iter().rev().find(|(k, _)| k == key) {
                return Some(v);
            }
        }
        None
    }
    /// Check if a key is visible in any scope.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    /// Number of current scope levels.
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
    /// Total number of bindings across all scopes.
    pub fn total_bindings(&self) -> usize {
        self.scopes.iter().map(|s| s.len()).sum()
    }
    /// Clear all scopes (but keep one empty global scope).
    pub fn clear(&mut self) {
        self.scopes.clear();
        self.scopes.push(Vec::new());
    }
}
/// A map counting how many times each key has been seen.
#[derive(Debug, Clone)]
pub struct FreqMap<K: PartialEq + Clone> {
    pub(super) counts: AssocMap<K, usize>,
}
impl<K: PartialEq + Clone> FreqMap<K> {
    /// Create an empty frequency map.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one occurrence of `key`.
    pub fn record(&mut self, key: K) {
        if let Some(c) = self.counts.get_mut(&key) {
            *c += 1;
        } else {
            self.counts.insert(key, 1);
        }
    }
    /// Get the count for `key`.
    pub fn count(&self, key: &K) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }
    /// Get the key with the highest count.
    pub fn most_common(&self) -> Option<(&K, usize)> {
        self.counts
            .iter()
            .map(|(k, c)| (k, *c))
            .max_by_key(|(_, c)| *c)
    }
    /// Total number of distinct keys.
    pub fn distinct_keys(&self) -> usize {
        self.counts.len()
    }
    /// Total number of recorded occurrences.
    pub fn total_count(&self) -> usize {
        self.counts.values().sum()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
    /// Clear all counts.
    pub fn clear(&mut self) {
        self.counts.clear();
    }
}
/// A vector-backed map where keys are dense integers 0..n.
///
/// Provides O(1) lookup and O(1) insertion (amortized).
#[derive(Debug, Clone)]
pub struct IndexedMap<V: Clone> {
    pub(super) slots: Vec<Option<V>>,
    pub(super) count: usize,
}
impl<V: Clone> IndexedMap<V> {
    /// Create an empty indexed map.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert or overwrite the value at `index`.
    pub fn insert(&mut self, index: usize, value: V) {
        if index >= self.slots.len() {
            self.slots.resize(index + 1, None);
        }
        if self.slots[index].is_none() {
            self.count += 1;
        }
        self.slots[index] = Some(value);
    }
    /// Get the value at `index`.
    pub fn get(&self, index: usize) -> Option<&V> {
        self.slots.get(index).and_then(|s| s.as_ref())
    }
    /// Remove the value at `index`.
    pub fn remove(&mut self, index: usize) -> Option<V> {
        let slot = self.slots.get_mut(index)?;
        let old = slot.take()?;
        self.count -= 1;
        Some(old)
    }
    /// Check if index is occupied.
    pub fn contains(&self, index: usize) -> bool {
        self.get(index).is_some()
    }
    /// Number of occupied slots.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Capacity (highest index + 1).
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
    /// Iterate over (index, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (usize, &V)> {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.as_ref().map(|v| (i, v)))
    }
    /// Clear all slots.
    pub fn clear(&mut self) {
        self.slots.clear();
        self.count = 0;
    }
}
/// Represents the structural difference between two HashMaps.
#[allow(dead_code)]
pub struct HashMapDiff<K, V> {
    pub added: std::collections::HashMap<K, V>,
    pub removed: std::collections::HashSet<K>,
}
