//! # RubyExtEmitStats - Trait Implementations
//!
//! This module contains trait implementations for `RubyExtEmitStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyExtEmitStats;
use std::fmt;

impl std::fmt::Display for RubyExtEmitStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RubyExtEmitStats {{ bytes={}, classes={}, modules={}, methods={}, errors={} }}",
            self.bytes_written,
            self.classes_emitted,
            self.modules_emitted,
            self.methods_emitted,
            self.errors,
        )
    }
}
