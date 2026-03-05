//! # DiagnosticBudget - Trait Implementations
//!
//! This module contains trait implementations for `DiagnosticBudget`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagnosticBudget;
use std::fmt;

impl Default for DiagnosticBudget {
    fn default() -> Self {
        Self {
            max_errors: 100,
            max_warnings: 200,
            max_total: 500,
        }
    }
}
