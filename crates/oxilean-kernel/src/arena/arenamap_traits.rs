//! # ArenaMap - Trait Implementations
//!
//! This module contains trait implementations for `ArenaMap`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArenaMap;

impl<T, V> Default for ArenaMap<T, V> {
    fn default() -> Self {
        Self::new()
    }
}
