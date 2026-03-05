//! # OxiHashSet - any_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Check whether any element satisfies `predicate`.
    pub fn any<F: Fn(&T) -> bool>(&self, predicate: F) -> bool {
        self.inner.iter().any(predicate)
    }
}
