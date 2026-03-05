//! # CraneliftType - Trait Implementations
//!
//! This module contains trait implementations for `CraneliftType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CraneliftType;
use std::fmt;

impl fmt::Display for CraneliftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CraneliftType::B1 => write!(f, "b1"),
            CraneliftType::I8 => write!(f, "i8"),
            CraneliftType::I16 => write!(f, "i16"),
            CraneliftType::I32 => write!(f, "i32"),
            CraneliftType::I64 => write!(f, "i64"),
            CraneliftType::I128 => write!(f, "i128"),
            CraneliftType::F32 => write!(f, "f32"),
            CraneliftType::F64 => write!(f, "f64"),
            CraneliftType::R32 => write!(f, "r32"),
            CraneliftType::R64 => write!(f, "r64"),
            CraneliftType::Vector(base, lanes) => write!(f, "{}x{}", base, lanes),
            CraneliftType::Void => write!(f, "void"),
        }
    }
}
