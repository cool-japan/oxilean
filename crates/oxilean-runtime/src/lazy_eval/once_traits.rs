//! # Once - Trait Implementations
//!
//! This module contains trait implementations for `Once`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Once;
use std::fmt;

impl<T: Clone + fmt::Debug + Send + Sync + 'static> Default for Once<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + fmt::Debug + Send + Sync + 'static> fmt::Debug for Once<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.get() {
            Some(v) => write!(f, "Once::Initialized({:?})", v),
            None => write!(f, "Once::Uninitialized"),
        }
    }
}
