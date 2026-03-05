//! # ParseErrorReport - Trait Implementations
//!
//! This module contains trait implementations for `ParseErrorReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseErrorReport;

impl std::fmt::Display for ParseErrorReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParseErrorReport[{}]({} errors, {} warnings)",
            self.filename,
            self.error_count(),
            self.warning_count()
        )
    }
}
