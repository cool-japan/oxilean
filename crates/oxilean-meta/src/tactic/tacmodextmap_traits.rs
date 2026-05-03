//! # TacModExtMap - Trait Implementations
//!
//! This module contains trait implementations for `TacModExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacModExtMap;

impl<V: Clone + Default> Default for TacModExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
