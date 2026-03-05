//! # DiagnosticThreshold - Trait Implementations
//!
//! This module contains trait implementations for `DiagnosticThreshold`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagnosticThreshold;
use std::fmt;

impl Default for DiagnosticThreshold {
    fn default() -> Self {
        Self {
            promote_info_to_warning: false,
            promote_warnings_to_errors: false,
            demote_errors_to_warnings: false,
        }
    }
}
