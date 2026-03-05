//! # BetaReductionPass - Trait Implementations
//!
//! This module contains trait implementations for `BetaReductionPass`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Debug`
//! - `OptPass`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};

use super::functions::OptPass;
use super::types::BetaReductionPass;
use std::fmt;

impl Default for BetaReductionPass {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BetaReductionPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BetaReductionPass(reductions={})", self.reductions)
    }
}

impl OptPass for BetaReductionPass {
    fn name(&self) -> &str {
        "beta_reduction"
    }
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> usize {
        let before = self.reductions;
        self.run(decls);
        (self.reductions - before) as usize
    }
}
