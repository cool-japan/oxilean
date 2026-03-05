//! # CallConvention - Trait Implementations
//!
//! This module contains trait implementations for `CallConvention`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CallConvention;
use std::fmt;

impl fmt::Display for CallConvention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallConvention::ClosureCall => write!(f, "closure"),
            CallConvention::DirectCall => write!(f, "direct"),
            CallConvention::TailCall => write!(f, "tail"),
            CallConvention::IndirectCall => write!(f, "indirect"),
            CallConvention::BuiltinCall => write!(f, "builtin"),
        }
    }
}
