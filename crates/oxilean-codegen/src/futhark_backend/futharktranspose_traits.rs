//! # FutharkTranspose - Trait Implementations
//!
//! This module contains trait implementations for `FutharkTranspose`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkTranspose;
use std::fmt;

impl std::fmt::Display for FutharkTranspose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "transpose {}", self.array)
    }
}
