//! # FutharkTuningParam - Trait Implementations
//!
//! This module contains trait implementations for `FutharkTuningParam`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkTuningParam;
use std::fmt;

impl std::fmt::Display for FutharkTuningParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-- tuning: {} = {} (range [{}, {}])",
            self.name, self.default_value, self.min_value, self.max_value
        )
    }
}
