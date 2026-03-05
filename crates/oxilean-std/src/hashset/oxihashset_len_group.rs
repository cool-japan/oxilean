//! # OxiHashSet - len_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Return the number of elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Number of elements not shared with `other`.
    pub fn exclusive_count(&self, other: &Self) -> usize {
        self.difference(other).len()
    }
    /// Total elements in union minus shared elements (Jaccard denominator).
    pub fn union_size(&self, other: &Self) -> usize {
        self.union(other).len()
    }
    /// Jaccard similarity coefficient (0.0–1.0).
    pub fn jaccard(&self, other: &Self) -> f64 {
        let u = self.union_size(other);
        if u == 0 {
            1.0
        } else {
            self.intersection(other).len() as f64 / u as f64
        }
    }
}
