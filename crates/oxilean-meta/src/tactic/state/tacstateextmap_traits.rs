//! # TacStateExtMap - Trait Implementations
//!
//! This module contains trait implementations for `TacStateExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacStateExtMap;

impl<V: Clone + Default> Default for TacStateExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
