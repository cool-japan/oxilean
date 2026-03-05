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
use std::fmt;

impl fmt::Display for ProofObligation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Obligation: {}", self.description)?;
        if !self.context.is_empty() {
            write!(f, "\n  Context:")?;
            for (name, ty) in &self.context {
                write!(f, "\n    {} : {:?}", name, ty)?;
            }
        }
        write!(f, "\n  Goal: {:?}", self.goal)?;
        if self.auto_discharged {
            write!(f, " [auto]")?;
        }
        Ok(())
    }
}
