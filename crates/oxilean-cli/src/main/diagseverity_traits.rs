//! # DiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `DiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagSeverity;
use std::fmt;

impl std::fmt::Display for DiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagSeverity::Error => write!(f, "error"),
            DiagSeverity::Warning => write!(f, "warning"),
            DiagSeverity::Note => write!(f, "note"),
            DiagSeverity::Hint => write!(f, "hint"),
        }
    }
}
