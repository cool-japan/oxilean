//! # OxileanVersion - Trait Implementations
//!
//! This module contains trait implementations for `OxileanVersion`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::OxileanVersion;
use std::fmt;

impl std::fmt::Display for OxileanVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.pre {
            Some(pre) => {
                write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, pre)
            }
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}
