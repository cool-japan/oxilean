//! # RubyType - Trait Implementations
//!
//! This module contains trait implementations for `RubyType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyType;
use std::fmt;

impl fmt::Display for RubyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RubyType::Integer => write!(f, "Integer"),
            RubyType::Float => write!(f, "Float"),
            RubyType::String => write!(f, "String"),
            RubyType::Bool => write!(f, "T::Boolean"),
            RubyType::Nil => write!(f, "NilClass"),
            RubyType::Array(inner) => write!(f, "Array[{}]", inner),
            RubyType::Hash(k, v) => write!(f, "Hash[{}, {}]", k, v),
            RubyType::Symbol => write!(f, "Symbol"),
            RubyType::Object(name) => write!(f, "{}", name),
            RubyType::Proc => write!(f, "Proc"),
        }
    }
}
