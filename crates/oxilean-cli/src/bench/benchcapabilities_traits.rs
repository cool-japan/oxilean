//! # BenchCapabilities - Trait Implementations
//!
//! This module contains trait implementations for `BenchCapabilities`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BenchCapabilities;
use std::fmt;

#[allow(dead_code)]
impl Default for BenchCapabilities {
    fn default() -> Self {
        Self {
            supports_warmup: true,
            supports_parallel: false,
            supports_cpu_pinning: false,
            supports_memory_profiling: false,
            supports_flamegraph: true,
            max_duration_secs: 300,
        }
    }
}
