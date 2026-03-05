//! # OcamlLit - Trait Implementations
//!
//! This module contains trait implementations for `OcamlLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ocaml_expr;
use super::types::OcamlLit;
use std::fmt;

impl fmt::Display for OcamlLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OcamlLit::Int(n) => write!(f, "{}", n),
            OcamlLit::Float(x) => {
                if x.fract() == 0.0 && x.is_finite() {
                    write!(f, "{:.1}", x)
                } else {
                    write!(f, "{}", x)
                }
            }
            OcamlLit::Bool(b) => write!(f, "{}", b),
            OcamlLit::Char(c) => write!(f, "'{}'", c),
            OcamlLit::Str(s) => {
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
            OcamlLit::Unit => write!(f, "()"),
        }
    }
}
