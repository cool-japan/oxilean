//! # WhnfMetaExtMap - Trait Implementations
//!
//! This module contains trait implementations for `WhnfMetaExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WhnfMetaExtMap;

impl<V: Clone + Default> Default for WhnfMetaExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
