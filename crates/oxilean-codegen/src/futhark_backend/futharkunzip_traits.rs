//! # FutharkUnzip - Trait Implementations
//!
//! This module contains trait implementations for `FutharkUnzip`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkUnzip;
use std::fmt;

impl std::fmt::Display for FutharkUnzip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unzip {}", self.array)
    }
}
