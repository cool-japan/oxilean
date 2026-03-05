//! # LinearCombLogger - Trait Implementations
//!
//! This module contains trait implementations for `LinearCombLogger`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LinearCombLogger;

impl Default for LinearCombLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}
