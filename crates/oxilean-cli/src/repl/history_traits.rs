//! # History - Trait Implementations
//!
//! This module contains trait implementations for `History`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::History;
use std::fmt;

impl Default for History {
    fn default() -> Self {
        Self::new(1000)
    }
}
