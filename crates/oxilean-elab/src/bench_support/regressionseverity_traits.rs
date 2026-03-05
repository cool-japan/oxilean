//! # RegressionSeverity - Trait Implementations
//!
//! This module contains trait implementations for `RegressionSeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RegressionSeverity;
use std::fmt;

impl std::fmt::Display for RegressionSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegressionSeverity::Improvement => write!(f, "improvement"),
            RegressionSeverity::Minor => write!(f, "minor regression"),
            RegressionSeverity::Moderate => write!(f, "moderate regression"),
            RegressionSeverity::Major => write!(f, "major regression"),
        }
    }
}
