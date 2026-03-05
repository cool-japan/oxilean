//! # RubyFiber - Trait Implementations
//!
//! This module contains trait implementations for `RubyFiber`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyFiber;
use std::fmt;

impl std::fmt::Display for RubyFiber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_async {
            write!(
                f,
                "{} = Async {{ |{}| {} }}",
                self.name,
                self.params.join(", "),
                self.body
            )
        } else {
            write!(f, "{} = Fiber.new {{ {} }}", self.name, self.body)
        }
    }
}
