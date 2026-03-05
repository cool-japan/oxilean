//! # ElabContext - push_univ_param_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Add a universe parameter.
    pub fn push_univ_param(&mut self, name: Name) {
        self.univ_params.push(name);
    }
}
