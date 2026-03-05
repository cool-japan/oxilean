//! # FfiNativeType - Trait Implementations
//!
//! This module contains trait implementations for `FfiNativeType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiNativeType;
use std::fmt;

impl fmt::Display for FfiNativeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiNativeType::Void => write!(f, "void"),
            FfiNativeType::I8 => write!(f, "int8_t"),
            FfiNativeType::I16 => write!(f, "int16_t"),
            FfiNativeType::I32 => write!(f, "int32_t"),
            FfiNativeType::I64 => write!(f, "int64_t"),
            FfiNativeType::U8 => write!(f, "uint8_t"),
            FfiNativeType::U16 => write!(f, "uint16_t"),
            FfiNativeType::U32 => write!(f, "uint32_t"),
            FfiNativeType::U64 => write!(f, "uint64_t"),
            FfiNativeType::F32 => write!(f, "float"),
            FfiNativeType::F64 => write!(f, "double"),
            FfiNativeType::Bool => write!(f, "bool"),
            FfiNativeType::SizeT => write!(f, "size_t"),
            FfiNativeType::CStr => write!(f, "const char*"),
            FfiNativeType::Ptr(inner) => write!(f, "{}*", inner),
            FfiNativeType::OpaquePtr => write!(f, "void*"),
            FfiNativeType::LeanObject => write!(f, "lean_object*"),
            FfiNativeType::Struct(name, _) => write!(f, "struct {}", name),
            FfiNativeType::FnPtr(params, ret) => {
                write!(f, "{} (*)(", ret)?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
        }
    }
}
