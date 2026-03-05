//! # MetaConvenienceExtMap - Trait Implementations
//!
//! This module contains trait implementations for `MetaConvenienceExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaConvenienceExtMap;

impl<V: Clone + Default> Default for MetaConvenienceExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
