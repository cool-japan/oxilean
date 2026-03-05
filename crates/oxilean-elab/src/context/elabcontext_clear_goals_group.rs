//! # ElabContext - clear_goals_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Clear all pending goals.
    pub fn clear_goals(&mut self) {
        self.pending_goals.clear();
    }
}
