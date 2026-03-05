//! # RubyRequire - Trait Implementations
//!
//! This module contains trait implementations for `RubyRequire`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyRequire;
use std::fmt;

impl std::fmt::Display for RubyRequire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyRequire::Require(p) => write!(f, "require \"{}\"", p),
            RubyRequire::RequireRelative(p) => write!(f, "require_relative \"{}\"", p),
            RubyRequire::Autoload(n, p) => write!(f, "autoload :{}, \"{}\"", n, p),
        }
    }
}
