//! # LamportClock - Trait Implementations
//!
//! This module contains trait implementations for `LamportClock`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LamportClock;

impl Default for LamportClock {
    fn default() -> Self {
        Self::new()
    }
}
