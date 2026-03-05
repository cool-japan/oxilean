//! # MetaVarContext - pop_scope_var_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Pop the most recently introduced scope variable.
    ///
    /// This should be called when leaving the binder that introduced the variable.
    pub fn pop_scope_var(&mut self) {
        self.current_scope.pop();
    }
}
