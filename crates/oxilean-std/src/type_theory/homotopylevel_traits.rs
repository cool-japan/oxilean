//! # HomotopyLevel - Trait Implementations
//!
//! This module contains trait implementations for `HomotopyLevel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::HomotopyLevel;
use std::fmt;

impl std::fmt::Display for HomotopyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomotopyLevel::Contractible => write!(f, "Contractible (h-level -2)"),
            HomotopyLevel::Proposition => write!(f, "Proposition (h-level -1)"),
            HomotopyLevel::Set => write!(f, "Set (h-level 0)"),
            HomotopyLevel::Groupoid => write!(f, "Groupoid (h-level 1)"),
            HomotopyLevel::TwoGroupoid => write!(f, "2-Groupoid (h-level 2)"),
            HomotopyLevel::N(n) => write!(f, "{n}-Groupoid (h-level {n})"),
        }
    }
}
