//! # TacCtorExtMap - Trait Implementations
//!
//! This module contains trait implementations for `TacCtorExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacCtorExtMap;

impl<V: Clone + Default> Default for TacCtorExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
