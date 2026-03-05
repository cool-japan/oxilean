//! # PoolMirror - Trait Implementations
//!
//! This module contains trait implementations for `PoolMirror`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PoolMirror;

impl<T: Clone> Default for PoolMirror<T> {
    fn default() -> Self {
        Self::new()
    }
}
