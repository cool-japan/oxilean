//! # JsLit - Trait Implementations
//!
//! This module contains trait implementations for `JsLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::JsLit;
use std::fmt;

impl fmt::Display for JsLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsLit::Num(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            JsLit::BigInt(n) => write!(f, "{}n", n),
            JsLit::Str(s) => {
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
            JsLit::Bool(b) => write!(f, "{}", b),
            JsLit::Null => write!(f, "null"),
            JsLit::Undefined => write!(f, "undefined"),
        }
    }
}
