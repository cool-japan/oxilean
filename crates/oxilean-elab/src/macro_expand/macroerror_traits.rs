//! # MacroError - Trait Implementations
//!
//! This module contains trait implementations for `MacroError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MacroError;
use std::fmt;

impl std::fmt::Display for MacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroError::PatternMismatch(s) => write!(f, "pattern mismatch: {}", s),
            MacroError::DepthExceeded => write!(f, "macro expansion depth exceeded"),
            MacroError::UndefinedMacro(s) => write!(f, "undefined macro: {}", s),
            MacroError::AmbiguousMatch(s) => write!(f, "ambiguous match: {}", s),
            MacroError::HygieneViolation(s) => write!(f, "hygiene violation: {}", s),
            MacroError::ExpansionError(s) => write!(f, "expansion error: {}", s),
        }
    }
}
