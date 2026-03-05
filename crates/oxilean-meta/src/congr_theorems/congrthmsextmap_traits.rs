//! # CongrThmsExtMap - Trait Implementations
//!
//! This module contains trait implementations for `CongrThmsExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CongrThmsExtMap;

impl<V: Clone + Default> Default for CongrThmsExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
