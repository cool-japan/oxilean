//! # RubyBlock - Trait Implementations
//!
//! This module contains trait implementations for `RubyBlock`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyBlock;
use std::fmt;

impl std::fmt::Display for RubyBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.params.is_empty() {
            write!(f, "{{ {} }}", self.body)
        } else {
            write!(f, "{{ |{}| {} }}", self.params.join(", "), self.body)
        }
    }
}
