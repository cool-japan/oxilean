//! # FfiSafety - Trait Implementations
//!
//! This module contains trait implementations for `FfiSafety`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FfiSafety;
use std::fmt;

impl fmt::Display for FfiSafety {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiSafety::Safe => write!(f, "safe"),
            FfiSafety::Unsafe => write!(f, "unsafe"),
            FfiSafety::System => write!(f, "system"),
        }
    }
}
