//! # TrackedDiscrTree - Trait Implementations
//!
//! This module contains trait implementations for `TrackedDiscrTree`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TrackedDiscrTree;

impl<T: Clone> Default for TrackedDiscrTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
