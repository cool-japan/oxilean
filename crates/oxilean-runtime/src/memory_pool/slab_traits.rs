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
        unsafe {
            alloc::dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}
