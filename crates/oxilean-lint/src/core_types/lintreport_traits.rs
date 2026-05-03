//! # LintReport - Trait Implementations
//!
//! This module contains trait implementations for `LintReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LintReport;
use std::fmt;

impl fmt::Display for LintReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LintReport[{}] {{ {} diags, {} }}",
            self.filename,
            self.diagnostics.len(),
            self.stats
        )
    }
}
