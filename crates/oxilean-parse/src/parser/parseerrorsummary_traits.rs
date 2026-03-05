//! # ParseErrorSummary - Trait Implementations
//!
//! This module contains trait implementations for `ParseErrorSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseErrorSummary;
use std::fmt;

impl fmt::Display for ParseErrorSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseErrorSummary {{ total: {} }}", self.total)
    }
}
