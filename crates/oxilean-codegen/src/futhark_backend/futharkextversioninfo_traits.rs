//! # FutharkExtVersionInfo - Trait Implementations
//!
//! This module contains trait implementations for `FutharkExtVersionInfo`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FutharkExtVersionInfo;
use std::fmt;

impl std::fmt::Display for FutharkExtVersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(rev) = &self.git_rev {
            write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, rev)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}
