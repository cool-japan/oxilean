//! # ElabContext - predicates Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::elabcontext_type::ElabContext;

impl<'env> ElabContext<'env> {
    /// Check whether there are pending goals.
    pub fn has_goals(&self) -> bool {
        !self.pending_goals.is_empty()
    }
}
