//! # RegAllocDiagMsg - Trait Implementations
//!
//! This module contains trait implementations for `RegAllocDiagMsg`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::RegAllocDiagMsg;
use std::fmt;

impl std::fmt::Display for RegAllocDiagMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.pass, self.message)
    }
}
