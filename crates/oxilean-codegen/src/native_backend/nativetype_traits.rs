//! # NativeType - Trait Implementations
//!
//! This module contains trait implementations for `NativeType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::NativeType;
use std::fmt;

impl fmt::Display for NativeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NativeType::I8 => write!(f, "i8"),
            NativeType::I16 => write!(f, "i16"),
            NativeType::I32 => write!(f, "i32"),
            NativeType::I64 => write!(f, "i64"),
            NativeType::F32 => write!(f, "f32"),
            NativeType::F64 => write!(f, "f64"),
            NativeType::Ptr => write!(f, "ptr"),
            NativeType::FuncRef => write!(f, "funcref"),
            NativeType::Void => write!(f, "void"),
        }
    }
}
