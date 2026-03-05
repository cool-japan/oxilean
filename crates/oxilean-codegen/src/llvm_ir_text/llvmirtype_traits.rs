//! # LlvmIrType - Trait Implementations
//!
//! This module contains trait implementations for `LlvmIrType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LlvmIrType;
use std::fmt;

impl fmt::Display for LlvmIrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlvmIrType::Void => write!(f, "void"),
            LlvmIrType::I1 => write!(f, "i1"),
            LlvmIrType::I8 => write!(f, "i8"),
            LlvmIrType::I16 => write!(f, "i16"),
            LlvmIrType::I32 => write!(f, "i32"),
            LlvmIrType::I64 => write!(f, "i64"),
            LlvmIrType::I128 => write!(f, "i128"),
            LlvmIrType::IArb(n) => write!(f, "i{}", n),
            LlvmIrType::Float => write!(f, "float"),
            LlvmIrType::Double => write!(f, "double"),
            LlvmIrType::Fp128 => write!(f, "fp128"),
            LlvmIrType::X86Fp80 => write!(f, "x86_fp80"),
            LlvmIrType::Ptr => write!(f, "ptr"),
            LlvmIrType::PtrAs(n) => write!(f, "ptr addrspace({})", n),
            LlvmIrType::Array(n, ty) => write!(f, "[{} x {}]", n, ty),
            LlvmIrType::Struct(fields) => {
                write!(f, "{{ ")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, " }}")
            }
            LlvmIrType::PackedStruct(fields) => {
                write!(f, "<{{ ")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, " }}>")
            }
            LlvmIrType::Vector(n, ty) => write!(f, "<{} x {}>", n, ty),
            LlvmIrType::ScalableVector(n, ty) => write!(f, "<vscale x {} x {}>", n, ty),
            LlvmIrType::Named(name) => write!(f, "%{}", name),
            LlvmIrType::Func {
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
            LlvmIrType::Label => write!(f, "label"),
            LlvmIrType::Metadata => write!(f, "metadata"),
            LlvmIrType::Token => write!(f, "token"),
        }
    }
}
