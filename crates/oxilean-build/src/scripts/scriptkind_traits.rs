//! # ScriptKind - Trait Implementations
//!
//! This module contains trait implementations for `ScriptKind`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScriptKind;
use std::fmt;

impl fmt::Display for ScriptKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PreBuild => write!(f, "pre-build"),
            Self::PostBuild => write!(f, "post-build"),
            Self::CodeGen => write!(f, "codegen"),
            Self::Custom(name) => write!(f, "custom:{}", name),
            Self::PreTest => write!(f, "pre-test"),
            Self::PostTest => write!(f, "post-test"),
            Self::PrePublish => write!(f, "pre-publish"),
            Self::PostPublish => write!(f, "post-publish"),
            Self::Clean => write!(f, "clean"),
        }
    }
}
