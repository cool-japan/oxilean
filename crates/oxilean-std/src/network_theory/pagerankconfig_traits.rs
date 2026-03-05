//! # PageRankConfig - Trait Implementations
//!
//! This module contains trait implementations for `PageRankConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PageRankConfig;

impl Default for PageRankConfig {
    fn default() -> Self {
        PageRankConfig {
            damping: 0.85,
            max_iter: 100,
            tol: 1e-8,
        }
    }
}
