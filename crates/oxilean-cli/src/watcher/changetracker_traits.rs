//! # ChangeTracker - Trait Implementations
//!
//! This module contains trait implementations for `ChangeTracker`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChangeTracker;
use std::fmt;

impl Default for ChangeTracker {
    fn default() -> Self {
        Self::new()
    }
}
