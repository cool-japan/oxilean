//! # FfiExtDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `FfiExtDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiExtDiagSeverity;
use std::fmt;

impl std::fmt::Display for FfiExtDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiExtDiagSeverity::Note => write!(f, "note"),
            FfiExtDiagSeverity::Warning => write!(f, "warning"),
            FfiExtDiagSeverity::Error => write!(f, "error"),
        }
    }
}
