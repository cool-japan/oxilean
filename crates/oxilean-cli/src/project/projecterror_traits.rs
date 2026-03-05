//! # ProjectError - Trait Implementations
//!
//! This module contains trait implementations for `ProjectError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//! - `Error`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProjectError;
use std::fmt;

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectError::ParseError { line, message } => {
                write!(f, "parse error at line {}: {}", line, message)
            }
            ProjectError::InvalidConfig(msg) => write!(f, "invalid config: {}", msg),
            ProjectError::NotFound(msg) => write!(f, "not found: {}", msg),
            ProjectError::IoError(msg) => write!(f, "IO error: {}", msg),
            ProjectError::CyclicDependency(modules) => {
                write!(f, "cyclic dependency: {}", modules.join(" -> "))
            }
            ProjectError::DependencyNotFound(name) => {
                write!(f, "dependency not found: {}", name)
            }
            ProjectError::VersionNotFound { name, version } => {
                write!(f, "version {} not found for {}", version, name)
            }
            ProjectError::BuildFailed(msg) => write!(f, "build failed: {}", msg),
        }
    }
}

impl std::error::Error for ProjectError {}
