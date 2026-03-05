//! # FfiErrorCode - Trait Implementations
//!
//! This module contains trait implementations for `FfiErrorCode`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::types::FfiErrorCode;
use std::fmt;

impl std::fmt::Display for FfiErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}): {}", self.name, self.value, self.description)
    }
}
