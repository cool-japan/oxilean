//! # TacInjectExtMap - Trait Implementations
//!
//! This module contains trait implementations for `TacInjectExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacInjectExtMap;

impl<V: Clone + Default> Default for TacInjectExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
