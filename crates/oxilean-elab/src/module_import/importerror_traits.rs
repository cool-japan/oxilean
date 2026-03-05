//! # ImportError - Trait Implementations
//!
//! This module contains trait implementations for `ImportError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ImportError;
use std::fmt;

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::ModuleNotFound(path) => write!(f, "module not found: {}", path),
            ImportError::NameNotFound { name, module } => {
                write!(f, "name '{}' not found in module {}", name, module)
            }
            ImportError::PrivateName { name, module } => {
                write!(f, "name '{}' is private in module {}", name, module)
            }
            ImportError::CircularImport { cycle } => {
                let cycle_str: Vec<String> = cycle.iter().map(|p| format!("{}", p)).collect();
                write!(f, "circular import: {}", cycle_str.join(" -> "))
            }
            ImportError::AmbiguousImport { name, candidates } => {
                let cands: Vec<String> = candidates.iter().map(|p| format!("{}", p)).collect();
                write!(
                    f,
                    "ambiguous import '{}': found in {}",
                    name,
                    cands.join(", ")
                )
            }
        }
    }
}
