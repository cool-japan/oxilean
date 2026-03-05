//! # TypedArena - Trait Implementations
//!
//! This module contains trait implementations for `TypedArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TypedArena;

impl<T> Default for TypedArena<T> {
    fn default() -> Self {
        Self::new()
    }
}
