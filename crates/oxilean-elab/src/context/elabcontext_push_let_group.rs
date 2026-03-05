//! # ElabContext - push_let_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::types::LocalEntry;

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Push a let-binding.
    pub fn push_let(&mut self, name: Name, ty: Expr, val: Expr) -> FVarId {
        let fvar = FVarId(self.next_fvar);
        self.next_fvar += 1;
        self.locals
            .push(LocalEntry::let_binding(fvar, name, ty, val, self.depth));
        fvar
    }
}
