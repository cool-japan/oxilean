//! # IncrementalConfig - Trait Implementations
//!
//! This module contains trait implementations for `IncrementalConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::path::{Path, PathBuf};

use super::types::IncrementalConfig;

impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            use_metadata_check: true,
            verify_cached: false,
            compiler_version: "0.1.1".to_string(),
            build_flags: Vec::new(),
            cache_dir: PathBuf::from(".oxilean-cache"),
            max_cache_bytes: 1024 * 1024 * 1024,
        }
    }
}
