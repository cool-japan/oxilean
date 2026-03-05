//! # CopyPropagationPass - Trait Implementations
//!
//! This module contains trait implementations for `CopyPropagationPass`.
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
use super::types::CopyPropagationPass;
use std::fmt;

impl Default for CopyPropagationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CopyPropagationPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CopyPropagationPass(subs={})", self.substitutions)
    }
}

impl OptPass for CopyPropagationPass {
    fn name(&self) -> &str {
        "copy_propagation"
    }
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> usize {
        let before = self.substitutions;
        self.run(decls);
        (self.substitutions - before) as usize
    }
}
