//! # MetaVarPriority - Trait Implementations
//!
//! This module contains trait implementations for `MetaVarPriority`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::MetaVarPriority;
use std::fmt;

impl Default for MetaVarPriority {
    fn default() -> Self {
        MetaVarPriority::Normal
    }
}

impl std::fmt::Display for MetaVarPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaVarPriority::Low => write!(f, "low"),
            MetaVarPriority::Normal => write!(f, "normal"),
            MetaVarPriority::High => write!(f, "high"),
            MetaVarPriority::Immediate => write!(f, "immediate"),
        }
    }
}
