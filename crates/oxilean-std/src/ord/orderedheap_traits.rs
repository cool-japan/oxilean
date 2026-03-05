//! # OrderedHeap - Trait Implementations
//!
//! This module contains trait implementations for `OrderedHeap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OrderedHeap;

impl<T: Ord> Default for OrderedHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
