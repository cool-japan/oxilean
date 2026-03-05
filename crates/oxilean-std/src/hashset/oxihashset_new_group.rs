//! # OxiHashSet - new_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashSet as StdHashSet;
use std::hash::Hash;

use super::oxihashset_type::OxiHashSet;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Create an empty set.
    pub fn new() -> Self {
        Self {
            inner: StdHashSet::new(),
        }
    }
    /// Create a singleton set containing exactly one element.
    pub fn singleton(elem: T) -> Self {
        let mut s = Self::new();
        s.insert(elem);
        s
    }
    /// Flat-map: apply `f` to each element and union all results.
    pub fn flat_map<U, F>(&self, f: F) -> OxiHashSet<U>
    where
        U: Eq + Hash + Clone,
        F: Fn(&T) -> OxiHashSet<U>,
    {
        let mut result = OxiHashSet::new();
        for elem in &self.inner {
            result = result.union(&f(elem));
        }
        result
    }
    /// Return the power set (set of all subsets) — only feasible for small sets.
    pub fn power_set(&self) -> Vec<Self> {
        let elems: Vec<T> = self.inner.iter().cloned().collect();
        let n = elems.len();
        (0usize..(1 << n))
            .map(|mask| {
                let mut s = Self::new();
                for (i, e) in elems.iter().enumerate() {
                    if mask & (1 << i) != 0 {
                        s.insert(e.clone());
                    }
                }
                s
            })
            .collect()
    }
}
