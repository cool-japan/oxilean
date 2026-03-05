//! # ElabContext - predicates Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;

impl<'env> ElabContext<'env> {
    /// Check if a name is a universe parameter.
    pub fn is_univ_param(&self, name: &Name) -> bool {
        self.univ_params.contains(name)
    }
}
