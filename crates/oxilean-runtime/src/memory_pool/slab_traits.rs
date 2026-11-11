//! # Slab - Trait Implementations
//!
//! This module contains trait implementations for `Slab`.
//!
//! ## Implemented Traits
//!
//! - `Drop`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Slab;
use std::alloc;

impl Drop for Slab {
    fn drop(&mut self) {
        // SAFETY: `self.ptr` was returned by `alloc::alloc(self.layout)` in
        // `Slab::new`, the only place a `Slab` is built (its private
        // `capacity` field keeps struct literals inside `types.rs`, where
        // `Slab::new` is the only one), and neither `ptr` nor `layout` is
        // ever reassigned. `Slab` is neither `Clone` nor `Copy`, so each
        // allocation belongs to exactly one `Slab` and is freed once, here,
        // with the layout it was allocated with.
        unsafe {
            alloc::dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}
