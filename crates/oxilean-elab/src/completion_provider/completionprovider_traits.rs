//! # CompletionProvider - Trait Implementations
//!
//! This module contains trait implementations for `CompletionProvider`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CompletionProvider;
use std::fmt;

impl Default for CompletionProvider {
    fn default() -> Self {
        CompletionProvider::new()
    }
}
