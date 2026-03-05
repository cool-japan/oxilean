//! # Rc - Trait Implementations
//!
//! This module contains trait implementations for `Rc`.
//!
//! ## Implemented Traits
//!
//! - `Debug`
//! - `Display`
//! - `PartialEq`
//! - `Eq`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Rc;
use std::fmt;

impl<T: fmt::Debug> fmt::Debug for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rc")
            .field("value", &self.inner.value)
            .field("strong_count", &self.strong_count())
            .field("weak_count", &self.weak_count())
            .finish()
    }
}

impl<T: fmt::Display> fmt::Display for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.value.fmt(f)
    }
}

impl<T: PartialEq> PartialEq for Rc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.value == other.inner.value
    }
}

impl<T: Eq> Eq for Rc<T> {}
