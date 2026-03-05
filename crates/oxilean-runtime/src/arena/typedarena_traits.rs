//! # TypedArena - Trait Implementations
//!
//! This module contains trait implementations for `TypedArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TypedArena;
use std::fmt;

impl<T> Default for TypedArena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for TypedArena<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedArena")
            .field("len", &self.values.len())
            .field("capacity", &self.values.capacity())
            .finish()
    }
}
