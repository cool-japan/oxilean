//! # SimpTypesExtMap - Trait Implementations
//!
//! This module contains trait implementations for `SimpTypesExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SimpTypesExtMap;

impl<V: Clone + Default> Default for SimpTypesExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
