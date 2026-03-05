//! # ElabContext - push_expected_type_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Push an expected type.
    pub fn push_expected_type(&mut self, ty: Option<Expr>) {
        self.expected_type_stack.push(ty);
    }
}
