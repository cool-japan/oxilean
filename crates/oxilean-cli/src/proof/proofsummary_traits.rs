//! # ProofSummary - Trait Implementations
//!
//! This module contains trait implementations for `ProofSummary`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProofSummary;
use std::fmt;

impl fmt::Display for ProofSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Proof summary: {}/{} verified, {} failed, {} partial, {} unchecked",
            self.verified_count(),
            self.total(),
            self.failed_count(),
            self.partial_count(),
            self.unchecked_count()
        )?;
        for record in &self.records {
            writeln!(f, "  {}", record)?;
        }
        Ok(())
    }
}
