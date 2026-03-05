//! # ScalaLit - Trait Implementations
//!
//! This module contains trait implementations for `ScalaLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ScalaLit;
use std::fmt;

impl fmt::Display for ScalaLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalaLit::Int(n) => write!(f, "{}", n),
            ScalaLit::Long(n) => write!(f, "{}L", n),
            ScalaLit::Double(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0", *v as i64)
                } else {
                    write!(f, "{}", v)
                }
            }
            ScalaLit::Float(v) => write!(f, "{}f", v),
            ScalaLit::Bool(b) => write!(f, "{}", b),
            ScalaLit::Char(c) => write!(f, "'{}'", c),
            ScalaLit::Null => write!(f, "null"),
            ScalaLit::Unit => write!(f, "()"),
            ScalaLit::Str(s) => {
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
