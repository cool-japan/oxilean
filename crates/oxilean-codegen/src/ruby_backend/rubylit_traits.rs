//! # RubyLit - Trait Implementations
//!
//! This module contains trait implementations for `RubyLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_ruby_class, fmt_ruby_method, fmt_ruby_module_stmt, fmt_ruby_stmt};
use super::types::RubyLit;
use std::fmt;

impl fmt::Display for RubyLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RubyLit::Int(n) => write!(f, "{}", n),
            RubyLit::Float(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}.0", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            RubyLit::Str(s) => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '"' => write!(f, "\\\"")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        '\r' => write!(f, "\\r")?,
                        '\t' => write!(f, "\\t")?,
                        '#' => write!(f, "\\#")?,
                        c => write!(f, "{}", c)?,
                    }
                }
                write!(f, "\"")
            }
            RubyLit::Bool(b) => write!(f, "{}", b),
            RubyLit::Nil => write!(f, "nil"),
            RubyLit::Symbol(name) => write!(f, ":{}", name),
        }
    }
}
