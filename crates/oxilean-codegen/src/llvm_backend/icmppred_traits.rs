//! # IcmpPred - Trait Implementations
//!
//! This module contains trait implementations for `IcmpPred`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::IcmpPred;
use std::fmt;

impl fmt::Display for IcmpPred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IcmpPred::Eq => write!(f, "eq"),
            IcmpPred::Ne => write!(f, "ne"),
            IcmpPred::Slt => write!(f, "slt"),
            IcmpPred::Sgt => write!(f, "sgt"),
            IcmpPred::Sle => write!(f, "sle"),
            IcmpPred::Sge => write!(f, "sge"),
            IcmpPred::Ult => write!(f, "ult"),
            IcmpPred::Ugt => write!(f, "ugt"),
            IcmpPred::Ule => write!(f, "ule"),
            IcmpPred::Uge => write!(f, "uge"),
        }
    }
}
