//! # OrdResult - Trait Implementations
//!
//! This module contains trait implementations for `OrdResult`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OrdResult;
use std::fmt;

impl std::fmt::Display for OrdResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrdResult::Less => write!(f, "lt"),
            OrdResult::Equal => write!(f, "eq"),
            OrdResult::Greater => write!(f, "gt"),
        }
    }
}
