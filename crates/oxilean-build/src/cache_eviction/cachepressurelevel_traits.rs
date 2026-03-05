//! # CachePressureLevel - Trait Implementations
//!
//! This module contains trait implementations for `CachePressureLevel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::CachePressureLevel;

impl std::fmt::Display for CachePressureLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CachePressureLevel::Low => write!(f, "low"),
            CachePressureLevel::Medium => write!(f, "medium"),
            CachePressureLevel::High => write!(f, "high"),
            CachePressureLevel::Critical => write!(f, "critical"),
        }
    }
}
