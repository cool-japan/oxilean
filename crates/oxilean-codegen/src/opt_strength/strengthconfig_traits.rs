//! # StrengthConfig - Trait Implementations
//!
//! This module contains trait implementations for `StrengthConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::StrengthConfig;
use std::fmt;

impl Default for StrengthConfig {
    fn default() -> Self {
        StrengthConfig {
            max_shift_count: 3,
            optimize_div: true,
            optimize_inc: true,
        }
    }
}

impl fmt::Display for StrengthConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StrengthConfig {{ max_shift={}, opt_div={}, opt_inc={} }}",
            self.max_shift_count, self.optimize_div, self.optimize_inc
        )
    }
}
