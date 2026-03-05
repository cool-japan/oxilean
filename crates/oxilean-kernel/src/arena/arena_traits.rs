//! # Arena - Trait Implementations
//!
//! This module contains trait implementations for `Arena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Index`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Arena, Idx};

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::ops::Index<Idx<T>> for Arena<T> {
    type Output = T;
    fn index(&self, idx: Idx<T>) -> &T {
        self.get(idx)
    }
}
