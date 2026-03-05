//! # LlvmType - Trait Implementations
//!
//! This module contains trait implementations for `LlvmType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LlvmType;
use std::fmt;

impl fmt::Display for LlvmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlvmType::Void => write!(f, "void"),
            LlvmType::I1 => write!(f, "i1"),
            LlvmType::I8 => write!(f, "i8"),
            LlvmType::I16 => write!(f, "i16"),
            LlvmType::I32 => write!(f, "i32"),
            LlvmType::I64 => write!(f, "i64"),
            LlvmType::I128 => write!(f, "i128"),
            LlvmType::F32 => write!(f, "float"),
            LlvmType::F64 => write!(f, "double"),
            LlvmType::F128 => write!(f, "fp128"),
            LlvmType::Ptr => write!(f, "ptr"),
            LlvmType::Array(n, ty) => write!(f, "[{} x {}]", n, ty),
            LlvmType::Struct(fields) => {
                write!(f, "{{ ")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, " }}")
            }
            LlvmType::Vector(n, ty) => write!(f, "<{} x {}>", n, ty),
            LlvmType::FuncType {
                ret,
                params,
                variadic,
            } => {
                write!(f, "{} (", ret)?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                if *variadic {
                    if !params.is_empty() {
                        write!(f, ", ")?;
                    }
                    write!(f, "...")?;
                }
                write!(f, ")")
            }
            LlvmType::Named(name) => write!(f, "%{}", name),
        }
    }
}
