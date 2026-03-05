//! # OParDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `OParDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OParDiagSeverity;
use std::fmt;

impl std::fmt::Display for OParDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OParDiagSeverity::Note => write!(f, "note"),
            OParDiagSeverity::Warning => write!(f, "warning"),
            OParDiagSeverity::Error => write!(f, "error"),
        }
    }
}
