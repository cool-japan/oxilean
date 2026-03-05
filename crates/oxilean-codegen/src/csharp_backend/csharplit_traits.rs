//! # CSharpLit - Trait Implementations
//!
//! This module contains trait implementations for `CSharpLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CSharpLit;
use std::fmt;

impl fmt::Display for CSharpLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CSharpLit::Int(n) => write!(f, "{}", n),
            CSharpLit::Long(n) => write!(f, "{}L", n),
            CSharpLit::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            CSharpLit::Null => write!(f, "null"),
            CSharpLit::Float(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0f", *v as i64)
                } else {
                    write!(f, "{}f", v)
                }
            }
            CSharpLit::Double(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0", *v as i64)
                } else {
                    write!(f, "{}", v)
                }
            }
            CSharpLit::Char(c) => write!(f, "'{}'", c),
            CSharpLit::Str(s) => {
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
        }
    }
}
