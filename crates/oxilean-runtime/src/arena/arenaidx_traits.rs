//! # ArenaIdx - Trait Implementations
//!
//! This module contains trait implementations for `ArenaIdx`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ArenaIdx;
use std::fmt;

impl fmt::Display for ArenaIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}
