//! # ElabContext - accessors Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;

impl<'env> ElabContext<'env> {
    /// Get a metavariable assignment.
    pub fn get_meta(&self, id: u64) -> Option<&Expr> {
        self.metas.get(&id)
    }
}
