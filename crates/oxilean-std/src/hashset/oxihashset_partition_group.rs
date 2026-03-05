//! # OxiHashSet - partition_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashSet as StdHashSet;
use std::hash::Hash;

use super::oxihashset_type::OxiHashSet;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Partition into two sets by a predicate.
    pub fn partition<F: Fn(&T) -> bool>(&self, predicate: F) -> (Self, Self) {
        let (yes, no): (StdHashSet<_>, StdHashSet<_>) =
            self.inner.iter().cloned().partition(|e| predicate(e));
        (Self { inner: yes }, Self { inner: no })
    }
}
