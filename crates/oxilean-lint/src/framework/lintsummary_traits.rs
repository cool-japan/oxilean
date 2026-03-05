//! # LintSummary - Trait Implementations
//!
//! This module contains trait implementations for `LintSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::types::LintSummary;

impl fmt::Display for LintSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} diagnostic(s): {} error(s), {} warning(s), {} hint(s) ({} fixable)",
            self.total, self.errors, self.warnings, self.hints, self.fixable
        )
    }
}
