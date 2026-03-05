//! # ChainGroup - Trait Implementations
//!
//! This module contains trait implementations for `ChainGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChainGroup;
use std::fmt;

impl std::fmt::Display for ChainGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(rank={})", self.name, self.rank)
    }
}
