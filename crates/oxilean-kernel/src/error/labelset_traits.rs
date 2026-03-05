//! # LabelSet - Trait Implementations
//!
//! This module contains trait implementations for `LabelSet`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LabelSet;
use std::fmt;

impl Default for LabelSet {
    fn default() -> Self {
        Self::new()
    }
}
