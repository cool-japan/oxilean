//! # MatchBasicLogger - Trait Implementations
//!
//! This module contains trait implementations for `MatchBasicLogger`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MatchBasicLogger;

impl Default for MatchBasicLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}
