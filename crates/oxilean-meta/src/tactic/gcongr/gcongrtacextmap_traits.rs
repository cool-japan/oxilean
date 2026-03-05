//! # GCongrTacExtMap - Trait Implementations
//!
//! This module contains trait implementations for `GCongrTacExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GCongrTacExtMap;

impl<V: Clone + Default> Default for GCongrTacExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
