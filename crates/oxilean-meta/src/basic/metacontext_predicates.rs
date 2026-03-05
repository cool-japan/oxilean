//! # MetaContext - predicates Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MVarId;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Check if a metavariable is assigned.
    pub fn is_mvar_assigned(&self, id: MVarId) -> bool {
        self.mvar_assignments.contains_key(&id)
    }
}
