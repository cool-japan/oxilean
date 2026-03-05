//! # LevelDefEqExtMap - Trait Implementations
//!
//! This module contains trait implementations for `LevelDefEqExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LevelDefEqExtMap;

impl<V: Clone + Default> Default for LevelDefEqExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
