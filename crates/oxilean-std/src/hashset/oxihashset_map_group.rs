//! # OxiHashSet - map_group Methods
//!
//! This module contains method implementations for `OxiHashSet`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::hash::Hash;

use super::oxihashset_type::OxiHashSet;

impl<T: Eq + Hash + Clone> OxiHashSet<T> {
    /// Apply a function to every element, returning a new set.
    pub fn map<U, F>(&self, f: F) -> OxiHashSet<U>
    where
        U: Eq + Hash + Clone,
        F: Fn(&T) -> U,
    {
        OxiHashSet {
            inner: self.inner.iter().map(f).collect(),
        }
    }
    /// Return an iterator over the elements.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}
