//! # PartialEvalBenchConfig - Trait Implementations
//!
//! This module contains trait implementations for `PartialEvalBenchConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PartialEvalBenchConfig;
use std::fmt;

impl Default for PartialEvalBenchConfig {
    fn default() -> Self {
        Self {
            max_steps: 100_000,
            record_step_times: false,
            nf_only: false,
        }
    }
}
