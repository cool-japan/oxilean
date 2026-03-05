//! # UriPrefixFilter - Trait Implementations
//!
//! This module contains trait implementations for `UriPrefixFilter`.
//!
//! ## Implemented Traits
//!
//! - `DiagnosticFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::DiagnosticFilter;
use super::types::{AdvDiagnostic, UriPrefixFilter};
use std::fmt;

impl DiagnosticFilter for UriPrefixFilter {
    fn accepts(&self, diag: &AdvDiagnostic) -> bool {
        diag.uri.starts_with(&self.prefix)
    }
}
