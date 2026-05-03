//! # IrType - Trait Implementations
//!
//! This module contains trait implementations for `IrType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::IrType;
use std::fmt;

impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrType::Unit => write!(f, "()"),
            IrType::Bool => write!(f, "bool"),
            IrType::Nat => write!(f, "nat"),
            IrType::Int => write!(f, "i64"),
            IrType::String => write!(f, "string"),
            IrType::Var(name) => write!(f, "{}", name),
            IrType::Function { params, ret } => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            IrType::Struct { name, fields } => {
                write!(f, "struct {} {{ ", name)?;
                for (i, (fname, ftype)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", fname, ftype)?;
                }
                write!(f, " }}")
            }
            IrType::Array { elem, len } => write!(f, "[{}; {}]", elem, len),
            IrType::Pointer(ty) => write!(f, "*{}", ty),
            IrType::Unknown => write!(f, "unknown"),
        }
    }
}
