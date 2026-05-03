//! # MetaLibExtMap - Trait Implementations
//!
//! This module contains trait implementations for `MetaLibExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaLibExtMap;

impl<V: Clone + Default> Default for MetaLibExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
