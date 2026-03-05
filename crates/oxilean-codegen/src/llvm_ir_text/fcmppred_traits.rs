//! # FcmpPred - Trait Implementations
//!
//! This module contains trait implementations for `FcmpPred`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FcmpPred;
use std::fmt;

impl fmt::Display for FcmpPred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FcmpPred::False => "false",
            FcmpPred::Oeq => "oeq",
            FcmpPred::Ogt => "ogt",
            FcmpPred::Oge => "oge",
            FcmpPred::Olt => "olt",
            FcmpPred::Ole => "ole",
            FcmpPred::One => "one",
            FcmpPred::Ord => "ord",
            FcmpPred::Ueq => "ueq",
            FcmpPred::Ugt => "ugt",
            FcmpPred::Uge => "uge",
            FcmpPred::Ult => "ult",
            FcmpPred::Ule => "ule",
            FcmpPred::Une => "une",
            FcmpPred::Uno => "uno",
            FcmpPred::True => "true",
        };
        write!(f, "{}", s)
    }
}
