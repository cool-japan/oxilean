//! # ElabMessage - Trait Implementations
//!
//! This module contains trait implementations for `ElabMessage`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabMessage;
use std::fmt;

impl std::fmt::Display for ElabMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_diagnostic())
    }
}
