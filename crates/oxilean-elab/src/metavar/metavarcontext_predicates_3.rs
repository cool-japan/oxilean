//! # MetaVarContext - predicates Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Check whether a FVar ID is in the current scope.
    pub fn is_in_scope(&self, fvar_id: u64) -> bool {
        if fvar_id >= 1_000_000 {
            return true;
        }
        self.current_scope.contains(&fvar_id)
    }
}
