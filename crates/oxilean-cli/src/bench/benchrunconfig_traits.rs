//! # BenchRunConfig - Trait Implementations
//!
//! This module contains trait implementations for `BenchRunConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BenchRunConfig;
use std::fmt;

#[allow(dead_code)]
impl Default for BenchRunConfig {
    fn default() -> Self {
        Self {
            output_dir: std::path::PathBuf::from(".oxilean/bench"),
            save_results: true,
            compare_with_baseline: false,
            regression_threshold_pct: 5.0,
            verbose: false,
            filter: None,
        }
    }
}
