//! # ReuseAllocRecord - Trait Implementations
//!
//! This module contains trait implementations for `ReuseAllocRecord`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::ReuseAllocRecord;
use std::fmt;

impl std::fmt::Display for ReuseAllocRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reused = self
            .reused_from
            .map(|v| format!(" (reuse #{})", v))
            .unwrap_or_default();
        write!(f, "var#{} {} {}B{}", self.var, self.kind, self.size, reused)
    }
}
