//! # ModRefResult - Trait Implementations
//!
//! This module contains trait implementations for `ModRefResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ModRefResult;
use std::fmt;

impl std::fmt::Display for ModRefResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModRefResult::NoModRef => write!(f, "NoModRef"),
            ModRefResult::Ref => write!(f, "Ref"),
            ModRefResult::Mod => write!(f, "Mod"),
            ModRefResult::ModRef => write!(f, "ModRef"),
        }
    }
}
