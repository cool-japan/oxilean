//! # IntCC - Trait Implementations
//!
//! This module contains trait implementations for `IntCC`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IntCC;
use std::fmt;

impl fmt::Display for IntCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntCC::Equal => write!(f, "eq"),
            IntCC::NotEqual => write!(f, "ne"),
            IntCC::SignedLessThan => write!(f, "slt"),
            IntCC::SignedLessThanOrEqual => write!(f, "sle"),
            IntCC::SignedGreaterThan => write!(f, "sgt"),
            IntCC::SignedGreaterThanOrEqual => write!(f, "sge"),
            IntCC::UnsignedLessThan => write!(f, "ult"),
            IntCC::UnsignedLessThanOrEqual => write!(f, "ule"),
            IntCC::UnsignedGreaterThan => write!(f, "ugt"),
            IntCC::UnsignedGreaterThanOrEqual => write!(f, "uge"),
            IntCC::Overflow => write!(f, "of"),
            IntCC::NotOverflow => write!(f, "nof"),
        }
    }
}
