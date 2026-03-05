//! # TropicalElement - Trait Implementations
//!
//! This module contains trait implementations for `TropicalElement`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::TropicalElement;
use std::fmt;

impl fmt::Display for TropicalElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TropicalElement::NegInfinity => write!(f, "∞"),
            TropicalElement::Finite(v) => write!(f, "{v}"),
        }
    }
}
