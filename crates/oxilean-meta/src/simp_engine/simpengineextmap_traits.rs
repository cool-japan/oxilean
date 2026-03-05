//! # SimpEngineExtMap - Trait Implementations
//!
//! This module contains trait implementations for `SimpEngineExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SimpEngineExtMap;

impl<V: Clone + Default> Default for SimpEngineExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
