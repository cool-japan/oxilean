//! # GcConfig - Trait Implementations
//!
//! This module contains trait implementations for `GcConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{GcConfig, GcStrategy};

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            strategy: GcStrategy::MarkSweep,
            heap_limit: 64 * 1024 * 1024,
            collection_threshold: 0.75,
            incremental_steps: 100,
            write_barriers: true,
            young_gen_size: 64 * 1024,
            old_gen_size: 512 * 1024,
        }
    }
}
