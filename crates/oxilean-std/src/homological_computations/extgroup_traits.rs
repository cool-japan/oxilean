//! # ExtGroup - Trait Implementations
//!
//! This module contains trait implementations for `ExtGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExtGroup;
use std::fmt;

impl std::fmt::Display for ExtGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ext^{}(rank={})", self.degree, self.rank)
    }
}
