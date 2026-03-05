//! # DerivedScheme - Trait Implementations
//!
//! This module contains trait implementations for `DerivedScheme`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DerivedScheme;
use std::fmt;

impl fmt::Display for DerivedScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DerivedScheme({}, affine={}, amplitude=[{},{}])",
            self.name, self.is_affine, self.amplitude.0, self.amplitude.1
        )
    }
}
