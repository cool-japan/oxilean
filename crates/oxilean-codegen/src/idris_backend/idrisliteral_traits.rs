//! # IdrisLiteral - Trait Implementations
//!
//! This module contains trait implementations for `IdrisLiteral`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IdrisLiteral;
use std::fmt;

impl fmt::Display for IdrisLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdrisLiteral::Int(n) => write!(f, "{}", n),
            IdrisLiteral::Nat(n) => write!(f, "{}", n),
            IdrisLiteral::Float(x) => write!(f, "{}", x),
            IdrisLiteral::Char(c) => write!(f, "'{}'", c),
            IdrisLiteral::Str(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            IdrisLiteral::True => write!(f, "True"),
            IdrisLiteral::False => write!(f, "False"),
            IdrisLiteral::Unit => write!(f, "()"),
        }
    }
}
