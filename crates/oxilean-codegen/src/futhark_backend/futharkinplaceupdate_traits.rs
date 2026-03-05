//! # FutharkInPlaceUpdate - Trait Implementations
//!
//! This module contains trait implementations for `FutharkInPlaceUpdate`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkInPlaceUpdate;
use std::fmt;

impl std::fmt::Display for FutharkInPlaceUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} with [{}] = {}", self.array, self.index, self.value)
    }
}
