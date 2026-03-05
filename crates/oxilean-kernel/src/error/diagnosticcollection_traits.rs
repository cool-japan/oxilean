//! # DiagnosticCollection - Trait Implementations
//!
//! This module contains trait implementations for `DiagnosticCollection`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagnosticCollection;
use std::fmt;

impl fmt::Display for DiagnosticCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in &self.diagnostics {
            writeln!(f, "{}", d)?;
        }
        Ok(())
    }
}
