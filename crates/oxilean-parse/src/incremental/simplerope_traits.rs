//! # SimpleRope - Trait Implementations
//!
//! This module contains trait implementations for `SimpleRope`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SimpleRope;

impl std::fmt::Display for SimpleRope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chunks.concat())
    }
}
