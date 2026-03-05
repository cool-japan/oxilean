//! # ElabContext - predicates Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::elabcontext_type::ElabContext;

impl<'env> ElabContext<'env> {
    /// Check whether a metavariable is assigned.
    pub fn is_meta_assigned(&self, id: u64) -> bool {
        self.metas.contains_key(&id)
    }
}
