//! # WGSLType - Trait Implementations
//!
//! This module contains trait implementations for `WGSLType`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WGSLType;
use std::fmt;

impl fmt::Display for WGSLType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.keyword())
    }
}
