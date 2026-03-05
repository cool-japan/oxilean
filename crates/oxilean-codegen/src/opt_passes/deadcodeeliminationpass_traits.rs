//! # DeadCodeEliminationPass - Trait Implementations
//!
//! This module contains trait implementations for `DeadCodeEliminationPass`.
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
use super::types::DeadCodeEliminationPass;
use std::fmt;

impl Default for DeadCodeEliminationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for DeadCodeEliminationPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeadCodeEliminationPass(removed={})", self.removed)
    }
}

impl OptPass for DeadCodeEliminationPass {
    fn name(&self) -> &str {
        "dead_code_elimination"
    }
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> usize {
        let before = self.removed;
        self.run(decls);
        (self.removed - before) as usize
    }
}
