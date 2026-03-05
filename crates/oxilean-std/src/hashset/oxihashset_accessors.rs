//! # OxiHashSet - accessors Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::oxihashset_type::OxiHashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Check structural equality with another set.
    pub fn set_eq(&self, other: &Self) -> bool {
        self == other
    }
}
