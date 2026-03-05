//! # LlvmValue - Trait Implementations
//!
//! This module contains trait implementations for `LlvmValue`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::LlvmValue;
use std::fmt;

impl fmt::Display for LlvmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlvmValue::Const(n) => write!(f, "{}", n),
            LlvmValue::Float(v) => write!(f, "{:.17e}", v),
            LlvmValue::Undef => write!(f, "undef"),
            LlvmValue::Null => write!(f, "null"),
            LlvmValue::True_ => write!(f, "true"),
            LlvmValue::False_ => write!(f, "false"),
            LlvmValue::GlobalRef(name) => write!(f, "@{}", name),
            LlvmValue::LocalRef(name) => write!(f, "%{}", name),
            LlvmValue::ConstArray(ty, elems) => {
                write!(f, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} {}", ty, e)?;
                }
                write!(f, "]")
            }
            LlvmValue::ConstStruct(fields) => {
                write!(f, "{{ ")?;
                for (i, field) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", field)?;
                }
                write!(f, " }}")
            }
            LlvmValue::ZeroInitializer => write!(f, "zeroinitializer"),
        }
    }
}
