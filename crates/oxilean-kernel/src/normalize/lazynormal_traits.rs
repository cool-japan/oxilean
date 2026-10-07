//! # LazyNormal - Trait Implementations
//!
//! This module contains trait implementations for `LazyNormal`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyNormal;

impl std::fmt::Debug for LazyNormal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.normal.get() {
            Some(normal) => write!(f, "LazyNormal::Evaluated({:?})", normal),
            None => write!(f, "LazyNormal::Pending({:?})", self.original),
        }
    }
}
#[cfg(test)]
mod debug_tests;
