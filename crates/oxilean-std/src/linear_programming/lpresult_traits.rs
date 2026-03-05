//! # LpResult - Trait Implementations
//!
//! This module contains trait implementations for `LpResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LpResult;
use std::fmt;

impl std::fmt::Display for LpResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LpResult::Optimal {
                objective,
                solution,
            } => {
                write!(f, "Optimal(obj={}, x={:?})", objective, solution)
            }
            LpResult::Infeasible => write!(f, "Infeasible"),
            LpResult::Unbounded => write!(f, "Unbounded"),
        }
    }
}
