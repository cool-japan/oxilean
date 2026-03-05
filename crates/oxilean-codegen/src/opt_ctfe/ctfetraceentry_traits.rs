//! # CtfeTraceEntry - Trait Implementations
//!
//! This module contains trait implementations for `CtfeTraceEntry`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CtfeTraceEntry;
use std::fmt;

impl std::fmt::Display for CtfeTraceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = "  ".repeat(self.depth);
        let result = self.result_repr.as_deref().unwrap_or("...");
        write!(
            f,
            "{}{}({}) => {}",
            indent, self.func, self.args_repr, result
        )
    }
}
