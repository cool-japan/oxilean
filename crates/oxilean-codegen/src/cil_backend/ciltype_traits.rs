//! # CilType - Trait Implementations
//!
//! This module contains trait implementations for `CilType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilType;
use std::fmt;

impl fmt::Display for CilType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CilType::Void => write!(f, "void"),
            CilType::Bool => write!(f, "bool"),
            CilType::Int8 => write!(f, "int8"),
            CilType::Int16 => write!(f, "int16"),
            CilType::Int32 => write!(f, "int32"),
            CilType::Int64 => write!(f, "int64"),
            CilType::UInt8 => write!(f, "uint8"),
            CilType::UInt16 => write!(f, "uint16"),
            CilType::UInt32 => write!(f, "uint32"),
            CilType::UInt64 => write!(f, "uint64"),
            CilType::Float32 => write!(f, "float32"),
            CilType::Float64 => write!(f, "float64"),
            CilType::Char => write!(f, "char"),
            CilType::String => write!(f, "string"),
            CilType::Object => write!(f, "object"),
            CilType::Class {
                assembly,
                namespace,
                name,
            } => {
                write!(f, "class ")?;
                if let Some(asm) = assembly {
                    write!(f, "[{}]", asm)?;
                }
                if namespace.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}.{}", namespace, name)
                }
            }
            CilType::ValueType {
                assembly,
                namespace,
                name,
            } => {
                write!(f, "valuetype ")?;
                if let Some(asm) = assembly {
                    write!(f, "[{}]", asm)?;
                }
                if namespace.is_empty() {
                    write!(f, "{}", name)
                } else {
                    write!(f, "{}.{}", namespace, name)
                }
            }
            CilType::Array(inner) => write!(f, "{}[]", inner),
            CilType::MdArray(inner, dims) => {
                write!(f, "{}", inner)?;
                write!(f, "[")?;
                for i in 0..*dims {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
            CilType::Ptr(inner) => write!(f, "{}*", inner),
            CilType::ByRef(inner) => write!(f, "{}&", inner),
            CilType::Generic(base, args) => {
                write!(f, "{}<", base)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ">")
            }
            CilType::GenericParam(n) => write!(f, "!{}", n),
            CilType::GenericMethodParam(n) => write!(f, "!!{}", n),
            CilType::NativeInt => write!(f, "native int"),
            CilType::NativeUInt => write!(f, "native uint"),
        }
    }
}
