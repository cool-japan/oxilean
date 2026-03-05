//! # ParseError - Trait Implementations
//!
//! This module contains trait implementations for `ParseError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseError;
use std::fmt;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "parse error at line {}, column {}: {}",
            self.span.line, self.span.column, self.kind
        )
    }
}

impl std::error::Error for ParseError {}
