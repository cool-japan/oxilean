//! # OxiHashSet - contains_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Check whether the set contains an element.
    pub fn contains(&self, elem: &T) -> bool {
        self.inner.contains(elem)
    }
    /// Keep only elements that also appear in `other` (intersection in place).
    pub fn intersect_with(&mut self, other: &Self) {
        self.inner.retain(|e| other.inner.contains(e));
    }
}
