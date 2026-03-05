//! # ScopedArena - Trait Implementations
//!
//! This module contains trait implementations for `ScopedArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScopedArena;

impl<T> Default for ScopedArena<T> {
    fn default() -> Self {
        Self::new()
    }
}
