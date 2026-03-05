//! # HsExtDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `HsExtDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::HsExtDiagSeverity;
use std::fmt;

impl std::fmt::Display for HsExtDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HsExtDiagSeverity::Note => write!(f, "note"),
            HsExtDiagSeverity::Warning => write!(f, "warning"),
            HsExtDiagSeverity::Error => write!(f, "error"),
        }
    }
}
