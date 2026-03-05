//! # DistributedCacheConfig - Trait Implementations
//!
//! This module contains trait implementations for `DistributedCacheConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DistributedCacheConfig;

impl Default for DistributedCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: String::new(),
            max_artifact_mb: 512,
            read_timeout_secs: 30,
            write_timeout_secs: 60,
        }
    }
}
