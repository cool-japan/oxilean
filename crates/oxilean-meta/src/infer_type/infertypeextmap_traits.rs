//! # InferTypeExtMap - Trait Implementations
//!
//! This module contains trait implementations for `InferTypeExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InferTypeExtMap;

impl<V: Clone + Default> Default for InferTypeExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
