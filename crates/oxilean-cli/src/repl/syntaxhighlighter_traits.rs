//! # SyntaxHighlighter - Trait Implementations
//!
//! This module contains trait implementations for `SyntaxHighlighter`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SyntaxHighlighter;
use std::fmt;

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new(true)
    }
}
