//! # BenchConfig - Trait Implementations
//!
//! This module contains trait implementations for `BenchConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BenchConfig;
use std::fmt;

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            warmup_rounds: 5,
            time_limit_ms: None,
        }
    }
}
