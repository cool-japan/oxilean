//! # BuildExecutionReport - Trait Implementations
//!
//! This module contains trait implementations for `BuildExecutionReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildExecutionReport;

impl std::fmt::Display for BuildExecutionReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Report[succeeded={} failed={} skipped={} wall={}ms]",
            self.succeeded.len(),
            self.failed.len(),
            self.skipped.len(),
            self.wall_ms,
        )
    }
}
