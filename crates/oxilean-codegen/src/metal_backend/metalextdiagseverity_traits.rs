//! # MetalExtDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `MetalExtDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetalExtDiagSeverity;
use std::fmt;

impl std::fmt::Display for MetalExtDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetalExtDiagSeverity::Note => write!(f, "note"),
            MetalExtDiagSeverity::Warning => write!(f, "warning"),
            MetalExtDiagSeverity::Error => write!(f, "error"),
        }
    }
}
