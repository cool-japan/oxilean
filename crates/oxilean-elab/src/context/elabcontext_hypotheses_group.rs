//! # ElabContext - hypotheses_group Methods
//!
//! This module contains method implementations for `ElabContext`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Environment, Expr, FVarId, Name};

use super::elabcontext_type::ElabContext;
use super::functions::*;

impl<'env> ElabContext<'env> {
    /// Collect hypotheses as (name, type) pairs.
    pub fn hypotheses(&self) -> Vec<(&Name, &Expr)> {
        self.locals
            .iter()
            .filter(|e| e.is_hypothesis())
            .map(|e| (&e.name, &e.ty))
            .collect()
    }
}
