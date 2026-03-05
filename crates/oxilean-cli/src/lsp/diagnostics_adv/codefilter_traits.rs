//! # CodeFilter - Trait Implementations
//!
//! This module contains trait implementations for `CodeFilter`.
//!
//! ## Implemented Traits
//!
//! - `DiagnosticFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::DiagnosticFilter;
use super::types::{AdvDiagnostic, CodeFilter};
use std::fmt;

impl DiagnosticFilter for CodeFilter {
    fn accepts(&self, diag: &AdvDiagnostic) -> bool {
        self.allowed_codes.contains(&diag.code)
    }
}
