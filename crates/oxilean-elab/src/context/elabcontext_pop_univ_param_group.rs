//! # ElabContext - pop_univ_param_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Remove the most recently added universe parameter.
    pub fn pop_univ_param(&mut self) -> Option<Name> {
        self.univ_params.pop()
    }
}
