//! # CompletionRegistry - Trait Implementations
//!
//! This module contains trait implementations for `CompletionRegistry`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CompletionRegistry;
use std::fmt;

impl Default for CompletionRegistry {
    fn default() -> Self {
        Self::new()
    }
}
