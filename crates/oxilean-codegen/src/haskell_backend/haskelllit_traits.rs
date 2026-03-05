//! # HaskellLit - Trait Implementations
//!
//! This module contains trait implementations for `HaskellLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::HaskellLit;
use std::fmt;

impl fmt::Display for HaskellLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HaskellLit::Int(n) => {
                if *n < 0 {
                    write!(f, "({})", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            HaskellLit::Float(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0", *v as i64)
                } else {
                    write!(f, "{}", v)
                }
            }
            HaskellLit::Char(c) => write!(f, "'{}'", c),
            HaskellLit::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            HaskellLit::Unit => write!(f, "()"),
            HaskellLit::Str(s) => {
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
