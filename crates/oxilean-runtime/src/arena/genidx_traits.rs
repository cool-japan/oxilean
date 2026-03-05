//! # GenIdx - Trait Implementations
//!
//! This module contains trait implementations for `GenIdx`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::GenIdx;
use std::fmt;

impl fmt::Display for GenIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}g{}", self.index, self.generation)
    }
}
