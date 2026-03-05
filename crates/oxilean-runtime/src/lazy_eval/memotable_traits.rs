//! # MemoTable - Trait Implementations
//!
//! This module contains trait implementations for `MemoTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MemoTable;
use std::fmt;

impl Default for MemoTable {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for MemoTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MemoTable({} entries)", self.entries.len())
    }
}
