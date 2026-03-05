//! # ElabTrace - Trait Implementations
//!
//! This module contains trait implementations for `ElabTrace`.
//!
//! ## Implemented Traits
//!
//! - `Display`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::ElabTrace;
use std::fmt;

impl std::fmt::Display for ElabTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, e) in self.entries.iter().enumerate() {
            writeln!(f, "{:>4}: {}", i, e)?;
        }
        Ok(())
    }
}
