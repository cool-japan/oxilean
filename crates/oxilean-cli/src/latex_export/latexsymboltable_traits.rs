//! # LatexSymbolTable - Trait Implementations
//!
//! This module contains trait implementations for `LatexSymbolTable`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LatexSymbolTable;
use std::fmt;

impl Default for LatexSymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
