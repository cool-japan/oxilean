//! # BuildPhase - Trait Implementations
//!
//! This module contains trait implementations for `BuildPhase`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildPhase;
use std::fmt;

impl std::fmt::Display for BuildPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildPhase::Parse => write!(f, "parse"),
            BuildPhase::TypeCheck => write!(f, "type-check"),
            BuildPhase::Codegen => write!(f, "codegen"),
            BuildPhase::Link => write!(f, "link"),
            BuildPhase::Package => write!(f, "package"),
        }
    }
}
