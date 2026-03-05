//! # Diagnostic - Trait Implementations
//!
//! This module contains trait implementations for `Diagnostic`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{Diagnostic, Severity};
use std::fmt;

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity_str = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        };
        if let Some(code) = &self.code {
            write!(
                f,
                "{}[{}]: {} at {}:{}",
                severity_str, code, self.message, self.span.line, self.span.column
            )?;
        } else {
            write!(
                f,
                "{}: {} at {}:{}",
                severity_str, self.message, self.span.line, self.span.column
            )?;
        }
        for label in &self.labels {
            write!(f, "\n  {}", label.text)?;
        }
        if let Some(help) = &self.help {
            write!(f, "\nhelp: {}", help)?;
        }
        Ok(())
    }
}
