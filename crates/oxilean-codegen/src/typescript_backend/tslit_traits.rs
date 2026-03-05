//! # TsLit - Trait Implementations
//!
//! This module contains trait implementations for `TsLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsLit;
use std::fmt;

impl fmt::Display for TsLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsLit::Num(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            TsLit::Str(s) => {
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
            TsLit::Bool(b) => write!(f, "{}", b),
            TsLit::Null => write!(f, "null"),
            TsLit::Undefined => write!(f, "undefined"),
            TsLit::BigInt(n) => write!(f, "{}n", n),
        }
    }
}
