//! # InputBuffer - Trait Implementations
//!
//! This module contains trait implementations for `InputBuffer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::InputBuffer;
use std::fmt;

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}
