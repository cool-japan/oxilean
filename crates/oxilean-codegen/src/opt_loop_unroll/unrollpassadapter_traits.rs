//! # UnrollPassAdapter - Trait Implementations
//!
//! This module contains trait implementations for `UnrollPassAdapter`.
//!
//! ## Implemented Traits
//!
//! - `LoopOptPass`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};

use super::functions::LoopOptPass;
use super::types::{UnrollPassAdapter, UnrollReport};

impl LoopOptPass for UnrollPassAdapter {
    fn name(&self) -> &str {
        "loop-unroll"
    }
    fn run_pass(&mut self, decls: &mut [LcnfFunDecl]) -> UnrollReport {
        self.inner.run(decls);
        self.inner.report.clone()
    }
}
