//! # TokenColorizer - Trait Implementations
//!
//! This module contains trait implementations for `TokenColorizer`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TokenColorizer;
use std::fmt;

impl Default for TokenColorizer {
    fn default() -> Self {
        Self::new()
    }
}
