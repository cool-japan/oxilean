//! # MetaContext - instantiate_mvars_group Methods
//!
//! This module contains method implementations for `MetaContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Expr;

use super::metacontext_type::MetaContext;

impl MetaContext {
    /// Instantiate metavariables in an expression.
    ///
    /// Replaces all assigned mvar placeholders with their values,
    /// recursively.
    pub fn instantiate_mvars(&self, expr: &Expr) -> Expr {
        self.instantiate_mvars_impl(expr, 0)
    }
}
