//! # HomologyGroup - Trait Implementations
//!
//! This module contains trait implementations for `HomologyGroup`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HomologyGroup;
use std::fmt;

impl std::fmt::Display for HomologyGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_trivial() {
            write!(f, "H_{}=0", self.degree)?;
            return Ok(());
        }
        write!(f, "H_{}=Z^{}", self.degree, self.rank)?;
        for &t in &self.torsion {
            write!(f, "⊕Z/{}", t)?;
        }
        Ok(())
    }
}
