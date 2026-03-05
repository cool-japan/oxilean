//! # VyperCompilationCtx - Trait Implementations
//!
//! This module contains trait implementations for `VyperCompilationCtx`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VyperCompilationCtx;

impl Default for VyperCompilationCtx {
    fn default() -> Self {
        Self {
            version: "0.3.10".into(),
            abi_v2: true,
            include_runtime: false,
            extra_pragmas: Vec::new(),
        }
    }
}
