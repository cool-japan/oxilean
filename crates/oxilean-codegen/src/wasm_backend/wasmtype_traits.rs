//! # WasmType - Trait Implementations
//!
//! This module contains trait implementations for `WasmType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::WasmType;
use std::fmt;

impl fmt::Display for WasmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmType::I32 => write!(f, "i32"),
            WasmType::I64 => write!(f, "i64"),
            WasmType::F32 => write!(f, "f32"),
            WasmType::F64 => write!(f, "f64"),
            WasmType::FuncRef => write!(f, "funcref"),
            WasmType::ExternRef => write!(f, "externref"),
            WasmType::V128 => write!(f, "v128"),
        }
    }
}
