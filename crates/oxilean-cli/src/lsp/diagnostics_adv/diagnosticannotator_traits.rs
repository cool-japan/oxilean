//! # DiagnosticAnnotator - Trait Implementations
//!
//! This module contains trait implementations for `DiagnosticAnnotator`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagnosticAnnotator;
use std::fmt;

impl Default for DiagnosticAnnotator {
    fn default() -> Self {
        Self::new()
    }
}
