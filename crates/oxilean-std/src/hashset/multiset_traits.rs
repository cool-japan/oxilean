//! # MultiSet - Trait Implementations
//!
//! This module contains trait implementations for `MultiSet`.
//!
//! ## Implemented Traits
//!
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::hash::Hash;

use super::types::MultiSet;

impl<T: Eq + Hash + Clone> From<Vec<T>> for MultiSet<T> {
    fn from(v: Vec<T>) -> Self {
        let mut ms = Self::new();
        for elem in v {
            ms.insert(elem);
        }
        ms
    }
}
