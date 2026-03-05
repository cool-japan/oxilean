//! # CraneliftCallingConvention - Trait Implementations
//!
//! This module contains trait implementations for `CraneliftCallingConvention`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CraneliftCallingConvention;
use std::fmt;

impl fmt::Display for CraneliftCallingConvention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CraneliftCallingConvention::SystemV => write!(f, "system_v"),
            CraneliftCallingConvention::WindowsFastcall => write!(f, "windows_fastcall"),
            CraneliftCallingConvention::WasmtimeSystem => write!(f, "wasmtime_system_v"),
            CraneliftCallingConvention::Cold => write!(f, "cold"),
            CraneliftCallingConvention::Tail => write!(f, "tail"),
            CraneliftCallingConvention::Fast => write!(f, "fast"),
        }
    }
}
