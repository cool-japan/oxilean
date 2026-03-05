//! # IoMetrics - Trait Implementations
//!
//! This module contains trait implementations for `IoMetrics`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IoMetrics;

impl Default for IoMetrics {
    fn default() -> Self {
        Self::new(1000, 256)
    }
}
