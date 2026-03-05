//! # BinaryMinHeap - Trait Implementations
//!
//! This module contains trait implementations for `BinaryMinHeap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BinaryMinHeap;

impl<T: Ord + Clone> Default for BinaryMinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
