//! # SimplificationPhase - Trait Implementations
//!
//! This module contains trait implementations for `SimplificationPhase`.
//!
//! ## Implemented Traits
//!
//! - `SolverPhase`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::infer::{Constraint, MetaVarId};
use oxilean_kernel::{Expr, Level, Literal, Name};
use std::collections::HashMap;
use std::fmt;

use super::functions::SolverPhase;
use super::types::{ConstraintSimplifier, SimplificationPhase};

#[allow(dead_code)]
impl SolverPhase for SimplificationPhase {
    fn name(&self) -> &'static str {
        "simplification"
    }
    fn run(
        &self,
        constraints: Vec<Constraint>,
        _assignments: &mut HashMap<MetaVarId, Expr>,
    ) -> (Vec<Constraint>, Vec<String>) {
        let simplified = ConstraintSimplifier::simplify(constraints);
        (simplified, vec![])
    }
}
