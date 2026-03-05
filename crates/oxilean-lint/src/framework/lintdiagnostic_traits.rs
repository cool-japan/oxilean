//! # LintDiagnostic - Trait Implementations
//!
//! This module contains trait implementations for `LintDiagnostic`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;

use super::types::LintDiagnostic;

impl fmt::Display for LintDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {} (at {})",
            self.lint_id, self.severity, self.message, self.range
        )?;
        for note in &self.notes {
            write!(f, "\n  note: {}", note)?;
        }
        if let Some(ref fix) = self.fix {
            write!(f, "\n  fix: {}", fix)?;
        }
        Ok(())
    }
}
