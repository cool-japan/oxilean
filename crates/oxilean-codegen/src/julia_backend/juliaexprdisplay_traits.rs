//! # JuliaExprDisplay - Trait Implementations
//!
//! This module contains trait implementations for `JuliaExprDisplay`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{emit_expr, emit_stmt_inline};
use super::types::JuliaExprDisplay;
use std::fmt;

impl<'a> fmt::Display for JuliaExprDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        emit_expr(f, self.0)
    }
}
