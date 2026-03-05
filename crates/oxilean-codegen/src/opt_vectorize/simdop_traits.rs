//! # SIMDOp - Trait Implementations
//!
//! This module contains trait implementations for `SIMDOp`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SIMDOp;
use std::fmt;

impl fmt::Display for SIMDOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SIMDOp::Add => write!(f, "vadd"),
            SIMDOp::Sub => write!(f, "vsub"),
            SIMDOp::Mul => write!(f, "vmul"),
            SIMDOp::Div => write!(f, "vdiv"),
            SIMDOp::Sqrt => write!(f, "vsqrt"),
            SIMDOp::Fma => write!(f, "vfma"),
            SIMDOp::Broadcast => write!(f, "vbroadcast"),
            SIMDOp::Load => write!(f, "vload"),
            SIMDOp::Store => write!(f, "vstore"),
            SIMDOp::Shuffle => write!(f, "vshuffle"),
            SIMDOp::Blend => write!(f, "vblend"),
            SIMDOp::Compare(cmp) => write!(f, "vcmp.{}", cmp),
            SIMDOp::Min => write!(f, "vmin"),
            SIMDOp::Max => write!(f, "vmax"),
            SIMDOp::HorizontalAdd => write!(f, "vhadd"),
        }
    }
}
