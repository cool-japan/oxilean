//! # QuasiIsomorphism - Trait Implementations
//!
//! This module contains trait implementations for `QuasiIsomorphism`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::QuasiIsomorphism;
use std::fmt;

impl fmt::Display for QuasiIsomorphism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "qis: {} ~> {} [{}]",
            self.source, self.target, self.cohomology_iso_desc
        )
    }
}
