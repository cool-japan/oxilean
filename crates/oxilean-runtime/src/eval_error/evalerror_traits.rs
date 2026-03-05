//! # EvalError - Trait Implementations
//!
//! This module contains trait implementations for `EvalError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::EvalError;
use std::fmt;

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.kind)?;
        if let Some(span) = &self.span {
            write!(f, "\n  --> {}", span)?;
        }
        if !self.frames.is_empty() {
            write!(f, "\ncall stack (innermost first):")?;
            for frame in &self.frames {
                write!(f, "\n{}", frame)?;
            }
        }
        if let Some(note) = &self.note {
            write!(f, "\nnote: {}", note)?;
        }
        for hint in &self.hints {
            write!(f, "\nhint: {}", hint)?;
        }
        Ok(())
    }
}

impl std::error::Error for EvalError {}
