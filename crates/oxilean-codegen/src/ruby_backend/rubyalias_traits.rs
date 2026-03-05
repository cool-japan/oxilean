//! # RubyAlias - Trait Implementations
//!
//! This module contains trait implementations for `RubyAlias`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyAlias;
use std::fmt;

impl std::fmt::Display for RubyAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "alias :{} :{}", self.new_name, self.old_name)
    }
}
