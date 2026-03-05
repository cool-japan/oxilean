//! # ProfilerConfig - Trait Implementations
//!
//! This module contains trait implementations for `ProfilerConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProfilerConfig;

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            event_profiling: false,
            sampling_profiling: false,
            sampling_interval_ns: 1_000_000,
            max_events: 100_000,
            track_gc: true,
            track_allocs: true,
        }
    }
}
