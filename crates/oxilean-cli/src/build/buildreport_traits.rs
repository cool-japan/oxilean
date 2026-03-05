//! # BuildReport - Trait Implementations
//!
//! This module contains trait implementations for `BuildReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_build_report;
use super::types::BuildReport;
use std::fmt;

impl fmt::Display for BuildReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_build_report(self))
    }
}
