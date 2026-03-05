//! # DiagnosticDiffTracker - Trait Implementations
//!
//! This module contains trait implementations for `DiagnosticDiffTracker`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagnosticDiffTracker;
use std::fmt;

impl Default for DiagnosticDiffTracker {
    fn default() -> Self {
        Self::new()
    }
}
