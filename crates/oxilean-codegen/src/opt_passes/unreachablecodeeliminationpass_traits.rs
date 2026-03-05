//! # UnreachableCodeEliminationPass - Trait Implementations
//!
//! This module contains trait implementations for `UnreachableCodeEliminationPass`.
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
use super::types::UnreachableCodeEliminationPass;
use std::fmt;

impl Default for UnreachableCodeEliminationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for UnreachableCodeEliminationPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UnreachableCodeEliminationPass(elim={})",
            self.eliminated
        )
    }
}

impl OptPass for UnreachableCodeEliminationPass {
    fn name(&self) -> &str {
        "unreachable_code_elimination"
    }
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> usize {
        let before = self.eliminated;
        self.run(decls);
        (self.eliminated - before) as usize
    }
}
