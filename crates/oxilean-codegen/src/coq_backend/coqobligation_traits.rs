//! # CoqObligation - Trait Implementations
//!
//! This module contains trait implementations for `CoqObligation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CoqObligation;
use std::fmt;

impl std::fmt::Display for CoqObligation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Next Obligation.\n")?;
        for t in &self.tactics {
            write!(f, "  {}.\n", t)?;
        }
        write!(f, "Qed.")
    }
}
