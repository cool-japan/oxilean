//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;
use std::collections::{HashMap, HashSet};

use super::types::{MetaCheckpoint, MetaConstraint, MetaVar, MetaVarKind};

/// Context managing metavariables during elaboration.
pub struct MetaVarContext {
    pub(super) metas: HashMap<u64, MetaVar>,
    pub(super) next_id: u64,
    pub(super) constraints: Vec<MetaConstraint>,
    pub(super) depth: u32,
    pub(super) frozen: HashSet<u64>,
    /// Current local variable FVar IDs in scope (maintained by push/pop scope).
    pub(super) current_scope: Vec<u64>,
}
impl MetaVarContext {
    /// Create a checkpoint.
    pub fn checkpoint(&self) -> MetaCheckpoint {
        MetaCheckpoint {
            meta_count: self.next_id,
            depth: self.depth,
        }
    }
    /// Roll back to a checkpoint: discard all metas created after it.
    pub fn rollback(&mut self, cp: MetaCheckpoint) {
        self.metas.retain(|id, _| *id < cp.meta_count);
        self.depth = cp.depth;
    }
    /// Return the number of metas assigned at or below the given depth.
    pub fn assigned_at_depth(&self, depth: u32) -> usize {
        self.metas
            .values()
            .filter(|m| m.depth <= depth && m.is_assigned())
            .count()
    }
    /// Collect all metas of a specific kind.
    pub fn metas_of_kind(&self, kind: &MetaVarKind) -> Vec<u64> {
        self.metas
            .iter()
            .filter(|(_, m)| &m.kind == kind)
            .map(|(id, _)| *id)
            .collect()
    }
    /// Check whether any frozen meta is unassigned.
    pub fn has_frozen_unassigned(&self) -> bool {
        self.frozen.iter().any(|id| !self.is_solved(*id))
    }
    /// Return a map of meta ID to its assigned value (for all assigned metas).
    pub fn assignment_map(&self) -> HashMap<u64, Expr> {
        self.metas
            .iter()
            .filter_map(|(id, m)| m.assignment.as_ref().map(|v| (*id, v.clone())))
            .collect()
    }
    /// Print a compact summary.
    pub fn debug_summary(&self) -> String {
        let total = self.metas.len();
        let assigned = self.metas.values().filter(|m| m.is_assigned()).count();
        format!(
            "MetaVarContext: {}/{} assigned, depth={}",
            assigned, total, self.depth
        )
    }
}
