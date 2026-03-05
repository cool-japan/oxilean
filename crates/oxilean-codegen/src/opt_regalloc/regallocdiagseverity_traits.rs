//! # RegAllocDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `RegAllocDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RegAllocDiagSeverity;
use std::fmt;

impl std::fmt::Display for RegAllocDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegAllocDiagSeverity::Note => write!(f, "note"),
            RegAllocDiagSeverity::Warning => write!(f, "warning"),
            RegAllocDiagSeverity::Error => write!(f, "error"),
        }
    }
}
