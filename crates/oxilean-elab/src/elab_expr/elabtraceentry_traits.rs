//! # ElabTraceEntry - Trait Implementations
//!
//! This module contains trait implementations for `ElabTraceEntry`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabTraceEntry;
use std::fmt;

impl std::fmt::Display for ElabTraceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} ({})", self.label, self.expr_dbg, self.mode)
    }
}
