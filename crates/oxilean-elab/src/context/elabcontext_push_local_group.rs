//! # ElabContext - push_local_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::types::{LocalEntry, LocalKind};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Push a local variable (returns its FVarId).
    pub fn push_local(&mut self, name: Name, ty: Expr, val: Option<Expr>) -> FVarId {
        let fvar = FVarId(self.next_fvar);
        self.next_fvar += 1;
        let kind = if val.is_some() {
            LocalKind::LetBinding
        } else {
            LocalKind::Hypothesis
        };
        self.locals.push(LocalEntry {
            fvar,
            name,
            ty,
            val,
            kind,
            depth: self.depth,
        });
        fvar
    }
}
