//! # ElabContext - pop_expected_type_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Pop the expected type.
    pub fn pop_expected_type(&mut self) -> Option<Option<Expr>> {
        self.expected_type_stack.pop()
    }
}
