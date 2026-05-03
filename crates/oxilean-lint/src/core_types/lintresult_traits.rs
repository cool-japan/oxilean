//! # LintResult - Trait Implementations
//!
//! This module contains trait implementations for `LintResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LintResult;
use std::fmt;

impl std::fmt::Display for LintResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LintResult({} diagnostics)", self.diagnostics.len())
    }
}
