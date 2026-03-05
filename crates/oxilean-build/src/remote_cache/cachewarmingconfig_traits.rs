//! # CacheWarmingConfig - Trait Implementations
//!
//! This module contains trait implementations for `CacheWarmingConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CacheWarmingConfig;

impl Default for CacheWarmingConfig {
    fn default() -> Self {
        Self::disabled()
    }
}
