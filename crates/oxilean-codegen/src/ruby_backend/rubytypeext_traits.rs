//! # RubyTypeExt - Trait Implementations
//!
//! This module contains trait implementations for `RubyTypeExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyTypeExt;
use std::fmt;

impl std::fmt::Display for RubyTypeExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RubyTypeExt::Integer => write!(f, "Integer"),
            RubyTypeExt::Float => write!(f, "Float"),
            RubyTypeExt::String => write!(f, "String"),
            RubyTypeExt::Symbol => write!(f, "Symbol"),
            RubyTypeExt::Bool => write!(f, "T::Boolean"),
            RubyTypeExt::Nil => write!(f, "NilClass"),
            RubyTypeExt::Array(t) => write!(f, "T::Array[{}]", t),
            RubyTypeExt::Hash(k, v) => write!(f, "T::Hash[{}, {}]", k, v),
            RubyTypeExt::Proc => write!(f, "Proc"),
            RubyTypeExt::Lambda => write!(f, "T.proc"),
            RubyTypeExt::Range => write!(f, "T::Range[Integer]"),
            RubyTypeExt::Struct(n) => write!(f, "{}", n),
            RubyTypeExt::Class(n) => write!(f, "T.class_of({})", n),
            RubyTypeExt::Module(n) => write!(f, "{}", n),
            RubyTypeExt::Any => write!(f, "T.untyped"),
        }
    }
}
