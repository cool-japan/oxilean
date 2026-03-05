//! # OxiHashSet - builders Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashSet as StdHashSet;
use std::hash::Hash;

use super::oxihashset_type::OxiHashSet;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Create a set with the given capacity pre-allocated.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: StdHashSet::with_capacity(capacity),
        }
    }
}
