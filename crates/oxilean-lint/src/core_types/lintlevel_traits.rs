//! # LintLevel - Trait Implementations
//!
//! This module contains trait implementations for `LintLevel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LintLevel;
use std::fmt;

impl std::fmt::Display for LintLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LintLevel::Allow => write!(f, "allow"),
            LintLevel::Warn => write!(f, "warn"),
            LintLevel::Deny => write!(f, "deny"),
        }
    }
}
