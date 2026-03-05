//! # MetaContext - local_decls_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LocalDecl;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get all local declarations.
    pub fn local_decls(&self) -> &[LocalDecl] {
        &self.local_decls
    }
}
