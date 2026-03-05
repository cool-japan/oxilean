//! # SecuritySeverity - Trait Implementations
//!
//! This module contains trait implementations for `SecuritySeverity`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::SecuritySeverity;

impl std::fmt::Display for SecuritySeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecuritySeverity::Critical => write!(f, "critical"),
            SecuritySeverity::High => write!(f, "high"),
            SecuritySeverity::Medium => write!(f, "medium"),
            SecuritySeverity::Low => write!(f, "low"),
            SecuritySeverity::Info => write!(f, "info"),
        }
    }
}

