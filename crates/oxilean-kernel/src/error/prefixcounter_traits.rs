//! # PrefixCounter - Trait Implementations
//!
//! This module contains trait implementations for `PrefixCounter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PrefixCounter;
use std::fmt;

impl Default for PrefixCounter {
    fn default() -> Self {
        Self::new()
    }
}
