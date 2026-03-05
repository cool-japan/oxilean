//! # MetaVarContext - clear_constraints_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Clear constraints.
    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }
}
