//! # ExecutorError - Trait Implementations
//!
//! This module contains trait implementations for `ExecutorError`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ExecutorError;
use std::fmt;

impl fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CyclicDependency => write!(f, "cyclic dependency in build DAG"),
            Self::EmptyDag => write!(f, "empty build DAG"),
            Self::StepFailed { step_id, error } => {
                write!(f, "build step {} failed: {}", step_id, error)
            }
            Self::MultipleFailures(failures) => {
                write!(f, "{} build steps failed:", failures.len())?;
                for (id, err) in failures {
                    write!(f, "\n  {}: {}", id, err)?;
                }
                Ok(())
            }
            Self::InvalidConfig(msg) => write!(f, "invalid config: {}", msg),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::Cancelled => write!(f, "build cancelled"),
        }
    }
}
