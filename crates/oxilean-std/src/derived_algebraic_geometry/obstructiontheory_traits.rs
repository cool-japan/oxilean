//! # ObstructionTheory - Trait Implementations
//!
//! This module contains trait implementations for `ObstructionTheory`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ObstructionTheory;
use std::fmt;

impl fmt::Display for ObstructionTheory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ObstrThy({}: T^1={}, T^2={}, vdim={})",
            self.space, self.tangent, self.obstruction, self.virtual_dim
        )
    }
}
