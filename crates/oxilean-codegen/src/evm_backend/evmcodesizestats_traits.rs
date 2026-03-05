//! # EvmCodeSizeStats - Trait Implementations
//!
//! This module contains trait implementations for `EvmCodeSizeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmCodeSizeStats;
use std::fmt;

impl std::fmt::Display for EvmCodeSizeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EvmCodeSizeStats {{ bytecode={}B, deploy={}B, ctor={}B, fns={} }}",
            self.bytecode_size,
            self.deploy_bytecode_size,
            self.constructor_size,
            self.function_sizes.len(),
        )
    }
}
