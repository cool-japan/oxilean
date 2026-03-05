//! # MetaVarContext - unsolved_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Get all unsolved metavariables.
    pub fn unsolved(&self) -> Vec<u64> {
        self.metas
            .values()
            .filter(|m| !m.is_solved())
            .map(|m| m.id)
            .collect()
    }
}
