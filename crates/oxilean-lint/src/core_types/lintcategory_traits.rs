//! # LintCategory - Trait Implementations
//!
//! This module contains trait implementations for `LintCategory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LintCategory;
use std::fmt;

impl fmt::Display for LintCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LintCategory::Correctness => write!(f, "correctness"),
            LintCategory::Style => write!(f, "style"),
            LintCategory::Performance => write!(f, "performance"),
            LintCategory::Complexity => write!(f, "complexity"),
            LintCategory::Deprecation => write!(f, "deprecation"),
            LintCategory::Documentation => write!(f, "documentation"),
            LintCategory::Naming => write!(f, "naming"),
            LintCategory::Redundancy => write!(f, "redundancy"),
            LintCategory::Security => write!(f, "security"),
            LintCategory::Custom(ref name) => write!(f, "custom:{}", name),
        }
    }
}
