//! # CompilePhaseExt - Trait Implementations
//!
//! This module contains trait implementations for `CompilePhaseExt`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CompilePhaseExt;
use std::fmt;

impl std::fmt::Display for CompilePhaseExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilePhaseExt::Lex => write!(f, "lex"),
            CompilePhaseExt::Parse => write!(f, "parse"),
            CompilePhaseExt::Elaborate => write!(f, "elaborate"),
            CompilePhaseExt::Tactic => write!(f, "tactic"),
            CompilePhaseExt::CodeGen => write!(f, "codegen"),
        }
    }
}
