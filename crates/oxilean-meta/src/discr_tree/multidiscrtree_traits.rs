//! # MultiDiscrTree - Trait Implementations
//!
//! This module contains trait implementations for `MultiDiscrTree`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MultiDiscrTree;

impl<T: Clone + PartialEq> Default for MultiDiscrTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
