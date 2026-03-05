//! # ParallelConfig - Trait Implementations
//!
//! This module contains trait implementations for `ParallelConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParallelConfig;

impl Default for ParallelConfig {
    fn default() -> Self {
        ParallelConfig {
            min_speedup_threshold: 1.5,
            max_functions: 1024,
            allow_speculative: false,
            hardware_threads: 8,
        }
    }
}
