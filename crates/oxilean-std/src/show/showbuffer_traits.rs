//! # ShowBuffer - Trait Implementations
//!
//! This module contains trait implementations for `ShowBuffer`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ShowBuffer;
use std::fmt;

impl fmt::Display for ShowBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.buf)
    }
}
