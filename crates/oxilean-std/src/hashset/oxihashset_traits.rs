//! # OxiHashSet - Trait Implementations
//!
//! This module contains trait implementations for `OxiHashSet`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//! - `From`
//! - `From`
//! - `IntoIterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashSet as StdHashSet;
use std::fmt;
use std::hash::Hash;

use super::functions::*;
use super::oxihashset_type::OxiHashSet;

impl<T: Eq + Hash + Clone> Default for OxiHashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash + Clone + fmt::Display> fmt::Display for OxiHashSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut elems: Vec<String> = self.inner.iter().map(|e| e.to_string()).collect();
        elems.sort();
        write!(f, "{{{}}}", elems.join(", "))
    }
}

impl<T: Eq + Hash + Clone> From<Vec<T>> for OxiHashSet<T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            inner: v.into_iter().collect(),
        }
    }
}

impl<T: Eq + Hash + Clone> From<StdHashSet<T>> for OxiHashSet<T> {
    fn from(s: StdHashSet<T>) -> Self {
        Self { inner: s }
    }
}

impl<T: Eq + Hash + Clone> IntoIterator for OxiHashSet<T> {
    type Item = T;
    type IntoIter = std::collections::hash_set::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
