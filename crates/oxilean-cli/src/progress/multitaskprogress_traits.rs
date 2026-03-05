//! # MultiTaskProgress - Trait Implementations
//!
//! This module contains trait implementations for `MultiTaskProgress`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MultiTaskProgress;
use std::fmt;

impl Default for MultiTaskProgress {
    fn default() -> Self {
        Self::new()
    }
}
