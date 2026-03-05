//! # OxiHashSet - queries Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Find an element satisfying `predicate`.
    pub fn find<F: Fn(&T) -> bool>(&self, predicate: F) -> Option<&T> {
        self.inner.iter().find(|e| predicate(e))
    }
}
