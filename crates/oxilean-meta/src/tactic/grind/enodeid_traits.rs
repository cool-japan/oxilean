//! # ENodeId - Trait Implementations
//!
//! This module contains trait implementations for `ENodeId`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ENodeId;
use std::fmt;

impl fmt::Display for ENodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "n{}", self.0)
    }
}
