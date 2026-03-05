//! # Timer - Trait Implementations
//!
//! This module contains trait implementations for `Timer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Timer;
use std::fmt;

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
