//! # DiffTestReport - Trait Implementations
//!
//! This module contains trait implementations for `DiffTestReport`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiffTestReport;
use std::fmt;

impl std::fmt::Display for DiffTestReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DiffTestReport {{ total: {}, passed: {}, failed: {}, errors: {}, pass_rate: {:.1}% }}",
            self.total,
            self.passed,
            self.failed,
            self.errors,
            self.pass_rate() * 100.0
        )
    }
}
