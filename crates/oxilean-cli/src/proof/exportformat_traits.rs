//! # ExportFormat - Trait Implementations
//!
//! This module contains trait implementations for `ExportFormat`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExportFormat;
use std::fmt;

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportFormat::Text => write!(f, "text"),
            ExportFormat::Json => write!(f, "json"),
            ExportFormat::Lean4 => write!(f, "lean4"),
            ExportFormat::Markdown => write!(f, "markdown"),
        }
    }
}
