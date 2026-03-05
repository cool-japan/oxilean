//! # BenchmarkConfig - Trait Implementations
//!
//! This module contains trait implementations for `BenchmarkConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::time::{Duration, Instant};

use super::types::BenchmarkConfig;

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 5,
            measure_iterations: 100,
            timeout: Duration::from_secs(60),
            warmup_iters: 5,
            measurement_iters: 100,
            verbose: false,
            filter: None,
        }
    }
}
