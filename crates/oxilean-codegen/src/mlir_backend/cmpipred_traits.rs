//! # CmpiPred - Trait Implementations
//!
//! This module contains trait implementations for `CmpiPred`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CmpiPred;
use std::fmt;

impl fmt::Display for CmpiPred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmpiPred::Eq => write!(f, "eq"),
            CmpiPred::Ne => write!(f, "ne"),
            CmpiPred::Slt => write!(f, "slt"),
            CmpiPred::Sle => write!(f, "sle"),
            CmpiPred::Sgt => write!(f, "sgt"),
            CmpiPred::Sge => write!(f, "sge"),
            CmpiPred::Ult => write!(f, "ult"),
            CmpiPred::Ule => write!(f, "ule"),
            CmpiPred::Ugt => write!(f, "ugt"),
            CmpiPred::Uge => write!(f, "uge"),
        }
    }
}
