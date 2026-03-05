//! # SeverityFilter - Trait Implementations
//!
//! This module contains trait implementations for `SeverityFilter`.
//!
//! ## Implemented Traits
//!
//! - `DiagnosticFilter`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lsp::{
    analyze_document, DiagnosticSeverity, Document, DocumentStore, JsonValue, Location, Range,
    SymbolKind, TextEdit,
};
use std::fmt;

use super::functions::DiagnosticFilter;
use super::types::{AdvDiagnostic, SeverityFilter};

impl DiagnosticFilter for SeverityFilter {
    fn accepts(&self, diag: &AdvDiagnostic) -> bool {
        let severity_rank = |s: &DiagnosticSeverity| match s {
            DiagnosticSeverity::Error => 0,
            DiagnosticSeverity::Warning => 1,
            DiagnosticSeverity::Information => 2,
            DiagnosticSeverity::Hint => 3,
        };
        severity_rank(&diag.severity) <= severity_rank(&self.min_severity)
    }
}
