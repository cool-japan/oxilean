//! # MacroErrorKind - Trait Implementations
//!
//! This module contains trait implementations for `MacroErrorKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MacroErrorKind;
use std::fmt;

impl fmt::Display for MacroErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MacroErrorKind::UnknownMacro => write!(f, "unknown macro"),
            MacroErrorKind::PatternMismatch => write!(f, "pattern mismatch"),
            MacroErrorKind::HygieneViolation => write!(f, "hygiene violation"),
            MacroErrorKind::AmbiguousMatch => write!(f, "ambiguous match"),
            MacroErrorKind::ExpansionError => write!(f, "expansion error"),
        }
    }
}
