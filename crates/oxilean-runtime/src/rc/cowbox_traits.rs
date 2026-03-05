//! # CowBox - Trait Implementations
//!
//! This module contains trait implementations for `CowBox`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//! - `PartialEq`
//! - `Eq`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CowBox;
use std::fmt;

impl<T: fmt::Debug> fmt::Debug for CowBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CowBox")
            .field("value", self.inner.as_ref())
            .field("unique", &self.inner.is_unique())
            .field("copied", &self.copied.get())
            .finish()
    }
}

impl<T: Clone + PartialEq> PartialEq for CowBox<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.as_ref() == other.inner.as_ref()
    }
}

impl<T: Clone + Eq> Eq for CowBox<T> {}
