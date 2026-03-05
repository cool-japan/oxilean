//! # MetalExtDiagMsg - Trait Implementations
//!
//! This module contains trait implementations for `MetalExtDiagMsg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalExtDiagMsg;
use std::fmt;

impl std::fmt::Display for MetalExtDiagMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.pass, self.message)
    }
}
