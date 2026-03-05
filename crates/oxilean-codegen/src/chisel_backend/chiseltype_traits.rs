//! # ChiselType - Trait Implementations
//!
//! This module contains trait implementations for `ChiselType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChiselType;
use std::fmt;

impl fmt::Display for ChiselType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChiselType::UInt(w) => write!(f, "UInt({w}.W)"),
            ChiselType::SInt(w) => write!(f, "SInt({w}.W)"),
            ChiselType::Bool => write!(f, "Bool()"),
            ChiselType::Clock => write!(f, "Clock()"),
            ChiselType::Reset => write!(f, "Reset()"),
            ChiselType::AsyncReset => write!(f, "AsyncReset()"),
            ChiselType::Vec(n, ty) => write!(f, "Vec({n}, {ty})"),
            ChiselType::Bundle(fields) => {
                write!(f, "new Bundle {{")?;
                for (name, ty) in fields {
                    write!(f, " val {name} = Output({ty});")?;
                }
                write!(f, " }}")
            }
        }
    }
}
