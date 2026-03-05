//! # JuliaParam - Trait Implementations
//!
//! This module contains trait implementations for `JuliaParam`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{emit_expr, emit_stmt_inline};
use super::types::{JuliaExprDisplay, JuliaParam};
use std::fmt;

impl fmt::Display for JuliaParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ref ty) = self.ty {
            write!(f, "::{}", ty)?;
        }
        if self.is_splat {
            write!(f, "...")?;
        }
        if let Some(ref default) = self.default {
            write!(f, " = {}", JuliaExprDisplay(default))?;
        }
        Ok(())
    }
}
