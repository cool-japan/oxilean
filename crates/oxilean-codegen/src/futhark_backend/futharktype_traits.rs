//! # FutharkType - Trait Implementations
//!
//! This module contains trait implementations for `FutharkType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkType;
use std::fmt;

impl fmt::Display for FutharkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FutharkType::I8 => write!(f, "i8"),
            FutharkType::I16 => write!(f, "i16"),
            FutharkType::I32 => write!(f, "i32"),
            FutharkType::I64 => write!(f, "i64"),
            FutharkType::U8 => write!(f, "u8"),
            FutharkType::U16 => write!(f, "u16"),
            FutharkType::U32 => write!(f, "u32"),
            FutharkType::U64 => write!(f, "u64"),
            FutharkType::F16 => write!(f, "f16"),
            FutharkType::F32 => write!(f, "f32"),
            FutharkType::F64 => write!(f, "f64"),
            FutharkType::Bool => write!(f, "bool"),
            FutharkType::Array(elem, dims) => {
                for dim in dims {
                    match dim {
                        Some(name) => write!(f, "[{name}]")?,
                        None => write!(f, "[]")?,
                    }
                }
                write!(f, "{elem}")
            }
            FutharkType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{e}")?;
                }
                write!(f, ")")
            }
            FutharkType::Record(fields) => {
                write!(f, "{{")?;
                for (i, (name, ty)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{name}: {ty}")?;
                }
                write!(f, "}}")
            }
            FutharkType::Opaque(name) => write!(f, "{name}"),
            FutharkType::Named(name) => write!(f, "{name}"),
            FutharkType::Parametric(name, params) => {
                write!(f, "{name}")?;
                for p in params {
                    write!(f, " '{p}")?;
                }
                Ok(())
            }
        }
    }
}
