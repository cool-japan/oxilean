//! # CacheOptConfig - Trait Implementations
//!
//! This module contains trait implementations for `CacheOptConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{CacheOptConfig, LoopTilingConfig};

impl Default for CacheOptConfig {
    fn default() -> Self {
        CacheOptConfig {
            cache_line_size: 64,
            prefetch_distance: 8,
            enable_prefetch: true,
            tiling: LoopTilingConfig::default(),
        }
    }
}
