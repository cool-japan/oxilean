//! # SemanticTokenBuilder - Trait Implementations
//!
//! This module contains trait implementations for `SemanticTokenBuilder`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SemanticTokenBuilder;
use std::fmt;

impl Default for SemanticTokenBuilder {
    fn default() -> Self {
        Self::new()
    }
}
