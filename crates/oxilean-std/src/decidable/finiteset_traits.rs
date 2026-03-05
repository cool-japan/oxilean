//! # FiniteSet - Trait Implementations
//!
//! This module contains trait implementations for `FiniteSet`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `FromIterator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FiniteSet;

impl<T: PartialEq> Default for FiniteSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> FromIterator<T> for FiniteSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for x in iter {
            set.insert(x);
        }
        set
    }
}
