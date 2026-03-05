//! # CacheConfig - Trait Implementations
//!
//! This module contains trait implementations for `CacheConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::path::PathBuf;

use super::types::CacheConfig;

impl Default for CacheConfig {
    fn default() -> Self {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        Self {
            enabled: true,
            cache_dir: home.join(".oxilean").join("cache"),
            max_size: 1 << 30,
            ttl: 0,
        }
    }
}
