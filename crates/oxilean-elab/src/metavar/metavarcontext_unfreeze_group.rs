//! # MetaVarContext - unfreeze_group Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Unfreeze a metavariable.
    pub fn unfreeze(&mut self, id: u64) {
        self.frozen.remove(&id);
    }
}
