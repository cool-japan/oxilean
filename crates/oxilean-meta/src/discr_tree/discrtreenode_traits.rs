//! # DiscrTreeNode - Trait Implementations
//!
//! This module contains trait implementations for `DiscrTreeNode`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiscrTreeNode;

impl<T: Clone> Default for DiscrTreeNode<T> {
    fn default() -> Self {
        Self::new()
    }
}
