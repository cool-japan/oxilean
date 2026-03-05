//! # LlvmIrValue - Trait Implementations
//!
//! This module contains trait implementations for `LlvmIrValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LlvmIrValue;
use std::fmt;

impl fmt::Display for LlvmIrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlvmIrValue::ConstInt(n) => write!(f, "{}", n),
            LlvmIrValue::ConstUint(n) => write!(f, "{}", n),
            LlvmIrValue::ConstFloat(v) => write!(f, "{:e}", v),
            LlvmIrValue::ConstNull => write!(f, "null"),
            LlvmIrValue::ZeroInitializer => write!(f, "zeroinitializer"),
            LlvmIrValue::Undef => write!(f, "undef"),
            LlvmIrValue::Poison => write!(f, "poison"),
            LlvmIrValue::Register(name) => write!(f, "%{}", name),
            LlvmIrValue::Global(name) => write!(f, "@{}", name),
            LlvmIrValue::ConstGep { ty, base, indices } => {
                write!(f, "getelementptr ({}", ty)?;
                write!(f, ", ptr {}", base)?;
                for (idx_ty, idx_val) in indices {
                    write!(f, ", {} {}", idx_ty, idx_val)?;
                }
                write!(f, ")")
            }
            LlvmIrValue::ConstBitcast {
                val,
                from_ty,
                to_ty,
            } => {
                write!(f, "bitcast ({} {} to {})", from_ty, val, to_ty)
            }
            LlvmIrValue::ConstArray { elem_ty, elems } => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", elem_ty, e)?;
                }
                write!(f, "]")
            }
            LlvmIrValue::ConstStruct { fields } => {
                write!(f, "{{ ")?;
                for (i, (ty, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", ty, val)?;
                }
                write!(f, " }}")
            }
        }
    }
}
