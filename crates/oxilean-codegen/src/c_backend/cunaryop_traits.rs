//! # CUnaryOp - Trait Implementations
//!
//! This module contains trait implementations for `CUnaryOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CUnaryOp;
use std::fmt;

impl fmt::Display for CUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CUnaryOp::Neg => write!(f, "-"),
            CUnaryOp::Not => write!(f, "!"),
            CUnaryOp::BitNot => write!(f, "~"),
            CUnaryOp::Deref => write!(f, "*"),
            CUnaryOp::AddrOf => write!(f, "&"),
        }
    }
}
