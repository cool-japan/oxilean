//! # IdentityEliminationPass - Trait Implementations
//!
//! This module contains trait implementations for `IdentityEliminationPass`.
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
use super::types::IdentityEliminationPass;
use std::fmt;

impl Default for IdentityEliminationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for IdentityEliminationPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IdentityEliminationPass(elim={})", self.eliminated)
    }
}

impl OptPass for IdentityEliminationPass {
    fn name(&self) -> &str {
        "identity_elimination"
    }
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> usize {
        let before = self.eliminated;
        self.run(decls);
        (self.eliminated - before) as usize
    }
}
