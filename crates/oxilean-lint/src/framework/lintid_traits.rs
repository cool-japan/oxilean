//! # LintId - Trait Implementations
//!
//! This module contains trait implementations for `LintId`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `From`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::types::LintId;

impl fmt::Display for LintId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for LintId {
    fn from(s: &str) -> Self {
        LintId::new(s)
    }
}
