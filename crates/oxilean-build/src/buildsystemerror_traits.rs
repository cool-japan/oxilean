//! # BuildSystemError - Trait Implementations
//!
//! This module contains trait implementations for `BuildSystemError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildSystemError;
use std::fmt;

impl std::fmt::Display for BuildSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildSystemError::InvalidConfig(msg) => write!(f, "InvalidConfig: {}", msg),
            BuildSystemError::SourceNotFound(p) => write!(f, "SourceNotFound: {:?}", p),
            BuildSystemError::DependencyCycle(cycle) => {
                write!(f, "DependencyCycle: {}", cycle.join(" -> "))
            }
            BuildSystemError::CompilationFailed { target, reason } => {
                write!(f, "CompilationFailed[{}]: {}", target, reason)
            }
            BuildSystemError::Io(msg) => write!(f, "IoError: {}", msg),
            BuildSystemError::Plugin { plugin, message } => {
                write!(f, "PluginError[{}]: {}", plugin, message)
            }
        }
    }
}
