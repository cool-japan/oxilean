//! # MetaVarContext - all_solved_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Check if all are solved.
    pub fn all_solved(&self) -> bool {
        self.metas.values().all(|m| m.is_solved())
    }
}
