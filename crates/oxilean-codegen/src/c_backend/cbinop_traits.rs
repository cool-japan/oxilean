//! # CBinOp - Trait Implementations
//!
//! This module contains trait implementations for `CBinOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CBinOp;
use std::fmt;

impl fmt::Display for CBinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CBinOp::Add => write!(f, "+"),
            CBinOp::Sub => write!(f, "-"),
            CBinOp::Mul => write!(f, "*"),
            CBinOp::Div => write!(f, "/"),
            CBinOp::Mod => write!(f, "%"),
            CBinOp::Eq => write!(f, "=="),
            CBinOp::Neq => write!(f, "!="),
            CBinOp::Lt => write!(f, "<"),
            CBinOp::Le => write!(f, "<="),
            CBinOp::Gt => write!(f, ">"),
            CBinOp::Ge => write!(f, ">="),
            CBinOp::And => write!(f, "&&"),
            CBinOp::Or => write!(f, "||"),
            CBinOp::BitAnd => write!(f, "&"),
            CBinOp::BitOr => write!(f, "|"),
            CBinOp::BitXor => write!(f, "^"),
            CBinOp::Shl => write!(f, "<<"),
            CBinOp::Shr => write!(f, ">>"),
        }
    }
}
