//! # MetaContext - assign_level_mvar_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Level;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Assign a level metavariable.
    pub fn assign_level_mvar(&mut self, id: u64, level: Level) {
        self.level_assignments.insert(id, level);
    }
}
