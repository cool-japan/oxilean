//! # MetaContext - num_locals_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Number of local declarations.
    pub fn num_locals(&self) -> usize {
        self.local_decls.len()
    }
}
