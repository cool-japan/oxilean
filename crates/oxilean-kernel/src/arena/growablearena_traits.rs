//! # GrowableArena - Trait Implementations
//!
//! This module contains trait implementations for `GrowableArena`.
//!
//! ## Implemented Traits
//!
//! - `ArenaAllocator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ArenaAllocator;
use super::types::GrowableArena;

impl ArenaAllocator for GrowableArena {
    fn alloc_raw(&mut self, bytes: usize, align: usize) -> Option<usize> {
        Some(self.alloc(bytes, align))
    }
    fn used_bytes(&self) -> usize {
        self.used()
    }
}
