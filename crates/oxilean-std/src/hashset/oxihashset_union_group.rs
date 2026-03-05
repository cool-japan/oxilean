//! # OxiHashSet - union_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Return a new set that is the union of `self` and `other`.
    pub fn union(&self, other: &Self) -> Self {
        Self {
            inner: self.inner.union(&other.inner).cloned().collect(),
        }
    }
}
