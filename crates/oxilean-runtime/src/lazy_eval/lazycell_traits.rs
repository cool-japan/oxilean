//! # LazyCell - Trait Implementations
//!
//! This module contains trait implementations for `LazyCell`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LazyCell;
use std::fmt;

impl<T: Clone + fmt::Debug> Default for LazyCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + fmt::Debug> fmt::Debug for LazyCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.get() {
            Some(v) => write!(f, "LazyCell::Initialized({:?})", v),
            None => write!(f, "LazyCell::Uninitialized"),
        }
    }
}
