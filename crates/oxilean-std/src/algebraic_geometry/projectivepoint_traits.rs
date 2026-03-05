//! # ProjectivePoint - Trait Implementations
//!
//! This module contains trait implementations for `ProjectivePoint`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ProjectivePoint;
use std::fmt;

impl std::fmt::Display for ProjectivePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self.coords.iter().map(|c| c.to_string()).collect();
        write!(f, "[{}]", parts.join(" : "))
    }
}
