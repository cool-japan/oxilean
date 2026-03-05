//! # RubyCaseIn - Trait Implementations
//!
//! This module contains trait implementations for `RubyCaseIn`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyCaseIn;
use std::fmt;

impl std::fmt::Display for RubyCaseIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "case {}", self.scrutinee)?;
        for (pat, body) in &self.arms {
            writeln!(f, "in {}", pat)?;
            writeln!(f, "  {}", body)?;
        }
        if let Some(els) = &self.else_body {
            writeln!(f, "else")?;
            writeln!(f, "  {}", els)?;
        }
        write!(f, "end")
    }
}
