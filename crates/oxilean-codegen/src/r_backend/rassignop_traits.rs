//! # RAssignOp - Trait Implementations
//!
//! This module contains trait implementations for `RAssignOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RAssignOp;
use std::fmt;

impl fmt::Display for RAssignOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RAssignOp::LeftArrow => write!(f, "<-"),
            RAssignOp::SuperArrow => write!(f, "<<-"),
            RAssignOp::Equals => write!(f, "="),
            RAssignOp::RightArrow => write!(f, "->"),
        }
    }
}
