//! # ElabContext - pop_local_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LocalEntry;

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Pop the most recently pushed local and decrement depth.
    pub fn pop_local(&mut self) -> Option<LocalEntry> {
        let entry = self.locals.pop();
        if entry.is_some() && self.depth > 0 {
            self.depth -= 1;
        }
        entry
    }
}
