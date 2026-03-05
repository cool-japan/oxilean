//! # BenchHarnessV2 - Trait Implementations
//!
//! This module contains trait implementations for `BenchHarnessV2`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BenchHarnessV2;

impl Default for BenchHarnessV2 {
    fn default() -> Self {
        Self::new(10, 100)
    }
}
