//! # RootSystem - Trait Implementations
//!
//! This module contains trait implementations for `RootSystem`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RootSystem;
use std::fmt;

impl std::fmt::Display for RootSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (rank {})", self.root_type, self.rank)
    }
}
