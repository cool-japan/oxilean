//! # RubyVisibility - Trait Implementations
//!
//! This module contains trait implementations for `RubyVisibility`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyVisibility;
use std::fmt;

impl fmt::Display for RubyVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RubyVisibility::Public => write!(f, "public"),
            RubyVisibility::Protected => write!(f, "protected"),
            RubyVisibility::Private => write!(f, "private"),
        }
    }
}
