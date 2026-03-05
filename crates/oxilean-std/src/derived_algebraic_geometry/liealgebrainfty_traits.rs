//! # LieAlgebraInfty - Trait Implementations
//!
//! This module contains trait implementations for `LieAlgebraInfty`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LieAlgebraInfty;
use std::fmt;

impl fmt::Display for LieAlgebraInfty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = if self.is_dg_lie { "dg Lie" } else { "L∞" };
        write!(
            f,
            "{}-algebra {} (bracket deg {})",
            kind, self.name, self.bracket_degree
        )
    }
}
