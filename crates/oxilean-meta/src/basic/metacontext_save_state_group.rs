//! # MetaContext - save_state_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaState;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Save the current state for backtracking.
    pub fn save_state(&self) -> MetaState {
        MetaState {
            num_mvars: self.next_mvar_id,
            num_locals: self.local_decls.len() as u32,
            mvar_assignments: self.mvar_assignments.clone(),
            level_assignments: self.level_assignments.clone(),
            num_postponed: self.postponed.len(),
        }
    }
}
