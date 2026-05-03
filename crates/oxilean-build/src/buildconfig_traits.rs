//! # BuildConfig - Trait Implementations
//!
//! This module contains trait implementations for `BuildConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::path::PathBuf;

use super::functions::num_cpus;
use super::types::{BuildConfig, BuildProfileKind};

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            out_dir: PathBuf::from("build"),
            profile: BuildProfileKind::Debug,
            jobs: num_cpus(),
            verbose: false,
            warnings: true,
            extra_flags: Vec::new(),
        }
    }
}
