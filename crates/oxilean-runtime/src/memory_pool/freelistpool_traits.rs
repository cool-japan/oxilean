//! # FreeListPool - Trait Implementations
//!
//! This module contains trait implementations for `FreeListPool`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FreeListPool;

impl<T> Default for FreeListPool<T> {
    fn default() -> Self {
        Self::new()
    }
}
