//! # OxiHashSet - symmetric_difference_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Return a new set that is the symmetric difference of `self` and `other`.
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        Self {
            inner: self
                .inner
                .symmetric_difference(&other.inner)
                .cloned()
                .collect(),
        }
    }
}
