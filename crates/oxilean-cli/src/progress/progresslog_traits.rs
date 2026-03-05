//! # ProgressLog - Trait Implementations
//!
//! This module contains trait implementations for `ProgressLog`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProgressLog;
use std::fmt;

impl Default for ProgressLog {
    fn default() -> Self {
        Self::new()
    }
}
