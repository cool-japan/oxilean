//! # FutharkZip - Trait Implementations
//!
//! This module contains trait implementations for `FutharkZip`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkZip;
use std::fmt;

impl std::fmt::Display for FutharkZip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "zip {}", self.arrays.join(" "))
    }
}
