//! # ArenaPool - Trait Implementations
//!
//! This module contains trait implementations for `ArenaPool`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArenaPool;
use std::fmt;

impl Default for ArenaPool {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ArenaPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArenaPool")
            .field("available", &self.available.len())
            .field("max_pool_size", &self.max_pool_size)
            .field("chunk_size", &self.chunk_size)
            .finish()
    }
}
