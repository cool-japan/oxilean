//! # ErrorNote - Trait Implementations
//!
//! This module contains trait implementations for `ErrorNote`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ErrorNote;
use std::fmt;

impl fmt::Display for ErrorNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, "note in '{}': {}", loc, self.message)
        } else {
            write!(f, "note: {}", self.message)
        }
    }
}
