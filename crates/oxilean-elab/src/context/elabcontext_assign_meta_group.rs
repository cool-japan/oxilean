//! # ElabContext - assign_meta_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Assign a metavariable.
    pub fn assign_meta(&mut self, id: u64, expr: Expr) {
        self.metas.insert(id, expr);
    }
}
