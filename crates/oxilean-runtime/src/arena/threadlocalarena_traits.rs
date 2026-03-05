//! # ThreadLocalArena - Trait Implementations
//!
//! This module contains trait implementations for `ThreadLocalArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ThreadLocalArena;

impl Default for ThreadLocalArena {
    fn default() -> Self {
        Self::new()
    }
}
