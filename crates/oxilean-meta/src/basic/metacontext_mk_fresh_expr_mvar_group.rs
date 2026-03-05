//! # MetaContext - mk_fresh_expr_mvar_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Name};

use super::types::{MVarId, MetavarKind};

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Create a fresh expression metavariable with the given type.
    ///
    /// Returns the MVarId and a placeholder expression `?m`.
    pub fn mk_fresh_expr_mvar(&mut self, ty: Expr, kind: MetavarKind) -> (MVarId, Expr) {
        self.mk_fresh_expr_mvar_with_name(ty, kind, Name::Anonymous)
    }
}
