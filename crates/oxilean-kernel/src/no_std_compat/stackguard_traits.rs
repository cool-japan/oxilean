//! # StackGuard - Trait Implementations
//!
//! This module contains trait implementations for `StackGuard`.
//!
//! ## Implemented Traits
//!
//! - `Drop`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::StackGuard;

impl Drop for StackGuard<'_> {
    fn drop(&mut self) {
        *self.depth -= 1;
    }
}
