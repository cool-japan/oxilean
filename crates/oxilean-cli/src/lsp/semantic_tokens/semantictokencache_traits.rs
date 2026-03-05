//! # SemanticTokenCache - Trait Implementations
//!
//! This module contains trait implementations for `SemanticTokenCache`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SemanticTokenCache;
use std::fmt;

impl Default for SemanticTokenCache {
    fn default() -> Self {
        Self::new()
    }
}
