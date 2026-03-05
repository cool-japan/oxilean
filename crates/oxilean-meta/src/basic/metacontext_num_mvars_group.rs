//! # MetaContext - num_mvars_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get the number of metavariables.
    pub fn num_mvars(&self) -> usize {
        self.mvar_decls.len()
    }
}
