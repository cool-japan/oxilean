//! # InterningArena - Trait Implementations
//!
//! This module contains trait implementations for `InterningArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InterningArena;

impl<T: PartialEq + Clone> Default for InterningArena<T> {
    fn default() -> Self {
        Self::new()
    }
}
