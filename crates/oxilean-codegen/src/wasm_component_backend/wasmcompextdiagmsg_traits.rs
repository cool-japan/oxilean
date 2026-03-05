//! # WasmCompExtDiagMsg - Trait Implementations
//!
//! This module contains trait implementations for `WasmCompExtDiagMsg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WasmCompExtDiagMsg;
use std::fmt;

impl std::fmt::Display for WasmCompExtDiagMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.pass, self.message)
    }
}
