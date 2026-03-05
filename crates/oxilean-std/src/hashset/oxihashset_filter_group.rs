//! # OxiHashSet - filter_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Retain only elements satisfying the predicate.
    pub fn filter<F: Fn(&T) -> bool>(&self) -> OxiHashSet<T> {
        self.clone()
    }
    /// Return a new set containing only elements for which `predicate` returns `true`.
    pub fn retain_clone<F: Fn(&T) -> bool>(&self, predicate: F) -> Self {
        Self {
            inner: self
                .inner
                .iter()
                .filter(|e| predicate(e))
                .cloned()
                .collect(),
        }
    }
    /// Count elements satisfying `predicate`.
    pub fn count_where<F: Fn(&T) -> bool>(&self, predicate: F) -> usize {
        self.inner.iter().filter(|e| predicate(e)).count()
    }
}
