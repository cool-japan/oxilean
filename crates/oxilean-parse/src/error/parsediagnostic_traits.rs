//! # ParseDiagnostic - Trait Implementations
//!
//! This module contains trait implementations for `ParseDiagnostic`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseDiagnostic;
use std::fmt;

impl fmt::Display for ParseDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {}: {}",
            self.filename, self.line, self.col, self.severity, self.message
        )?;
        if let Some(h) = &self.hint {
            write!(f, "\n  hint: {}", h)?;
        }
        Ok(())
    }
}
