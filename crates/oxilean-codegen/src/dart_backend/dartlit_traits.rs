//! # DartLit - Trait Implementations
//!
//! This module contains trait implementations for `DartLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::{fmt_args, fmt_typed_params};
use super::types::DartLit;
use std::fmt;

impl fmt::Display for DartLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DartLit::Int(n) => write!(f, "{}", n),
            DartLit::Double(v) => {
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{:.1}", v)
                } else {
                    write!(f, "{}", v)
                }
            }
            DartLit::Bool(b) => write!(f, "{}", b),
            DartLit::Str(s) => {
                write!(f, "'")?;
                for c in s.chars() {
                    match c {
                        '\'' => write!(f, "\\'")?,
                        '\\' => write!(f, "\\\\")?,
                        '\n' => write!(f, "\\n")?,
                        '\r' => write!(f, "\\r")?,
                        '\t' => write!(f, "\\t")?,
                        c => write!(f, "{}", c)?,
                    }
                }
                write!(f, "'")
            }
            DartLit::Null => write!(f, "null"),
        }
    }
}
