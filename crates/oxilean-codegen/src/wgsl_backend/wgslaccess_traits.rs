//! # WGSLAccess - Trait Implementations
//!
//! This module contains trait implementations for `WGSLAccess`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::WGSLAccess;
use std::fmt;

impl fmt::Display for WGSLAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WGSLAccess::Read => write!(f, "read"),
            WGSLAccess::Write => write!(f, "write"),
            WGSLAccess::ReadWrite => write!(f, "read_write"),
        }
    }
}
