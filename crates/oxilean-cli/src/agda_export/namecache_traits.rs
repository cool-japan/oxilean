//! # NameCache - Trait Implementations
//!
//! This module contains trait implementations for `NameCache`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::NameCache;
use std::fmt;

impl Default for NameCache {
    fn default() -> Self {
        Self::new()
    }
}
