//! # CudaUnOp - Trait Implementations
//!
//! This module contains trait implementations for `CudaUnOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CudaUnOp;
use std::fmt;

impl fmt::Display for CudaUnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CudaUnOp::Neg => "-",
            CudaUnOp::Not => "!",
            CudaUnOp::BitNot => "~",
            CudaUnOp::Deref => "*",
            CudaUnOp::AddrOf => "&",
        };
        write!(f, "{}", s)
    }
}
