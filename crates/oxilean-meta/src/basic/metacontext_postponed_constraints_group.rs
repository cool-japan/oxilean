//! # MetaContext - postponed_constraints_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::PostponedConstraint;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get all postponed constraints.
    pub fn postponed_constraints(&self) -> &[PostponedConstraint] {
        &self.postponed
    }
}
