//! # ChernCharacterComputer - Trait Implementations
//!
//! This module contains trait implementations for `ChernCharacterComputer`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ChernCharacterComputer;
use std::fmt;

impl std::fmt::Display for ChernCharacterComputer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}
