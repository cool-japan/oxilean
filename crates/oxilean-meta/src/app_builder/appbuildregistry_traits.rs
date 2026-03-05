//! # AppBuildRegistry - Trait Implementations
//!
//! This module contains trait implementations for `AppBuildRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AppBuildRegistry;

impl Default for AppBuildRegistry {
    fn default() -> Self {
        Self::new(1000)
    }
}
