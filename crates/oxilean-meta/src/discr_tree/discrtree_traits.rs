//! # DiscrTree - Trait Implementations
//!
//! This module contains trait implementations for `DiscrTree`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Clone`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiscrTree;

impl<T: Clone> Default for DiscrTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for DiscrTree<T> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            num_entries: self.num_entries,
        }
    }
}

impl<T: Clone + std::fmt::Debug> std::fmt::Debug for DiscrTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiscrTree")
            .field("num_entries", &self.num_entries)
            .finish()
    }
}
