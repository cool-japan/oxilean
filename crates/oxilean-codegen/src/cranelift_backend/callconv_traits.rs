//! # CallConv - Trait Implementations
//!
//! This module contains trait implementations for `CallConv`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CallConv;
use std::fmt;

impl fmt::Display for CallConv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallConv::Fast => write!(f, "fast"),
            CallConv::Cold => write!(f, "cold"),
            CallConv::SystemV => write!(f, "system_v"),
            CallConv::WindowsFastcall => write!(f, "windows_fastcall"),
            CallConv::WasmtimeSystemV => write!(f, "wasmtime_system_v"),
        }
    }
}
