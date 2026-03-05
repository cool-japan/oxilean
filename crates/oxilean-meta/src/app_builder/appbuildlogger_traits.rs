//! # AppBuildLogger - Trait Implementations
//!
//! This module contains trait implementations for `AppBuildLogger`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AppBuildLogger;

impl Default for AppBuildLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}
