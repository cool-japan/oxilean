//! # ElabContext - expected_type_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Get the current expected type.
    pub fn expected_type(&self) -> Option<&Expr> {
        self.expected_type_stack.last().and_then(|t| t.as_ref())
    }
}
