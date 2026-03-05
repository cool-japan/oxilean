//! # RubyModuleDef - Trait Implementations
//!
//! This module contains trait implementations for `RubyModuleDef`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyModuleDef;
use std::fmt;

impl std::fmt::Display for RubyModuleDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "module {}", self.name)?;
        for inc in &self.includes {
            writeln!(f, "  include {}", inc)?;
        }
        for (n, v) in &self.constants {
            writeln!(f, "  {} = {}", n, v)?;
        }
        for m in &self.methods {
            writeln!(f, "  {}", m)?;
        }
        write!(f, "end")
    }
}
