//! # ProofTrace - Trait Implementations
//!
//! This module contains trait implementations for `ProofTrace`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProofTrace;
use std::fmt;

impl fmt::Display for ProofTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for step in &self.steps {
            writeln!(f, "{}", step)?;
        }
        if self.is_complete() {
            writeln!(f, "Proof complete.")?;
        } else {
            writeln!(f, "Proof incomplete.")?;
        }
        Ok(())
    }
}
