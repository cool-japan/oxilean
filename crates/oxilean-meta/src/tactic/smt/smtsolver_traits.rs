//! # SmtSolver - Trait Implementations
//!
//! This module contains trait implementations for `SmtSolver`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SmtSolver;

impl std::fmt::Display for SmtSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmtSolver::Z3 => write!(f, "z3"),
            SmtSolver::Cvc5 => write!(f, "cvc5"),
            SmtSolver::Yices2 => write!(f, "yices2"),
            SmtSolver::Bitwuzla => write!(f, "bitwuzla"),
        }
    }
}
