//! # ChowGroup - Trait Implementations
//!
//! This module contains trait implementations for `ChowGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChowGroup;
use std::fmt;

impl std::fmt::Display for ChowGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CH^{}({})", self.codimension, self.scheme)
    }
}
