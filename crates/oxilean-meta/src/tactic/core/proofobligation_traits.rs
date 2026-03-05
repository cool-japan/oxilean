//! # ProofObligation - Trait Implementations
//!
//! This module contains trait implementations for `ProofObligation`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProofObligation;

impl std::fmt::Display for ProofObligation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "obligation {} : {:?}", self.name, self.ty)?;
        if let Some(src) = &self.source {
            write!(f, " [from {}]", src)?;
        }
        Ok(())
    }
}
