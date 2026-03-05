//! # Quantity - Trait Implementations
//!
//! This module contains trait implementations for `Quantity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::Quantity;
use std::fmt;

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quantity::Zero => write!(f, "0 "),
            Quantity::One => write!(f, "1 "),
            Quantity::Unrestricted => Ok(()),
        }
    }
}
