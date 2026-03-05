//! # MVarId - Trait Implementations
//!
//! This module contains trait implementations for `MVarId`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MVarId;

impl std::fmt::Display for MVarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?m_{}", self.0)
    }
}
