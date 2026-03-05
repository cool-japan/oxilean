//! # MetaContext - unassigned_mvars_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MVarId;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get all unassigned metavariables.
    pub fn unassigned_mvars(&self) -> Vec<MVarId> {
        self.mvar_decls
            .keys()
            .filter(|id| !self.mvar_assignments.contains_key(id))
            .copied()
            .collect()
    }
}
