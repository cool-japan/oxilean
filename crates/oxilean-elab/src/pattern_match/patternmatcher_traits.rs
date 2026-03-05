//! # PatternMatcher - Trait Implementations
//!
//! This module contains trait implementations for `PatternMatcher`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PatternMatcher;
use std::fmt;

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}
