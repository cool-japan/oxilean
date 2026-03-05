//! # RubyBackendCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `RubyBackendCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyBackendCodeStats;
use std::fmt;

impl std::fmt::Display for RubyBackendCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RubyBackendCodeStats {{ files={}, classes={}, modules={}, methods={}, lines={} }}",
            self.files, self.classes, self.modules, self.methods, self.lines,
        )
    }
}
