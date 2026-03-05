//! # ConstructorTable - Trait Implementations
//!
//! This module contains trait implementations for `ConstructorTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ConstructorTable;

impl Default for ConstructorTable {
    fn default() -> Self {
        Self::builtin()
    }
}
