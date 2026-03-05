//! # RubyStructDef - Trait Implementations
//!
//! This module contains trait implementations for `RubyStructDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyStructDef;
use std::fmt;

impl std::fmt::Display for RubyStructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let members: Vec<String> = self
            .members
            .iter()
            .map(|(n, _)| format!(":{}", n))
            .collect();
        write!(f, "{} = Struct.new({})", self.name, members.join(", "))
    }
}
