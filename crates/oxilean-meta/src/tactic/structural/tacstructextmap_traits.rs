//! # TacStructExtMap - Trait Implementations
//!
//! This module contains trait implementations for `TacStructExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TacStructExtMap;

impl<V: Clone + Default> Default for TacStructExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
