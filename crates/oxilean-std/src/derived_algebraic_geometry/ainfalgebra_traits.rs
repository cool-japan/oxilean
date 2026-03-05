//! # AInfAlgebra - Trait Implementations
//!
//! This module contains trait implementations for `AInfAlgebra`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::AInfAlgebra;
use std::fmt;

impl fmt::Display for AInfAlgebra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A∞-algebra {} over {} (max order m_{})",
            self.name, self.base_field, self.max_composition_order
        )
    }
}
