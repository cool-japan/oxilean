//! # CotangentComplex - Trait Implementations
//!
//! This module contains trait implementations for `CotangentComplex`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CotangentComplex;
use std::fmt;

impl fmt::Display for CotangentComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "L_{{{}/{}}}: cohom amplitude [{},{}]",
            self.source, self.target, self.amplitude.0, self.amplitude.1
        )
    }
}
