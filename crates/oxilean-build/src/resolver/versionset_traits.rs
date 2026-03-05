//! # VersionSet - Trait Implementations
//!
//! This module contains trait implementations for `VersionSet`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::VersionSet;
use std::fmt;

impl fmt::Display for VersionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ranges.is_empty() {
            write!(f, "(empty)")
        } else {
            for (i, r) in self.ranges.iter().enumerate() {
                if i > 0 {
                    write!(f, " || ")?;
                }
                write!(f, "{}", r)?;
            }
            Ok(())
        }
    }
}
