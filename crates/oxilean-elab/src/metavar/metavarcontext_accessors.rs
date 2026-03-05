//! # MetaVarContext - accessors Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::functions::occurs_check;
use super::types::MetaVar;

use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Get a mutable reference.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut MetaVar> {
        self.metas.get_mut(&id)
    }
    /// Assign a metavariable (returns false if frozen, not found, or occurs check fails).
    ///
    /// Performs an occurs check to prevent circular assignments: if `id` appears
    /// free in `expr` (following existing assignments transitively), the assignment
    /// is rejected and `false` is returned.
    pub fn assign(&mut self, id: u64, expr: Expr) -> bool {
        if self.frozen.contains(&id) {
            return false;
        }
        if occurs_check(id, &expr, self) {
            return false;
        }
        if let Some(meta) = self.metas.get_mut(&id) {
            meta.assign(expr);
            true
        } else {
            false
        }
    }
    /// Force-assign ignoring freeze.
    pub fn force_assign(&mut self, id: u64, expr: Expr) -> bool {
        if let Some(meta) = self.metas.get_mut(&id) {
            meta.reassign(expr);
            true
        } else {
            false
        }
    }
}
