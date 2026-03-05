//! # FixConfidence - Trait Implementations
//!
//! This module contains trait implementations for `FixConfidence`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FixConfidence;

impl std::fmt::Display for FixConfidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixConfidence::Certain => write!(f, "certain"),
            FixConfidence::High => write!(f, "high"),
            FixConfidence::Medium => write!(f, "medium"),
            FixConfidence::Low => write!(f, "low"),
        }
    }
}
