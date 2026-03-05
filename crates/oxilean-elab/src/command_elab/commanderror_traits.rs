//! # CommandError - Trait Implementations
//!
//! This module contains trait implementations for `CommandError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CommandError;
use std::fmt;

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::SectionMismatch { expected, got } => {
                write!(
                    f,
                    "section name mismatch: expected '{}', got '{}'",
                    expected, got
                )
            }
            CommandError::NoOpenSection => write!(f, "no section is currently open"),
            CommandError::NamespaceNotFound(ns) => {
                write!(f, "namespace '{}' not found", ns)
            }
            CommandError::DuplicateVariable(name) => {
                write!(f, "duplicate variable '{}'", name)
            }
            CommandError::DuplicateUniverse(name) => {
                write!(f, "duplicate universe variable '{}'", name)
            }
            CommandError::UnknownOption(opt) => write!(f, "unknown option '{}'", opt),
            CommandError::InvalidOptionValue { option, value } => {
                write!(f, "invalid value '{}' for option '{}'", value, option)
            }
            CommandError::NameNotFound(name) => write!(f, "name '{}' not found", name),
            CommandError::ElabError(msg) => write!(f, "elaboration error: {}", msg),
        }
    }
}
