//! # LspSession - Trait Implementations
//!
//! This module contains trait implementations for `LspSession`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LspSession;
use std::fmt;

impl Default for LspSession {
    fn default() -> Self {
        Self::new()
    }
}
