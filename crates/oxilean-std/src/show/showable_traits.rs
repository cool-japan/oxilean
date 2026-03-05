//! # Showable - Trait Implementations
//!
//! This module contains trait implementations for `Showable`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::Show;
use super::types::Showable;
use std::fmt;

impl<T: Show> fmt::Display for Showable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.show())
    }
}
