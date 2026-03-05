//! # ConstantFoldingPass - Trait Implementations
//!
//! This module contains trait implementations for `ConstantFoldingPass`.
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
use super::types::ConstantFoldingPass;
use std::fmt;

impl Default for ConstantFoldingPass {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ConstantFoldingPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConstantFoldingPass(folds={})", self.folds_performed)
    }
}

impl OptPass for ConstantFoldingPass {
    fn name(&self) -> &str {
        "constant_folding"
    }
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> usize {
        let before = self.folds_performed;
        self.run(decls);
        (self.folds_performed - before) as usize
    }
}
