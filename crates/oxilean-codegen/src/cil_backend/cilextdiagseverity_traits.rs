//! # CilExtDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `CilExtDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::CilExtDiagSeverity;
use std::fmt;

impl std::fmt::Display for CilExtDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CilExtDiagSeverity::Note => write!(f, "note"),
            CilExtDiagSeverity::Warning => write!(f, "warning"),
            CilExtDiagSeverity::Error => write!(f, "error"),
        }
    }
}
