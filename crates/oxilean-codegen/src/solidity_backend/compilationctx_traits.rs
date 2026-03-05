//! # CompilationCtx - Trait Implementations
//!
//! This module contains trait implementations for `CompilationCtx`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CompilationCtx;

impl Default for CompilationCtx {
    fn default() -> Self {
        Self {
            pragmas: vec!["^0.8.20".into()],
            imports: Vec::new(),
            include_runtime: false,
        }
    }
}
