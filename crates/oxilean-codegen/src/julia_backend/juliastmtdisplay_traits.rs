//! # JuliaStmtDisplay - Trait Implementations
//!
//! This module contains trait implementations for `JuliaStmtDisplay`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{emit_expr, emit_stmt_inline};
use super::types::JuliaStmtDisplay;
use std::fmt;

impl<'a> fmt::Display for JuliaStmtDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        emit_stmt_inline(f, self.0)
    }
}
