//! # MatlabLiteral - Trait Implementations
//!
//! This module contains trait implementations for `MatlabLiteral`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MatlabLiteral;
use std::fmt;

impl std::fmt::Display for MatlabLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatlabLiteral::Double(v) => {
                if v.fract() == 0.0 && v.abs() < 1e15 {
                    write!(f, "{}", *v as i64)
                } else {
                    write!(f, "{}", v)
                }
            }
            MatlabLiteral::Integer(n) => write!(f, "{}", n),
            MatlabLiteral::Logical(b) => {
                write!(f, "{}", if *b { "true" } else { "false" })
            }
            MatlabLiteral::Char(s) => write!(f, "'{}'", s.replace('\'', "''")),
            MatlabLiteral::Str(s) => write!(f, "\"{}\"", s.replace('"', "\"\"")),
            MatlabLiteral::Empty => write!(f, "[]"),
            MatlabLiteral::NaN => write!(f, "NaN"),
            MatlabLiteral::Inf(neg) => write!(f, "{}Inf", if *neg { "-" } else { "" }),
            MatlabLiteral::Pi => write!(f, "pi"),
            MatlabLiteral::Eps => write!(f, "eps"),
        }
    }
}
