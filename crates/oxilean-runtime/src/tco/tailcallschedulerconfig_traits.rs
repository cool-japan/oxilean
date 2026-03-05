//! # TailCallSchedulerConfig - Trait Implementations
//!
//! This module contains trait implementations for `TailCallSchedulerConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TailCallSchedulerConfig;

impl Default for TailCallSchedulerConfig {
    fn default() -> Self {
        Self {
            max_steps_per_batch: 1_000,
            step_limit: 10_000_000,
        }
    }
}
