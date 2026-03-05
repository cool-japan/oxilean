//! # LcnfLit - Trait Implementations
//!
//! This module contains trait implementations for `LcnfLit`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LcnfLit;
use std::fmt;

impl fmt::Display for LcnfLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LcnfLit::Nat(n) => write!(f, "{}", n),
            LcnfLit::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}
