//! # RustLit - Trait Implementations
//!
//! This module contains trait implementations for `RustLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::RustLit;
use std::fmt;

impl fmt::Display for RustLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustLit::Int(n) => write!(f, "{}", n),
            RustLit::UInt(n) => write!(f, "{}", n),
            RustLit::Float(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0", *v as i64)
                } else {
                    write!(f, "{}", v)
                }
            }
            RustLit::Bool(b) => write!(f, "{}", b),
            RustLit::Char(c) => write!(f, "'{}'", c),
            RustLit::Str(s) => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '"' => write!(f, "\\\"")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        '\r' => write!(f, "\\r")?,
                        '\t' => write!(f, "\\t")?,
                        c => write!(f, "{}", c)?,
                    }
                }
                write!(f, "\"")
            }
            RustLit::Unit => write!(f, "()"),
        }
    }
}
