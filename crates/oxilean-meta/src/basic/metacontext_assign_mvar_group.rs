//! # MetaContext - assign_mvar_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::types::MVarId;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Assign a metavariable.
    ///
    /// Returns false if the mvar is already assigned (use `reassign` to override).
    pub fn assign_mvar(&mut self, id: MVarId, val: Expr) -> bool {
        if self.mvar_assignments.contains_key(&id) {
            return false;
        }
        self.mvar_assignments.insert(id, val);
        true
    }
}
