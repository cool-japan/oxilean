//! # CudaBinOp - Trait Implementations
//!
//! This module contains trait implementations for `CudaBinOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CudaBinOp;
use std::fmt;

impl fmt::Display for CudaBinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CudaBinOp::Add => "+",
            CudaBinOp::Sub => "-",
            CudaBinOp::Mul => "*",
            CudaBinOp::Div => "/",
            CudaBinOp::Mod => "%",
            CudaBinOp::Eq => "==",
            CudaBinOp::Neq => "!=",
            CudaBinOp::Lt => "<",
            CudaBinOp::Le => "<=",
            CudaBinOp::Gt => ">",
            CudaBinOp::Ge => ">=",
            CudaBinOp::And => "&&",
            CudaBinOp::Or => "||",
            CudaBinOp::BitAnd => "&",
            CudaBinOp::BitOr => "|",
            CudaBinOp::BitXor => "^",
            CudaBinOp::Shl => "<<",
            CudaBinOp::Shr => ">>",
        };
        write!(f, "{}", s)
    }
}
