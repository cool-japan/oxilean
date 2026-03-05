//! # ParseErrorContext - Trait Implementations
//!
//! This module contains trait implementations for `ParseErrorContext`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ParseErrorContext;

impl std::fmt::Display for ParseErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(decl) = &self.decl_name {
            write!(f, "in decl '{}': ", decl)?;
        }
        if let Some(phase) = &self.phase {
            write!(f, "[{}] ", phase)?;
        }
        write!(f, "{}", self.error.message())
    }
}
