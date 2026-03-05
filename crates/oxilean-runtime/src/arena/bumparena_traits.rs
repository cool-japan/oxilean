//! # BumpArena - Trait Implementations
//!
//! This module contains trait implementations for `BumpArena`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BumpArena;
use std::fmt;

impl Default for BumpArena {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BumpArena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BumpArena")
            .field("num_chunks", &self.chunks.len())
            .field("bytes_used", &self.bytes_used())
            .field("total_capacity", &self.total_capacity())
            .finish()
    }
}
