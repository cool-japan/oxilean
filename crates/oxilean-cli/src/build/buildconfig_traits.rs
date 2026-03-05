//! # BuildConfig - Trait Implementations
//!
//! This module contains trait implementations for `BuildConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::path::{Path, PathBuf};

use super::types::{BuildConfig, BuildTarget, OptLevel};

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            project_root: PathBuf::from("."),
            output_dir: PathBuf::from("./build"),
            parallelism: 4,
            verbose: false,
            force_rebuild: false,
            cache_dir: PathBuf::from("./build/cache"),
            target: BuildTarget::Check,
            opt_level: OptLevel::Debug,
        }
    }
}
