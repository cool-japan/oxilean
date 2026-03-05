//! # PoolAllocator - Trait Implementations
//!
//! This module contains trait implementations for `PoolAllocator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PoolAllocator;

impl<T> Default for PoolAllocator<T> {
    fn default() -> Self {
        Self::new()
    }
}
