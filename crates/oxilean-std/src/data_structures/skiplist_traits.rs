//! # SkipList - Trait Implementations
//!
//! This module contains trait implementations for `SkipList`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SkipList;

impl<T: Ord + Clone> Default for SkipList<T> {
    fn default() -> Self {
        Self::new(16)
    }
}
