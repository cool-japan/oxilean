//! # FutharkScatter - Trait Implementations
//!
//! This module contains trait implementations for `FutharkScatter`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkScatter;
use std::fmt;

impl std::fmt::Display for FutharkScatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scatter {} {} {}", self.dest, self.indices, self.values)
    }
}
