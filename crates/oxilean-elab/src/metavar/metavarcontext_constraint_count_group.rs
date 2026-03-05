//! # MetaVarContext - constraint_count_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Count constraints.
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }
}
