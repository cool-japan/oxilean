//! # MetaContext - queries Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, FVarId};

use super::types::LocalDecl;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Find a local declaration by FVarId.
    pub fn find_local_decl(&self, fvar_id: FVarId) -> Option<&LocalDecl> {
        self.fvar_map
            .get(&fvar_id)
            .and_then(|&idx| self.local_decls.get(idx))
    }
    /// Get the type of a free variable.
    pub fn get_fvar_type(&self, fvar_id: FVarId) -> Option<&Expr> {
        self.find_local_decl(fvar_id).map(|d| &d.ty)
    }
    /// Get the value of a let-binding free variable.
    pub fn get_fvar_value(&self, fvar_id: FVarId) -> Option<&Expr> {
        self.find_local_decl(fvar_id).and_then(|d| d.value.as_ref())
    }
}
