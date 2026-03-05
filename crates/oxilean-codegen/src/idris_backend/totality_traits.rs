//! # Totality - Trait Implementations
//!
//! This module contains trait implementations for `Totality`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Totality;
use std::fmt;

impl fmt::Display for Totality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Totality::Total => writeln!(f, "total"),
            Totality::Partial => writeln!(f, "partial"),
            Totality::Covering => writeln!(f, "covering"),
            Totality::Default => Ok(()),
        }
    }
}
