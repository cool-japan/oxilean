//! # BenchResult - Trait Implementations
//!
//! This module contains trait implementations for `BenchResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BenchResult;

impl std::fmt::Display for BenchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:.3} ms total, {:.3} us/iter ({} iters)",
            self.name,
            self.duration_ms,
            self.avg_us(),
            self.iterations
        )
    }
}
