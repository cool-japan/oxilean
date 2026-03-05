//! # OptLevel - Trait Implementations
//!
//! This module contains trait implementations for `OptLevel`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OptLevel;
use std::fmt;

impl fmt::Display for OptLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptLevel::Debug => write!(f, "debug"),
            OptLevel::Release => write!(f, "release"),
            OptLevel::Size => write!(f, "size"),
        }
    }
}
