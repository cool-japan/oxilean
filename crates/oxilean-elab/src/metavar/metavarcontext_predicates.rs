//! # MetaVarContext - predicates Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::types::{MetaSnapshot, MetaVar};

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Get a metavariable by ID.
    pub fn get(&self, id: u64) -> Option<&MetaVar> {
        self.metas.get(&id)
    }
    /// Get the type of a metavariable.
    pub fn get_type(&self, id: u64) -> Option<&Expr> {
        self.metas.get(&id).map(|m| &m.ty)
    }
    /// Get the assignment.
    pub fn get_assignment(&self, id: u64) -> Option<&Expr> {
        self.metas.get(&id).and_then(|m| m.assignment.as_ref())
    }
    /// Assign a metavariable with full validation (occurs check + scope check).
    ///
    /// Returns `Err` with a descriptive message if:
    /// - the metavariable is frozen,
    /// - the metavariable does not exist,
    /// - the occurs check fails (circular assignment), or
    /// - the value references FVar IDs outside the metavariable's creation scope.
    pub fn assign_checked(&mut self, id: u64, expr: Expr) -> Result<(), String> {
        if self.frozen.contains(&id) {
            return Err(format!("metavariable ?{} is frozen", id));
        }
        if !self.metas.contains_key(&id) {
            return Err(format!("metavariable ?{} does not exist", id));
        }
        if occurs_check(id, &expr, self) {
            return Err(format!(
                "occurs check failed: ?{} appears in its own assignment",
                id
            ));
        }
        if let Some(meta) = self.metas.get(&id) {
            let scope = meta.scope_vars.clone();
            if !scope_check_expr(&expr, &scope) {
                return Err(format!(
                    "scope check failed: assignment to ?{} references out-of-scope variables",
                    id
                ));
            }
        }
        if let Some(meta) = self.metas.get_mut(&id) {
            meta.assign(expr);
        }
        Ok(())
    }
    /// Check if a metavariable is solved.
    pub fn is_solved(&self, id: u64) -> bool {
        self.metas.get(&id).is_some_and(|m| m.is_solved())
    }
    /// Restore to a snapshot.
    pub fn restore(&mut self, snapshot: &MetaSnapshot) {
        self.metas
            .retain(|id, _| (*id as usize) < snapshot.meta_count);
        for (id, meta) in &mut self.metas {
            meta.assignment = snapshot.assignments.get(id).cloned();
        }
        self.next_id = snapshot.next_id;
    }
}
