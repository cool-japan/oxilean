//! # MetaContext - reassign_mvar_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::types::MVarId;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Force-assign a metavariable (overriding previous assignment).
    pub fn reassign_mvar(&mut self, id: MVarId, val: Expr) {
        self.mvar_assignments.insert(id, val);
    }
}
