//! # HoverCache - Trait Implementations
//!
//! This module contains trait implementations for `HoverCache`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HoverCache;
use std::fmt;

impl Default for HoverCache {
    fn default() -> Self {
        Self::new(1000)
    }
}
