//! # MetaContext - accessors Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Level;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get the assignment of a level metavariable.
    pub fn get_level_assignment(&self, id: u64) -> Option<&Level> {
        self.level_assignments.get(&id)
    }
}
