//! # SwiftLit - Trait Implementations
//!
//! This module contains trait implementations for `SwiftLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::SwiftLit;
use std::fmt;

impl fmt::Display for SwiftLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwiftLit::Int(n) => write!(f, "{}", n),
            SwiftLit::Bool(b) => write!(f, "{}", b),
            SwiftLit::Nil => write!(f, "nil"),
            SwiftLit::Float(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{}.0", *v as i64)
                } else {
                    write!(f, "{}", v)
                }
            }
            SwiftLit::Str(s) => {
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
