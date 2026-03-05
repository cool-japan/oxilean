//! # EvmCodeStats - Trait Implementations
//!
//! This module contains trait implementations for `EvmCodeStats`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmCodeStats;
use std::fmt;

impl std::fmt::Display for EvmCodeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EvmCodeStats {{ fns={}, events={}, storage={}, bytecode={}B }}",
            self.functions, self.events, self.storage_vars, self.bytecode_size,
        )
    }
}
