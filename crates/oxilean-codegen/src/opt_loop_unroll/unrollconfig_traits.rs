//! # UnrollConfig - Trait Implementations
//!
//! This module contains trait implementations for `UnrollConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::UnrollConfig;

impl Default for UnrollConfig {
    fn default() -> Self {
        UnrollConfig {
            max_unroll_factor: 8,
            max_unrolled_size: 256,
            unroll_full_threshold: 16,
            enable_vectorizable: true,
            enable_jamming: true,
            min_trip_count_for_partial: 4,
        }
    }
}
