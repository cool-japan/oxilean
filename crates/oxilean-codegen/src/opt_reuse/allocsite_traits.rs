//! # AllocSite - Trait Implementations
//!
//! This module contains trait implementations for `AllocSite`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::AllocSite;
use std::fmt;

impl std::fmt::Display for AllocSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AllocSite#{}({:?} in {})",
            self.id, self.size_class, self.func
        )
    }
}
