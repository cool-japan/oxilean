//! # RubyCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `RubyCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyCodeStats;
use std::fmt;

impl std::fmt::Display for RubyCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RubyCodeStats {{ classes={}, modules={}, methods={}, lambdas={}, lines={} }}",
            self.classes, self.modules, self.methods, self.lambdas, self.total_lines,
        )
    }
}
