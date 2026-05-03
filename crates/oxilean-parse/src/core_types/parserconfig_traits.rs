//! # ParserConfig - Trait Implementations
//!
//! This module contains trait implementations for `ParserConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParserConfig;

impl Default for ParserConfig {
    fn default() -> Self {
        ParserConfig {
            max_depth: 512,
            unicode_ops: true,
            suggestions: true,
            error_recovery: false,
            allow_commands: true,
        }
    }
}
