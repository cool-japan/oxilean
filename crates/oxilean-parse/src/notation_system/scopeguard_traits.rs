//! # ScopeGuard - Trait Implementations
//!
//! This module contains trait implementations for `ScopeGuard`.
//!
//! ## Implemented Traits
//!
//! - `Drop`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScopeGuard;

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        self.env.pop_scope();
    }
}
