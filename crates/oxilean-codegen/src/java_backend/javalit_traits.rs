//! # JavaLit - Trait Implementations
//!
//! This module contains trait implementations for `JavaLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JavaLit;
use std::fmt;

impl fmt::Display for JavaLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JavaLit::Int(n) => write!(f, "{}", n),
            JavaLit::Long(n) => write!(f, "{}L", n),
            JavaLit::Double(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0", *v as i64)
                } else {
                    write!(f, "{}", v)
                }
            }
            JavaLit::Float(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0f", *v as i64)
                } else {
                    write!(f, "{}f", v)
                }
            }
            JavaLit::Bool(b) => write!(f, "{}", b),
            JavaLit::Char(c) => write!(f, "'{}'", c),
            JavaLit::Str(s) => {
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
            JavaLit::Null => write!(f, "null"),
        }
    }
}
