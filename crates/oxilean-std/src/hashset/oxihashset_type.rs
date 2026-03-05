//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashSet as StdHashSet;
use std::fmt;
use std::hash::Hash;

/// A wrapper around `std::collections::HashSet` with an extended API.
///
/// `OxiHashSet<T>` provides functional-style operations (map, filter, fold)
/// alongside the standard set operations (union, intersection, difference).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OxiHashSet<T>
where
    T: Eq + Hash + Clone,
{
    pub(super) inner: StdHashSet<T>,
}
impl<T: Eq + Hash + Clone + Ord> OxiHashSet<T> {
    /// Return elements sorted in ascending order.
    pub fn sorted_vec(&self) -> Vec<T> {
        let mut v = self.to_vec();
        v.sort();
        v
    }
    /// Return the minimum element (requires `Ord`).
    pub fn min(&self) -> Option<&T> {
        self.inner.iter().min()
    }
    /// Return the maximum element (requires `Ord`).
    pub fn max(&self) -> Option<&T> {
        self.inner.iter().max()
    }
}
impl<T: Eq + Hash + Clone + std::fmt::Debug> OxiHashSet<T> {
    /// Return a sorted vector of elements (requires Ord on T).
    pub fn to_sorted_vec(&self) -> Vec<T>
    where
        T: Ord,
    {
        let mut v: Vec<T> = self.inner.iter().cloned().collect();
        v.sort();
        v
    }
}
