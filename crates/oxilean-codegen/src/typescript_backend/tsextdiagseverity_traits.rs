//! # TsExtDiagSeverity - Trait Implementations
//!
//! This module contains trait implementations for `TsExtDiagSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{format_ts_stmt, format_ts_stmts};
use super::types::TsExtDiagSeverity;
use std::fmt;

impl std::fmt::Display for TsExtDiagSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsExtDiagSeverity::Note => write!(f, "note"),
            TsExtDiagSeverity::Warning => write!(f, "warning"),
            TsExtDiagSeverity::Error => write!(f, "error"),
        }
    }
}
