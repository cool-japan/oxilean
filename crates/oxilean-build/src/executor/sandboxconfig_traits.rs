//! # SandboxConfig - Trait Implementations
//!
//! This module contains trait implementations for `SandboxConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SandboxConfig;

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            read_only_paths: Vec::new(),
            scratch_dir: None,
            allow_network: false,
            max_memory_mb: 0,
            timeout_secs: 0,
        }
    }
}
