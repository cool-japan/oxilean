//! # LintStats - Trait Implementations
//!
//! This module contains trait implementations for `LintStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LintStats;
use std::fmt;

impl fmt::Display for LintStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LintStats {{ total: {}, errors: {}, warnings: {}, infos: {}, hints: {} }}",
            self.total_diagnostics, self.errors, self.warnings, self.infos, self.hints
        )
    }
}
