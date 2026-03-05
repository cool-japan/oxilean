//! # Prec - Trait Implementations
//!
//! This module contains trait implementations for `Prec`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Prec;
use std::fmt;

impl fmt::Display for Prec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
