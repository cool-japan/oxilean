//! # CommandConfig - Trait Implementations
//!
//! This module contains trait implementations for `CommandConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::fmt;
use std::path::{Path, PathBuf};

use super::types::CommandConfig;

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            color: true,
            project_dir: PathBuf::from("."),
            max_errors: 10,
        }
    }
}
