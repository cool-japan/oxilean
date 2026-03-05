//! # MetaVarContext - predicates Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Check if a metavariable is assigned (alias for is_solved).
    pub fn is_assigned(&self, id: u64) -> bool {
        self.is_solved(id)
    }
}
