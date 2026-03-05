//! # MetaVarContext - snapshot_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaSnapshot;

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Take a snapshot.
    pub fn snapshot(&self) -> MetaSnapshot {
        let assignments = self
            .metas
            .iter()
            .filter_map(|(id, m)| m.assignment.as_ref().map(|a| (*id, a.clone())))
            .collect();
        MetaSnapshot {
            meta_count: self.metas.len(),
            assignments,
            next_id: self.next_id,
        }
    }
}
