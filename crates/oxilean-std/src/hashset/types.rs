//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::hash::Hash;

use super::oxihashset_type::OxiHashSet;

use super::functions::*;
use std::collections::HashMap;

/// A bijective map between two sets (one-to-one and onto).
///
/// Useful for tracking variable renaming during alpha-equivalence checks.
#[derive(Clone, Debug, Default)]
pub struct Bijection<A: Eq + Hash + Clone, B: Eq + Hash + Clone> {
    forward: std::collections::HashMap<A, B>,
    backward: std::collections::HashMap<B, A>,
}
impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone> Bijection<A, B> {
    /// Create an empty bijection.
    pub fn new() -> Self {
        Self {
            forward: std::collections::HashMap::new(),
            backward: std::collections::HashMap::new(),
        }
    }
    /// Insert a pair (a, b). Returns `false` if either side is already bound.
    pub fn insert(&mut self, a: A, b: B) -> bool {
        if self.forward.contains_key(&a) || self.backward.contains_key(&b) {
            return false;
        }
        self.forward.insert(a.clone(), b.clone());
        self.backward.insert(b, a);
        true
    }
    /// Look up the image of `a`.
    pub fn forward(&self, a: &A) -> Option<&B> {
        self.forward.get(a)
    }
    /// Look up the preimage of `b`.
    pub fn backward(&self, b: &B) -> Option<&A> {
        self.backward.get(b)
    }
    /// Number of pairs.
    pub fn len(&self) -> usize {
        self.forward.len()
    }
    /// Check whether the bijection is empty.
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }
    /// Remove a pair by its left element. Returns the removed right element if present.
    pub fn remove(&mut self, a: &A) -> Option<B> {
        if let Some(b) = self.forward.remove(a) {
            self.backward.remove(&b);
            Some(b)
        } else {
            None
        }
    }
}
/// Bloom filter axioms: approximate membership with false-positive bounds.
#[allow(dead_code)]
pub struct BloomFilterExt {
    bit_array: Vec<bool>,
    hash_count: usize,
    expected_insertions: usize,
    false_positive_rate: f64,
}
/// Power set axioms: the set of all subsets.
#[allow(dead_code)]
pub struct PowerSetExt<T: Eq + std::hash::Hash + Clone> {
    base: OxiHashSet<T>,
    /// Cantor's theorem: |P(S)| = 2^|S|
    cardinality_bound: usize,
}
/// A multiset: like a set but tracks element multiplicities.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct MultiSet<T: Eq + Hash + Clone> {
    counts: std::collections::HashMap<T, usize>,
}
impl<T: Eq + Hash + Clone> MultiSet<T> {
    /// Create an empty multiset.
    pub fn new() -> Self {
        Self {
            counts: std::collections::HashMap::new(),
        }
    }
    /// Insert one occurrence of `elem`.
    pub fn insert(&mut self, elem: T) {
        *self.counts.entry(elem).or_insert(0) += 1;
    }
    /// Remove one occurrence of `elem`. Returns the new count (0 if absent).
    pub fn remove_one(&mut self, elem: &T) -> usize {
        if let Some(c) = self.counts.get_mut(elem) {
            if *c > 1 {
                *c -= 1;
                *c
            } else {
                self.counts.remove(elem);
                0
            }
        } else {
            0
        }
    }
    /// Count occurrences of `elem`.
    pub fn count(&self, elem: &T) -> usize {
        self.counts.get(elem).copied().unwrap_or(0)
    }
    /// Total number of elements (with multiplicity).
    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }
    /// Number of distinct elements.
    pub fn distinct_count(&self) -> usize {
        self.counts.len()
    }
    /// Convert to a plain set (discarding multiplicities).
    pub fn to_set(&self) -> OxiHashSet<T> {
        OxiHashSet::from_iter(self.counts.keys().cloned())
    }
    /// Check if the multiset is empty.
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
    /// Clear the multiset.
    pub fn clear(&mut self) {
        self.counts.clear();
    }
    /// Return `true` if `elem` is present at least once.
    pub fn contains(&self, elem: &T) -> bool {
        self.counts.contains_key(elem)
    }
}
/// Extended lattice structure for sets (join = union, meet = intersection).
#[allow(dead_code)]
pub struct SetLatticeExt<T: Eq + std::hash::Hash + Clone> {
    universe: OxiHashSet<T>,
    bottom: OxiHashSet<T>,
    top: OxiHashSet<T>,
}
/// A simple disjoint-set (union-find) data structure over integer IDs.
#[derive(Clone, Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    /// Create a new union-find with `n` singletons (0..n).
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    /// Find the representative of the component containing `x` (path compression).
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    /// Union the components containing `x` and `y`. Returns `true` if they were separate.
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
        true
    }
    /// Check whether `x` and `y` are in the same component.
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    /// Count the number of distinct components.
    pub fn component_count(&mut self) -> usize {
        let n = self.parent.len();
        (0..n).filter(|&i| self.find(i) == i).count()
    }
    /// Collect all elements in the same component as `x`.
    pub fn component_of(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        let n = self.parent.len();
        (0..n).filter(|&i| self.find(i) == root).collect()
    }
}
/// Set partition axioms: disjoint covering of a base set.
#[allow(dead_code)]
pub struct SetPartitionExt<T: Eq + std::hash::Hash + Clone> {
    base: OxiHashSet<T>,
    parts: Vec<OxiHashSet<T>>,
}
/// Disjoint union (coproduct) of sets.
#[allow(dead_code)]
pub struct DisjointUnionExt<T: Eq + std::hash::Hash + Clone> {
    left: OxiHashSet<T>,
    right: OxiHashSet<T>,
    /// Cardinality of the disjoint union = |left| + |right| (when disjoint)
    total_cardinality: usize,
}
