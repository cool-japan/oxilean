//! # MetaVarContext - count_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Count total metavariables.
    pub fn count(&self) -> usize {
        self.metas.len()
    }
    /// Count unsolved.
    pub fn unsolved_count(&self) -> usize {
        self.metas.values().filter(|m| !m.is_solved()).count()
    }
    /// Count solved.
    pub fn solved_count(&self) -> usize {
        self.metas.values().filter(|m| m.is_solved()).count()
    }
}
