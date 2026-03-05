//! # Diagnostic - Trait Implementations
//!
//! This module contains trait implementations for `Diagnostic`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Diagnostic;
use std::fmt;

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = &self.location {
            write!(f, "{} in '{}': {}", self.severity, loc, self.message)
        } else {
            write!(f, "{}: {}", self.severity, self.message)
        }
    }
}
