//! # ArenaPool - Trait Implementations
//!
//! This module contains trait implementations for `ArenaPool`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArenaPool;

impl<T> Default for ArenaPool<T> {
    fn default() -> Self {
        Self { pool: Vec::new() }
    }
}
