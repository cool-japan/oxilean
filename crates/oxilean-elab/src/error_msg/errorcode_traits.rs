//! # ErrorCode - Trait Implementations
//!
//! This module contains trait implementations for `ErrorCode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ErrorCode;
use std::fmt;

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{}", self.code_number())
    }
}
