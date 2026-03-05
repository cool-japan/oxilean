//! # LlvmBackend - Trait Implementations
//!
//! This module contains trait implementations for `LlvmBackend`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LlvmBackend;

impl Default for LlvmBackend {
    fn default() -> Self {
        Self::new()
    }
}
