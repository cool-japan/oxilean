//! # LspConfig - Trait Implementations
//!
//! This module contains trait implementations for `LspConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LspConfig;
use std::fmt;

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            port: None,
            incremental_sync: true,
            max_completions: 50,
            hover: true,
            definition: true,
            workspace_symbols: true,
        }
    }
}
