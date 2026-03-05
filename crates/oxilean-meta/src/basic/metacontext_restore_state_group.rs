//! # MetaContext - restore_state_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaState;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Restore to a saved state.
    pub fn restore_state(&mut self, state: MetaState) {
        self.mvar_assignments = state.mvar_assignments;
        self.level_assignments = state.level_assignments;
        self.postponed.truncate(state.num_postponed);
        while self.local_decls.len() > state.num_locals as usize {
            if let Some(decl) = self.local_decls.pop() {
                self.fvar_map.remove(&decl.fvar_id);
            }
        }
    }
}
