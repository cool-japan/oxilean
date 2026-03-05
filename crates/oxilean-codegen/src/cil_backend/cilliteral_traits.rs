//! # CilLiteral - Trait Implementations
//!
//! This module contains trait implementations for `CilLiteral`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilLiteral;
use std::fmt;

impl fmt::Display for CilLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CilLiteral::Bool(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            CilLiteral::Int32(n) => write!(f, "{}", n),
            CilLiteral::Int64(n) => write!(f, "{}L", n),
            CilLiteral::Float32(v) => write!(f, "{}f", v),
            CilLiteral::Float64(v) => write!(f, "{}", v),
            CilLiteral::String(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            CilLiteral::Null => write!(f, "null"),
        }
    }
}
