//! # PriorityDiscrTree - Trait Implementations
//!
//! This module contains trait implementations for `PriorityDiscrTree`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PriorityDiscrTree;

impl<T: Clone + PartialEq> Default for PriorityDiscrTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
