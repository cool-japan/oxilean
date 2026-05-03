//! # LintOutputFormat - Trait Implementations
//!
//! This module contains trait implementations for `LintOutputFormat`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::LintOutputFormat;
use std::fmt;

impl std::fmt::Display for LintOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LintOutputFormat::Text => write!(f, "text"),
            LintOutputFormat::Json => write!(f, "json"),
            LintOutputFormat::GitHubActions => write!(f, "github-actions"),
            LintOutputFormat::Count => write!(f, "count"),
        }
    }
}
