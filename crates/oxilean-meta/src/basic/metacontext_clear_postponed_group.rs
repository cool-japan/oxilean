//! # MetaContext - clear_postponed_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Clear all postponed constraints.
    pub fn clear_postponed(&mut self) {
        self.postponed.clear();
    }
}
