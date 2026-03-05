//! # OptionValue - Trait Implementations
//!
//! This module contains trait implementations for `OptionValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OptionValue;
use std::fmt;

impl fmt::Display for OptionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionValue::Bool(b) => write!(f, "{}", b),
            OptionValue::Nat(n) => write!(f, "{}", n),
            OptionValue::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}
