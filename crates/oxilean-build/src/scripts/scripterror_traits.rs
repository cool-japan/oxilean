//! # ScriptError - Trait Implementations
//!
//! This module contains trait implementations for `ScriptError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ScriptError;
use std::fmt;

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(path) => write!(f, "script not found: {}", path.display()),
            Self::ExecutionFailed {
                name,
                exit_code,
                stderr,
            } => {
                write!(f, "script '{}' failed", name)?;
                if let Some(code) = exit_code {
                    write!(f, " (exit code {})", code)?;
                }
                if !stderr.is_empty() {
                    write!(f, ": {}", stderr)?;
                }
                Ok(())
            }
            Self::Timeout { name, timeout } => {
                write!(
                    f,
                    "script '{}' timed out after {:.1}s",
                    name,
                    timeout.as_secs_f64()
                )
            }
            Self::InvalidSource(msg) => write!(f, "invalid script source: {}", msg),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}
