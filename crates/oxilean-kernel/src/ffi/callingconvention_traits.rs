//! # CallingConvention - Trait Implementations
//!
//! This module contains trait implementations for `CallingConvention`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CallingConvention;
use std::fmt;

impl fmt::Display for CallingConvention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallingConvention::Rust => write!(f, "rust"),
            CallingConvention::C => write!(f, "c"),
            CallingConvention::System => write!(f, "system"),
        }
    }
}
