//! # MetaContext - accessors Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{MVarId, MetavarDecl};

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get the declaration of a metavariable.
    pub fn get_mvar_decl(&self, id: MVarId) -> Option<&MetavarDecl> {
        self.mvar_decls.get(&id)
    }
}
