//! # PrioritySolver - Trait Implementations
//!
//! This module contains trait implementations for `PrioritySolver`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrioritySolver;
use std::fmt;

impl Default for PrioritySolver {
    fn default() -> Self {
        Self::new()
    }
}
