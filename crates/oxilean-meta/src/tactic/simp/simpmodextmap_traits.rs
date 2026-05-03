//! # SimpModExtMap - Trait Implementations
//!
//! This module contains trait implementations for `SimpModExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::simp_types::SimpModExtMap;

impl<V: Clone + Default> Default for SimpModExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
