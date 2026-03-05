//! # FormalModuliProblem - Trait Implementations
//!
//! This module contains trait implementations for `FormalModuliProblem`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FormalModuliProblem;
use std::fmt;

impl fmt::Display for FormalModuliProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FMP[{}](classifies: {})", self.name, self.classifies)?;
        if let Some(ref l) = self.tangent_lie {
            write!(f, " T_L∞={}", l)?;
        }
        Ok(())
    }
}
