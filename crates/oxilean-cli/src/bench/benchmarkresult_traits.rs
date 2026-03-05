//! # BenchmarkResult - Trait Implementations
//!
//! This module contains trait implementations for `BenchmarkResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::format_ns;
use super::types::BenchmarkResult;
use std::fmt;

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<30} {:>8} iters  mean={:>10}  min={:>10}  max={:>10}  sd={:>10}",
            self.name,
            self.iterations,
            format_ns(self.mean_time),
            format_ns(self.min_time),
            format_ns(self.max_time),
            format_ns(self.std_dev),
        )
    }
}
