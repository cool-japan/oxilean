//! # LintCategory - Trait Implementations
//!
//! This module contains trait implementations for `LintCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::types::LintCategory;

impl fmt::Display for LintCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LintCategory::Correctness => write!(f, "correctness"),
            LintCategory::Style => write!(f, "style"),
            LintCategory::Performance => write!(f, "performance"),
            LintCategory::Complexity => write!(f, "complexity"),
            LintCategory::Documentation => write!(f, "documentation"),
            LintCategory::Deprecated => write!(f, "deprecated"),
        }
    }
}
