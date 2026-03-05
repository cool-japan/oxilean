//! # ArenaAllocator - Trait Implementations
//!
//! This module contains trait implementations for `ArenaAllocator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArenaAllocator;

impl Default for ArenaAllocator {
    fn default() -> Self {
        Self::new(4096)
    }
}
