//! # CType - Trait Implementations
//!
//! This module contains trait implementations for `CType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CType;
use std::fmt;

impl fmt::Display for CType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CType::Void => write!(f, "void"),
            CType::Int => write!(f, "int64_t"),
            CType::UInt => write!(f, "uint64_t"),
            CType::Bool => write!(f, "uint8_t"),
            CType::Char => write!(f, "char"),
            CType::Ptr(inner) => write!(f, "{}*", inner),
            CType::Struct(name) => write!(f, "struct {}", name),
            CType::FnPtr(params, ret) => {
                write!(f, "{} (*)(", ret)?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            CType::Array(elem, size) => write!(f, "{}[{}]", elem, size),
            CType::SizeT => write!(f, "size_t"),
            CType::U8 => write!(f, "uint8_t"),
            CType::LeanObject => write!(f, "lean_object*"),
        }
    }
}
