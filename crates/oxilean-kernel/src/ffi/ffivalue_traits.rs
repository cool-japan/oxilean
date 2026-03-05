//! # FfiValue - Trait Implementations
//!
//! This module contains trait implementations for `FfiValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FfiValue;
use std::fmt;

impl fmt::Display for FfiValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiValue::Bool(b) => write!(f, "{}", b),
            FfiValue::UInt(n) => write!(f, "{}", n),
            FfiValue::Int(n) => write!(f, "{}", n),
            FfiValue::Float(fl) => write!(f, "{}", fl),
            FfiValue::Str(s) => write!(f, "\"{}\"", s),
            FfiValue::Bytes(bs) => write!(f, "{:?}", bs),
            FfiValue::Unit => write!(f, "()"),
        }
    }
}
