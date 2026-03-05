//! # FunctionTable - Trait Implementations
//!
//! This module contains trait implementations for `FunctionTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FunctionTable;
use std::fmt;

impl Default for FunctionTable {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for FunctionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionTable")
            .field("len", &self.entries.len())
            .finish()
    }
}
