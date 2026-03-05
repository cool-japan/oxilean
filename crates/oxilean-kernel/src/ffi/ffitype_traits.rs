//! # FfiType - Trait Implementations
//!
//! This module contains trait implementations for `FfiType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FfiType;
use std::fmt;

impl fmt::Display for FfiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiType::Bool => write!(f, "bool"),
            FfiType::UInt8 => write!(f, "u8"),
            FfiType::UInt16 => write!(f, "u16"),
            FfiType::UInt32 => write!(f, "u32"),
            FfiType::UInt64 => write!(f, "u64"),
            FfiType::Int8 => write!(f, "i8"),
            FfiType::Int16 => write!(f, "i16"),
            FfiType::Int32 => write!(f, "i32"),
            FfiType::Int64 => write!(f, "i64"),
            FfiType::Float32 => write!(f, "f32"),
            FfiType::Float64 => write!(f, "f64"),
            FfiType::String => write!(f, "string"),
            FfiType::ByteArray => write!(f, "bytes"),
            FfiType::Unit => write!(f, "()"),
            FfiType::Ptr(inner) => write!(f, "*{}", inner),
            FfiType::Fn(params, ret) => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ret)
            }
            FfiType::OxiLean(name) => write!(f, "OxiLean({})", name),
        }
    }
}
