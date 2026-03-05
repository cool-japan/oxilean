//! # ElabContext - lookup_fvar_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::types::LocalEntry;

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Look up a local variable by free variable ID.
    pub fn lookup_fvar(&self, fvar: FVarId) -> Option<&LocalEntry> {
        self.locals.iter().find(|e| e.fvar == fvar)
    }
}
