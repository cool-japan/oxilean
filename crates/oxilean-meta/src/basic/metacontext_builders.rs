//! # MetaContext - builders Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Expr, FVarId, Name};

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Execute a closure with a temporary local declaration.
    /// The declaration is removed after the closure returns.
    pub fn with_local_decl<F, R>(
        &mut self,
        user_name: Name,
        ty: Expr,
        binder_info: BinderInfo,
        f: F,
    ) -> R
    where
        F: FnOnce(&mut Self, FVarId) -> R,
    {
        let fvar_id = self.mk_local_decl(user_name, ty, binder_info);
        let result = f(self, fvar_id);
        self.fvar_map.remove(&fvar_id);
        self.local_decls.pop();
        result
    }
}
