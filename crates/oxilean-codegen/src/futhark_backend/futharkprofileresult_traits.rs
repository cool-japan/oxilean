//! # FutharkProfileResult - Trait Implementations
//!
//! This module contains trait implementations for `FutharkProfileResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkProfileResult;
use std::fmt;

impl std::fmt::Display for FutharkProfileResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: runs={}, mean={:.1}us, min={}us, max={}us",
            self.kernel_name, self.runs, self.mean_us, self.min_us, self.max_us
        )
    }
}
