//! # FutharkExtConfig - Trait Implementations
//!
//! This module contains trait implementations for `FutharkExtConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{FutharkExtConfig, FutharkVersion};

impl Default for FutharkExtConfig {
    fn default() -> Self {
        Self {
            target_version: FutharkVersion::Latest,
            emit_safety_checks: true,
            inline_threshold: 20,
            vectorize_threshold: 64,
            emit_comments: true,
            mangle_names: false,
            backend_target: "opencl".to_string(),
        }
    }
}
