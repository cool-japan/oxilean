//! # BitWidth - Trait Implementations
//!
//! This module contains trait implementations for `BitWidth`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BitWidth;
use std::fmt;

impl fmt::Display for BitWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
