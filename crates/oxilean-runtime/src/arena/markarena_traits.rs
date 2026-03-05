//! # MarkArena - Trait Implementations
//!
//! This module contains trait implementations for `MarkArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MarkArena;

impl Default for MarkArena {
    fn default() -> Self {
        Self::new(4096)
    }
}
