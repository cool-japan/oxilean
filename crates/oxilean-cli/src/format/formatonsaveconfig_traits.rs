//! # FormatOnSaveConfig - Trait Implementations
//!
//! This module contains trait implementations for `FormatOnSaveConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FormatOnSaveConfig;
use std::fmt;

impl Default for FormatOnSaveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            patterns: vec!["*.lean".to_string()],
            keep_backups: true,
        }
    }
}
