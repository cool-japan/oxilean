//! # FutharkMemBlock - Trait Implementations
//!
//! This module contains trait implementations for `FutharkMemBlock`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkMemBlock;
use std::fmt;

impl std::fmt::Display for FutharkMemBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FutharkMem#{}({} bytes @ {}, pinned={})",
            self.block_id, self.size_bytes, self.device, self.is_pinned
        )
    }
}
