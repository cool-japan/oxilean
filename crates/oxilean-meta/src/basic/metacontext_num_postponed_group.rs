//! # MetaContext - num_postponed_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get the number of postponed constraints.
    pub fn num_postponed(&self) -> usize {
        self.postponed.len()
    }
}
