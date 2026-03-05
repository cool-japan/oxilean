//! # RoundTripConfig - Trait Implementations
//!
//! This module contains trait implementations for `RoundTripConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RoundTripConfig;

impl Default for RoundTripConfig {
    fn default() -> Self {
        Self {
            normalize_whitespace: true,
            max_diff_chars: 0,
            ignore_spans: true,
        }
    }
}
