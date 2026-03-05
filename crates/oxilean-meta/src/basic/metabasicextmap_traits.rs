//! # MetaBasicExtMap - Trait Implementations
//!
//! This module contains trait implementations for `MetaBasicExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaBasicExtMap;

impl<V: Clone + Default> Default for MetaBasicExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
