//! # ParallelElabConfig - Trait Implementations
//!
//! This module contains trait implementations for `ParallelElabConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParallelElabConfig;
use std::fmt;

impl Default for ParallelElabConfig {
    fn default() -> Self {
        Self {
            num_threads: 0,
            max_parallel_tasks: 256,
            work_stealing: true,
            collect_timing: false,
        }
    }
}
