//! # MetaContext - accessors Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::types::MVarId;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Get the type of a metavariable.
    pub fn get_mvar_type(&self, id: MVarId) -> Option<&Expr> {
        self.mvar_decls.get(&id).map(|d| &d.ty)
    }
}
