//! # CodegenConfig - Trait Implementations
//!
//! This module contains trait implementations for `CodegenConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{CodegenConfig, CodegenTarget};

impl Default for CodegenConfig {
    fn default() -> Self {
        CodegenConfig {
            target: CodegenTarget::Rust,
            optimize: true,
            debug_info: false,
            emit_comments: true,
            inline_threshold: 50,
        }
    }
}
