//! # OxiHashSet - pick_any_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Return an arbitrary element (useful for single-element sets).
    pub fn pick_any(&self) -> Option<&T> {
        self.inner.iter().next()
    }
}
