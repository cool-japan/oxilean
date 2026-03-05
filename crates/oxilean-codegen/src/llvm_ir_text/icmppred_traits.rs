//! # IcmpPred - Trait Implementations
//!
//! This module contains trait implementations for `IcmpPred`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IcmpPred;
use std::fmt;

impl fmt::Display for IcmpPred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IcmpPred::Eq => "eq",
            IcmpPred::Ne => "ne",
            IcmpPred::Ugt => "ugt",
            IcmpPred::Uge => "uge",
            IcmpPred::Ult => "ult",
            IcmpPred::Ule => "ule",
            IcmpPred::Sgt => "sgt",
            IcmpPred::Sge => "sge",
            IcmpPred::Slt => "slt",
            IcmpPred::Sle => "sle",
        };
        write!(f, "{}", s)
    }
}
