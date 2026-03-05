//! # ReplState - Trait Implementations
//!
//! This module contains trait implementations for `ReplState`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ReplState;
use std::fmt;

impl Default for ReplState {
    fn default() -> Self {
        Self::new()
    }
}
