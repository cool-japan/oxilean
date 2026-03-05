//! # MemFlags - Trait Implementations
//!
//! This module contains trait implementations for `MemFlags`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemFlags;
use std::fmt;

impl Default for MemFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MemFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = vec![];
        if self.aligned {
            parts.push("aligned");
        }
        if self.notrap {
            parts.push("notrap");
        }
        if self.readonly {
            parts.push("readonly");
        }
        write!(f, "{}", parts.join(" "))
    }
}
