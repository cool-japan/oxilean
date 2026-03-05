//! # OxiHashSet - fold_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Fold over all elements with a given initial value.
    pub fn fold<B, F: Fn(B, &T) -> B>(&self, init: B, f: F) -> B {
        self.inner.iter().fold(init, f)
    }
}
