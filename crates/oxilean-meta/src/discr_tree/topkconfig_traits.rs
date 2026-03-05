//! # TopKConfig - Trait Implementations
//!
//! This module contains trait implementations for `TopKConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TopKConfig;

impl Default for TopKConfig {
    fn default() -> Self {
        Self {
            k: 10,
            min_score: i64::MIN,
            include_approximate: true,
        }
    }
}
