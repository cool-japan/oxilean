//! # ScopedArena - Trait Implementations
//!
//! This module contains trait implementations for `ScopedArena`.
//!
//! ## Implemented Traits
//!
//! - `Drop`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScopedArena;

impl<'pool> Drop for ScopedArena<'pool> {
    fn drop(&mut self) {
        let arena = self.arena.take_for_release();
        self.pool.release(arena);
    }
}
