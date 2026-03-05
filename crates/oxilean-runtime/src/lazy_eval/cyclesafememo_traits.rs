//! # CycleSafeMemo - Trait Implementations
//!
//! This module contains trait implementations for `CycleSafeMemo`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CycleSafeMemo;
use std::fmt;

impl<T: Clone + Default + fmt::Debug> Default for CycleSafeMemo<T> {
    fn default() -> Self {
        Self::new()
    }
}
