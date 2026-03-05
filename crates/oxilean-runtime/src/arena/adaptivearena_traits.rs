//! # AdaptiveArena - Trait Implementations
//!
//! This module contains trait implementations for `AdaptiveArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AdaptiveArena;

impl Default for AdaptiveArena {
    fn default() -> Self {
        Self::new(0.75, 10)
    }
}
