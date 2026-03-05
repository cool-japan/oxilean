//! # OxiHashSet - remove_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Remove an element. Returns `true` if it was present.
    pub fn remove(&mut self, elem: &T) -> bool {
        self.inner.remove(elem)
    }
    /// Remove all elements that appear in `other` (difference in place).
    pub fn subtract(&mut self, other: &Self) {
        for elem in &other.inner {
            self.inner.remove(elem);
        }
    }
}
