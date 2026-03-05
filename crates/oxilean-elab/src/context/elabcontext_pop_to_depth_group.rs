//! # ElabContext - pop_to_depth_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Pop all locals introduced at or above the given depth.
    pub fn pop_to_depth(&mut self, target_depth: u32) {
        self.locals.retain(|e| e.depth < target_depth);
    }
}
