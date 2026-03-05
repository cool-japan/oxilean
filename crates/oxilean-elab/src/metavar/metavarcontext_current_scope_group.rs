//! # MetaVarContext - current_scope_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Get the current scope (FVar IDs of all in-scope local variables).
    pub fn current_scope(&self) -> &[u64] {
        &self.current_scope
    }
}
