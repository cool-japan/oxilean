//! # OxiHashSet - insert_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Insert an element. Returns `true` if the element was not already present.
    pub fn insert(&mut self, elem: T) -> bool {
        self.inner.insert(elem)
    }
    /// Extend with elements from another set (union in place).
    pub fn union_with(&mut self, other: &Self) {
        for elem in &other.inner {
            self.inner.insert(elem.clone());
        }
    }
}
