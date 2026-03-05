//! # LayeredDiscrTree - Trait Implementations
//!
//! This module contains trait implementations for `LayeredDiscrTree`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LayeredDiscrTree;

impl<T: Clone> Default for LayeredDiscrTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
