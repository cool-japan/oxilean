//! # FcmpPred - Trait Implementations
//!
//! This module contains trait implementations for `FcmpPred`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FcmpPred;
use std::fmt;

impl fmt::Display for FcmpPred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FcmpPred::Oeq => write!(f, "oeq"),
            FcmpPred::One => write!(f, "one"),
            FcmpPred::Olt => write!(f, "olt"),
            FcmpPred::Ogt => write!(f, "ogt"),
            FcmpPred::Ole => write!(f, "ole"),
            FcmpPred::Oge => write!(f, "oge"),
            FcmpPred::Uno => write!(f, "uno"),
            FcmpPred::Ord => write!(f, "ord"),
            FcmpPred::True_ => write!(f, "true"),
            FcmpPred::False_ => write!(f, "false"),
        }
    }
}
