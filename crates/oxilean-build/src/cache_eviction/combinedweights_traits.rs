//! # CombinedWeights - Trait Implementations
//!
//! This module contains trait implementations for `CombinedWeights`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CombinedWeights;

impl Default for CombinedWeights {
    fn default() -> Self {
        Self {
            recency: 1.0,
            frequency: 1.0,
            size: 1.0,
            age: 1.0,
        }
    }
}
