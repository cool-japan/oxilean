//! # MetaVarContext - predicates Methods
//!
//! This module contains method implementations for `MetaVarContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metavarcontext_type::MetaVarContext;

impl MetaVarContext {
    /// Check if frozen.
    pub fn is_frozen(&self, id: u64) -> bool {
        self.frozen.contains(&id)
    }
}
