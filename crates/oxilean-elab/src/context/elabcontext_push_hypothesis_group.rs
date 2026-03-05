//! # ElabContext - push_hypothesis_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::types::LocalEntry;

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Push a hypothesis and increment depth.
    pub fn push_hypothesis(&mut self, name: Name, ty: Expr) -> FVarId {
        let fvar = FVarId(self.next_fvar);
        self.next_fvar += 1;
        self.locals
            .push(LocalEntry::hypothesis(fvar, name, ty, self.depth));
        self.depth += 1;
        fvar
    }
}
