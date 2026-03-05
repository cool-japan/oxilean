//! # DiagnosticFormatter - Trait Implementations
//!
//! This module contains trait implementations for `DiagnosticFormatter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagnosticFormatter;

impl Default for DiagnosticFormatter {
    fn default() -> Self {
        Self {
            use_color: false,
            show_snippets: true,
            show_fixes: true,
            context_lines: 2,
        }
    }
}
