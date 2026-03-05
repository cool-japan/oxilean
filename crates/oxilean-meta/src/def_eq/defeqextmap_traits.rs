//! # DefEqExtMap - Trait Implementations
//!
//! This module contains trait implementations for `DefEqExtMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DefEqExtMap;

impl<V: Clone + Default> Default for DefEqExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
