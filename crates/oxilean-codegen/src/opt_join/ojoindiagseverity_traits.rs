//! # OJoinDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `OJoinDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::OJoinDiagSeverity;
use std::fmt;

impl std::fmt::Display for OJoinDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OJoinDiagSeverity::Note => write!(f, "note"),
            OJoinDiagSeverity::Warning => write!(f, "warning"),
            OJoinDiagSeverity::Error => write!(f, "error"),
        }
    }
}
