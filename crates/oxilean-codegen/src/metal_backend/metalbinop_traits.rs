//! # MetalBinOp - Trait Implementations
//!
//! This module contains trait implementations for `MetalBinOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalBinOp;
use std::fmt;

impl fmt::Display for MetalBinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MetalBinOp::Add => "+",
            MetalBinOp::Sub => "-",
            MetalBinOp::Mul => "*",
            MetalBinOp::Div => "/",
            MetalBinOp::Mod => "%",
            MetalBinOp::Eq => "==",
            MetalBinOp::Neq => "!=",
            MetalBinOp::Lt => "<",
            MetalBinOp::Le => "<=",
            MetalBinOp::Gt => ">",
            MetalBinOp::Ge => ">=",
            MetalBinOp::And => "&&",
            MetalBinOp::Or => "||",
            MetalBinOp::BitAnd => "&",
            MetalBinOp::BitOr => "|",
            MetalBinOp::BitXor => "^",
            MetalBinOp::Shl => "<<",
            MetalBinOp::Shr => ">>",
        };
        write!(f, "{}", s)
    }
}
