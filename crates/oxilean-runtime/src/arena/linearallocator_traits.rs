//! # LinearAllocator - Trait Implementations
//!
//! This module contains trait implementations for `LinearAllocator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LinearAllocator;

impl Default for LinearAllocator {
    fn default() -> Self {
        Self::new(4096)
    }
}
