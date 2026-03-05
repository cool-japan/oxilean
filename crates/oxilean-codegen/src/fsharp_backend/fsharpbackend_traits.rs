//! # FSharpBackend - Trait Implementations
//!
//! This module contains trait implementations for `FSharpBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FSharpBackend;
use std::fmt;

impl Default for FSharpBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FSharpBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FSharpBackend")
    }
}
