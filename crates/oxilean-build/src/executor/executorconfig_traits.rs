//! # ExecutorConfig - Trait Implementations
//!
//! This module contains trait implementations for `ExecutorConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::{BuildProfile, OptLevel, Version};
use std::path::{Path, PathBuf};

use super::types::ExecutorConfig;

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            parallelism: 1,
            fail_fast: true,
            profile: BuildProfile::debug(),
            output_dir: PathBuf::from("build"),
            show_progress: true,
            verbose: false,
            step_timeout: None,
            package_version: Version::new(0, 1, 0),
        }
    }
}
