//! # DiagnosticStateMachine - Trait Implementations
//!
//! This module contains trait implementations for `DiagnosticStateMachine`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::DiagnosticStateMachine;
use std::fmt;

impl Default for DiagnosticStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
