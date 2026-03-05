//! # LinearArena - Trait Implementations
//!
//! This module contains trait implementations for `LinearArena`.
//!
//! ## Implemented Traits
//!
//! - `ArenaAllocator`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::ArenaAllocator;
use super::types::LinearArena;

impl ArenaAllocator for LinearArena {
    fn alloc_raw(&mut self, bytes: usize, align: usize) -> Option<usize> {
        self.alloc(bytes, align)
    }
    fn used_bytes(&self) -> usize {
        self.used()
    }
}
