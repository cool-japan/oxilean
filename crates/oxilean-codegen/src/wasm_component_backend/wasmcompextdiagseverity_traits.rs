//! # WasmCompExtDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `WasmCompExtDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WasmCompExtDiagSeverity;
use std::fmt;

impl std::fmt::Display for WasmCompExtDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WasmCompExtDiagSeverity::Note => write!(f, "note"),
            WasmCompExtDiagSeverity::Warning => write!(f, "warning"),
            WasmCompExtDiagSeverity::Error => write!(f, "error"),
        }
    }
}
