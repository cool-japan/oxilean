//! # MetalUnOp - Trait Implementations
//!
//! This module contains trait implementations for `MetalUnOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalUnOp;
use std::fmt;

impl fmt::Display for MetalUnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MetalUnOp::Neg => "-",
            MetalUnOp::Not => "!",
            MetalUnOp::BitNot => "~",
        };
        write!(f, "{}", s)
    }
}
