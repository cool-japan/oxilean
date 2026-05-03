//! # BuildNotification - Trait Implementations
//!
//! This module contains trait implementations for `BuildNotification`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::BuildNotification;
use std::fmt;

impl std::fmt::Display for BuildNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.label())
    }
}
