//! # EvmPassSummary - Trait Implementations
//!
//! This module contains trait implementations for `EvmPassSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvmPassSummary;
use std::fmt;

impl std::fmt::Display for EvmPassSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EvmPassSummary[{}] {{ fns={}, bytecodes={}, opts={}, {}us }}",
            self.pass_name,
            self.functions_compiled,
            self.bytecodes_generated,
            self.optimizations_applied,
            self.duration_us,
        )
    }
}
