//! # ElabContext - fresh_meta_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Create a fresh metavariable.
    pub fn fresh_meta(&mut self, _ty: Expr) -> (u64, Expr) {
        let id = self.next_meta;
        self.next_meta += 1;
        let meta_expr = Expr::FVar(FVarId(1_000_000 + id));
        (id, meta_expr)
    }
}
