//! # PythonLit - Trait Implementations
//!
//! This module contains trait implementations for `PythonLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::PythonLit;
use std::fmt;

impl fmt::Display for PythonLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PythonLit::Int(n) => write!(f, "{}", n),
            PythonLit::Float(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            PythonLit::Str(s) => {
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
            PythonLit::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            PythonLit::None => write!(f, "None"),
            PythonLit::Bytes(bytes) => {
                write!(f, "b\"")?;
                for b in bytes {
                    write!(f, "\\x{:02x}", b)?;
                }
                write!(f, "\"")
            }
            PythonLit::Ellipsis => write!(f, "..."),
        }
    }
}
