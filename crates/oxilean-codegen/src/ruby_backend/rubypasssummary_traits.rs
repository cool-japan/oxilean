//! # RubyPassSummary - Trait Implementations
//!
//! This module contains trait implementations for `RubyPassSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyPassSummary;
use std::fmt;

impl std::fmt::Display for RubyPassSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RubyPassSummary[{}] {{ fns={}, classes={}, modules={}, {}us }}",
            self.pass_name,
            self.functions_compiled,
            self.classes_emitted,
            self.modules_emitted,
            self.duration_us,
        )
    }
}
