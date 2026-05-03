//! # ElabErrorCode - Trait Implementations
//!
//! This module contains trait implementations for `ElabErrorCode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabErrorCode;
use std::fmt;

impl std::fmt::Display for ElabErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ElabErrorCode::UnknownName => "unknown name",
            ElabErrorCode::TypeMismatch => "type mismatch",
            ElabErrorCode::UnsolvedMvar => "unsolved metavariable",
            ElabErrorCode::AmbiguousInstance => "ambiguous instance",
            ElabErrorCode::NoInstance => "no instance found",
            ElabErrorCode::UnificationFailed => "unification failed",
            ElabErrorCode::IllTyped => "ill-typed expression",
            ElabErrorCode::TacticFailed => "tactic failed",
            ElabErrorCode::NonExhaustiveMatch => "non-exhaustive match",
            ElabErrorCode::SyntaxError => "syntax error",
            ElabErrorCode::KernelRejected => "kernel rejected term",
            ElabErrorCode::SorryNotAllowed => "sorry not allowed",
            ElabErrorCode::RecursionLimit => "recursion limit exceeded",
            ElabErrorCode::MutualCycle => "mutual recursion cycle",
            ElabErrorCode::Other => "elaboration error",
        };
        write!(f, "{}", s)
    }
}
