//! # SpectralSequence - Trait Implementations
//!
//! This module contains trait implementations for `SpectralSequence`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SpectralSequence;
use std::fmt;

impl fmt::Display for SpectralSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SS[{}](start=E_{}, d_r=bideg({},{}), {})",
            self.name,
            self.start_page,
            self.differential_bidegree.0,
            self.differential_bidegree.1,
            self.convergence
        )
    }
}
