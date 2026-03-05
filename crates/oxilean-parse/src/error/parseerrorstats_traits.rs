//! # ParseErrorStats - Trait Implementations
//!
//! This module contains trait implementations for `ParseErrorStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseErrorStats;
use std::fmt;

impl fmt::Display for ParseErrorStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ParseErrorStats {{ total: {}, eof: {}, located: {} }}",
            self.total, self.eof_errors, self.located_errors
        )
    }
}
