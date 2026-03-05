//! # GenerationalArena - Trait Implementations
//!
//! This module contains trait implementations for `GenerationalArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GenerationalArena;
use std::fmt;

impl<T> Default for GenerationalArena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for GenerationalArena<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GenerationalArena")
            .field("len", &self.len())
            .field("capacity", &self.entries.len())
            .field("generation", &self.generation)
            .finish()
    }
}
