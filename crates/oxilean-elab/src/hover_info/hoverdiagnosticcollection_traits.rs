//! # HoverDiagnosticCollection - Trait Implementations
//!
//! This module contains trait implementations for `HoverDiagnosticCollection`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HoverDiagnosticCollection;
use std::fmt;

impl Default for HoverDiagnosticCollection {
    fn default() -> Self {
        HoverDiagnosticCollection::new()
    }
}
