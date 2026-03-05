//! # MetaVarContext - freeze_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Freeze a metavariable.
    pub fn freeze(&mut self, id: u64) {
        self.frozen.insert(id);
    }
}
