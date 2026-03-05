//! # CoqEvalCmd - Trait Implementations
//!
//! This module contains trait implementations for `CoqEvalCmd`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqEvalCmd;
use std::fmt;

impl std::fmt::Display for CoqEvalCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoqEvalCmd::Compute(e) => write!(f, "Compute {}.", e),
            CoqEvalCmd::Eval(strat, e) => write!(f, "Eval {} in {}.", strat, e),
            CoqEvalCmd::Check(e) => write!(f, "Check {}.", e),
            CoqEvalCmd::Print(n) => write!(f, "Print {}.", n),
            CoqEvalCmd::About(n) => write!(f, "About {}.", n),
            CoqEvalCmd::SearchPattern(p) => write!(f, "SearchPattern ({}).", p),
        }
    }
}
