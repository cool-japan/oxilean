//! # TropicalNumber - Trait Implementations
//!
//! This module contains trait implementations for `TropicalNumber`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TropicalNumber;
use std::fmt;

impl fmt::Display for TropicalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TropicalNumber::Finite(v) => write!(f, "{v}"),
            TropicalNumber::PosInfinity => write!(f, "+∞"),
        }
    }
}
