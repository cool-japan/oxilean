//! # DiagnosticCode - Trait Implementations
//!
//! This module contains trait implementations for `DiagnosticCode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagnosticCode;
use std::fmt;

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            DiagnosticCode::E0001 => "E0001",
            DiagnosticCode::E0002 => "E0002",
            DiagnosticCode::E0003 => "E0003",
            DiagnosticCode::E0004 => "E0004",
            DiagnosticCode::E0005 => "E0005",
            DiagnosticCode::E0100 => "E0100",
            DiagnosticCode::E0101 => "E0101",
            DiagnosticCode::E0102 => "E0102",
            DiagnosticCode::E0103 => "E0103",
            DiagnosticCode::E0104 => "E0104",
            DiagnosticCode::E0200 => "E0200",
            DiagnosticCode::E0201 => "E0201",
            DiagnosticCode::E0202 => "E0202",
            DiagnosticCode::E0900 => "E0900",
            DiagnosticCode::E0901 => "E0901",
        };
        write!(f, "{}", code)
    }
}
