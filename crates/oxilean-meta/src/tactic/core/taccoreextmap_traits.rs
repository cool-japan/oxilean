//! # TacCoreExtMap - Trait Implementations
//!
//! This module contains trait implementations for `TacCoreExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacCoreExtMap;

impl<V: Clone + Default> Default for TacCoreExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
