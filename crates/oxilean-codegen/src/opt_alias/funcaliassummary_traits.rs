//! # FuncAliasSummary - Trait Implementations
//!
//! This module contains trait implementations for `FuncAliasSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FuncAliasSummary;
use std::fmt;

impl std::fmt::Display for FuncAliasSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FuncAlias[{}] {{ effect={}, modifies={}, reads={} }}",
            self.func_name,
            self.mem_effect,
            self.modifies.len(),
            self.reads.len(),
        )
    }
}
