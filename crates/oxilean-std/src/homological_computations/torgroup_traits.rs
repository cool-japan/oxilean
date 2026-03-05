//! # TorGroup - Trait Implementations
//!
//! This module contains trait implementations for `TorGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TorGroup;
use std::fmt;

impl std::fmt::Display for TorGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tor_{}(rank={})", self.degree, self.rank)
    }
}
