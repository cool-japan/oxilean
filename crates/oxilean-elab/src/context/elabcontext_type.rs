//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};
use std::collections::HashMap;

use super::functions::type_contains_const;
use super::types::{ElabOptions, ElabSnapshot, LocalEntry, LocalView};

/// Elaboration context that tracks local variables and metavariables.
pub struct ElabContext<'env> {
    pub(super) env: &'env Environment,
    pub(super) locals: Vec<LocalEntry>,
    pub(super) metas: HashMap<u64, Expr>,
    pub(super) next_meta: u64,
    pub(super) next_fvar: u64,
    pub(super) univ_params: Vec<Name>,
    pub(super) options: ElabOptions,
    pub(super) depth: u32,
    pub(super) pending_goals: Vec<Expr>,
    pub(super) expected_type_stack: Vec<Option<Expr>>,
}
impl<'env> ElabContext<'env> {
    /// Take a snapshot of the current state.
    pub fn snapshot(&self) -> ElabSnapshot {
        ElabSnapshot {
            local_count: self.locals.len(),
            meta_count: self.next_meta,
            depth: self.depth,
            goal_count: self.pending_goals.len(),
        }
    }
    /// Restore context to a snapshot (removing locals/goals added after).
    pub fn restore(&mut self, snap: ElabSnapshot) {
        self.locals.truncate(snap.local_count);
        self.depth = snap.depth;
        self.pending_goals.truncate(snap.goal_count);
    }
    /// Pop the top-level goal and return it.
    pub fn pop_goal(&mut self) -> Option<oxilean_kernel::Expr> {
        self.pending_goals.pop()
    }
    /// Replace the goals list with the given goals.
    pub fn set_goals(&mut self, goals: Vec<oxilean_kernel::Expr>) {
        self.pending_goals = goals;
    }
    /// Remove the first goal and return it.
    pub fn take_first_goal(&mut self) -> Option<oxilean_kernel::Expr> {
        if self.pending_goals.is_empty() {
            None
        } else {
            Some(self.pending_goals.remove(0))
        }
    }
    /// Return the current first goal without removing it.
    pub fn current_goal(&self) -> Option<&oxilean_kernel::Expr> {
        self.pending_goals.first()
    }
    /// Count hypothesis-style locals (non-let).
    pub fn hypothesis_count(&self) -> usize {
        self.locals.iter().filter(|e| e.is_hypothesis()).count()
    }
    /// Count let-bound locals.
    pub fn let_count(&self) -> usize {
        self.locals.iter().filter(|e| e.is_let()).count()
    }
    /// Check whether an FVarId is in scope.
    pub fn is_in_scope(&self, fvar: oxilean_kernel::FVarId) -> bool {
        self.locals.iter().any(|e| e.fvar == fvar)
    }
    /// Remove the local with the given FVarId (if present).
    pub fn remove_fvar(&mut self, fvar: oxilean_kernel::FVarId) {
        self.locals.retain(|e| e.fvar != fvar);
    }
    /// Get an immutable reference to the most recently pushed local.
    pub fn top_local(&self) -> Option<&LocalEntry> {
        self.locals.last()
    }
    /// Get all let-bound locals.
    pub fn let_bindings(&self) -> Vec<(&Name, &oxilean_kernel::Expr, &oxilean_kernel::Expr)> {
        self.locals
            .iter()
            .filter_map(|e| e.val.as_ref().map(|v| (&e.name, &e.ty, v)))
            .collect()
    }
    /// Replace the type of a local variable (identified by FVarId).
    pub fn update_local_type(
        &mut self,
        fvar: oxilean_kernel::FVarId,
        new_ty: oxilean_kernel::Expr,
    ) -> bool {
        for entry in &mut self.locals {
            if entry.fvar == fvar {
                entry.ty = new_ty;
                return true;
            }
        }
        false
    }
    /// Clear all metavariable assignments.
    pub fn clear_metas(&mut self) {
        self.metas.clear();
    }
    /// Get all (id, value) pairs for assigned metas.
    pub fn meta_assignments(&self) -> Vec<(u64, &oxilean_kernel::Expr)> {
        self.metas.iter().map(|(id, v)| (*id, v)).collect()
    }
    /// Check whether the elaboration context has any goals or metas pending.
    pub fn is_clean(&self) -> bool {
        self.pending_goals.is_empty() && self.metas.is_empty()
    }
    /// Get the number of universe parameters registered.
    pub fn univ_param_count(&self) -> usize {
        self.univ_params.len()
    }
    /// Check if the expected type stack is empty.
    pub fn has_expected_type(&self) -> bool {
        self.expected_type().is_some()
    }
    /// Push a "no expected type" frame.
    pub fn push_no_expected_type(&mut self) {
        self.push_expected_type(None);
    }
    /// Absorb all goals from another context.
    pub fn merge_goals_from(&mut self, other: &mut ElabContext<'_>) {
        self.pending_goals.append(&mut other.pending_goals);
    }
}
impl<'env> ElabContext<'env> {
    /// Get a read-only view of the local context.
    pub fn local_view(&self) -> LocalView<'_> {
        LocalView::new(&self.locals)
    }
    /// Find all locals whose type contains a given constant name.
    pub fn locals_with_type_const(&self, name: &Name) -> Vec<&LocalEntry> {
        self.locals
            .iter()
            .filter(|e| type_contains_const(&e.ty, name))
            .collect()
    }
    /// Rename a local variable.
    pub fn rename_local(&mut self, fvar: oxilean_kernel::FVarId, new_name: Name) -> bool {
        for entry in &mut self.locals {
            if entry.fvar == fvar {
                entry.name = new_name;
                return true;
            }
        }
        false
    }
}
