//! # DiffTestResult - Trait Implementations
//!
//! This module contains trait implementations for `DiffTestResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiffTestResult;
use std::fmt;

impl std::fmt::Display for DiffTestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffTestResult::Pass => write!(f, "PASS"),
            DiffTestResult::Fail { actual, expected } => {
                write!(f, "FAIL\n  expected: {}\n  actual:   {}", expected, actual)
            }
            DiffTestResult::Error(msg) => write!(f, "ERROR: {}", msg),
            DiffTestResult::Unexpected => write!(f, "UNEXPECTED"),
        }
    }
}
