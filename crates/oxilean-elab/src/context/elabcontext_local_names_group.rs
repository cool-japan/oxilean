//! # ElabContext - local_names_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Get all local names.
    pub fn local_names(&self) -> Vec<&Name> {
        self.locals.iter().map(|e| &e.name).collect()
    }
}
