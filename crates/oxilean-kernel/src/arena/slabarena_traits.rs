//! # SlabArena - Trait Implementations
//!
//! This module contains trait implementations for `SlabArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SlabArena;

impl<T> Default for SlabArena<T> {
    fn default() -> Self {
        Self::new()
    }
}
