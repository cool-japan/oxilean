//! # LLVMExtPassConfig - Trait Implementations
//!
//! This module contains trait implementations for `LLVMExtPassConfig`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LLVMExtPassConfig;

impl Default for LLVMExtPassConfig {
    fn default() -> Self {
        Self::new("default")
    }
}
