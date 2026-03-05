//! # ExtErrorCode - Trait Implementations
//!
//! This module contains trait implementations for `ExtErrorCode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExtErrorCode;
use std::fmt;

impl std::fmt::Display for ExtErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E{}", self.code_number())
    }
}
