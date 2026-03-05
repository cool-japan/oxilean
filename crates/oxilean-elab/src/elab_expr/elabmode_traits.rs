//! # ElabMode - Trait Implementations
//!
//! This module contains trait implementations for `ElabMode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabMode;
use std::fmt;

impl std::fmt::Display for ElabMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElabMode::Synth => write!(f, "synth"),
            ElabMode::Check => write!(f, "check"),
        }
    }
}
