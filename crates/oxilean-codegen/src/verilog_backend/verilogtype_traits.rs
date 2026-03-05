//! # VerilogType - Trait Implementations
//!
//! This module contains trait implementations for `VerilogType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use super::types::VerilogType;
use std::fmt;

impl fmt::Display for VerilogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerilogType::Wire(w) => write!(f, "wire{}", range_suffix(*w)),
            VerilogType::Reg(w) => write!(f, "reg{}", range_suffix(*w)),
            VerilogType::Logic(w) => write!(f, "logic{}", range_suffix(*w)),
            VerilogType::Integer => write!(f, "integer"),
            VerilogType::Real => write!(f, "real"),
        }
    }
}
