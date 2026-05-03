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

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            max_diagnostics: 100,
            enable_completions: true,
            enable_hover: true,
            enable_goto_definition: true,
        }
    }
}
