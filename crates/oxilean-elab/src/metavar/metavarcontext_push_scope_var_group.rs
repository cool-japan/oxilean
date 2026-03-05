//! # MetaVarContext - push_scope_var_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Push a local FVar ID into the current scope.
    ///
    /// This should be called whenever a new local variable is introduced
    /// (e.g., when entering a lambda or pi binder during elaboration).
    pub fn push_scope_var(&mut self, fvar_id: u64) {
        self.current_scope.push(fvar_id);
    }
}
