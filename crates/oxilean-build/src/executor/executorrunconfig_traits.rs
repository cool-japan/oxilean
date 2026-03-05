//! # ExecutorRunConfig - Trait Implementations
//!
//! This module contains trait implementations for `ExecutorRunConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ExecutorRunConfig, SandboxConfig};

impl Default for ExecutorRunConfig {
    fn default() -> Self {
        Self {
            max_workers: 4,
            sandbox: SandboxConfig::default(),
            fail_fast: false,
            verbose: false,
        }
    }
}
