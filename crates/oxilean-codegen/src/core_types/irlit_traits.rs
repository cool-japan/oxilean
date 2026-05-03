//! # IrLit - Trait Implementations
//!
//! This module contains trait implementations for `IrLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IrLit;
use std::fmt;

impl fmt::Display for IrLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrLit::Unit => write!(f, "()"),
            IrLit::Bool(b) => write!(f, "{}", b),
            IrLit::Nat(n) => write!(f, "{}", n),
            IrLit::Int(i) => write!(f, "{}", i),
            IrLit::String(s) => write!(f, "\"{}\"", s),
        }
    }
}
