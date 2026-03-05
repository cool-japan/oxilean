//! # CmpfPred - Trait Implementations
//!
//! This module contains trait implementations for `CmpfPred`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CmpfPred;
use std::fmt;

impl fmt::Display for CmpfPred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmpfPred::Oeq => write!(f, "oeq"),
            CmpfPred::One => write!(f, "one"),
            CmpfPred::Olt => write!(f, "olt"),
            CmpfPred::Ole => write!(f, "ole"),
            CmpfPred::Ogt => write!(f, "ogt"),
            CmpfPred::Oge => write!(f, "oge"),
            CmpfPred::Ueq => write!(f, "ueq"),
            CmpfPred::Une => write!(f, "une"),
        }
    }
}
